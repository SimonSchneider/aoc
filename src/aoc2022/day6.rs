use anyhow::{anyhow, Result};
use iter_tools::Itertools;

fn done<const N: usize>(v: &[char; N]) -> bool {
    return v.iter().unique().count() == N;
}

fn run_first<const N: usize>(inp: &str) -> Result<usize> {
    let mut curr = [' '; N];
    let mut chars = inp.chars();
    for i in 0..N {
        curr[i] = chars.next().unwrap();
    }
    if done(&curr) {
        return Ok(N);
    }
    let mut i = 0;
    for c in chars {
        curr[i % N] = c;
        if done(&curr) {
            return Ok(N + i + 1);
        }
        i += 1;
    }
    Err(anyhow!("not found"))
}

pub fn first(inp: &str) -> Result<String> {
    let res = run_first::<4>(inp)?;
    Ok(res.to_string())
}

pub fn second(inp: &str) -> Result<String> {
    let res = run_first::<14>(inp)?;
    Ok(res.to_string())
}
