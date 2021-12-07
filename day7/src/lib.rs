use anyhow::Context;
use nom::{
    character::complete::{char, digit1},
    multi::separated_list1,
};
use nom_supreme::parse_from_str;

pub fn challenge1(input: &str) -> anyhow::Result<usize> {
    challenge(input, |distance| distance)
}

pub fn challenge2(input: &str) -> anyhow::Result<usize> {
    challenge(input, |distance| (distance + distance * distance) / 2)
}

fn challenge(input: &str, mut cost: impl FnMut(usize) -> usize) -> anyhow::Result<usize> {
    let positions: Vec<usize> =
        common::parse(input, separated_list1(char(','), parse_from_str(digit1)))?;
    let min = *positions.iter().min().context("No input")?;
    let max = *positions.iter().max().context("No input")?;

    let minimum_fuel = (min..=max)
        .map(|i| {
            positions
                .iter()
                .copied()
                .map(|j| if j > i { cost(j - i) } else { cost(i - j) })
                .sum::<usize>()
        })
        .min()
        .context("No input")?;

    Ok(minimum_fuel)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_challenge1() -> anyhow::Result<()> {
        let expected = [
            // Add tests
            ("16,1,2,0,4,2,7,1,2,14", 37),
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
            ("16,1,2,0,4,2,7,1,2,14", 168),
        ];
        for (input, output) in expected {
            assert_eq!(crate::challenge2(input)?, output, "For input {}", input)
        }
        Ok(())
    }
}
