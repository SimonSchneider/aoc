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
    find_n_most_cals::<R, 1>(inp)
}

pub fn second<R: BufRead>(inp: R) -> Result<String> {
    find_n_most_cals::<R, 3>(inp)
}

pub fn find_n_most_cals<R: BufRead, const N: usize>(inp: R) -> Result<String> {
    let (max, curr) = inp.lines().map_ok(|e| e.trim().parse::<Line>()).flatten().fold(Ok(([0; N], 0)), |agg, elem| {
        and(agg, elem).map(|((max, curr), line)| {
            match line {
                Line::Cal(cal) => (max, curr + cal),
                Line::NewLine => {
                    (fill_max(max, curr), 0)
                }
            }
        })
    })?;
    let res = fill_max(max, curr);
    Ok(format!("{}", res.into_iter().sum::<u32>()))
}

pub fn fill_max<T: Ord + Debug + Clone, const N: usize>(mut res: [T; N], curr: T) -> [T; N] {
    if let Some((idx, _)) = res.iter().enumerate().min_by(|(_, a), (_, b)| a.cmp(b)).filter(|(_, min)| &&curr > min) {
        res[idx] = curr;
    }
    res
}