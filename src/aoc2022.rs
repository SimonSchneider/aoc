use anyhow::Result;
pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

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
        "7-1" => day7::first,
        "7-2" => day7::second,
        "8-1" => day8::first,
        "8-2" => day8::second,
        "9-1" => day9::first,
        "9-2" => day9::second,
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

    aoc_tests!(
        day7,
        r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#,
        "95437",
        "24933642"
    );

    aoc_tests!(
        day8,
        r#"30373
25512
65332
33549
35390"#,
        "21",
        "8"
    );

    aoc_tests!(
        day9,
        r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#,
        "13",
        "1"
    );

    mod day9ext {
        use crate::aoc2022::exec;
        part_n_test!(
            ext,
            day9,
            "2",
            r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"#,
            "36"
        );
    }

    aoc_tests!(
        day10,
        r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"#,
        "13140",
        "1"
    );
}
