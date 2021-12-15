use std::collections::HashMap;

use anyhow::Context;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, anychar, newline},
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    Parser,
};
use nom_supreme::{error::ErrorTree, ParserExt};

fn insertion<'i>() -> impl Parser<&'i str, ((char, char), char), ErrorTree<&'i str>> {
    separated_pair(tuple((anychar, anychar)), tag(" -> "), anychar)
}

fn puzzle<'i>() -> impl Parser<&'i str, (&'i str, Vec<((char, char), char)>), ErrorTree<&'i str>> {
    separated_pair(
        alpha1,
        newline.terminated(newline),
        separated_list1(newline, insertion()),
    )
}

fn merge(acc: &mut HashMap<char, usize>, source: impl Iterator<Item = (char, usize)>) {
    for (c, n) in source {
        *acc.entry(c).or_default() += n
    }
}

fn count(
    pair: (char, char),
    depth: usize,
    map: &HashMap<(char, char), char>,
    cache: &mut HashMap<((char, char), usize), HashMap<char, usize>>,
) -> impl Iterator<Item = (char, usize)> {
    let counts = if let Some(cached) = cache.get(&(pair, depth)) {
        cached.clone()
    } else {
        let mut counts = HashMap::default();
        if let Some(c) = map.get(&pair) {
            counts.insert(*c, 1);

            if depth > 0 {
                merge(&mut counts, count((pair.0, *c), depth - 1, map, cache));
                merge(&mut counts, count((*c, pair.1), depth - 1, map, cache));
            }
        }

        cache.insert((pair, depth), counts.clone());
        counts
    };

    counts.into_iter()
}

pub fn challenge(input: &str, rounds: usize) -> anyhow::Result<usize> {
    let (template, inserts) = common::parse(input, puzzle())?;
    let inserts: HashMap<(char, char), char> = inserts.iter().copied().collect();

    let mut result = template.chars().counts();
    let mut cache = Default::default();
    for pair in template.chars().tuple_windows() {
        merge(&mut result, count(pair, rounds - 1, &inserts, &mut cache));
    }

    result
        .values()
        .minmax()
        .into_option()
        .map(|(min, max)| max - min)
        .context("No counts")
}

pub fn challenge1(input: &str) -> anyhow::Result<usize> {
    challenge(input, 10)
}

pub fn challenge2(input: &str) -> anyhow::Result<usize> {
    challenge(input, 40)
}

#[cfg(test)]
mod tests {
    const EXAMPLE: &str = r#"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"#;
    #[test]
    fn test_challenge1() -> anyhow::Result<()> {
        let expected = [
            // Add tests
            (EXAMPLE, 1588),
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
            (EXAMPLE, 2188189693529),
        ];
        for (input, output) in expected {
            assert_eq!(crate::challenge2(input)?, output, "For input {}", input)
        }
        Ok(())
    }
}
