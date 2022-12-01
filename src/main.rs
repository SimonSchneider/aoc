use std::fs::File;
use std::io::{BufReader};
use anyhow::anyhow;
use clap::Parser;

mod aoc2022;

#[derive(Parser, Debug)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    year: String,

    /// Number of times to greet
    #[arg(short, long)]
    day: String,

    #[arg(short, long)]
    prob: String,

    #[arg(short, long)]
    file: String,
}

fn main() -> anyhow::Result<()> {
    let Args { year, day, prob, file } = Args::parse();

    let f = File::open(file)?;
    let inp = BufReader::new(f);

    let res = match (year.as_str(), day.as_str(), prob.as_str()) {
        ("2022", "1", "1") => aoc2022::day1::first(inp),
        ("2022", "1", "2") => aoc2022::day1::second(inp),
        _ => Err(anyhow!("not recognized year")),
    }?;

    println!("{}", res);
    Ok(())
}
