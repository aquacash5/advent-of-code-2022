pub use anyhow::Result as AocResult;
pub use clap::Parser;
use clap::{builder::PossibleValue, ValueEnum};
use std::path::PathBuf;

pub type ParseResult<'a, T> = nom::IResult<&'a str, T>;

#[derive(Debug, Parser)]
#[command(author, about, long_about = None)]
pub struct Cli {
    /// Input file
    input_file: PathBuf,

    /// Part of the task to do
    #[arg(short, long, value_enum, default_value_t)]
    part: SolutionPart,
}

impl Cli {
    pub fn input(&self) -> AocResult<String> {
        Ok(std::fs::read_to_string(&self.input_file)?)
    }

    pub fn should_run(&self, part: SolutionPart) -> bool {
        if self.part == SolutionPart::Both {
            true
        } else {
            self.part == part
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum SolutionPart {
    PartOne,
    PartTwo,
    #[default]
    Both,
}

impl ValueEnum for SolutionPart {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::PartOne, Self::PartTwo, Self::Both]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        use SolutionPart::*;
        match self {
            PartOne => Some(PossibleValue::new("1")),
            PartTwo => Some(PossibleValue::new("2")),
            Both => Some(PossibleValue::new("both")),
        }
    }

    fn from_str(input: &str, ignore_case: bool) -> Result<Self, String> {
        use SolutionPart::*;
        let mut input_new = input.to_string();
        if ignore_case {
            input_new = input_new.to_uppercase();
        }
        if input_new == "1" {
            Ok(PartOne)
        } else if input_new == "2" {
            Ok(PartTwo)
        } else if input_new == "both" {
            Ok(Both)
        } else {
            Err(format!("value {input} is not a valid <PART>"))
        }
    }
}

#[macro_export]
macro_rules! aoc_main {
    ($parse:ident, $part1:ident, $part2:ident) => {
        fn main() -> AocResult<()> {
            let cli = Cli::parse();
            let input = cli.input()?;
            let parsed = $parse(&input);
            let (_, parsed) = aoc_main!(@finalize, parsed);
            if cli.should_run(SolutionPart::PartOne) {
                let part1 = $part1(&parsed)?;
                println!("Part 1: {:?}", part1);
            }
            if cli.should_run(SolutionPart::PartTwo) {
                let part2 = $part2(&parsed)?;
                println!("Part 2: {:?}", part2);
            }
            Ok(())
        }
    };

    (@finalize, $parsed:expr) => {
        nom::Finish::finish($parsed).map_err(|nom::error::Error { input, code }| {
            nom::error::Error {
                input: input.to_string(),
                code,
            }
        })?
    }
}

#[macro_export]
macro_rules! assert_part {
    ($parse:ident, $part:ident, $input:expr, $equals:expr) => {{
        let input = $parse($input).unwrap().1;
        let part = $part(&input).unwrap();
        assert_eq!(format!("{:?}", part), format!("{:?}", $equals))
    }};
}

#[macro_export]
macro_rules! assert_parser {
    ($parse:ident, $input:expr, $equals:expr) => {{
        let input = $parse($input).unwrap().1;
        assert_eq!(format!("{:?}", input), format!("{:?}", $equals))
    }};
}
