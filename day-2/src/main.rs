#[allow(clippy::wildcard_imports)]
use utils::*;

trait Score {
    fn score(&self) -> i32;
}

#[derive(Debug, thiserror::Error)]
enum ParseError {
    #[error("Unable to parse `{0}` into Move")]
    Move(char),
    #[error("Unable to parse `{0}` into Outcome")]
    Outcome(char),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    const fn rig(self, outcome: Outcome) -> Self {
        use Move::{Paper, Rock, Scissors};
        use Outcome::{Draw, Lose, Win};
        match (outcome, self) {
            (Draw, _) => self,
            (Lose, Rock) | (Win, Paper) => Scissors,
            (Lose, Paper) | (Win, Scissors) => Rock,
            (Lose, Scissors) | (Win, Rock) => Paper,
        }
    }

    const fn outcome(self, other: Self) -> Outcome {
        use Move::{Paper, Rock, Scissors};
        use Outcome::{Draw, Lose, Win};
        match (self, other) {
            (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => Lose,
            (Paper, Rock) | (Scissors, Paper) | (Rock, Scissors) => Win,
            _ => Draw,
        }
    }
}

impl TryFrom<char> for Move {
    type Error = ParseError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use Move::{Paper, Rock, Scissors};
        match value {
            'A' | 'X' => Ok(Rock),
            'B' | 'Y' => Ok(Paper),
            'C' | 'Z' => Ok(Scissors),
            c => Err(ParseError::Move(c)),
        }
    }
}

impl Score for Move {
    fn score(&self) -> i32 {
        use Move::{Paper, Rock, Scissors};
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}

#[derive(Debug)]
struct Round {
    player1: Move,
    player2: Move,
}

impl Round {
    fn score_player2(&self) -> i32 {
        self.player2.score() + self.player2.outcome(self.player1).score()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

impl TryFrom<char> for Outcome {
    type Error = ParseError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use Outcome::{Draw, Lose, Win};
        match value {
            'X' => Ok(Lose),
            'Y' => Ok(Draw),
            'Z' => Ok(Win),
            _ => Err(ParseError::Outcome(value)),
        }
    }
}

impl Score for Outcome {
    fn score(&self) -> i32 {
        use Outcome::{Draw, Lose, Win};
        match self {
            Lose => 0,
            Draw => 3,
            Win => 6,
        }
    }
}

#[derive(Debug)]
struct StrategicRound {
    player1: Move,
    outcome: Outcome,
}

impl StrategicRound {
    fn score_player2(&self) -> i32 {
        self.player1.rig(self.outcome).score() + self.outcome.score()
    }
}

#[derive(Debug)]
struct InputData1 {
    rounds: Vec<Round>,
}

#[derive(Debug)]
struct InputData2 {
    rounds: Vec<StrategicRound>,
}

fn parse1(input: &str) -> ParseResult<InputData1> {
    use nom::{
        character::complete::{char, line_ending, one_of},
        combinator::{map, map_res},
        multi::separated_list1,
        sequence::separated_pair,
    };
    let player1 = one_of("ABC");
    let player2 = one_of("XYZ");
    let round = separated_pair(player1, char(' '), player2);
    let round = map_res(round, |(p1, p2)| {
        Ok::<Round, ParseError>(Round {
            player1: p1.try_into()?,
            player2: p2.try_into()?,
        })
    });
    let rounds = separated_list1(line_ending, round);
    let mut parse = map(rounds, |v| InputData1 { rounds: v });
    parse(input)
}

fn parse2(input: &str) -> ParseResult<InputData2> {
    use nom::{
        character::complete::{char, line_ending, one_of},
        combinator::{map, map_res},
        multi::separated_list1,
        sequence::tuple,
    };
    let player1 = one_of("ABC");
    let player2 = one_of("XYZ");
    let round = tuple((player1, char(' '), player2));
    let round = map_res(round, |(p1, _, p2)| {
        Ok::<StrategicRound, ParseError>(StrategicRound {
            player1: p1.try_into()?,
            outcome: p2.try_into()?,
        })
    });
    let rounds = separated_list1(line_ending, round);
    let mut parse = map(rounds, |v| InputData2 { rounds: v });
    parse(input)
}

#[allow(clippy::unnecessary_wraps)]
fn part1(input: &InputData1) -> AocResult<i32> {
    Ok(input.rounds.iter().map(Round::score_player2).sum())
}

#[allow(clippy::unnecessary_wraps)]
fn part2(input: &InputData2) -> AocResult<i32> {
    Ok(input.rounds.iter().map(StrategicRound::score_player2).sum())
}

aoc_main!(parse1, parse2, part1, part2);

#[test]
fn test_rounds() {
    use Move::{Paper, Rock};
    assert_eq!(
        Round {
            player1: Rock,
            player2: Paper
        }
        .score_player2(),
        8
    );
}

#[test]
fn test1() {
    use Move::{Paper, Rock, Scissors};
    let input = "A Y\nB X\nC Z";
    assert_parser!(
        parse1,
        input,
        InputData1 {
            rounds: vec![
                Round {
                    player1: Rock,
                    player2: Paper
                },
                Round {
                    player1: Paper,
                    player2: Rock
                },
                Round {
                    player1: Scissors,
                    player2: Scissors
                },
            ]
        }
    );
    assert_part!(parse1, part1, input, 15);
}

#[test]
fn test2() {
    use Move::{Paper, Rock, Scissors};
    use Outcome::{Draw, Lose, Win};
    let input = "A Y\nB X\nC Z";
    assert_parser!(
        parse2,
        input,
        InputData2 {
            rounds: vec![
                StrategicRound {
                    player1: Rock,
                    outcome: Draw,
                },
                StrategicRound {
                    player1: Paper,
                    outcome: Lose,
                },
                StrategicRound {
                    player1: Scissors,
                    outcome: Win,
                }
            ]
        }
    );
    assert_part!(parse2, part2, input, 12);
}
