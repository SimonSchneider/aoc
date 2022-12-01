use std::fmt::Debug;
use std::io::{BufRead};
use std::str::FromStr;
use anyhow::Result;
use iter_tools::Itertools;

enum Line {
    Cal(u32),
    NewLine,
}

impl FromStr for Line {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s.is_empty() {
            Ok(Line::NewLine)
        } else {
            Ok(Line::Cal(s.parse()?))
        }
    }
}

fn and<A, B>(r: Result<A>, b: Result<B>) -> Result<(A, B)> {
    r.and_then(|a| b.map(|b| (a, b)))
}

pub fn first<R: BufRead>(inp: R) -> Result<String> {
    find_n_most_cals(inp, 1)
}

pub fn second<R: BufRead>(inp: R) -> Result<String> {
    find_n_most_cals(inp, 3)
}

pub fn find_n_most_cals<R: BufRead>(inp: R, n: usize) -> Result<String> {
    let (max, curr) = inp.lines().map_ok(|e| e.trim().parse::<Line>()).flatten().fold(Ok((vec![], 0)), |agg, elem| {
        and(agg, elem).map(|((max, curr), line)| {
            match line {
                Line::Cal(cal) => (max, curr + cal),
                Line::NewLine => {
                    (fill_max(max, curr, n), 0)
                }
            }
        })
    })?;
    let res = fill_max(max, curr, n);
    Ok(format!("{}", res.into_iter().sum::<u32>()))
}

pub fn fill_max<T: Ord + Debug + Clone>(mut max: Vec<T>, curr: T, n: usize) -> Vec<T> {
    if max.len() < n {
        max.push(curr);
    } else if let Some((idx, _)) = max.iter().enumerate().min_by(|(_, a), (_, b)| a.cmp(b)).filter(|(_, min)| &&curr > min) {
        max[idx] = curr;
    }
    max
}