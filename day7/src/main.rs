use anyhow::Result;
use clap::Parser;
use std::{borrow::Cow, fs};

const DEFAULT_INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt"));
#[derive(Parser)]
struct App {
    #[clap(subcommand)]
    part: Challenge,
}

#[derive(Parser)]
struct Opts {
    /// Path to the input file
    input: Option<String>,
}

#[derive(Parser)]
enum Challenge {
    /// Run challenge part 1
    Part1(Opts),
    /// Run challenge part 2
    Part2(Opts),
}

fn main() -> Result<()> {
    let opts = App::parse();

    match opts.part {
        Challenge::Part1(Opts { input }) => {
            let data = if let Some(path) = input {
                Cow::Owned(fs::read_to_string(path)?)
            } else {
                Cow::Borrowed(DEFAULT_INPUT)
            };
            println!("{}", day7::challenge1(&data)?);
        }
        Challenge::Part2(Opts { input }) => {
            let data = if let Some(path) = input {
                Cow::Owned(fs::read_to_string(path)?)
            } else {
                Cow::Borrowed(DEFAULT_INPUT)
            };
            println!("{}", day7::challenge2(&data)?);
        }
    }

    Ok(())
}
