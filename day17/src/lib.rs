use anyhow::Context;
use itertools::Itertools;
use nom::{
    character::complete::digit1,
    combinator::{opt, recognize},
    sequence::{preceded, separated_pair, tuple},
    Parser,
};
use nom_supreme::{error::ErrorTree, parse_from_str, tag::complete::tag};

fn number<'i>() -> impl Parser<&'i str, i32, ErrorTree<&'i str>> {
    parse_from_str(recognize(tuple((opt(tag("-")), digit1))))
}

fn range<'i>() -> impl Parser<&'i str, (i32, i32), ErrorTree<&'i str>> {
    separated_pair(number(), tag(".."), number())
}

fn puzzle<'i>() -> impl Parser<&'i str, ((i32, i32), (i32, i32)), ErrorTree<&'i str>> {
    preceded(
        tag("target area: x="),
        separated_pair(range(), tag(", y="), range()),
    )
}

fn challenge(input: &str) -> anyhow::Result<impl Iterator<Item = i32>> {
    let ((xmin, xmax), (ymin, ymax)) = common::parse(input, puzzle())?;

    Ok((1..=xmax)
        .cartesian_product(ymin..=-ymin)
        .filter_map(move |velocity| {
            let mut max_height = 0;
            for (x, y) in (0..).scan(((0, 0), velocity), |((x, y), (dx, dy)), _| {
                *x += *dx;
                *y += *dy;
                *dx = 0.max(*dx - 1);
                *dy -= 1;
                Some((*x, *y))
            }) {
                max_height = max_height.max(y);
                if (x == 0 && x < xmin) || x > xmax || y < ymin {
                    return None;
                } else if x >= xmin && x <= xmax && y >= ymin && y <= ymax {
                    return Some(max_height);
                }
            }
            None
        }))
}

pub fn challenge1(input: &str) -> anyhow::Result<i32> {
    challenge(input)?.max().context("Nothing went in")
}

pub fn challenge2(input: &str) -> anyhow::Result<usize> {
    Ok(challenge(input)?.count())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_challenge1() -> anyhow::Result<()> {
        let expected = [
            // Add tests
            ("target area: x=20..30, y=-10..-5", 45),
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
            ("target area: x=20..30, y=-10..-5", 112),
        ];
        for (input, output) in expected {
            assert_eq!(crate::challenge2(input)?, output, "For input {}", input)
        }
        Ok(())
    }
}
