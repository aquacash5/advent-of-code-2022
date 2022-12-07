use itertools::Itertools;
#[allow(clippy::wildcard_imports)]
use utils::*;

#[derive(Debug)]
struct InputData(Vec<char>);

#[allow(clippy::unnecessary_wraps)]
fn parse(input: &str) -> ParseResult<InputData> {
    Ok(("", InputData(input.chars().collect())))
}

#[allow(clippy::unnecessary_wraps)]
fn part1(input: &InputData) -> AocResult<usize> {
    const WINDOW_SIZE: usize = 4;

    Ok(input
        .0
        .windows(WINDOW_SIZE)
        .find_position(|a| a.iter().combinations(2).all(|v| v[0] != v[1]))
        .map_or(0, |(i, _)| i)
        + WINDOW_SIZE)
}

#[allow(clippy::unnecessary_wraps)]
fn part2(input: &InputData) -> AocResult<usize> {
    const WINDOW_SIZE: usize = 14;

    Ok(input
        .0
        .windows(WINDOW_SIZE)
        .find_position(|a| a.iter().combinations(2).all(|v| v[0] != v[1]))
        .map_or(0, |(i, _)| i)
        + WINDOW_SIZE)
}

aoc_main!(parse, part1, part2);

#[test]
fn test() {
    assert_part!(parse, part1, "mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7);
    assert_part!(parse, part1, "bvwbjplbgvbhsrlpgdmjqwftvncz", 5);
    assert_part!(parse, part1, "nppdvjthqldpwncqszvftbrmjlhg", 6);
    assert_part!(parse, part1, "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10);
    assert_part!(parse, part1, "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11);

    assert_part!(parse, part2, "mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19);
    assert_part!(parse, part2, "bvwbjplbgvbhsrlpgdmjqwftvncz", 23);
    assert_part!(parse, part2, "nppdvjthqldpwncqszvftbrmjlhg", 23);
    assert_part!(parse, part2, "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29);
    assert_part!(parse, part2, "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26);
}
