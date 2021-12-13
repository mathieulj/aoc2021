use std::collections::HashSet;

use anyhow::{bail, Context};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, newline},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    Parser,
};
use nom_supreme::{error::ErrorTree, ParserExt};

fn points<'i>() -> impl Parser<&'i str, Vec<(usize, usize)>, ErrorTree<&'i str>> {
    separated_list1(
        newline,
        separated_pair(digit1.parse_from_str(), char(','), digit1.parse_from_str()),
    )
}

fn folds<'i>() -> impl Parser<&'i str, Vec<(bool, usize)>, ErrorTree<&'i str>> {
    separated_list1(
        newline,
        preceded(
            tag("fold along "),
            separated_pair(
                alt((char('y').value(true), char('x').value(false))),
                char('='),
                digit1.parse_from_str(),
            ),
        ),
    )
}

fn fold(map: &mut HashSet<(usize, usize)>, (vertical, position): (bool, usize)) {
    // drain_filter is going to be so nice
    let (original, mirror) = map
        .drain()
        .filter(|(x, y)| if vertical { *y } else { *x } != position)
        .partition::<HashSet<_>, _>(|(x, y)| if vertical { *y } else { *x } < position);

    map.extend(original);
    map.extend(mirror.into_iter().map(|(x, y)| {
        if vertical {
            (x, position * 2 - y)
        } else {
            (position * 2 - x, y)
        }
    }));
}

pub fn challenge1(input: &str) -> anyhow::Result<usize> {
    let (points, folds) = common::parse(
        input,
        separated_pair(points().terminated(newline), newline, folds()),
    )?;

    let mut map: HashSet<(usize, usize)> = points.into_iter().collect();
    fold(&mut map, *folds.get(0).context("No folds")?);

    Ok(map.len())
}

// Answer is between 983 and 766
pub fn challenge2(input: &str) -> anyhow::Result<usize> {
    let (points, folds) = common::parse(
        input,
        separated_pair(points().terminated(newline), newline, folds()),
    )?;

    let mut map: HashSet<(usize, usize)> = points.into_iter().collect();

    for instruction in folds {
        fold(&mut map, instruction);
    }

    let (width, height) = map
        .iter()
        .copied()
        .reduce(|acc, (x, y)| (acc.0.max(x), acc.1.max(y)))
        .map(|(x, y)| (x + 1, y + 1))
        .context("not points")?;

    for j in 0..height {
        for i in 0..width {
            if map.contains(&(i, j)) {
                print!("█");
            } else {
                print!(" ");
            }
        }
        println!("");
    }

    Ok(map.len())
}

#[cfg(test)]
mod tests {
    const EXAMPLE: &str = r#"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5"#;

    const EXAMPLE2: &str = r#"10,10
6,14
4,13
10,12
9,14
6,10
6,12
2,14
3,10
9,10
0,14
0,11
3,14
8,10
1,10
0,13
4,11

fold along x=5"#;

    #[test]
    fn test_challenge1() -> anyhow::Result<()> {
        let expected = [
            // Add tests
            (EXAMPLE, 17),
            (EXAMPLE2, 16),
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
        ];
        for (input, output) in expected {
            assert_eq!(crate::challenge2(input)?, output, "For input {}", input)
        }
        Ok(())
    }
}
