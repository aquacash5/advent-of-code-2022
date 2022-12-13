use std::cmp::Ordering;

use itertools::Itertools;
#[allow(clippy::wildcard_imports)]
use utils::*;

#[derive(Debug, Clone, Eq)]
enum Packet {
    List(Vec<Packet>),
    Value(i32),
}

impl Packet {
    fn parse(input: &str) -> ParseResult<Packet> {
        use nom::{
            branch::alt,
            character::complete::{char, i32},
            combinator::map,
            multi::separated_list0,
            sequence::delimited,
        };

        let list = map(
            delimited(
                char('['),
                separated_list0(char(','), Packet::parse),
                char(']'),
            ),
            Self::List,
        );
        let value = map(i32, Self::Value);
        alt((value, list))(input)
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Value(l), Self::Value(r)) => l.cmp(r),
            (Self::List(l), Self::List(r)) => l.cmp(r),
            (Self::Value(l), Self::List(_)) => Self::List(vec![Self::Value(*l)]).cmp(other),
            (Self::List(_), Self::Value(r)) => self.cmp(&Self::List(vec![Self::Value(*r)])),
        }
    }
}

#[derive(Debug, PartialEq)]
struct InputData {
    packet_pairs: Vec<(Packet, Packet)>,
}

fn parse(input: &str) -> ParseResult<InputData> {
    use nom::{
        character::complete::line_ending,
        combinator::map,
        multi::separated_list1,
        sequence::{pair, separated_pair},
    };
    let packet_pair = separated_pair(Packet::parse, line_ending, Packet::parse);
    let double_line = pair(line_ending, line_ending);
    let mut parser = map(separated_list1(double_line, packet_pair), |packet_pairs| {
        InputData { packet_pairs }
    });
    parser(input)
}

#[allow(clippy::unnecessary_wraps)]
fn part1(input: &InputData) -> AocResult<usize> {
    Ok(input
        .packet_pairs
        .iter()
        .enumerate()
        .filter_map(|(i, (l, r))| if l < r { Some(i + 1) } else { None })
        .sum())
}

#[allow(clippy::unnecessary_wraps)]
fn part2(input: &InputData) -> AocResult<usize> {
    let divider_packets: [Packet; 2] = [
        Packet::List(vec![Packet::List(vec![Packet::Value(2)])]),
        Packet::List(vec![Packet::List(vec![Packet::Value(6)])]),
    ];
    Ok(input
        .packet_pairs
        .iter()
        .flat_map(|(l, r)| [l, r])
        .chain(divider_packets.iter())
        .sorted()
        .enumerate()
        .filter_map(|(i, p)| {
            if divider_packets.contains(p) {
                Some(i + 1)
            } else {
                None
            }
        })
        .product())
}

aoc_main!(parse, part1, part2);

#[test]
fn test() {
    use Packet::{List, Value};

    let input = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";
    assert_parser!(
        parse,
        input,
        InputData {
            packet_pairs: vec![
                (
                    List(vec![Value(1), Value(1), Value(3), Value(1), Value(1)]),
                    List(vec![Value(1), Value(1), Value(5), Value(1), Value(1)])
                ),
                (
                    List(vec![
                        List(vec![Value(1)]),
                        List(vec![Value(2), Value(3), Value(4)])
                    ]),
                    List(vec![List(vec![Value(1)]), Value(4)]),
                ),
                (
                    List(vec![Value(9)]),
                    List(vec![List(vec![Value(8), Value(7), Value(6)])])
                ),
                (
                    List(vec![List(vec![Value(4), Value(4)]), Value(4), Value(4)]),
                    List(vec![
                        List(vec![Value(4), Value(4)]),
                        Value(4),
                        Value(4),
                        Value(4),
                    ])
                ),
                (
                    List(vec![Value(7), Value(7), Value(7), Value(7)]),
                    List(vec![Value(7), Value(7), Value(7)])
                ),
                (List(vec![]), List(vec![Value(3)])),
                (
                    List(vec![List(vec![List(vec![])])]),
                    List(vec![List(vec![])])
                ),
                (
                    List(vec![
                        Value(1),
                        List(vec![
                            Value(2),
                            List(vec![
                                Value(3),
                                List(vec![Value(4), List(vec![Value(5), Value(6), Value(7)])])
                            ])
                        ]),
                        Value(8),
                        Value(9)
                    ]),
                    List(vec![
                        Value(1),
                        List(vec![
                            Value(2),
                            List(vec![
                                Value(3),
                                List(vec![Value(4), List(vec![Value(5), Value(6), Value(0)])])
                            ])
                        ]),
                        Value(8),
                        Value(9)
                    ])
                )
            ]
        }
    );
    assert_part!(parse, part1, input, 13);
    assert_part!(parse, part2, input, 140);
}
