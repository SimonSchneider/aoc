use anyhow::{anyhow, Result};
use iter_tools::Itertools;
use std::fmt::{format, Display, Formatter};
use std::iter::FromIterator;
use std::str::FromStr;

#[derive(Debug)]
struct Grid<T> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

impl Display for Grid<u8> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for x in 0..self.rows {
            let lb = x * self.cols;
            let ub = (x + 1) * self.cols;
            writeln!(
                f,
                "{}",
                self.data[lb..ub]
                    .iter()
                    .map(|v| {
                        let num = (v & 0b01111111) - 1;
                        if *v & 0b10000000 == 0 {
                            format!("{}v-", num)
                        } else {
                            format!("{}i-", num)
                        }
                    })
                    .join("")
            )?
        }
        Ok(())
    }
}

impl FromStr for Grid<u8> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let rows = s.lines().count();
        let cols = s.len() / rows;
        let data = s
            .lines()
            .flat_map(|i| i.bytes().map(|b| ((b - 48) | 0b10000000) + 1))
            .collect();
        Ok(Self { data, rows, cols })
    }
}

impl FromStr for Grid<u32> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let rows = s.lines().count();
        let cols = s.len() / rows;
        let data = s
            .lines()
            .flat_map(|i| i.bytes().map(|b| (b - 48) as u32))
            .collect();
        Ok(Self { data, rows, cols })
    }
}

fn mark_forward(e: &mut [u8]) {
    let mut max = 0;
    for i in 0..e.len() {
        let curr = e[i] & 0b01111111;
        if curr > max {
            e[i] = curr;
            max = curr;
        }
    }
}

fn iter_flat_mut<T, F>(data: &mut [T], height: usize, width: usize, mapper: F)
where
    F: Fn(&mut [T]),
{
    for (x) in 0..height {
        let lb = x * width;
        let ub = (x + 1) * width;
        mapper(&mut data[lb..ub]);
    }
}

fn iter_flat_no_sides_mut<T, F>(data: &mut [T], height: usize, width: usize, mapper: F)
where
    F: Fn(&mut [T]),
{
    for (x) in 1..height - 1 {
        let lb = x * width;
        let ub = (x + 1) * width;
        mapper(&mut data[lb..ub]);
    }
}

impl Grid<u8> {
    fn mark(mut self) -> Self {
        iter_flat_mut(&mut self.data, self.rows, self.cols, mark_forward);
        self.data.reverse();
        iter_flat_mut(&mut self.data, self.rows, self.cols, mark_forward);
        let mut transposed = vec![0; self.data.len()];
        transpose::transpose(&self.data, &mut transposed, self.cols, self.rows);
        iter_flat_mut(&mut transposed, self.cols, self.rows, mark_forward);
        transposed.reverse();
        iter_flat_mut(&mut transposed, self.cols, self.rows, mark_forward);
        transpose::transpose(&transposed, &mut self.data, self.rows, self.cols);
        self
    }

    fn marked(&self) -> usize {
        self.data.iter().map(|v| (!v >> 7) as usize).sum::<usize>()
    }
}

const SET_MASK: u32 = 1 << 7;
const VALUE_MASK: u32 = !(!0 << 6);
const SCENE_MASK: u32 = (!0 << 8);

fn add_forward_sceneic(v: &mut [u32]) {
    v[1..v.len() - 2]
        .iter()
        .enumerate()
        .map(|(i, v)| (i + 1, v))
        .map(|(i, x)| {
            let height = *x & VALUE_MASK;
            let count = v[i + 1..v.len() - 1]
                .iter()
                .take_while(|c| (*c & VALUE_MASK) < height)
                .count()
                + 1;
            (i, count)
        })
        .collect::<Vec<_>>()
        .into_iter()
        .for_each(|(i, c)| {
            let mut scene = ((v[i] & SCENE_MASK) >> 8);
            if scene == 0 {
                scene = 1;
            }
            let new_scene = scene * c as u32;
            v[i] = new_scene << 8 | (v[i] & (VALUE_MASK | SET_MASK));
        });
}

impl Grid<u32> {
    fn calc_scores(mut self) -> Self {
        iter_flat_no_sides_mut(&mut self.data, self.rows, self.cols, add_forward_sceneic);
        self.data.reverse();
        println!("");
        iter_flat_no_sides_mut(&mut self.data, self.rows, self.cols, add_forward_sceneic);
        let mut transposed = vec![0; self.data.len()];
        transpose::transpose(&self.data, &mut transposed, self.cols, self.rows);
        iter_flat_no_sides_mut(&mut transposed, self.cols, self.rows, add_forward_sceneic);
        transposed.reverse();
        iter_flat_no_sides_mut(&mut transposed, self.cols, self.rows, add_forward_sceneic);
        transpose::transpose(&transposed, &mut self.data, self.rows, self.cols);
        self
    }

    fn score(&self) -> Option<u32> {
        self.data.iter().map(|v| (v & SCENE_MASK) >> 8).max()
    }
}

impl From<Grid<u8>> for Grid<u32> {
    fn from(o: Grid<u8>) -> Self {
        Self {
            data: o.data.into_iter().map(|e| e as u32).collect(),
            rows: o.rows,
            cols: o.cols,
        }
    }
}

pub fn first(inp: &str) -> Result<String> {
    let grid: Grid<u8> = inp.parse()?;
    let marked_grid = grid.mark();
    Ok(marked_grid.marked().to_string())
}

pub fn second(inp: &str) -> Result<String> {
    let grid = inp.parse::<Grid<u32>>()?;
    let scored = grid.calc_scores();
    Ok(scored.score().unwrap().to_string())
}
