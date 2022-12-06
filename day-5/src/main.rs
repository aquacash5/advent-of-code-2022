use itertools::Itertools;
#[allow(clippy::wildcard_imports)]
use utils::*;

#[derive(Debug, Clone, Copy)]
struct Instruction {
    amount: usize,
    source: usize,
    destination: usize,
}

#[derive(Debug, Clone)]
struct InputData {
    crates: Vec<Vec<char>>,
    instructions: Vec<Instruction>,
}

// https://stackoverflow.com/a/64499219/804119
fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v
        .into_iter()
        .map(std::iter::IntoIterator::into_iter)
        .collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn parse(input: &str) -> ParseResult<InputData> {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{char, line_ending, one_of, u32},
        combinator::map,
        multi::{many1, separated_list1},
        sequence::{delimited, preceded, separated_pair, terminated, tuple},
    };
    // Character set
    let alphas: String = ('A'..='Z').collect();

    // Crates
    let alpha = delimited(char('['), one_of(alphas.as_str()), char(']'));
    let space = delimited(char(' '), char(' '), char(' '));
    let line = separated_list1(char(' '), alt((alpha, space)));
    let crates = separated_list1(line_ending, line);

    // Ignore Stack labels
    let line = tuple((line_ending, many1(one_of("0123456789 ")), line_ending));
    let crates = terminated(crates, line);

    // Instructions
    let move_ = preceded(tag("move "), u32);
    let from = preceded(tag(" from "), u32);
    let to = preceded(tag(" to "), u32);
    let instruction = map(tuple((move_, from, to)), |(a, f, t)| Instruction {
        amount: a as usize,
        // Change for 0 based indexing
        source: f as usize - 1,
        destination: t as usize - 1,
    });
    let instructions = separated_list1(line_ending, instruction);

    // Parse everything
    let body_split = separated_pair(crates, line_ending, instructions);
    let mut parse = map(body_split, |(crates, instructions)| {
        // Rotate crates
        let mut crates = transpose(crates);

        // Reverse and truncate crate stacks
        for stack in &mut crates {
            stack.reverse();
            // Remove space characters
            if let Some((space, _)) = stack.iter().find_position(|c| c == &&' ') {
                stack.truncate(space);
            }
        }

        InputData {
            crates,
            instructions,
        }
    });
    parse(input)
}

#[allow(clippy::unnecessary_wraps)]
fn part1(input: &InputData) -> AocResult<String> {
    let mut crates = input.crates.clone();
    for i in &input.instructions {
        for _ in 0..i.amount {
            let temp = crates[i.source].pop().unwrap();
            crates[i.destination].push(temp);
        }
    }
    Ok(crates.iter().map(|cs| cs.last().unwrap()).collect())
}

#[allow(clippy::unnecessary_wraps)]
fn part2(input: &InputData) -> AocResult<String> {
    let mut crates = input.crates.clone();
    for i in &input.instructions {
        let temp = crates[i.source].len() - i.amount;
        let mut temp = crates[i.source].split_off(temp);
        crates[i.destination].append(&mut temp);
    }
    Ok(crates.iter().map(|cs| cs.last().unwrap()).collect())
}

aoc_main!(parse, part1, part2);

#[test]
fn test() {
    let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
    assert_parser!(
        parse,
        input,
        InputData {
            crates: vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P'],],
            instructions: vec![
                Instruction {
                    amount: 1,
                    source: 1,
                    destination: 0
                },
                Instruction {
                    amount: 3,
                    source: 0,
                    destination: 2
                },
                Instruction {
                    amount: 2,
                    source: 1,
                    destination: 0
                },
                Instruction {
                    amount: 1,
                    source: 0,
                    destination: 1
                },
            ]
        }
    );
    assert_part!(parse, part1, input, "CMZ");
    assert_part!(parse, part2, input, "MCD");
}
