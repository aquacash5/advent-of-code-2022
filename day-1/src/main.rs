use utils::*;

#[derive(Debug, PartialEq, Eq)]
struct InputData {
    depths: Vec<i32>,
}

fn parse(input: &str) -> ParseResult<InputData> {
    use nom::{
        character::complete::{i32, line_ending},
        combinator::map,
        multi::separated_list1,
    };
    let numbers = separated_list1(line_ending, i32);
    let mut parse = map(numbers, |depths| InputData { depths });
    parse(input)
}

fn part1(input: &InputData) -> AocResult<usize> {
    Ok(input
        .depths
        .windows(2)
        .filter(|a| a.first() < a.last())
        .count())
}

fn part2(input: &InputData) -> AocResult<usize> {
    Ok(input
        .depths
        .windows(4)
        .filter(|a| a.first() < a.last())
        .count())
}

aoc_main!(parse, part1, part2);

#[test]
fn test() {
    let input = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263";
    assert_parser!(
        parse,
        &input,
        InputData {
            depths: vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]
        }
    );
    assert_part!(parse, part1, input, 7);
    assert_part!(parse, part2, input, 5);
}
