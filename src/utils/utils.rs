use anyhow::Result;

pub fn and<A, B>(r: Result<A>, b: Result<B>) -> Result<(A, B)> {
    r.and_then(|a| b.map(|b| (a, b)))
}

pub fn non_empty_lines(s: &str) -> impl Iterator<Item=&str> {
    s.lines().filter(|l| !l.trim().is_empty())
}