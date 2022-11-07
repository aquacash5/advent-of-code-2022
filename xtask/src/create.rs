use anyhow::Context;
use indoc::indoc;
use log::debug;
use reqwest::blocking as req;
use std::{
    fs::{self, read_to_string, File, OpenOptions},
    io::{self, Write},
    path::Path,
};

const AOC_YEAR: &str = "2022";

const MAIN_SCAFFOLDING: &str = indoc! { r#"
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
fn test() {}

"# };

const CARGO_TOML_SCAFFOLDING: &str = indoc! { r#"
[package]
name = "day-{DAY-NUMBER}"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
nom = "7.1.1"
utils = { path = "../utils", version = "*" }

"# };

fn create_new<P: AsRef<Path>>(path: P) -> io::Result<File> {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create_new(true)
        .open(path.as_ref())
}

pub fn create_day(day: u64) -> anyhow::Result<()> {
    let day_folder = format!("day-{day}");
    let location = Path::new(env!("CARGO_MANIFEST_DIR")).with_file_name(day_folder);
    debug!("New folder location: {}", location.display());
    fs::create_dir_all(location.join("src"))?;
    if let Ok(mut file) = create_new(location.join("Cargo.toml")) {
        println!("Creating Cargo.toml");
        file.write_all(
            CARGO_TOML_SCAFFOLDING
                .replace("{DAY-NUMBER}", &day.to_string())
                .as_bytes(),
        )?;
    } else {
        println!("Cargo.toml exists");
    }
    if let Ok(mut file) = create_new(location.join("src").join("main.rs")) {
        println!("Creating main.rs");
        file.write_all(MAIN_SCAFFOLDING.as_bytes())?;
    } else {
        println!("main.rs exists");
    }
    if !location.join("input.txt").exists() {
        println!("Creating input.txt");
        let aoc_session = read_to_string(
            dirs::home_dir()
                .context("No home directory")?
                .join(".adventofcode"),
        )?
        .trim()
        .to_string();
        let client = req::Client::new();
        let input_data = client
            .request(
                reqwest::Method::GET,
                format!("https://adventofcode.com/{AOC_YEAR}/day/{day}/input"),
            )
            .header(reqwest::header::COOKIE, format!("session={aoc_session}"))
            .send()?
            .text()?;
        fs::write(location.join("input.txt"), input_data)?;
    } else {
        println!("input.txt exists");
    }
    Ok(())
}
