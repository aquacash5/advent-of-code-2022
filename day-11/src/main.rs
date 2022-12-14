use std::collections::VecDeque;
#[allow(clippy::wildcard_imports)]
use utils::*;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Operand {
    Old,
    Const(u64),
}

impl Operand {
    fn parse(input: &str) -> ParseResult<Self> {
        use nom::{branch::alt, bytes::complete::tag, character::complete::u64, combinator::map};
        let old = map(tag("old"), |_| Self::Old);
        let const_ = map(u64, Self::Const);
        alt((old, const_))(input)
    }

    const fn value(&self, n: u64) -> u64 {
        match self {
            Self::Const(i) => *i,
            Self::Old => n,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Operator {
    Add,
    Multiply,
}

impl Operator {
    fn parse(input: &str) -> ParseResult<Self> {
        use nom::{branch::alt, bytes::complete::tag, combinator::map};
        let add = map(tag(" + "), |_| Self::Add);
        let mul = map(tag(" * "), |_| Self::Multiply);
        alt((add, mul))(input)
    }

    const fn operate(self, i: u64, j: u64) -> u64 {
        match self {
            Self::Add => i + j,
            Self::Multiply => i * j,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Relief(u64);

impl Relief {
    const fn relieve(self, item: Item) -> Item {
        Item(item.0 / self.0)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Operation {
    operator: Operator,
    operand1: Operand,
    operand2: Operand,
}

impl Operation {
    fn parse(input: &str) -> ParseResult<Self> {
        use nom::{combinator::map, sequence::tuple};
        map(
            tuple((Operand::parse, Operator::parse, Operand::parse)),
            |(operand1, operator, operand2)| Self {
                operator,
                operand1,
                operand2,
            },
        )(input)
    }

    const fn operate(&self, n: u64) -> u64 {
        self.operator
            .operate(self.operand1.value(n), self.operand2.value(n))
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Item(u64);

impl Item {
    fn parse(input: &str) -> ParseResult<Self> {
        use nom::{character::complete::u64, combinator::map};

        map(u64, Self)(input)
    }

    const fn inspect(self, op: &Operation) -> Self {
        Self(op.operate(self.0))
    }

    const fn reduce(self, amount: u64) -> Self {
        Self(self.0 % amount)
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Monkey {
    items: VecDeque<Item>,
    operation: Operation,
    test: u64,
    true_test: usize,
    false_test: usize,
}

impl Monkey {
    fn parse(input: &str) -> ParseResult<Self> {
        use nom::{
            bytes::complete::tag,
            character::complete::{line_ending, u64},
            combinator::map,
            multi::separated_list1,
            sequence::{delimited, pair, tuple},
            ToUsize,
        };
        let heading = delimited(tag("Monkey "), u64, pair(tag(":"), line_ending));
        let items = map(
            delimited(
                tag("  Starting items: "),
                separated_list1(tag(", "), Item::parse),
                line_ending,
            ),
            VecDeque::from,
        );
        let operation = delimited(tag("  Operation: new = "), Operation::parse, line_ending);
        let test = delimited(tag("  Test: divisible by "), u64, line_ending);
        let true_test = delimited(tag("    If true: throw to monkey "), u64, line_ending);
        let false_test = delimited(tag("    If false: throw to monkey "), u64, line_ending);
        map(
            tuple((heading, items, operation, test, true_test, false_test)),
            |(_, items, operation, test, true_test, false_test)| Self {
                items,
                operation,
                test,
                true_test: true_test.to_usize(),
                false_test: false_test.to_usize(),
            },
        )(input)
    }

    fn catch(&mut self, n: Item) {
        self.items.push_back(n);
    }

    const fn throw(&self, item: Item) -> usize {
        if item.0 % self.test == 0 {
            self.true_test
        } else {
            self.false_test
        }
    }

    fn inspect(&mut self, relief: Relief) -> Option<Item> {
        let item = self.items.pop_front()?;
        let item = item.inspect(&self.operation);
        Some(relief.relieve(item))
    }
}

#[derive(Debug, PartialEq)]
struct InputData {
    monkeys: Vec<Monkey>,
}

fn parse(input: &str) -> ParseResult<InputData> {
    use nom::{character::complete::line_ending, combinator::map, multi::separated_list1};
    map(separated_list1(line_ending, Monkey::parse), |monkeys| {
        InputData { monkeys }
    })(input)
}

#[allow(clippy::unnecessary_wraps)]
fn part1(input: &InputData) -> AocResult<usize> {
    const ROUNDS: usize = 20;
    let mut monkeys: Vec<Monkey> = input.monkeys.clone();
    let mut inspected: Vec<usize> = vec![0; monkeys.len()];
    let item_reduce: u64 = input.monkeys.iter().map(|m| m.test).product();

    for _round in 0..ROUNDS {
        for turn in 0..monkeys.len() {
            while let Some(item) = monkeys[turn].inspect(Relief(3)) {
                let recipient = monkeys[turn].throw(item);
                monkeys[recipient].catch(item.reduce(item_reduce));
                inspected[turn] += 1;
            }
        }
    }
    inspected.sort_by_key(|i| std::cmp::Reverse(*i));
    Ok(inspected.iter().take(2).product())
}

#[allow(clippy::unnecessary_wraps)]
fn part2(input: &InputData) -> AocResult<usize> {
    const ROUNDS: usize = 10_000;
    let mut monkeys: Vec<Monkey> = input.monkeys.clone();
    let mut inspected: Vec<usize> = vec![0; monkeys.len()];
    let item_reduce: u64 = input.monkeys.iter().map(|m| m.test).product();

    for _round in 0..ROUNDS {
        for turn in 0..monkeys.len() {
            while let Some(item) = monkeys[turn].inspect(Relief(1)) {
                let recipient = monkeys[turn].throw(item);
                monkeys[recipient].catch(item.reduce(item_reduce));
                inspected[turn] += 1;
            }
        }
    }
    inspected.sort_by_key(|i| std::cmp::Reverse(*i));
    Ok(inspected.iter().take(2).product())
}

aoc_main!(parse, part1, part2);

#[test]
fn test() {
    let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";
    assert_parser!(
        parse,
        input,
        InputData {
            monkeys: vec![
                Monkey {
                    items: vec![Item(79), Item(98)].into(),
                    operation: Operation {
                        operator: Operator::Multiply,
                        operand1: Operand::Old,
                        operand2: Operand::Const(19),
                    },
                    test: 23,
                    true_test: 2,
                    false_test: 3,
                },
                Monkey {
                    items: vec![Item(54), Item(65), Item(75), Item(74)].into(),
                    operation: Operation {
                        operator: Operator::Add,
                        operand1: Operand::Old,
                        operand2: Operand::Const(6),
                    },
                    test: 19,
                    true_test: 2,
                    false_test: 0,
                },
                Monkey {
                    items: vec![Item(79), Item(60), Item(97)].into(),
                    operation: Operation {
                        operator: Operator::Multiply,
                        operand1: Operand::Old,
                        operand2: Operand::Old,
                    },
                    test: 13,
                    true_test: 1,
                    false_test: 3,
                },
                Monkey {
                    items: vec![Item(74)].into(),
                    operation: Operation {
                        operator: Operator::Add,
                        operand1: Operand::Old,
                        operand2: Operand::Const(3),
                    },
                    test: 17,
                    true_test: 0,
                    false_test: 1,
                }
            ]
        }
    );
    assert_part!(parse, part1, input, 10_605_usize);
    assert_part!(parse, part2, input, 2_713_310_158_usize);
}
