use anyhow::Context;
use itertools::Itertools;

pub fn bit_counts<'i>(input: impl Iterator<Item = &'i str>) -> Option<Vec<i32>> {
    input
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '1' => 1,
                    _ => -1,
                })
                .collect_vec()
        })
        .reduce(|acc, line| {
            acc.into_iter()
                .zip(line.into_iter())
                .map(|(acc, bit)| acc + bit)
                .collect_vec()
        })
}

pub fn challenge1(input: &str) -> anyhow::Result<i64> {
    let bit_counts = bit_counts(input.lines()).context("No lines")?;

    let gamma = bit_counts.iter().fold(
        0,
        |acc, bit| if *bit >= 0 { acc << 1 | 1 } else { acc << 1 },
    );

    let epsilon = bit_counts
        .iter()
        .fold(0, |acc, bit| if *bit < 0 { acc << 1 | 1 } else { acc << 1 });

    Ok(gamma * epsilon)
}

pub fn challenge2(input: &str) -> anyhow::Result<i64> {
    // let co2_criteria: String = bit_counts
    //     .iter()
    //     .map(|i| if *i < 0 { '1' } else { '0' })
    //     .collect();

    let mut oxygen = input.lines().collect_vec();
    for index in 0..oxygen.first().context("No lines")?.len() {
        if oxygen.len() <= 1 {
            break;
        }
        let oxygen_criteria = bit_counts(oxygen.iter().copied())
            .context("No lines")?
            .iter()
            .map(|i| if *i >= 0 { '1' } else { '0' })
            .skip(index)
            .next()
            .context("One line shorted than the rest")?;

        oxygen.retain(|f| {
            f.chars()
                .skip(index)
                .next()
                .map(|c| c == oxygen_criteria)
                .unwrap_or_default()
        })
    }

    let oxygen_rating = oxygen
        .into_iter()
        .exactly_one()
        .ok()
        .context("There was not exactly one match")?;

    let mut co2 = input.lines().collect_vec();
    for index in 0..co2.first().context("No lines")?.len() {
        if co2.len() <= 1 {
            break;
        }
        let oxygen_criteria = bit_counts(co2.iter().copied())
            .context("No lines")?
            .iter()
            .map(|i| if *i < 0 { '1' } else { '0' })
            .skip(index)
            .next()
            .context("One line shorted than the rest")?;

        co2.retain(|f| {
            f.chars()
                .skip(index)
                .next()
                .map(|c| c == oxygen_criteria)
                .unwrap_or_default()
        })
    }
    let co2_rating = co2
        .into_iter()
        .exactly_one()
        .ok()
        .context("There was not exactly one match")?;

    println!("{} {}", oxygen_rating, oxygen_rating);
    Ok(i64::from_str_radix(oxygen_rating, 2)? * i64::from_str_radix(co2_rating, 2)?)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_challenge1() -> anyhow::Result<()> {
        let expected = [
            // Add tests
            (
                r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"#,
                198,
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
                r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"#,
                230,
            ),
        ];
        for (input, output) in expected {
            assert_eq!(crate::challenge2(input)?, output, "For input {}", input)
        }
        Ok(())
    }
}
