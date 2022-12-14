mod create;

use cargo_metadata::{Metadata, MetadataCommand};
use clap::{builder::PossibleValue, Parser, ValueEnum};
use log::trace;
use std::fmt::Display;
use xshell::{cmd, Shell};

use crate::create::{generate_day, generate_input};

/// Tasks to use and maintain this project
#[derive(Parser, Debug)]
#[command(author, about, long_about = None)]
enum Cli {
    /// Runs clippy on all projects
    Clippy,

    /// Creates the scaffolding for the days packages
    Create {
        /// The day to run
        #[arg(value_parser = clap::value_parser!(u64).range(1..=25))]
        day: u64,
    },

    /// Run the solution for the day
    Day {
        /// The day to run
        #[arg(value_parser = clap::value_parser!(u64).range(1..=25))]
        day: u64,

        /// part of the task to do
        #[arg(short, long, value_enum, default_value_t)]
        part: SolutionPart,
    },

    /// Print out a lovely christmas tree
    Tree,

    /// Test a particular day
    Test {
        /// The day to test
        #[arg(value_parser = clap::value_parser!(u64).range(1..=25))]
        day: u64,
    },

    /// Test all days
    TestAll,
}

#[derive(Debug, Default, Clone, Copy)]
enum SolutionPart {
    PartOne,
    PartTwo,
    #[default]
    Both,
}

impl Display for SolutionPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PartOne => write!(f, "1"),
            Self::PartTwo => write!(f, "2"),
            Self::Both => write!(f, "both"),
        }
    }
}

impl ValueEnum for SolutionPart {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::PartOne, Self::PartTwo, Self::Both]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            Self::PartOne => Some(PossibleValue::new("1")),
            Self::PartTwo => Some(PossibleValue::new("2")),
            Self::Both => Some(PossibleValue::new("both")),
        }
    }

    fn from_str(input: &str, ignore_case: bool) -> Result<Self, String> {
        let mut input_new = input.to_string();
        if ignore_case {
            input_new = input_new.to_uppercase();
        }
        if input_new == "1" {
            Ok(Self::PartOne)
        } else if input_new == "2" {
            Ok(Self::PartTwo)
        } else if input_new == "both" {
            Ok(Self::Both)
        } else {
            Err(format!("value {input} is not a valid <PART>"))
        }
    }
}

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let cli = Cli::parse();
    trace!("CLI arguments: {cli:?}");
    let metadata = MetadataCommand::new().no_deps().exec()?;

    let sh = Shell::new()?;
    match cli {
        Cli::Clippy => {
            cmd!(
                sh,
                "cargo clippy -q -- -W clippy::all -W clippy::pedantic -W clippy::nursery"
            )
            .run()?;
        }
        Cli::Create { day } => {
            generate_day(day, &metadata)?;
        }
        Cli::Day { day, part } => {
            let package = format!("day-{day}");
            let path = metadata
                .workspace_root
                .as_std_path()
                .join(&package)
                .join("input.txt");
            if !path.exists() {
                generate_input(day, &path)?;
            }
            let part = format!("{part}");
            cmd!(
                sh,
                "cargo run -q --release --package {package} -- {path} -p {part}"
            )
            .run()?;
        }
        Cli::Tree => {
            cmd!(sh, "cargo run -q --release --package tree").run()?;
        }
        Cli::Test { day } => {
            let day = format!("day-{day}");
            cmd!(sh, "cargo test -q --package {day}").run()?;
        }
        Cli::TestAll => {
            test_all(&sh, &metadata)?;
        }
    }

    Ok(())
}

/// Tests all of the Advent of Code projects in the workspace
fn test_all(sh: &Shell, metadata: &Metadata) -> anyhow::Result<()> {
    metadata
        .workspace_packages()
        .iter()
        .filter(|p| p.name.starts_with("day"))
        .try_for_each(|p| {
            let name = p.name.clone();
            cmd!(sh, "cargo test -q -p {name}")
                .run()
                .map_err(anyhow::Error::from)
        })
}
