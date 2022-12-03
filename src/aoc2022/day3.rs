use crate::utils::utils::non_empty_lines;
use anyhow::Result;
use iter_tools::Itertools;

fn u8_one_high_encoding(e: u8) -> u64 {
    let shifted = (e ^ 0b00100000) & 0b0111111;
    let one_high: u64 = 1 << (shifted);
    let upper_part: u64 = one_high & (u64::MAX << 32);
    let lower_part: u64 = one_high & (u64::MAX >> 32);
    (upper_part) >> 6 | lower_part
}

fn str_chars(s: &str) -> u64 {
    s.as_bytes()
        .iter()
        .map(|b| u8_one_high_encoding(*b))
        .fold(0, |sum, a| sum | a)
}

pub fn first(inp: &str) -> Result<String> {
    let res: u32 = non_empty_lines(inp)
        .map(|l| l.split_at(l.len() / 2))
        .map(|p| [p.0, p.1].map(str_chars))
        .map(|[a, b]| (a & b).trailing_zeros())
        .sum();
    Ok(format!("{res}"))
}

pub fn second(inp: &str) -> Result<String> {
    let res: u32 = non_empty_lines(inp)
        .map(str_chars)
        .chunks(3)
        .into_iter()
        .flat_map(|g| g.reduce(|s, e| s & e))
        .map(|e| e.trailing_zeros())
        .sum();
    Ok(format!("{res}"))
}

#[cfg(test)]
mod test {
    use crate::aoc2022::day3::{first, second};

    #[test]
    fn prob_1_test() {
        let res = first(
            r#"
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
        "#,
        )
        .unwrap();
        assert_eq!(res, "157");
    }

    #[test]
    fn prob_2_test() {
        let res = second(
            r#"
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
        "#,
        )
        .unwrap();
        assert_eq!(res, "70");
    }
}
