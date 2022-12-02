use std::io::BufRead;
use std::str::FromStr;
use anyhow::{anyhow, Result};
use iter_tools::Itertools;
use crate::aoc2022::day2::Outcome::{Draw, Loss, Win};
use crate::utils::utils::and;

#[derive(Debug)]
enum Outcome {
    Win,
    Loss,
    Draw,
}

impl Outcome {
    fn points(&self) -> u64 {
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
    fn points(&self) -> u64 {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }

    fn game_outcome(&self, o: &Self) -> Outcome {
        use Choice::{Rock, Paper, Scissors};
        match (self, o) {
            (Rock, Scissors) | (Scissors, Paper) | (Paper, Rock) => Win,
            (a, b) if a == b => Draw,
            _ => Loss,
        }
    }

    fn opponent_play(&self, outcome: &Outcome) -> Self {
        use Choice::{Rock, Paper, Scissors};
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
    fn points(&self) -> u64 {
        self.elf.opponent_play(&self.outcome.invert()).points() + self.outcome.points()
    }
}

struct GameFirst {
    a: Choice,
    b: Choice,
}

impl Game for GameFirst {
    fn points(&self) -> u64 {
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
    s.split_once(" ").ok_or_else(|| anyhow!("can't split game line: '{}'", s))
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
    fn points(&self) -> u64;
}

pub fn second<R: BufRead>(inp: R) -> Result<String> {
    run::<GameOutcome, R>(inp)
}

pub fn first<R: BufRead>(inp: R) -> Result<String> {
    run::<GameFirst, R>(inp)
}

fn run<G: Game + FromStr<Err=anyhow::Error>, R: BufRead>(inp: R) -> Result<String> {
    let res = inp.lines().filter_ok(|l| !l.trim().is_empty()).map_ok(|g| G::from_str(g.trim())).flatten().fold(Ok(0), |sum, e| {
        and(sum, e).map(|(sum, e)| sum + e.points())
    })?;
    Ok(format!("{res}"))
}

#[cfg(test)]
mod test {
    use crate::aoc2022::day2::{first, second};

    #[test]
    fn prob_1_test() {
        let res = first(r#"
A Y
B X
C Z
        "#.as_bytes()).unwrap();
        assert_eq!(res, "15");
    }

    #[test]
    fn prob_2_test() {
        let res = second(r#"
A Y
B X
C Z
        "#.as_bytes()).unwrap();
        assert_eq!(res, "12");
    }
}