# advent-of-code-2022
Advent of code solutions for 2022

## Setup

To look around and test algorithms you just need [rust](https://www.rust-lang.org/tools/install).

If you want to solve [Advent of Code](https://adventofcode.com/2022) problems, you wil need to get a Advent of Code [session token](https://github.com/wimglenn/advent-of-code-wim/issues/1) and put it into a file `$HOME/.adventofcode`.

### Run build

```shell
cargo build
```

## xtask

Use the `cargo xtask` command to easily use this repository.

```
Tasks to use and maintain this project

Usage: xtask <COMMAND>

Commands:
  clippy    Runs clippy on all projects
  create    Creates the scaffolding for the days packages
  day       Run the solution for the day
  tree      Print out a lovely christmas tree
  test      Test a particular day
  test-all  Test all days
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```
