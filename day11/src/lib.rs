use std::iter;

use anyhow::Context;
use itertools::Itertools;
use nom::{
    character::complete::{anychar, newline},
    combinator::map_opt,
    multi::{many1, separated_list1},
    Parser,
};
use nom_supreme::error::ErrorTree;

fn map<'i>() -> impl Parser<&'i str, Vec<Vec<u32>>, ErrorTree<&'i str>> {
    separated_list1(newline, many1(map_opt(anychar, |c| c.to_digit(10))))
}

fn get_mut(
    map: &mut Vec<Vec<u32>>,
    (i, j): (usize, usize),
    direction: Direction,
) -> Option<&mut u32> {
    let (i, j) = match direction {
        Direction::North => (i.checked_sub(1)?, j),
        Direction::NorthEast => (i.checked_sub(1)?, j + 1),
        Direction::East => (i, j + 1),
        Direction::SouthEast => (i + 1, j + 1),
        Direction::South => (i + 1, j),
        Direction::SouthWest => (i + 1, j.checked_sub(1)?),
        Direction::West => (i, j.checked_sub(1)?),
        Direction::NorthWest => (i.checked_sub(1)?, j.checked_sub(1)?),
    };

    map.get_mut(i)?.get_mut(j)
}

// 🤩 #![feature(mixed_integer_ops)] would be better than this
enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

fn simulate(mut map: Vec<Vec<u32>>) -> impl Iterator<Item = usize> {
    use Direction::*;
    iter::repeat_with(move || {
        let mut flashed = map.iter().map(|row| vec![false; row.len()]).collect_vec();
        map.iter_mut()
            .flat_map(|r| r.iter_mut())
            .for_each(|e| *e += 1);

        loop {
            let flashers =
                map.iter()
                    .zip(flashed.iter())
                    .enumerate()
                    .flat_map(|(i, (row, flashed))| {
                        row.iter().zip(flashed.iter()).enumerate().filter_map(
                            move |(j, (v, flashed))| (!*flashed && *v > 9).then(|| (i, j)),
                        )
                    })
                    .collect_vec();

            if flashers.is_empty() {
                break;
            }

            for position in flashers {
                flashed[position.0][position.1] = true;
                for direction in [
                    North, NorthEast, East, SouthEast, South, SouthWest, West, NorthWest,
                ] {
                    if let Some(v) = get_mut(&mut map, position, direction) {
                        *v += 1;
                    }
                }
            }
        }

        map.iter_mut()
            .zip(flashed.iter())
            .flat_map(|(row, flashed)| row.iter_mut().zip(flashed.iter()))
            .filter(|(_, flashed)| **flashed)
            .for_each(|(v, _)| *v = 0);

        flashed
            .into_iter()
            .flat_map(Vec::into_iter)
            .filter(|f| *f)
            .count()
    })
}

pub fn challenge1(input: &str) -> anyhow::Result<usize> {
    let map = common::parse(input, map())?;
    Ok(simulate(map).take(100).sum())
}

pub fn challenge2(input: &str) -> anyhow::Result<usize> {
    let map = common::parse(input, map())?;
    let goal = map.iter().flat_map(|i| i.iter()).count();
    simulate(map)
        .position(|c| c == goal)
        .map(|p| p + 1)
        .context("No steps to run")
}

#[cfg(test)]
mod tests {
    const EXAMPLE: &str = r#"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"#;
    #[test]
    fn test_challenge1() -> anyhow::Result<()> {
        let expected = [
            // Add tests
            (EXAMPLE, 1656),
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
            (EXAMPLE, 195),
        ];
        for (input, output) in expected {
            assert_eq!(crate::challenge2(input)?, output, "For input {}", input)
        }
        Ok(())
    }
}
