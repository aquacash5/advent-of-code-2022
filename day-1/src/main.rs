use std::cmp::Reverse;

use itertools::Itertools;
use utils::*;

#[derive(Debug, PartialEq, Eq)]
struct InputData {
    elfs: Vec<Vec<i64>>,
}

fn parse(input: &str) -> ParseResult<InputData> {
    use nom::{
        character::complete::{i64, line_ending},
        combinator::map,
        multi::separated_list1,
        sequence::tuple,
    };
    let elf = separated_list1(line_ending, i64);
    let elfs = separated_list1(tuple((line_ending, line_ending)), elf);
    let mut parse = map(elfs, |elfs| InputData { elfs });
    parse(input)
}

#[allow(clippy::unnecessary_wraps)]
fn part1(input: &InputData) -> AocResult<i64> {
    Ok(input.elfs.iter().map(|v| v.iter().sum()).max().unwrap())
}

#[allow(clippy::unnecessary_wraps)]
fn part2(input: &InputData) -> AocResult<i64> {
    Ok(input
        .elfs
        .iter()
        .map(|v| v.iter().sum::<i64>())
        .sorted_by_key(|&s| Reverse(s))
        .take(3)
        .sum())
}

aoc_main!(parse, part1, part2);

#[test]
fn test() {
    let input = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";
    assert_parser!(
        parse,
        input,
        InputData {
            elfs: vec![
                vec![1000, 2000, 3000],
                vec![4000],
                vec![5000, 6000],
                vec![7000, 8000, 9000],
                vec![10_000]
            ]
        }
    );
    assert_part!(parse, part1, input, 24_000);
    assert_part!(parse, part2, input, 45_000);
}
