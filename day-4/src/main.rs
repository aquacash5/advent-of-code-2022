use std::ops::RangeInclusive;

use nom::IResult;
#[allow(clippy::wildcard_imports)]
use utils::*;

#[derive(Debug)]
struct InputData {
    assignments: Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>,
}

fn assignment(input: &str) -> IResult<&str, RangeInclusive<u32>> {
    use nom::{
        character::complete::{char, u32},
        combinator::map,
        sequence::separated_pair,
    };
    map(separated_pair(u32, char('-'), u32), |(s, e)| s..=e)(input)
}

fn parse(input: &str) -> ParseResult<InputData> {
    use nom::{
        character::complete::{char, line_ending},
        combinator::map,
        multi::separated_list1,
        sequence::separated_pair,
    };
    let assignment_pair = separated_pair(assignment, char(','), assignment);
    let mut parser = map(
        separated_list1(line_ending, assignment_pair),
        |assignments| InputData { assignments },
    );
    parser(input)
}

fn is_range_subset(r1: &RangeInclusive<u32>, r2: &RangeInclusive<u32>) -> bool {
    r1.contains(r2.start()) && r1.contains(r2.end())
}

fn either_subset((r1, r2): &&(RangeInclusive<u32>, RangeInclusive<u32>)) -> bool {
    is_range_subset(r1, r2) || is_range_subset(r2, r1)
}

fn overlap((r1, r2): &&(RangeInclusive<u32>, RangeInclusive<u32>)) -> bool {
    !(r1.end() < r2.start() || r2.end() < r1.start())
}

#[allow(clippy::unnecessary_wraps)]
fn part1(input: &InputData) -> AocResult<usize> {
    Ok(input.assignments.iter().filter(either_subset).count())
}

#[allow(clippy::unnecessary_wraps)]
fn part2(input: &InputData) -> AocResult<usize> {
    Ok(input.assignments.iter().filter(overlap).count())
}

aoc_main!(parse, part1, part2);

#[test]
fn test() {
    let input = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8";
    assert_parser!(
        parse,
        input,
        InputData {
            assignments: vec![
                (2..=4, 6..=8),
                (2..=3, 4..=5),
                (5..=7, 7..=9),
                (2..=8, 3..=7),
                (6..=6, 4..=6),
                (2..=6, 4..=8),
            ]
        }
    );
    assert_part!(parse, part1, input, 2);
    assert_part!(parse, part2, input, 4);
}
