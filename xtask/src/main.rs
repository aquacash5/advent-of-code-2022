mod create;

use cargo::{core::Workspace, Config};
use clap::{builder::PossibleValue, Parser, ValueEnum};
use log::trace;
use std::path::Path;
use xshell::{cmd, Shell};

use crate::create::create_day;

/// Tasks to use and maintain this project
#[derive(Parser, Debug)]
#[command(author, about, long_about = None)]
enum Cli {
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

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let cli = Cli::parse();
    trace!("CLI arguments: {cli:?}");

    let sh = Shell::new()?;
    match cli {
        Cli::Create { day } => {
            create_day(day)?;
        }
        Cli::Day { .. } => {
            todo!();
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
