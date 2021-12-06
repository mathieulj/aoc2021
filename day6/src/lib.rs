use anyhow::Context;
use nom::{
    character::complete::{char, digit1},
    multi::separated_list1,
};
use nom_supreme::parse_from_str;

pub fn challenge1(input: &str) -> anyhow::Result<u64> {
    challenge(input, 80)
}

pub fn challenge2(input: &str) -> anyhow::Result<u64> {
    challenge(input, 256)
}

fn challenge(input: &str, days: usize) -> anyhow::Result<u64> {
    let initial_state: Vec<usize> =
        common::parse(input, separated_list1(char(','), parse_from_str(digit1)))?;

    let mut state = initial_state
        .into_iter()
        .try_fold([0; 9], |mut acc, value| {
            *acc.get_mut(value)? += 1;
            Some(acc)
        })
        .context("Some number out of bounds")?;

    for _ in 0..days {
        let carry = state[0];
        for i in 1..state.len() {
            state[i - 1] = state[i];
        }
        state[6] += carry;
        state[8] = carry;
    }

    Ok(state.into_iter().sum())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_challenge1() -> anyhow::Result<()> {
        let expected = [
            // Add tests
            ("3,4,3,1,2", 5934),
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
            ("3,4,3,1,2", 26984457539),
        ];
        for (input, output) in expected {
            assert_eq!(crate::challenge2(input)?, output, "For input {}", input)
        }
        Ok(())
    }
}
