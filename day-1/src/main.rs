use std::cmp::Reverse;

use itertools::Itertools;
#[allow(clippy::wildcard_imports)]
use utils::*;

#[derive(Debug, PartialEq, Eq)]
struct InputData {
    elfs: Vec<Vec<u64>>,
}

fn parse(input: &str) -> ParseResult<InputData> {
    use nom::{
        character::complete::{line_ending, u64},
        combinator::map,
        multi::separated_list1,
        sequence::tuple,
    };
    let elf = separated_list1(line_ending, u64);
    let elfs = separated_list1(tuple((line_ending, line_ending)), elf);
    let mut parse = map(elfs, |elfs| InputData { elfs });
    parse(input)
}

#[allow(clippy::unnecessary_wraps)]
fn part1(input: &InputData) -> AocResult<u64> {
    Ok(input
        .elfs
        .iter()
        // Sum all values
        .map(|v| v.iter().sum())
        // Find the max value
        .max()
        .unwrap_or_default())
}

#[allow(clippy::unnecessary_wraps)]
fn part2(input: &InputData) -> AocResult<u64> {
    Ok(input
        .elfs
        .iter()
        // Sum all values
        .map(|v| v.iter().sum::<u64>())
        // Sort in reverse order
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
                vec![10_000],
            ]
        }
    );
    assert_part!(parse, part1, input, 24_000);
    assert_part!(parse, part2, input, 45_000);
}
