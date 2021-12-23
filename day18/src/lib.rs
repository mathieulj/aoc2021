use std::ops::{Add, AddAssign, ControlFlow, Deref};

use anyhow::Context;
use nom::{
    branch::alt,
    character::complete::{char, digit1, newline},
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    IResult, Parser,
};
use nom_supreme::{error::ErrorTree, ParserExt};

#[derive(Clone)]
enum Number {
    Literal(u32),
    Pair(Box<Number>, Box<Number>),
}

impl Add<Number> for Number {
    type Output = Number;

    fn add(self, right: Self) -> Self::Output {
        Number::pair(self, right)
    }
}

impl AddAssign<Number> for Number {
    fn add_assign(&mut self, right: Number) {
        let left = std::mem::replace(self, Number::Literal(0));
        *self = Number::pair(left, right);
    }
}

impl Number {
    fn pair(left: Number, right: Number) -> Self {
        Self::Pair(Box::new(left), Box::new(right))
    }

    fn parse(input: &str) -> IResult<&str, Number, ErrorTree<&str>> {
        alt((
            delimited(
                char('['),
                separated_pair(Number::parse, char(','), Number::parse),
                char(']'),
            )
            .map(|(left, right)| Number::pair(left, right)),
            digit1.parse_from_str().map(|l| Number::Literal(l)),
        ))
        .parse(input)
    }

    fn rightmost_mut(&mut self) -> &mut Self {
        match self {
            lit @ Number::Literal(_) => lit,
            Number::Pair(_, right) => right.rightmost_mut(),
        }
    }

    fn leftmost_mut(&mut self) -> &mut Self {
        match self {
            lit @ Number::Literal(_) => lit,
            Number::Pair(left, _) => left.leftmost_mut(),
        }
    }

    fn reduce(&mut self) -> anyhow::Result<()> {
        loop {
            match self.explode(None, None, 0) {
                ControlFlow::Continue(_) => {}
                ControlFlow::Break(r) => {
                    r?;
                    continue;
                }
            }

            match self.split() {
                ControlFlow::Continue(_) => break,
                ControlFlow::Break(r) => {
                    r?;
                    continue;
                }
            }
        }
        Ok(())
    }

    fn split(&mut self) -> ControlFlow<anyhow::Result<()>> {
        if let Number::Literal(n) = &*self {
            if *n >= 10 {
                let left = *n >> 1;
                let right = left + if (n & 0x1) == 0 { 0 } else { 1 };

                *self = Self::pair(Self::Literal(left), Self::Literal(right));
                ControlFlow::Break(Ok(()))
            } else {
                ControlFlow::Continue(())
            }
        } else if let Number::Pair(a, b) = self {
            a.split()?;
            b.split()?;
            ControlFlow::Continue(())
        } else {
            unreachable!()
        }
    }

    fn explode(
        &mut self,
        left: Option<&mut Self>,
        right: Option<&mut Self>,
        depth: usize,
    ) -> ControlFlow<anyhow::Result<()>> {
        if depth < 4 {
            return match self {
                Number::Literal(_) => ControlFlow::Continue(()),
                Number::Pair(a, b) => {
                    a.explode(left, Some(b.leftmost_mut()), depth + 1)?;
                    b.explode(Some(a.rightmost_mut()), right, depth + 1)?;
                    ControlFlow::Continue(())
                }
            };
        }

        match &*self {
            Self::Literal(..) => ControlFlow::Continue(()),
            Self::Pair(a, b) => match (a.deref(), b.deref()) {
                (Number::Literal(a), Number::Literal(b)) => {
                    if let Some(Self::Literal(left)) = left {
                        *left += *a;
                    }
                    if let Some(Self::Literal(right)) = right {
                        *right += *b;
                    }
                    *self = Self::Literal(0);
                    ControlFlow::Break(Ok(()))
                }
                (Number::Literal(_), Number::Pair(_, _)) => {
                    ControlFlow::Break(Err(anyhow::format_err!("Unexpected right pair")))
                }
                (Number::Pair(_, _), Number::Literal(_)) => {
                    ControlFlow::Break(Err(anyhow::format_err!("Unexpected left pair")))
                }
                (Number::Pair(_, _), Number::Pair(_, _)) => {
                    ControlFlow::Break(Err(anyhow::format_err!("Unexpected double pairs")))
                }
            },
        }
    }

    fn magnitude(&self) -> u32 {
        match self {
            Number::Literal(n) => *n,
            Number::Pair(left, right) => left.magnitude() * 3 + right.magnitude() * 2,
        }
    }
}

pub fn challenge1(input: &str) -> anyhow::Result<u32> {
    let mut lines = common::parse(input, separated_list1(newline, Number::parse))?.into_iter();
    let mut number = lines.next().context("No lines")?;

    for next in lines {
        number += next;
        number.reduce()?;
    }

    Ok(number.magnitude())
}

pub fn challenge2(input: &str) -> anyhow::Result<u32> {
    let lines = common::parse(input, separated_list1(newline, Number::parse))?;

    let combinations = (0..lines.len())
        .flat_map(|a| (0..lines.len()).map(move |b| (a, b)))
        .filter(|(a, b)| a != b);

    let mut max = 0;
    for (a, b) in combinations {
        let mut number = lines[a].clone() + lines[b].clone();
        number.reduce()?;
        max = max.max(number.magnitude());
    }

    Ok(max)
}

#[cfg(test)]
mod tests {
    const EXAMPLE: &str = r#"[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"#;

    #[test]
    fn test_challenge1() -> anyhow::Result<()> {
        let expected = [
            // Add tests
            (EXAMPLE, 4140),
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
            (EXAMPLE, 3993),
        ];
        for (input, output) in expected {
            assert_eq!(crate::challenge2(input)?, output, "For input {}", input)
        }
        Ok(())
    }
}
