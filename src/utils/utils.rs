pub fn non_empty_lines(s: &str) -> impl Iterator<Item = &str> {
    s.lines().filter(|l| !l.trim().is_empty())
}
