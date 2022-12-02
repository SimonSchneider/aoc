use anyhow::Result;

pub fn and<A, B>(r: Result<A>, b: Result<B>) -> Result<(A, B)> {
    r.and_then(|a| b.map(|b| (a, b)))
}
