use std::collections::HashSet;

use itertools::Itertools;
use lending_iterator::{lending_iterator::constructors::windows_mut, LendingIterator};
#[allow(clippy::wildcard_imports)]
use utils::*;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Movement {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Rope1 {
    head: (i32, i32),
    tail: (i32, i32),
}

impl Rope1 {
    fn step(&mut self, head_move: Movement) -> (i32, i32) {
        use Movement::{Down, Left, Right, Up};

        let prev = self.head;
        match head_move {
            Up => self.head.1 += 1,
            Down => self.head.1 -= 1,
            Left => self.head.0 -= 1,
            Right => self.head.0 += 1,
        }
        if self.head.0.abs_diff(self.tail.0) > 1 || self.head.1.abs_diff(self.tail.1) > 1 {
            self.tail = prev;
        }
        self.tail
    }
}

const fn unit(i: i32) -> i32 {
    if i.is_positive() {
        1
    } else {
        -1
    }
}

fn create_grid(base: (i32, i32)) -> Vec<(i32, i32)> {
    let x_range = (base.0 - 1)..=(base.0 + 1);
    let y_range = (base.1 - 1)..=(base.1 + 1);
    x_range.cartesian_product(y_range).collect()
}

#[derive(Debug, PartialEq, Clone)]
struct Rope2 {
    body: Vec<(i32, i32)>,
}

impl Rope2 {
    fn step(&mut self, head_move: Movement) -> (i32, i32) {
        use Movement::{Down, Left, Right, Up};

        match head_move {
            Up => self.body[0].1 += 1,
            Down => self.body[0].1 -= 1,
            Left => self.body[0].0 -= 1,
            Right => self.body[0].0 += 1,
        }
        // https://github.com/ChristopherBiscardi/advent-of-code/blob/0aa7054ae4c38d348d93e1669a88bcdea5c61d1d/2022/rust/day-09/src/lib.rs#L122-L206
        // Still trying to figure out what the rules for multi knot ropes are.
        let mut rope_windows = self.body.windows_mut::<2>();
        while let Some([ref mut head, ref mut tail]) = rope_windows.next() {
            let tail_is_connected = create_grid(*head).contains(tail);

            if !tail_is_connected {
                if head.0 == tail.0 {
                    tail.1 += unit(head.1 - tail.1);
                } else if head.1 == tail.1 {
                    tail.0 += unit(head.0 - tail.0);
                } else {
                    // diagonal
                    let head_options = create_grid(*head);
                    let tail_options = create_grid(*tail);

                    let maybe_new_tail: Vec<(i32, i32)> = tail_options
                        .iter()
                        .filter(|&tuple| head_options.contains(tuple))
                        .copied()
                        .collect();
                    match maybe_new_tail.len() {
                        2 => {
                            let head_positions = [
                                (head.0 - 1, head.1),
                                (head.0 + 1, head.1),
                                (head.0, head.1 - 1),
                                (head.0, head.1 + 1),
                            ];
                            let &next = maybe_new_tail
                                .iter()
                                .find(|tuple| head_positions.contains(tuple))
                                .unwrap();
                            *tail = next;
                        }
                        1 => {
                            *tail = maybe_new_tail[0];
                        }
                        _ => {
                            panic!("unknown tail length");
                        }
                    };
                }
            }
        }
        *self.body.last().unwrap()
    }
}

#[derive(Debug, PartialEq)]
struct InputData {
    movements: Vec<(Movement, u32)>,
}

fn parse(input: &str) -> ParseResult<InputData> {
    use nom::{
        character::complete::{char, line_ending, one_of, u32},
        combinator::map,
        multi::separated_list1,
        sequence::separated_pair,
    };
    use Movement::{Down, Left, Right, Up};
    let movement = map(one_of("UDLR"), |c| match c {
        'R' => Right,
        'L' => Left,
        'U' => Up,
        'D' => Down,
        _ => unreachable!("Nom no worky"),
    });
    let line = separated_pair(movement, char(' '), u32);
    let mut parser = map(separated_list1(line_ending, line), |movements| InputData {
        movements,
    });
    parser(input)
}

#[allow(clippy::unnecessary_wraps)]
fn part1(input: &InputData) -> AocResult<usize> {
    let mut set: HashSet<(i32, i32)> = HashSet::from([(0, 0)]);
    let mut rope = Rope1 {
        head: (0, 0),
        tail: (0, 0),
    };
    for (d, n) in &input.movements {
        for _ in 0..*n {
            set.insert(rope.step(*d));
        }
    }
    Ok(set.len())
}

#[allow(clippy::unnecessary_wraps)]
fn part2(input: &InputData) -> AocResult<usize> {
    let mut set: HashSet<(i32, i32)> = HashSet::from([(0, 0)]);
    let mut rope = Rope2 {
        body: vec![(0, 0); 10],
    };
    for (d, n) in &input.movements {
        for _ in 0..*n {
            set.insert(rope.step(*d));
        }
    }
    Ok(set.len())
}

aoc_main!(parse, part1, part2);

#[test]
fn test() {
    use Movement::*;
    let input1 = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2";
    assert_parser!(
        parse,
        input1,
        InputData {
            movements: vec![
                (Right, 4),
                (Up, 4),
                (Left, 3),
                (Down, 1),
                (Right, 4),
                (Down, 1),
                (Left, 5),
                (Right, 2),
            ]
        }
    );
    assert_part!(parse, part1, input1, 13);
    let input2 = "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20";
    assert_part!(parse, part2, input2, 36);
}
