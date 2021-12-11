use anyhow::Context;
use itertools::Itertools;

fn matching(left: char, right: char) -> bool {
    matches!(
        (left, right),
        ('(', ')') | ('[', ']') | ('{', '}') | ('<', '>')
    )
}

fn is_opening(left: char) -> bool {
    matches!(left, '(' | '[' | '{' | '<')
}

fn score1(right: char) -> u64 {
    match right {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn score2(left: char) -> u64 {
    match left {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => 0,
    }
}

pub fn challenge1(input: &str) -> anyhow::Result<u64> {
    Ok(input
        .lines()
        .map(|line| {
            let mut stack = Vec::with_capacity(line.len());
            for c in line.chars() {
                if is_opening(c) {
                    stack.push(c)
                } else if !matching(stack.pop().unwrap_or_default(), c) {
                    return score1(c);
                }
            }
            return 0;
        })
        .sum())
}

pub fn challenge2(input: &str) -> anyhow::Result<u64> {
    let mut scores = input
        .lines()
        .filter_map(|line| {
            let mut stack = Vec::with_capacity(line.len());
            for c in line.chars() {
                if is_opening(c) {
                    stack.push(c);
                } else if !matching(stack.pop()?, c) {
                    return None;
                }
            }

            stack
                .into_iter()
                .rev()
                .map(score2)
                .reduce(|acc, v| acc * 5 + v)
        })
        .collect_vec();

    scores.sort();
    scores.get(scores.len() / 2).copied().context("Not scores")
}

#[cfg(test)]
mod tests {
    const EXAMPLE: &str = r#"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"#;

    #[test]
    fn test_challenge1() -> anyhow::Result<()> {
        let expected = [
            // Add tests
            (EXAMPLE, 26397),
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
            (EXAMPLE, 288957),
        ];
        for (input, output) in expected {
            assert_eq!(crate::challenge2(input)?, output, "For input {}", input)
        }
        Ok(())
    }
}
