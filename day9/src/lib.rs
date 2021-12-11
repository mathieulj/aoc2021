use std::collections::HashSet;

use anyhow::Context;
use itertools::Itertools;
use nom::{
    character::complete::{anychar, newline},
    combinator::map_opt,
    multi::{many1, separated_list1},
    Parser,
};
use nom_supreme::error::ErrorTree;

fn get<T>(map: &Vec<Vec<T>>, x: isize, y: isize) -> Option<&T> {
    let x: usize = x.try_into().ok()?;
    let y: usize = y.try_into().ok()?;

    let row = map.get(x)?;
    row.get(y)
}

fn local_minima(map: &Vec<Vec<u32>>) -> anyhow::Result<impl '_ + Iterator<Item = (isize, isize)>> {
    let height: isize = map.len().try_into()?;
    let width: isize = map
        .iter()
        .map(Vec::len)
        .max()
        .context("No rows")?
        .try_into()?;

    Ok((0..height)
        .cartesian_product(0..width)
        .filter(move |(i, j)| {
            if let Some(position_value) = get(&map, *i, *j) {
                [
                    get(&map, i - 1, *j),
                    get(&map, i + 1, *j),
                    get(&map, *i, j - 1),
                    get(&map, *i, j + 1),
                ]
                .into_iter()
                .flatten()
                .all(|value| value > position_value)
            } else {
                false
            }
        }))
}

fn map<'i>() -> impl Parser<&'i str, Vec<Vec<u32>>, ErrorTree<&'i str>> {
    separated_list1(newline, many1(map_opt(anychar, |c| c.to_digit(10))))
}

pub fn challenge1(input: &str) -> anyhow::Result<u32> {
    let map = common::parse(input, map())?;

    let sum = local_minima(&map)?
        .filter_map(|(i, j)| get(&map, i, j).map(|v| v + 1))
        .sum();

    Ok(sum)
}

fn visit((x, y): (isize, isize), map: &Vec<Vec<u32>>, visited: &mut HashSet<(isize, isize)>) {
    if !matches!(get(map, x, y), Some(9) | None) && visited.insert((x, y)) {
        for neighbor in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
            visit(neighbor, map, visited);
        }
    }
}
pub fn challenge2(input: &str) -> anyhow::Result<usize> {
    let map = common::parse(input, map())?;

    let score = local_minima(&map)?
        .map(|low_point| {
            let mut visited = HashSet::new();
            visit(low_point, &map, &mut visited);
            visited.len()
        })
        .sorted()
        .rev()
        .take(3)
        .product();

    Ok(score)
}

#[cfg(test)]
mod tests {
    const EXAMPLE: &str = r#"2199943210
3987894921
9856789892
8767896789
9899965678"#;
    #[test]
    fn test_challenge1() -> anyhow::Result<()> {
        let expected = [
            // Add tests
            (EXAMPLE, 15),
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
            (EXAMPLE, 1134),
        ];
        for (input, output) in expected {
            assert_eq!(crate::challenge2(input)?, output, "For input {}", input)
        }
        Ok(())
    }
}
