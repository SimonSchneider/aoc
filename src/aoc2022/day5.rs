use crate::utils::utils::non_empty_lines;
use anyhow::{anyhow, Result};
use iter_tools::Itertools;
use std::str::FromStr;

#[derive(Debug)]
struct Stack(Vec<char>);
#[derive(Debug)]
struct Stacks(Vec<Stack>);

impl Stacks {
    pub(crate) fn output(&self) -> String {
        self.0.iter().flat_map(|s| s.0.last().cloned()).join("")
    }
}

#[derive(Debug)]
struct Move {
    from: usize,
    to: usize,
    n: usize,
}

impl FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let elms = s.split_whitespace().collect::<Vec<_>>();
        Ok(Move {
            from: elms[3].parse::<usize>()? - 1,
            to: elms[5].parse::<usize>()? - 1,
            n: elms[1].parse::<usize>()?,
        })
    }
}

trait Crane {
    fn execute_moves(mut stacks: Stacks, moves: &[Move]) -> Stacks {
        for m in moves {
            stacks = Self::execute_move(stacks, m)
        }
        stacks
    }
    fn execute_move(stacks: Stacks, m: &Move) -> Stacks;
}

struct Crane9000;

impl Crane for Crane9000 {
    fn execute_move(mut stacks: Stacks, m: &Move) -> Stacks {
        for _ in 0..m.n {
            if let Some(e) = stacks.0[m.from].0.pop() {
                stacks.0[m.to].0.push(e);
            }
        }
        stacks
    }
}

struct Crane9001;

impl Crane for Crane9001 {
    fn execute_move(mut stacks: Stacks, m: &Move) -> Stacks {
        let from_len = stacks.0[m.from].0.len();
        let mut first_elems = stacks.0[m.from].0.split_off(from_len - m.n);
        stacks.0[m.to].0.append(&mut first_elems);
        stacks
    }
}

impl FromStr for Stacks {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut lines: Vec<_> = s.lines().collect();
        let nums = lines.pop().ok_or_else(|| anyhow!("to few lines"))?;
        let mut stacks = Stacks(
            nums.split_whitespace()
                .map(|_| Stack(Vec::new()))
                .collect::<Vec<_>>(),
        );
        let idxs: Vec<_> = (0..stacks.0.len()).map(|i| (i, 4 * (i + 1) - 3)).collect();
        lines.iter().for_each(|l| {
            for (si, i) in &idxs {
                l.chars()
                    .nth(*i)
                    .filter(|c| !c.is_whitespace())
                    .iter()
                    .for_each(|c| stacks.0[*si].0.push(*c));
            }
        });
        for s in &mut stacks.0 {
            s.0.reverse();
        }
        Ok(stacks)
    }
}

fn run<C: Crane>(inp: &str) -> Result<String> {
    let (stack_inp, act_inp) = inp
        .split_once("\n\n")
        .ok_or_else(|| anyhow!("illegal input"))?;
    let stacks: Stacks = stack_inp.parse()?;
    let moves: Vec<Move> = non_empty_lines(act_inp)
        .map(|l| l.parse())
        .collect::<Result<_>>()?;
    Ok(C::execute_moves(stacks, &moves).output())
}

pub fn first(inp: &str) -> Result<String> {
    run::<Crane9000>(inp)
}

pub fn second(inp: &str) -> Result<String> {
    run::<Crane9001>(inp)
}
