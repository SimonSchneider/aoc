use crate::aoc2022::day2::Outcome::{Draw, Loss, Win};
use crate::utils::utils::non_empty_lines;
use anyhow::{anyhow, Result};
use std::str::FromStr;

#[derive(Debug)]
enum Outcome {
    Win,
    Loss,
    Draw,
}

impl Outcome {
    fn points(&self) -> usize {
        match self {
            Win => 6,
            Draw => 3,
            Loss => 0,
        }
    }

    fn invert(&self) -> Self {
        match self {
            Win => Loss,
            Loss => Win,
            Draw => Draw,
        }
    }
}

impl FromStr for Outcome {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "X" => Ok(Loss),
            "Y" => Ok(Draw),
            "Z" => Ok(Win),
            _ => Err(anyhow!("invalid player B choice")),
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    fn points(&self) -> usize {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }

    fn game_outcome(&self, o: &Self) -> Outcome {
        use Choice::{Paper, Rock, Scissors};
        match (self, o) {
            (Rock, Scissors) | (Scissors, Paper) | (Paper, Rock) => Win,
            (a, b) if a == b => Draw,
            _ => Loss,
        }
    }

    fn opponent_play(&self, outcome: &Outcome) -> Self {
        use Choice::{Paper, Rock, Scissors};
        match (self, outcome) {
            (a, Draw) => a.clone(),
            (Rock, Win) | (Paper, Loss) => Scissors,
            (Rock, Loss) | (Scissors, Win) => Paper,
            (Scissors, Loss) | (Paper, Win) => Rock,
        }
    }
}

struct GameOutcome {
    elf: Choice,
    outcome: Outcome,
}

impl Game for GameOutcome {
    fn points(&self) -> usize {
        self.elf.opponent_play(&self.outcome.invert()).points() + self.outcome.points()
    }
}

struct GameFirst {
    a: Choice,
    b: Choice,
}

impl Game for GameFirst {
    fn points(&self) -> usize {
        self.b.points() + self.b.game_outcome(&self.a).points()
    }
}

impl FromStr for GameOutcome {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (a, b) = get_line(s)?;
        let ca = elf_play(a)?;
        let outcome = b.parse()?;
        Ok(Self { elf: ca, outcome })
    }
}

fn get_line(s: &str) -> Result<(&str, &str)> {
    s.split_once(" ")
        .ok_or_else(|| anyhow!("can't split game line: '{}'", s))
}

fn elf_play(s: &str) -> Result<Choice> {
    match s {
        "A" => Ok(Choice::Rock),
        "B" => Ok(Choice::Paper),
        "C" => Ok(Choice::Scissors),
        _ => Err(anyhow!("invalid player A choice")),
    }
}

impl FromStr for GameFirst {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (a, b) = get_line(s)?;
        let ca = elf_play(a)?;
        let cb = match b {
            "X" => Ok(Choice::Rock),
            "Y" => Ok(Choice::Paper),
            "Z" => Ok(Choice::Scissors),
            _ => Err(anyhow!("invalid player B choice")),
        }?;
        Ok(GameFirst { a: ca, b: cb })
    }
}

pub trait Game {
    fn points(&self) -> usize;
}

pub fn second(inp: &str) -> Result<String> {
    run::<GameOutcome>(inp)
}

pub fn first(inp: &str) -> Result<String> {
    run::<GameFirst>(inp)
}

fn run<G: Game + FromStr<Err = anyhow::Error>>(inp: &str) -> Result<String> {
    Ok(non_empty_lines(inp)
        .map(|g| G::from_str(g.trim()).unwrap())
        .fold(0, |sum, e| sum + e.points())
        .to_string())
}
