use std::collections::HashMap;

use anyhow::Context;
use itertools::Itertools;
use nom::{
    character::complete::{char, digit1, newline, space0, space1},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    Parser,
};
use nom_supreme::{error::ErrorTree, parse_from_str};

fn board<'i>() -> impl Parser<&'i str, Vec<Vec<u64>>, ErrorTree<&'i str>> {
    preceded(
        newline,
        separated_list1(
            newline,
            preceded(space0, separated_list1(space1, parse_from_str(digit1))),
        ),
    )
}

fn moves_to_win(board: &Vec<Vec<u64>>, choices: &HashMap<u64, usize>) -> Option<usize> {
    let width = board.len();
    let height = board.get(0)?.len();

    if board.iter().any(|row| row.len() != height) {
        return None;
    }

    let timeline = board
        .iter()
        .map(|r| r.iter().map(|n| choices.get(n).copied()).collect_vec())
        .collect_vec();

    fn zip_max(acc: Option<usize>, cell: Option<usize>) -> Option<usize> {
        Some(acc?.max(cell?))
    }

    let best_row = timeline
        .iter()
        .filter_map(|row| row.iter().copied().reduce(zip_max).flatten())
        .min();

    let best_col = (0..height)
        .filter_map(|j| (0..width).map(|i| timeline[i][j]).reduce(zip_max).flatten())
        .min();

    [best_row, best_col]
        .into_iter()
        .fold(None, |best, value| match (best, value) {
            (None, value) => value,
            (best, None) => best,
            (Some(best), Some(value)) => Some(best.min(value)),
        })
}

fn game<C>(input: &str, select: C) -> anyhow::Result<u64>
where
    C: for<'i> FnOnce(
        Box<dyn Iterator<Item = &'i Vec<Vec<u64>>> + 'i>,
        &HashMap<u64, usize>,
    ) -> Option<(&'i Vec<Vec<u64>>, usize)>,
{
    let (numbers, boards): (Vec<u64>, Vec<Vec<Vec<u64>>>) = common::parse(
        input,
        separated_pair(
            separated_list1(char(','), parse_from_str(digit1)),
            newline,
            separated_list1(newline, board()),
        ),
    )?;

    let choices = numbers
        .iter()
        .copied()
        .enumerate()
        .map(|(i, v)| (v, i))
        .collect();

    let (winning_board, moves) = select(Box::new(boards.iter()), &choices).context("No winner")?;

    let unmarked: u64 = winning_board
        .iter()
        .flat_map(|r| r)
        .copied()
        .filter(|n| choices.get(n).filter(|v| **v <= moves).is_none())
        .sum();

    println!("{:?} {}", winning_board, unmarked);
    Ok(unmarked * numbers[moves])
}

pub fn challenge1<'i, 'b>(input: &'i str) -> anyhow::Result<u64> {
    game(input, |boards, choices| {
        boards
            .filter_map(|board| Some((board, moves_to_win(board, &choices)?)))
            .min_by_key(|(_board, count)| *count)
    })
}

pub fn challenge2<'i, 'b>(input: &'i str) -> anyhow::Result<u64> {
    game(input, |boards, choices| {
        boards
            .filter_map(|board| Some((board, moves_to_win(board, &choices)?)))
            .max_by_key(|(_board, count)| *count)
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_challenge1() -> anyhow::Result<()> {
        let expected = [
            // Add tests
            (
                r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"#,
                4512,
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
                r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"#,
                1924,
            ),
        ];
        for (input, output) in expected {
            assert_eq!(crate::challenge2(input)?, output, "For input {}", input)
        }
        Ok(())
    }
}
