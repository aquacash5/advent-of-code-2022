use itertools::Itertools;
use ndarray::{s, Array2};
#[allow(clippy::wildcard_imports)]
use utils::*;

#[derive(Debug, PartialEq)]
struct InputData {
    walls: Vec<Vec<(usize, usize)>>,
}

fn parse(input: &str) -> ParseResult<InputData> {
    use nom::{
        bytes::complete::tag,
        character::complete::{char, line_ending, u32},
        combinator::map,
        multi::separated_list1,
        sequence::separated_pair,
    };

    let pair = map(separated_pair(u32, char(','), u32), |(x, y)| {
        (x as usize, y as usize)
    });
    let wall = separated_list1(tag(" -> "), pair);
    let walls = separated_list1(line_ending, wall);
    let mut parser = map(walls, |walls| InputData { walls });
    parser(input)
}

const fn min_max(l: usize, r: usize) -> (usize, usize) {
    if l < r {
        (l, r)
    } else {
        (r, l)
    }
}

fn settle_sand(start: (usize, usize), arr: &Array2<bool>) -> Option<(usize, usize)> {
    let (x, mut y) = start;
    while !*arr.get((x, y))? {
        y += 1;
    }

    match (arr.get((x - 1, y)), arr.get((x + 1, y))) {
        (Some(false), _) => settle_sand((x - 1, y), arr),
        (_, Some(false)) => settle_sand((x + 1, y), arr),
        (Some(true), Some(true)) => Some((x, y - 1)),
        _ => None,
    }
}

#[allow(clippy::unnecessary_wraps)]
fn part1(input: &InputData) -> AocResult<usize> {
    let (max_x, max_y) = input
        .walls
        .iter()
        .flatten()
        .fold((usize::MIN, usize::MIN), |(max_x, max_y), (x, y)| {
            (max_x.max(*x), max_y.max(*y))
        });
    let mut arr = Array2::<bool>::default((max_x + 1, max_y + 1));
    arr.fill(false);
    for wall in &input.walls {
        for ((x1, y1), (x2, y2)) in wall.iter().tuple_windows() {
            let (x_min, x_max) = min_max(*x1, *x2);
            let (y_min, y_max) = min_max(*y1, *y2);
            arr.slice_mut(s![x_min..=x_max, y_min..=y_max]).fill(true);
        }
    }
    let mut steps = 0;
    while let Some(pos) = settle_sand((500, 0), &arr) {
        steps += 1;
        *arr.get_mut(pos).unwrap() = true;
    }

    Ok(steps)
}

#[allow(clippy::unnecessary_wraps)]
fn part2(input: &InputData) -> AocResult<usize> {
    let (max_x, max_y) = input
        .walls
        .iter()
        .flatten()
        .fold((usize::MIN, usize::MIN), |(max_x, max_y), (x, y)| {
            (max_x.max(*x), max_y.max(*y))
        });
    let mut arr = Array2::<bool>::default((max_x * 2, max_y + 3));
    arr.fill(false);
    arr.slice_mut(s![.., max_y + 2]).fill(true);
    for wall in &input.walls {
        for ((x1, y1), (x2, y2)) in wall.iter().tuple_windows() {
            let (x_min, x_max) = min_max(*x1, *x2);
            let (y_min, y_max) = min_max(*y1, *y2);
            arr.slice_mut(s![x_min..=x_max, y_min..=y_max]).fill(true);
        }
    }
    let mut steps: usize = 1;
    loop {
        if let Some(pos) = settle_sand((500, 0), &arr) {
            if pos == (500, 0) {
                break;
            }
            steps += 1;
            *arr.get_mut(pos).unwrap() = true;
        }
    }
    Ok(steps)
}

aoc_main!(parse, part1, part2);

#[test]
fn test() {
    let input = "498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9";
    assert_parser!(
        parse,
        input,
        InputData {
            walls: vec![
                vec![(498, 4), (498, 6), (496, 6)],
                vec![(503, 4), (502, 4), (502, 9), (494, 9)],
            ]
        }
    );
    assert_part!(parse, part1, input, 24);
    assert_part!(parse, part2, input, 93);
}
