use anyhow::Result;
pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;

fn get_prob(prob: &str) -> impl FnOnce(&str) -> Result<String> {
    match prob {
        "1-1" => day1::first,
        "1-2" => day1::second,
        "2-1" => day2::first,
        "2-2" => day2::second,
        "3-1" => day3::first,
        "3-2" => day3::second,
        "4-1" => day4::first,
        "4-2" => day4::second,
        _ => panic!("invalid problem {prob}"),
    }
}

pub fn exec(prob: &str, inp: &str) -> Result<String> {
    get_prob(prob)(inp)
}

#[cfg(test)]
mod test {

    macro_rules! aoc_tests {
        ($day:ident, $inp:expr, $first:expr, $second:expr) => {
            mod $day {
                use crate::aoc2022::exec;
                #[test]
                fn part_1() {
                    assert_eq!(
                        exec(
                            &format!("{}-1", stringify!($day).strip_prefix("day").unwrap()),
                            $inp
                        )
                        .unwrap(),
                        $first
                    )
                }
                #[test]
                fn part_2() {
                    assert_eq!(
                        exec(
                            &format!("{}-2", stringify!($day).strip_prefix("day").unwrap()),
                            $inp
                        )
                        .unwrap(),
                        $second
                    )
                }
            }
        };
    }

    aoc_tests!(
        day1,
        r#"
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
        "#,
        "24000",
        "45000"
    );
    aoc_tests!(
        day2,
        r#"
A Y
B X
C Z
        "#,
        "15",
        "12"
    );

    aoc_tests!(
        day3,
        r#"
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
        "#,
        "157",
        "70"
    );

    aoc_tests!(
        day4,
        r#"
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
        "#,
        "2",
        "4"
    );
}
