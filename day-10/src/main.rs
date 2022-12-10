use itertools::Itertools;
#[allow(clippy::wildcard_imports)]
use utils::*;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Instruction {
    NoOp,
    AddX(i64),
}

#[derive(Debug, PartialEq)]
struct InputData {
    instructions: Vec<Instruction>,
}

fn parse(input: &str) -> ParseResult<InputData> {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{i64, line_ending},
        combinator::map,
        multi::separated_list1,
        sequence::preceded,
    };
    let noop = map(tag("noop"), |_| Instruction::NoOp);
    let addx = map(preceded(tag("addx "), i64), Instruction::AddX);
    let mut parser = map(
        separated_list1(line_ending, alt((noop, addx))),
        |instructions| InputData { instructions },
    );
    parser(input)
}

#[allow(clippy::unnecessary_wraps)]
fn part1(input: &InputData) -> AocResult<i64> {
    let mut strengths: Vec<i64> = Vec::new();
    let mut cycle_count = 0;
    let mut x = 1;

    for &instruction in &input.instructions {
        match instruction {
            Instruction::NoOp => {
                cycle_count += 1;
                strengths.push(cycle_count * x);
            }
            Instruction::AddX(n) => {
                cycle_count += 1;
                strengths.push(cycle_count * x);
                cycle_count += 1;
                strengths.push(cycle_count * x);
                x += n;
            }
        }
    }
    Ok(strengths
        .iter()
        .skip(19)
        .step_by(40)
        .sorted_by_key(|i| std::cmp::Reverse(*i))
        .take(6)
        .sum())
}

#[allow(clippy::unnecessary_wraps)]
fn part2(input: &InputData) -> AocResult<Vec<String>> {
    let mut pixels: Vec<i64> = Vec::new();
    let mut x = 1;

    for &instruction in &input.instructions {
        match instruction {
            Instruction::NoOp => {
                pixels.push(x);
            }
            Instruction::AddX(n) => {
                pixels.push(x);
                pixels.push(x);
                x += n;
            }
        }
    }
    let mut crt: Vec<String> = vec![];
    let mut crt_line = String::new();

    for line in 0..6 {
        for pixel in 0..40 {
            if let Some(x) = pixels.get((line * 40) + pixel) {
                if (x - 1..=x + 1).contains(&(pixel as i64)) {
                    crt_line += "#";
                } else {
                    crt_line += " ";
                }
            } else {
                crt_line += " ";
            }
        }
        crt.push(crt_line.clone());
        crt_line.clear();
    }
    Ok(crt)
}

aoc_main!(parse, part1, part2);

#[test]
fn test() {
    use Instruction::*;
    let input1 = "noop\naddx 3\naddx -5";
    assert_parser!(
        parse,
        input1,
        InputData {
            instructions: vec![NoOp, AddX(3), AddX(-5)]
        }
    );
    let input2 = "addx 15\naddx -11\naddx 6\naddx -3\naddx 5\naddx -1\naddx -8\naddx 13\naddx 4\nnoop\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx -35\naddx 1\naddx 24\naddx -19\naddx 1\naddx 16\naddx -11\nnoop\nnoop\naddx 21\naddx -15\nnoop\nnoop\naddx -3\naddx 9\naddx 1\naddx -3\naddx 8\naddx 1\naddx 5\nnoop\nnoop\nnoop\nnoop\nnoop\naddx -36\nnoop\naddx 1\naddx 7\nnoop\nnoop\nnoop\naddx 2\naddx 6\nnoop\nnoop\nnoop\nnoop\nnoop\naddx 1\nnoop\nnoop\naddx 7\naddx 1\nnoop\naddx -13\naddx 13\naddx 7\nnoop\naddx 1\naddx -33\nnoop\nnoop\nnoop\naddx 2\nnoop\nnoop\nnoop\naddx 8\nnoop\naddx -1\naddx 2\naddx 1\nnoop\naddx 17\naddx -9\naddx 1\naddx 1\naddx -3\naddx 11\nnoop\nnoop\naddx 1\nnoop\naddx 1\nnoop\nnoop\naddx -13\naddx -19\naddx 1\naddx 3\naddx 26\naddx -30\naddx 12\naddx -1\naddx 3\naddx 1\nnoop\nnoop\nnoop\naddx -9\naddx 18\naddx 1\naddx 2\nnoop\nnoop\naddx 9\nnoop\nnoop\nnoop\naddx -1\naddx 2\naddx -37\naddx 1\naddx 3\nnoop\naddx 15\naddx -21\naddx 22\naddx -6\naddx 1\nnoop\naddx 2\naddx 1\nnoop\naddx -10\nnoop\nnoop\naddx 20\naddx 1\naddx 2\naddx 2\naddx -6\naddx -11\nnoop\nnoop\nnoop";
    assert_part!(parse, part1, input2, 13140);

    let output2 = vec![
        "##  ##  ##  ##  ##  ##  ##  ##  ##  ##  ",
        "###   ###   ###   ###   ###   ###   ### ",
        "####    ####    ####    ####    ####    ",
        "#####     #####     #####     #####     ",
        "######      ######      ######      ####",
        "#######       #######       #######     ",
    ];
    assert_part!(parse, part2, input2, output2);
}
