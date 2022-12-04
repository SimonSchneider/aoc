extern crate core;

use anyhow::{anyhow, Result};
use aoc::aoc2022;
use clap::Parser;
use std::fs::File;
use std::io::Read;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    prob: String,

    #[arg(short, long)]
    test: bool,
}

fn main() -> Result<()> {
    let Args { prob, test } = Args::parse();
    let [y, d, p] = prob_into_parts(&prob)?;

    let mut f = get_input(&format!("{y}-{d}"), test)?;

    let mut s = String::new();
    f.read_to_string(&mut s)?;

    let res = aoc2022::exec(&format!("{d}-{p}"), &s)?;
    println!("{res}");
    Ok(())
}

fn get_input(prob: &str, test: bool) -> Result<File> {
    let t_str = if test { "-test" } else { "" };
    let pth = format!("inputs/{prob}{t_str}.txt");
    Ok(File::open(pth)?)
}

fn prob_into_parts(prob: &str) -> Result<[&str; 3]> {
    let parts: Vec<_> = prob.split('-').collect();
    if parts.len() != 3 {
        return Err(anyhow!("illegal problem def {prob}"));
    }
    Ok([parts[0], parts[1], parts[2]])
}
