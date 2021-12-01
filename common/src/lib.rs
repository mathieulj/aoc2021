use nom::Parser;
use nom_supreme::{
    error::ErrorTree,
    final_parser::{final_parser, Location},
};

/// Simple helper function, exists solely to reduce boilerplate caused by having to specify generic
/// types.
pub fn parse<'i, O>(
    input: &'i str,
    parser: impl Parser<&'i str, O, ErrorTree<&'i str>>,
) -> Result<O, ErrorTree<Location>> {
    final_parser(parser)(input)
}

/// Simple dataless enum with an auto generated nom parser
///
/// # Examples
///
/// ```
/// use nom::{error::ParseError, Parser};
///
/// common::nom_enum!(
///     enum Sample {
///         Children = "children",
///         Cats = "cats",
///         Samoyeds = "samoyeds",
///     }
/// );
///
/// fn puzzle<'i, E: ParseError<&'i str>>(input: &'i str) -> nom::IResult<&'i str, Sample, E> {
///     Sample::parser().parse(input)
/// }
/// ```
#[macro_export]
macro_rules! nom_enum {
    (enum $type:ident {$($variant:ident = $alias:literal,)*}) => {
        #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
        enum $type {
            $($variant),*
        }

        impl $type {
            fn parser<'i, E: nom::error::ParseError<&'i str>>() -> impl nom::Parser<&'i str, Self, E> {
                nom::branch::alt((
                    $(nom::combinator::value(Self::$variant, nom::bytes::complete::tag($alias))),*
                ))
            }
        }
    };
}
