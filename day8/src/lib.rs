use std::collections::HashMap;

use anyhow::{bail, Context};
use nom::branch::alt;
use nom::character::complete::{char, newline, space1};
use nom::multi::{fold_many1, separated_list1};
use nom::Parser;
use nom::{bytes::complete::tag, sequence::separated_pair};
use nom_supreme::error::ErrorTree;
use nom_supreme::ParserExt;

fn segment<'i>() -> impl Parser<&'i str, u8, ErrorTree<&'i str>> {
    alt((
        char('a').value(0x01),
        char('b').value(0x02),
        char('c').value(0x04),
        char('d').value(0x08),
        char('e').value(0x10),
        char('f').value(0x20),
        char('g').value(0x40),
    ))
}

fn digit<'i>() -> impl Parser<&'i str, u8, ErrorTree<&'i str>> {
    fold_many1(segment(), || 0x00, |acc, digit| acc | digit)
}

fn line<'i>() -> impl Parser<&'i str, (Vec<u8>, Vec<u8>), ErrorTree<&'i str>> {
    separated_pair(
        separated_list1(space1, digit()),
        tag(" | "),
        separated_list1(space1, digit()),
    )
}

pub fn challenge1(input: &str) -> anyhow::Result<usize> {
    let lines = common::parse(input, separated_list1(newline, line()))?;

    Ok(lines
        .into_iter()
        .flat_map(|(_, output)| output)
        .filter(|digit| matches!(digit.count_ones(), 2 | 4 | 3 | 7))
        .count())
}

fn build_dictionary(mut options: Vec<u8>) -> anyhow::Result<HashMap<u8, u8>> {
    options.sort_by_key(|f| f.count_ones());

    let (trivial, mut complex) = options
        .into_iter()
        .partition::<Vec<u8>, _>(|digit| matches!(digit.count_ones(), 2 | 4 | 3 | 7));

    let [one, seven, four, eight]: [u8; 4] = trivial
        .try_into()
        .or_else(|v| bail!("Not exactly four trivials {:?}", v))?;

    complex.sort_by(|a, b| {
        a.count_ones().cmp(&b.count_ones()).then_with(|| {
            ((a & four) ^ one)
                .count_ones()
                .cmp(&((b & four) ^ one).count_ones())
        })
    });

    let [three, two, five, zero, nine, six]: [u8; 6] = complex
        .try_into()
        .or_else(|v| bail!("Not exactly six complex {:?}", v))?;

    Ok([
        (zero, 0),
        (one, 1),
        (two, 2),
        (three, 3),
        (four, 4),
        (five, 5),
        (six, 6),
        (seven, 7),
        (eight, 8),
        (nine, 9),
    ]
    .into_iter()
    .collect())
}

pub fn challenge2(input: &str) -> anyhow::Result<u64> {
    let lines = common::parse(input, separated_list1(newline, line()))?;

    lines.into_iter().try_fold(0, |sum, (variants, output)| {
        let map = build_dictionary(variants)?;

        let line_output = output
            .iter()
            .try_fold(0, |acc, value| -> anyhow::Result<u64> {
                Ok(10 * acc + u64::from(*map.get(value).context("Missing possibility")?))
            })?;

        Ok(sum + line_output)
    })
}

#[cfg(test)]
mod tests {
    const EXAMPLE: &str = r#"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"#;

    #[test]
    fn test_challenge1() -> anyhow::Result<()> {
        let expected = [
            // Add tests
            (EXAMPLE, 26),
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
            ("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf", 5353),
            (EXAMPLE, 61229),
        ];
        for (input, output) in expected {
            assert_eq!(crate::challenge2(input)?, output, "For input {}", input)
        }
        Ok(())
    }
}
