use indoc::indoc;
use log::debug;
use std::{fs, path::Path};

const MAIN_SCAFFOLDING: &'static str = indoc! { r#"
use utils::*;

#[derive(Debug)]
struct InputData {}

fn parse(input: &str) -> ParseResult<InputData> {
    todo!()
}

fn part1(input: &InputData) -> AocResult<()> {
    Ok(())
}

fn part2(input: &InputData) -> AocResult<()> {
    Ok(())
}

aoc_main!(parse, part1, part2);

#[test]
fn test() {

}

"# };

const CARGO_TOML_SCAFFOLDING: &'static str = indoc! { r#"
[package]
name = "day-{DAY-NUMBER}"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
nom = "7.1.1"
utils = { path = "../utils", version = "*" }

"# };

pub fn create_day(day: u64) -> anyhow::Result<()> {
    let day_folder = format!("day-{day}");
    let location = Path::new(env!("CARGO_MANIFEST_DIR")).with_file_name(&day_folder);
    debug!("New folder location: {}", location.display());
    fs::create_dir_all(location.join("src"))?;
    fs::write(
        location.join("Cargo.toml"),
        CARGO_TOML_SCAFFOLDING.replace("{DAY-NUMBER}", &day.to_string()),
    )?;
    fs::write(location.join("src").join("main.rs"), MAIN_SCAFFOLDING)?;
    Ok(())
}
