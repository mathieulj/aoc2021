use std::collections::HashMap;

use nom::{
    branch::alt,
    character::complete::{alpha1, char, newline},
    multi::separated_list1,
    sequence::separated_pair,
    Parser,
};
use nom_supreme::{error::ErrorTree, ParserExt};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Cave<'i> {
    Small(&'i str),
    Large(&'i str),
}

impl<'i> Cave<'i> {
    fn parser() -> impl Parser<&'i str, Cave<'i>, ErrorTree<&'i str>> {
        alt((
            alpha1
                .verify(|s: &&str| s.chars().all(|c| c.is_lowercase()))
                .map(|s| Cave::Small(s)),
            alpha1
                .verify(|s: &&str| s.chars().all(|c| c.is_uppercase()))
                .map(|s| Cave::Large(s)),
        ))
    }
}

fn walk<'c, F: FnMut(&HashMap<Cave<'c>, usize>, &Cave) -> bool>(
    links: &HashMap<Cave<'c>, Vec<Cave<'c>>>,
    stack: &mut HashMap<Cave<'c>, usize>,
    last: Cave<'c>,
    go: &mut F,
) -> usize {
    let mut count = 0;
    if last == Cave::Small("end") {
        return 1;
    }
    if let Some(options) = links.get(&last) {
        for next in options {
            if go(stack, next) {
                *stack.entry(*next).or_default() += 1;
                count += walk(links, stack, *next, go);
                *stack.entry(*next).or_default() -= 1;
            }
        }
    }
    count
}

fn challenge<F>(input: &str, mut go: F) -> anyhow::Result<usize>
where
    F: FnMut(&HashMap<Cave, usize>, &Cave) -> bool,
{
    let links = common::parse(
        input,
        separated_list1(
            newline,
            separated_pair(Cave::parser(), char('-'), Cave::parser()),
        ),
    )?;

    let link_map: HashMap<Cave, Vec<Cave>> =
        links.iter().fold(Default::default(), |mut acc, (a, b)| {
            acc.entry(*a).or_default().push(*b);
            acc.entry(*b).or_default().push(*a);
            acc
        });

    let mut stack = [(Cave::Small("start"), 1)].into_iter().collect();
    Ok(walk(&link_map, &mut stack, Cave::Small("start"), &mut go))
}

pub fn challenge1(input: &str) -> anyhow::Result<usize> {
    challenge(input, |previous, next| {
        matches!(next, Cave::Large(..)) || previous.get(next).copied().unwrap_or_default() == 0
    })
}

pub fn challenge2(input: &str) -> anyhow::Result<usize> {
    challenge(input, |previous, next| {
        if matches!(next, Cave::Large(..)) {
            return true;
        }
        if *next == Cave::Small("start") {
            return false;
        }

        previous
            .get(next)
            .map(|count| {
                if *count == 0 {
                    true
                } else if *count == 1 {
                    previous.iter().all(|(cave, n)| match cave {
                        Cave::Small(..) => *n <= 1,
                        Cave::Large(..) => true,
                    })
                } else {
                    false
                }
            })
            .unwrap_or(true)
    })
}

#[cfg(test)]
mod tests {

    const EXAMPLE1: &str = r#"start-A
start-b
A-c
A-b
b-d
A-end
b-end"#;

    const EXAMPLE2: &str = r#"dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc"#;

    const EXAMPLE3: &str = r#"fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW"#;

    #[test]
    fn test_challenge1() -> anyhow::Result<()> {
        let expected = [
            // Add tests
            (EXAMPLE1, 10),
            (EXAMPLE2, 19),
            (EXAMPLE3, 226),
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
            (EXAMPLE1, 36),
            (EXAMPLE2, 103),
            (EXAMPLE3, 3509),
        ];
        for (input, output) in expected {
            assert_eq!(crate::challenge2(input)?, output, "For input {}", input)
        }
        Ok(())
    }
}
