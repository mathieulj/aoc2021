use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use anyhow::Context;
use nom::{
    character::complete::{anychar, newline},
    combinator::map_opt,
    multi::{many1, separated_list1},
    Parser,
};
use nom_supreme::error::ErrorTree;

fn line<'i>() -> impl Parser<&'i str, Vec<u32>, ErrorTree<&'i str>> {
    many1(map_opt(anychar, |c| c.to_digit(10)))
}

pub fn challenge(
    mut risk_at: impl FnMut((usize, usize)) -> Option<u32>,
    goal: (usize, usize),
) -> anyhow::Result<u32> {
    let mut visited = HashSet::new();
    let mut lowest_risk = BinaryHeap::new();
    lowest_risk.extend([
        Reverse((risk_at((0, 1)).context("Missing map")?, (0, 1))),
        Reverse((risk_at((1, 0)).context("Missing map")?, (1, 0))),
    ]);

    while let Some(Reverse((risk, position @ (x, y)))) = lowest_risk.pop() {
        if position == goal {
            return Ok(risk);
        }

        lowest_risk.extend(
            [
                (x != 0).then(|| (x - 1, y)),
                (y != 0).then(|| (x, y - 1)),
                Some((x + 1, y)),
                Some((x, y + 1)),
            ]
            .into_iter()
            .flatten()
            .filter(|position| visited.insert(*position))
            .filter_map(|position| Some((risk + risk_at(position)?, position)))
            .map(Reverse),
        );
    }

    unreachable!()
}

pub fn challenge1(input: &str) -> anyhow::Result<u32> {
    let map = common::parse(input, separated_list1(newline, line()))?;
    let width = map.len();
    let height = map.first().context("Empty")?.len();

    challenge(
        move |(x, y)| map.get(x).and_then(|r| r.get(y)).copied(),
        (width - 1, height - 1),
    )
}

pub fn challenge2(input: &str) -> anyhow::Result<u32> {
    let map = common::parse(input, separated_list1(newline, line()))?;
    let width = map.len();
    let height = map.first().context("Empty")?.len();
    let goal = (width * 5 - 1, height * 5 - 1);

    challenge(
        move |(x, y)| {
            let row = map.get(x % map.len())?;
            let risk = row.get(y % row.len())?;
            let i = (x / map.len()) as u32;
            let j = (y / row.len()) as u32;

            (i < 5 && j < 5).then(|| ((*risk + i + j - 1) % 9) + 1)
        },
        goal,
    )
}

#[cfg(test)]
mod tests {

    const EXAMPLE: &str = r#"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"#;

    #[test]
    fn test_challenge1() -> anyhow::Result<()> {
        let expected = [
            // Add tests
            (EXAMPLE, 40),
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
            (EXAMPLE, 315),
        ];
        for (input, output) in expected {
            assert_eq!(crate::challenge2(input)?, output, "For input {}", input)
        }
        Ok(())
    }
}
