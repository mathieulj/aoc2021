use std::collections::HashMap;

use itertools::Either;
use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1, newline},
    multi::separated_list1,
    sequence::separated_pair,
    Parser,
};
use nom_supreme::{error::ErrorTree, parse_from_str};

type Point = (i64, i64);
fn point<'i>() -> impl Parser<&'i str, Point, ErrorTree<&'i str>> {
    separated_pair(parse_from_str(digit1), char(','), parse_from_str(digit1))
}

type Segment = (Point, Point);
fn segment<'i>() -> impl Parser<&'i str, Segment, ErrorTree<&'i str>> {
    separated_pair(point(), tag(" -> "), point())
}

fn range(a: i64, b: i64) -> impl Iterator<Item = i64> {
    if a <= b {
        Either::Left(a..=b)
    } else {
        Either::Right((b..=a).rev())
    }
}

fn challenge(input: &str, diagonals: bool) -> anyhow::Result<usize> {
    let segments = common::parse(input, separated_list1(newline, segment()))?;
    let hot_zones = segments.into_iter().fold(
        HashMap::<(i64, i64), usize>::new(),
        |mut acc, ((x1, y1), (x2, y2))| {
            if x1 == x2 {
                for y in range(y1, y2) {
                    *acc.entry((x1, y)).or_default() += 1;
                }
            } else if y1 == y2 {
                for x in range(x1, x2) {
                    *acc.entry((x, y1)).or_default() += 1;
                }
            } else if diagonals {
                for (x, y) in range(x1, x2).zip(range(y1, y2)) {
                    *acc.entry((x, y)).or_default() += 1;
                }
            }

            acc
        },
    );

    Ok(hot_zones
        .into_iter()
        .filter(|(_, count)| *count > 1)
        .count())
}

pub fn challenge1(input: &str) -> anyhow::Result<usize> {
    challenge(input, false)
}

pub fn challenge2(input: &str) -> anyhow::Result<usize> {
    challenge(input, true)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_challenge1() -> anyhow::Result<()> {
        let expected = [
            // Add tests
            (
                r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"#,
                5,
            ),
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
            (
                r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"#,
                12,
            ),
        ];
        for (input, output) in expected {
            assert_eq!(crate::challenge2(input)?, output, "For input {}", input)
        }
        Ok(())
    }
}
