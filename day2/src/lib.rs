use nom::{
    character::complete::{digit1, newline, space1},
    multi::separated_list1,
    sequence::separated_pair,
};
use nom_supreme::parse_from_str;

common::nom_enum!(
    enum Movement {
        Forward = "forward",
        Down = "down",
        Up = "up",
    }
);

fn movements(input: &str) -> anyhow::Result<Vec<(Movement, i64)>> {
    let movement = separated_pair(Movement::parser(), space1, parse_from_str(digit1));
    Ok(common::parse(input, separated_list1(newline, movement))?)
}

pub fn challenge1(input: &str) -> anyhow::Result<i64> {
    let actions = movements(input)?;

    let (final_depth, final_range) = actions.into_iter().fold(
        (0, 0),
        |(depth, range), (movement, distance)| match movement {
            Movement::Forward => (depth, range + distance),
            Movement::Down => (depth + distance, range),
            Movement::Up => (depth - distance, range),
        },
    );

    Ok(final_depth * final_range)
}

pub fn challenge2(input: &str) -> anyhow::Result<i64> {
    let actions = movements(input)?;

    let (final_depth, final_range, _) =
        actions
            .into_iter()
            .fold(
                (0, 0, 0),
                |(depth, range, aim), (movement, distance)| match movement {
                    Movement::Forward => (depth + distance * aim, range + distance, aim),
                    Movement::Down => (depth, range, aim + distance),
                    Movement::Up => (depth, range, aim - distance),
                },
            );

    Ok(final_depth * final_range)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_challenge1() -> anyhow::Result<()> {
        let expected = [
            // Add tests
            (
                r#"forward 5
down 5
forward 8
up 3
down 8
forward 2"#,
                150,
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
                r#"forward 5
down 5
forward 8
up 3
down 8
forward 2"#,
                900,
            ),
        ];
        for (input, output) in expected {
            assert_eq!(crate::challenge2(input)?, output, "For input {}", input)
        }
        Ok(())
    }
}
