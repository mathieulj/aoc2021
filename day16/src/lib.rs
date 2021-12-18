use anyhow::{bail, Context};
use itertools::Itertools;
use nom::bits::bits;
use nom::bits::complete::tag;
use nom::bits::complete::take;
use nom::branch::alt;
use nom::combinator::all_consuming;
use nom::combinator::map;
use nom::combinator::value;
use nom::error::context;
use nom::error::VerboseError;
use nom::multi::many0;
use nom::multi::many_till;
use nom::sequence::preceded;
use nom::sequence::terminated;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;

#[derive(Debug)]
enum Packet {
    Literal {
        value: u64,
    },
    Operator {
        operation: Operation,
        sub_packets: Vec<(u8, Packet)>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Operation {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

fn operation<'i>() -> impl Parser<(&'i [u8], usize), Operation, VerboseError<(&'i [u8], usize)>> {
    alt((
        value(Operation::Sum, tag(0, 3usize)),
        value(Operation::Product, tag(1, 3usize)),
        value(Operation::Minimum, tag(2, 3usize)),
        value(Operation::Maximum, tag(3, 3usize)),
        value(Operation::GreaterThan, tag(5, 3usize)),
        value(Operation::LessThan, tag(6, 3usize)),
        value(Operation::EqualTo, tag(7, 3usize)),
    ))
}

fn operator(
    input: (&[u8], usize),
) -> IResult<(&[u8], usize), Packet, VerboseError<(&[u8], usize)>> {
    let (r, operation) = operation().parse(input)?;
    let (r, len_id): (_, u8) = take(1usize)(r)?;
    let mut sub_packets = Vec::new();
    let mut remainder;
    if len_id == 0 {
        let (r, total_bit_len): (_, usize) = take(15usize)(r)?;

        remainder = r;
        let total = r.0.len() * 8 - r.1;
        while total - (remainder.0.len() * 8 - remainder.1) < total_bit_len {
            let (r, sub) = packet().parse(remainder)?;
            remainder = r;
            sub_packets.push(sub);
        }
    } else {
        let (r, sub_packet_count): (_, usize) = take(11usize)(r)?;

        remainder = r;
        for _ in 0..sub_packet_count {
            let (r, sub) = packet().parse(remainder)?;
            remainder = r;
            sub_packets.push(sub);
        }
    }

    Ok((
        remainder,
        Packet::Operator {
            operation,
            sub_packets,
        },
    ))
}

fn literal<'i>() -> impl Parser<(&'i [u8], usize), Packet, VerboseError<(&'i [u8], usize)>> {
    context(
        "literal",
        map(
            preceded(
                tag(4, 3usize),
                many_till(
                    preceded(tag(1, 1usize), take(4usize)),
                    preceded(tag(0, 1usize), take(4usize)),
                ),
            ),
            |(nibbles, final_nibble): (Vec<u8>, u8)| {
                let value = nibbles
                    .into_iter()
                    .chain([final_nibble])
                    .fold(0u64, |acc, nibble| (acc << 4) | u64::from(nibble));
                Packet::Literal { value }
            },
        ),
    )
}

fn packet<'i>() -> impl FnMut(
    (&'i [u8], usize),
) -> IResult<
    (&'i [u8], usize),
    (u8, Packet),
    VerboseError<(&'i [u8], usize)>,
> {
    tuple((
        take(3usize),
        alt((context("literal", literal()), context("operator", operator))),
    ))
}

fn sum_of_versions((version, packet): &(u8, Packet)) -> u64 {
    match packet {
        Packet::Literal { .. } => u64::from(*version),
        Packet::Operator { sub_packets, .. } => {
            sub_packets.into_iter().map(sum_of_versions).sum::<u64>() + u64::from(*version)
        }
    }
}

fn eval((_, packet): &(u8, Packet)) -> u64 {
    match packet {
        Packet::Literal { value } => return *value,
        Packet::Operator {
            operation,
            sub_packets,
        } => {
            let numbers = sub_packets.iter().map(eval);
            match operation {
                Operation::Sum => Some(numbers.sum()),
                Operation::Product => Some(numbers.product()),
                Operation::Minimum => numbers.reduce(|acc, value| acc.min(value)),
                Operation::Maximum => numbers.reduce(|acc, value| acc.max(value)),
                Operation::GreaterThan => numbers
                    .tuples()
                    .next()
                    .map(|(a, b)| if a > b { 1 } else { 0 }),
                Operation::LessThan => numbers
                    .tuples()
                    .next()
                    .map(|(a, b)| if a < b { 1 } else { 0 }),
                Operation::EqualTo => numbers
                    .tuples()
                    .next()
                    .map(|(a, b)| if a == b { 1 } else { 0 }),
            }
            .unwrap_or_default()
        }
    }
}

fn challenge(input: &str, tally: impl FnOnce(&(u8, Packet)) -> u64) -> anyhow::Result<u64> {
    let bytes = input
        .trim()
        .chars()
        .tuples()
        .map(|(a, b)| {
            a.to_digit(16)
                .zip(b.to_digit(16))
                .map(|(a, b)| (a << 4 | b) as u8)
                .context("Invalid hex")
        })
        .collect::<anyhow::Result<Vec<u8>>>()?;

    let message = bits(all_consuming(terminated(packet(), many0(tag(0, 1usize)))))
        .parse(bytes.as_slice())
        .map_err(|e: nom::Err<VerboseError<&[u8]>>| anyhow::format_err!("Unable to parse {}", e))?
        .1;

    Ok(tally(&message))
}

pub fn challenge1(input: &str) -> anyhow::Result<u64> {
    challenge(input, sum_of_versions)
}

pub fn challenge2(input: &str) -> anyhow::Result<u64> {
    challenge(input, eval)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_challenge1() -> anyhow::Result<()> {
        let expected = [
            // Add tests
            ("D2FE28", 6),
            ("38006F45291200", 9),
            ("EE00D40C823060", 14),
            ("8A004A801A8002F478", 16),
            ("620080001611562C8802118E34", 12),
            ("C0015000016115A2E0802F182340", 23),
            ("A0016C880162017C3686B18A3D4780", 31),
        ];
        for (input, output) in expected {
            assert_eq!(crate::challenge1(input)?, output, "For input {}", input)
        }
        Ok(())
    }

    #[test]
    fn test_challenge2() -> anyhow::Result<()> {
        let expected = [
            // Add tests
            ("C200B40A82", 3),
            ("04005AC33890", 54),
            ("880086C3E88112", 7),
            ("CE00C43D881120", 9),
            ("D8005AC2A8F0", 1),
            ("F600BC2D8F", 0),
            ("9C005AC2F8F0", 0),
            ("9C0141080250320F1802104A08", 1),
        ];
        for (input, output) in expected {
            assert_eq!(crate::challenge2(input)?, output, "For input {}", input)
        }
        Ok(())
    }
}
