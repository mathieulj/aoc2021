use itertools::Itertools;

pub fn challenge1(input: &str) -> anyhow::Result<usize> {
    let depths: Vec<u32> = input.lines().map(str::parse).try_collect()?;
    Ok(depths
        .into_iter()
        .tuple_windows()
        .filter(|(a, b)| a < b)
        .count())
}

pub fn challenge2(input: &str) -> anyhow::Result<usize> {
    let depths: Vec<u32> = input.lines().map(str::parse).try_collect()?;
    Ok(depths
        .into_iter()
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .filter(|(a, b)| a < b)
        .count())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_challenge1() -> anyhow::Result<()> {
        let expected = [
            // Add tests
            (
                r#"199
200
208
210
200
207
240
269
260
263"#,
                7,
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
                r#"199
200
208
210
200
207
240
269
260
263"#,
                5,
            ),
        ];
        for (input, output) in expected {
            assert_eq!(crate::challenge2(input)?, output, "For input {}", input)
        }
        Ok(())
    }
}
