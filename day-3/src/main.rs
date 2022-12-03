use std::collections::HashSet;

use itertools::Itertools;
#[allow(clippy::wildcard_imports)]
use utils::*;

#[derive(Debug)]
struct InputData {
    data: Vec<String>,
}

fn rank_char(c: char) -> u64 {
    match c {
        'a'..='z' => c as u64 - 'a' as u64 + 1,
        'A'..='Z' => c as u64 - 'A' as u64 + 27,
        _ => 0,
    }
}

fn str_intersection(s1: &str, s2: &str) -> String {
    let h1: HashSet<char> = HashSet::from_iter(s1.chars());
    let h2: HashSet<char> = HashSet::from_iter(s2.chars());
    h1.intersection(&h2).join("")
}

fn parse(input: &str) -> ParseResult<InputData> {
    use nom::{
        character::complete::{alpha0, line_ending},
        combinator::map,
        multi::separated_list1,
    };
    let line = separated_list1(line_ending, alpha0);
    let mut parse = map(line, |v| InputData {
        data: v.iter().map(|s: &&str| s.to_string()).collect(),
    });
    parse(input)
}

#[allow(clippy::unnecessary_wraps)]
fn part1(input: &InputData) -> AocResult<u64> {
    Ok(input
        .data
        .iter()
        .map(|s| s.split_at(s.len() / 2))
        .map(|(s1, s2)| str_intersection(s1, s2))
        .flat_map(|s| s.chars().next())
        .map(rank_char)
        .sum())
}

#[allow(clippy::unnecessary_wraps)]
fn part2(input: &InputData) -> AocResult<u64> {
    Ok(input
        .data
        .iter()
        .tuples()
        .map(|(s1, s2, s3)| str_intersection(&str_intersection(s1, s2), s3))
        .flat_map(|s| s.chars().next())
        .map(rank_char)
        .sum())
}

aoc_main!(parse, part1, part2);

#[test]
fn test_char_rank() {
    assert_eq!(1, rank_char('a'));
    assert_eq!(26, rank_char('z'));
    assert_eq!(27, rank_char('A'));
    assert_eq!(52, rank_char('Z'));
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
            data: vec![
                "vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
                "PmmdzqPrVvPwwTWBwg".to_string(),
                "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_string(),
                "ttgJtRGJQctTZtZT".to_string(),
                "CrZsJsPPZsGzwwsLwLmpwMDw".to_string(),
            ]
        }
    );
    assert_part!(parse, part1, input, 157);
    assert_part!(parse, part2, input, 70);
}
