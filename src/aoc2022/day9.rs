use anyhow::{anyhow, Result};
use iter_tools::Itertools;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Sub};
use std::str::FromStr;

#[derive(Copy, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

struct Move(Direction, u8);

impl FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (d, n) = s.split_once(" ").ok_or_else(|| anyhow!("illegal line"))?;
        Ok(Self(
            match d {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "R" => Direction::Right,
                "L" => Direction::Left,
                _ => return Err(anyhow!("illegal move")),
            },
            n.parse()?,
        ))
    }
}

#[derive(Default, Eq, PartialEq, Hash, Copy, Clone)]
struct Location(i32, i32);

impl Add<Direction> for Location {
    type Output = Location;

    fn add(self, rhs: Direction) -> Self::Output {
        let Self(x, y) = self;
        match rhs {
            Direction::Left => Location(x - 1, y),
            Direction::Right => Location(x + 1, y),
            Direction::Up => Location(x, y + 1),
            Direction::Down => Location(x, y - 1),
        }
    }
}

impl Sub<Location> for Location {
    type Output = (i32, i32);

    fn sub(self, rhs: Location) -> Self::Output {
        ((self.0 - rhs.0), (self.1 - rhs.1))
    }
}

fn truncate(n: i32) -> i32 {
    if n < 0 {
        -1
    } else if n > 0 {
        1
    } else {
        0
    }
}

impl Location {
    fn follow(self, o: &Self) -> Self {
        let (dx, dy) = *o - self;
        if dy.abs() < 2 && dx.abs() < 2 {
            return self;
        }
        let tx = truncate(dx);
        let ty = truncate(dy);
        Self(self.0 + truncate(dx), self.1 + truncate(dy))
    }
}

struct Ropes<const N: usize> {
    head: Location,
    followers: [Location; N],
}

impl<const N: usize> Add<Direction> for Ropes<N> {
    type Output = Self;

    fn add(mut self, rhs: Direction) -> Self::Output {
        let Ropes {
            head,
            mut followers,
        } = self;
        let new_head = head + rhs;
        let mut leader = new_head;
        for i in 0..followers.len() {
            let new_loc = followers[i].follow(&leader);
            leader = new_loc;
            followers[i] = new_loc;
        }
        Self {
            head: new_head,
            followers: followers,
        }
    }
}

impl<const N: usize> Ropes<N> {
    fn new() -> Self {
        Ropes {
            head: Default::default(),
            followers: [Default::default(); N],
        }
    }

    fn tail(&self) -> Location {
        *(self
            .followers
            .last()
            .expect("rope with 0 length not supported"))
    }

    fn print(&self, min: i32, max: i32) {
        let Ropes { head, followers } = self;
        let mut lines = vec![];
        for y in min..max {
            let mut s = String::new();
            for x in min..max {
                if head.0 == x && head.1 == y {
                    s.push('H')
                } else if let Some((i, _)) = followers
                    .iter()
                    .enumerate()
                    .find(|(_, f)| f.0 == x && f.1 == y)
                {
                    s.push(((i + 49) as u8) as char)
                } else {
                    s.push('.')
                }
            }
            lines.push(s);
        }

        println!(
            "{}\nH({},{}) - T({},{})\n",
            lines.into_iter().rev().join("\n"),
            head.0,
            head.1,
            self.tail().0,
            self.tail().1
        );
    }
}

pub fn run<const N: usize>(inp: &str) -> Result<String> {
    let mut pos: HashSet<Location> = HashSet::new();
    let mut ropes = Ropes::<N>::new();
    pos.insert(ropes.tail());
    for x in inp.lines() {
        let Move(d, n) = x.parse()?;
        for _ in 0..n {
            ropes = ropes + d;
            pos.insert(ropes.tail());
        }
    }
    Ok(pos.len().to_string())
}

pub fn first(inp: &str) -> Result<String> {
    run::<1>(inp)
}

pub fn second(inp: &str) -> Result<String> {
    run::<9>(inp)
}
