use std::collections::HashSet;

use itertools::Itertools;
#[allow(clippy::wildcard_imports)]
use utils::*;

#[derive(Debug)]
struct InputData {
    data: Vec<Vec<i32>>,
}

fn rank_alpha(c: char) -> i32 {
    match c {
        'a'..='z' => c as i32 - 'a' as i32 + 1,
        'A'..='Z' => c as i32 - 'A' as i32 + 27,
        _ => unreachable!("Bad value passed in {c}"),
    }
}

fn intersection(i1: &[i32], i2: &[i32]) -> Vec<i32> {
    let h1: HashSet<i32> = i1.iter().copied().collect();
    let h2: HashSet<i32> = i2.iter().copied().collect();
    h1.intersection(&h2).copied().collect()
}

fn parse(input: &str) -> ParseResult<InputData> {
    use nom::{
        character::{
            complete::{line_ending, satisfy},
            is_alphabetic,
        },
        combinator::map,
        multi::{many1, separated_list1},
    };

    let alpha = satisfy(|c| is_alphabetic(c as u8));
    let char_to_i32 = map(alpha, rank_alpha);
    let line = separated_list1(line_ending, many1(char_to_i32));
    let mut parse = map(line, |data| InputData { data });
    parse(input)
}

#[allow(clippy::unnecessary_wraps)]
fn part1(input: &InputData) -> AocResult<i32> {
    Ok(input
        .data
        .iter()
        .map(|s| s.split_at(s.len() / 2))
        .map(|(i1, i2)| intersection(i1, i2))
        .flat_map(|v| v.first().copied())
        .sum())
}

#[allow(clippy::unnecessary_wraps)]
fn part2(input: &InputData) -> AocResult<i32> {
    Ok(input
        .data
        .iter()
        .tuples()
        .map(|(i1, i2, i3)| intersection(&intersection(i1, i2), i3))
        .flat_map(|v| v.first().copied())
        .sum())
}

aoc_main!(parse, part1, part2);

#[test]
fn test_rank_char() {
    assert_eq!(1, rank_alpha('a'));
    assert_eq!(26, rank_alpha('z'));
    assert_eq!(27, rank_alpha('A'));
    assert_eq!(52, rank_alpha('Z'));
}

#[test]
fn test() {
    let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
    assert_parser!(
        parse,
        input,
        InputData {
            data: input
                .lines()
                .map(|l| l.chars().map(rank_alpha).collect())
                .collect()
        }
    );
    assert_part!(parse, part1, input, 157);
    assert_part!(parse, part2, input, 70);
}
