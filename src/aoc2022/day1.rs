use std::io::{BufRead};
use std::iter::Sum;
use std::ops::Add;
use std::str::FromStr;
use anyhow::Result;
use iter_tools::Itertools;
use crate::utils::utils::and;

enum Line<T> {
    Cal(T),
    NewLine,
}

impl FromStr for Line<usize> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s.is_empty() {
            Ok(Line::NewLine)
        } else {
            Ok(Line::Cal(s.parse()?))
        }
    }
}

pub fn first<R: BufRead>(inp: R) -> Result<String> {
    find_n_most_cals::<R, 1>(inp)
}

pub fn second<R: BufRead>(inp: R) -> Result<String> {
    find_n_most_cals::<R, 3>(inp)
}

pub fn find_n_most_cals<R: BufRead, const N: usize>(inp: R) -> Result<String> {
    let res = inp.lines().map_ok(|e| e.trim().parse::<Line<usize>>()).flatten().fold(Ok(Agg::<N, usize>::new()), |agg, elem| {
        and(agg, elem).map(|(agg, line)| agg.and(line))
    })?.result();
    Ok(format!("{}", res))
}

struct Agg<const N: usize, T> {
    max: [T; N],
    curr: T,
}

impl<const N: usize, T: Ord + Sum + Default + Add<Output=T> + Copy> Agg<N, T> {
    fn new() -> Self {
        Self { max: [T::default(); N], curr: T::default() }
    }

    fn and(self, line: Line<T>) -> Self {
        match line {
            Line::Cal(t) => Self { max: self.max, curr: self.curr + t },
            Line::NewLine => {
                Self { max: fill_max(self.max, self.curr), curr: T::default() }
            }
        }
    }

    fn result(self) -> T {
        fill_max(self.max, self.curr).into_iter().sum()
    }
}

fn fill_max<T: Ord, const N: usize>(mut res: [T; N], curr: T) -> [T; N] {
    if let Some((idx, _)) = res.iter().enumerate().min_by(|(_, a), (_, b)| a.cmp(b)).filter(|(_, min)| &&curr > min) {
        res[idx] = curr;
    }
    res
}
