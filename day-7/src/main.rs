use std::collections::HashMap;

use itertools::Itertools;
#[allow(clippy::wildcard_imports)]
use utils::*;

#[derive(Debug, PartialEq, Eq)]
struct InputData {
    fs: HashMap<String, u64>,
}

#[derive(Debug)]
struct SudoPath {
    path: Vec<String>,
}

impl SudoPath {
    fn new() -> Self {
        Self {
            path: vec!["/".to_string()],
        }
    }

    fn cd(&mut self, path: &str) {
        match path {
            "/" => self.path = vec!["/".to_string()],
            ".." => {
                if self.path.len() > 1 {
                    self.path.pop();
                }
            }
            dir => self.path.push(dir.to_string()),
        }
    }

    fn current_dir(&self) -> String {
        let mut output = self.path.join("/");
        while output.contains("//") {
            output = output.replace("//", "/");
        }
        output
    }

    fn make_path(&self, file: &str) -> String {
        let mut output = format!("{}/{file}", self.path.join("/"),);
        while output.contains("//") {
            output = output.replace("//", "/");
        }
        output
    }
}

#[allow(clippy::unnecessary_wraps)]
fn parse(input: &str) -> ParseResult<InputData> {
    let mut fs = HashMap::new();
    let mut dir = SudoPath::new();
    for line in input.lines() {
        let cmd: Vec<&str> = line.split(' ').collect();
        match &*cmd {
            ["$", "cd", d] => dir.cd(d),
            [size, file] => {
                fs.entry(dir.make_path(file))
                    .or_insert_with(|| size.parse().unwrap());
            }
            _ => (),
        }
    }
    Ok(("", InputData { fs }))
}

#[allow(clippy::unnecessary_wraps)]
fn part1(input: &InputData) -> AocResult<u64> {
    let mut counter: HashMap<String, u64> = HashMap::new();
    for (file, size) in &input.fs {
        let mut cur_dir = SudoPath::new();
        for (dir, _) in file.split('/').filter(|d| d != &"").tuple_windows() {
            cur_dir.cd(dir);
            counter
                .entry(cur_dir.current_dir())
                .and_modify(|i| *i += size)
                .or_insert_with(|| *size);
        }
    }
    Ok(counter.values().filter(|i| i < &&100_000).sum())
}

#[allow(clippy::unnecessary_wraps)]
fn part2(input: &InputData) -> AocResult<u64> {
    const MAX_SIZE_LEFT_OVER: u64 = 70_000_000 - 30_000_000;

    let mut counter: HashMap<String, u64> = HashMap::new();
    for (file, size) in &input.fs {
        let mut cur_dir = SudoPath::new();
        counter
            .entry("/".to_string())
            .and_modify(|i| *i += size)
            .or_insert_with(|| *size);
        for (dir, _) in file.split('/').filter(|d| d != &"").tuple_windows() {
            cur_dir.cd(dir);
            counter
                .entry(cur_dir.current_dir())
                .and_modify(|i| *i += size)
                .or_insert_with(|| *size);
        }
    }
    let total_size = counter.get("/").unwrap();
    Ok(*counter
        .values()
        .filter(|size| (total_size - *size) < MAX_SIZE_LEFT_OVER)
        .min_by_key(|size| MAX_SIZE_LEFT_OVER - (total_size - *size))
        .unwrap())
}

aoc_main!(parse, part1, part2);

#[test]
fn test() {
    let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
    assert_parser!(
        parse,
        input,
        InputData {
            fs: HashMap::from([
                ("/a/e/i".to_string(), 584),
                ("/a/f".to_string(), 29116),
                ("/a/g".to_string(), 2557),
                ("/a/h.lst".to_string(), 62596),
                ("/b.txt".to_string(), 14848514),
                ("/c.dat".to_string(), 8504156),
                ("/d/j".to_string(), 4060174),
                ("/d/d.log".to_string(), 8033020),
                ("/d/d.ext".to_string(), 5626152),
                ("/d/k".to_string(), 7214296)
            ])
        }
    );
    assert_part!(parse, part1, input, 95437);
    assert_part!(parse, part2, input, 24933642);
}
