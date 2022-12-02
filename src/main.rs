use std::fs::File;
use std::io::{BufReader};
use anyhow::{anyhow, Result};
use clap::Parser;

mod aoc2022;
mod utils;

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
    test: bool,
}

fn main() -> Result<()> {
    let Args { year, day, prob, test } = Args::parse();

    let f = get_input(&year, &day, test)?;
    let inp = BufReader::new(f);

    let res = match (year.as_str(), day.as_str(), prob.as_str()) {
        ("2022", "1", "1") => aoc2022::day1::first(inp),
        ("2022", "1", "2") => aoc2022::day1::second(inp),
        ("2022", "2", "1") => aoc2022::day2::first(inp),
        ("2022", "2", "2") => aoc2022::day2::second(inp),
        _ => Err(anyhow!("not recognized year")),
    }?;

    println!("{}", res);
    Ok(())
}

fn get_input(year: &str, day: &str, test: bool) -> Result<File> {
    let t_str = if test { "-test" } else {""};
    let pth = format!("inputs/{year}-{day}{t_str}.txt");
    Ok(File::open(pth)?)
}
