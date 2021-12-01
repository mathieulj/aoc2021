use anyhow::Result;
use clap::Parser;
use std::fs;

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
                fs::read_to_string(path)?
            } else {
                fs::read_to_string("data/input.txt")?
            };
            println!("{}", template::challenge1(&data)?);
        }
        Challenge::Part2(Opts { input }) => {
            let data = if let Some(path) = input {
                fs::read_to_string(path)?
            } else {
                fs::read_to_string("data/input.txt")?
            };
            println!("{}", template::challenge2(&data)?);
        }
    }

    Ok(())
}
