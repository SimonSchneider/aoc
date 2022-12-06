use anyhow::Result;
pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
mod day5;
mod day6;

pub fn get_prob_func(prob: &str) -> impl FnMut(&str) -> Result<String> {
    match prob {
        "1-1" => day1::first,
        "1-2" => day1::second,
        "2-1" => day2::first,
        "2-2" => day2::second,
        "3-1" => day3::first,
        "3-2" => day3::second,
        "4-1" => day4::first,
        "4-2" => day4::second,
        "5-1" => day5::first,
        "5-2" => day5::second,
        "6-1" => day6::first,
        "6-2" => day6::second,
        _ => panic!("invalid problem {prob}"),
    }
}

pub fn exec(prob: &str, inp: &str) -> Result<String> {
    Ok(get_prob_func(prob)(inp)?.to_string())
}

#[cfg(test)]
mod test {

    macro_rules! part_n_test {
        ($part: ident, $day: ident, $inp: expr, $res:expr) => {
            #[test]
            fn $part() {
                assert_eq!(
                    exec(
                        &format!(
                            "{}-{}",
                            stringify!($day).strip_prefix("day").unwrap(),
                            stringify!($part).strip_prefix("part").unwrap()
                        ),
                        $inp
                    )
                    .unwrap(),
                    $res
                )
            }
        };
        ($id: ident, $day: ident, $part:expr, $inp: expr, $res:expr) => {
            #[test]
            fn $id() {
                assert_eq!(
                    exec(
                        &format!(
                            "{}-{}",
                            stringify!($day).strip_prefix("day").unwrap(),
                            $part
                        ),
                        $inp
                    )
                    .unwrap(),
                    $res
                )
            }
        };
    }

    macro_rules! aoc_tests {
        ($day:ident, $inp:expr, $first:expr, $second:expr) => {
            mod $day {
                use crate::aoc2022::exec;
                part_n_test! { part1, $day, $inp, $first }
                part_n_test! { part2, $day, $inp, $second }
            }
        };
        ($spec:ident, $day:ident, $inp:expr, $first:expr, $second:expr) => {
            mod $spec {
                use crate::aoc2022::exec;
                part_n_test! { part1, $day, $inp, $first }
                part_n_test! { part2, $day, $inp, $second }
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

    aoc_tests!(
        day5,
        r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
        "#,
        "CMZ",
        "MCD"
    );

    mod day6 {
        use crate::aoc2022::exec;
        part_n_test!(p1a, day6, "1", r#"mjqjpqmgbljsphdztnvjfqwrcgsmlb"#, "7");
        part_n_test!(p1b, day6, "1", r#"bvwbjplbgvbhsrlpgdmjqwftvncz"#, "5");
        part_n_test!(p1c, day6, "1", r#"nppdvjthqldpwncqszvftbrmjlhg"#, "6");
        part_n_test!(p1d, day6, "1", r#"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"#, "10");
        part_n_test!(p1e, day6, "1", r#"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"#, "11");

        part_n_test!(p2a, day6, "2", r#"mjqjpqmgbljsphdztnvjfqwrcgsmlb"#, "19");
        part_n_test!(p2b, day6, "2", r#"bvwbjplbgvbhsrlpgdmjqwftvncz"#, "23");
        part_n_test!(p2c, day6, "2", r#"nppdvjthqldpwncqszvftbrmjlhg"#, "23");
        part_n_test!(p2d, day6, "2", r#"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"#, "29");
        part_n_test!(p2e, day6, "2", r#"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"#, "26");
    }
}
