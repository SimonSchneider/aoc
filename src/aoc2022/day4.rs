use crate::utils::utils::non_empty_lines;
use anyhow::Result;

fn contains(a: &[u32; 2], b: &[u32; 2]) -> bool {
    a[0] <= b[0] && b[1] <= a[1]
}

fn contained(a: &[u32; 2], b: &[u32; 2]) -> bool {
    contains(a, b) || contains(b, a)
}

fn any_overlap(a: &[u32; 2], b: &[u32; 2]) -> bool {
    !(a[1] < b[0] || b[1] < a[0])
}

pub fn first(inp: &str) -> Result<String> {
    parse_and_count(inp, contained)
}

pub fn second(inp: &str) -> Result<String> {
    parse_and_count(inp, any_overlap)
}

fn parse_and_count<F>(inp: &str, mut pred: F) -> Result<String>
where
    F: FnMut(&[u32; 2], &[u32; 2]) -> bool,
{
    let res = non_empty_lines(inp)
        .map(|l| l.split_once(",").unwrap())
        .map(|(a, b)| {
            [a, b]
                .map(|a| a.split_once("-").unwrap())
                .map(|(l, u)| [l, u].map(|e| e.parse::<u32>().unwrap()))
        })
        .filter(|[f, s]| pred(f, s))
        .count();
    Ok(format!("{res}"))
}
