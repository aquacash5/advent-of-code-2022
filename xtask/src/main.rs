mod create;

use cargo::{core::Workspace, Config};
use clap::{builder::PossibleValue, Parser, ValueEnum};
use log::trace;
use std::{fmt::Display, path::Path};
use xshell::{cmd, Shell};

use crate::create::generate_day;

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

    let sh = Shell::new()?;
    match cli {
        Cli::Clippy => {
            cmd!(
                sh,
                "cargo clippy -- -W clippy::all -W clippy::pedantic -W clippy::nursery"
            )
            .run()?;
        }
        Cli::Create { day } => {
            generate_day(day)?;
        }
        Cli::Day { day, part } => {
            let package = format!("day-{day}");
            let path = Path::new(env!("CARGO_MANIFEST_DIR"))
                .with_file_name(&package)
                .join("input.txt");
            let part = format!("{part}");
            cmd!(
                sh,
                "cargo run --release --package {package} -- {path} -p {part}"
            )
            .run()?;
        }
        Cli::Tree => {
            cmd!(sh, "cargo run --release --package tree").run()?;
        }
        Cli::Test { day } => {
            let day = format!("day-{day}");
            cmd!(sh, "cargo test --package {day}").run()?;
        }
        Cli::TestAll => {
            test_all(&sh)?;
        }
    }

    Ok(())
}

/// Tests all of the Advent of Code projects in the workspace
fn test_all(sh: &Shell) -> anyhow::Result<()> {
    // We expect CARGO_MANIFEST_DIR to be the directory of the
    // xtask package therefor calling `with_file_name` we replace
    // `.../xtask` with `.../Cargo.toml`. This is expected to be
    // the manifest of the workspace.
    let workspace_manifest = Path::new(env!("CARGO_MANIFEST_DIR")).with_file_name("Cargo.toml");
    let system_config = Config::default()?;
    let workspace = Workspace::new(&workspace_manifest, &system_config)?;
    let cargo_test = sh.cmd("cargo").arg("test").arg("--workspace");
    let cargo_test = workspace
        .members()
        .filter(|p| !p.name().starts_with("day"))
        .fold(cargo_test, |c, p| c.arg("--exclude").arg(p.name()));
    cargo_test.run()?;
    Ok(())
}
