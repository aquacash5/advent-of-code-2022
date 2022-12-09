use itertools::{FoldWhile, Itertools};
#[allow(clippy::wildcard_imports)]
use ndarray::prelude::*;
#[allow(clippy::wildcard_imports)]
use utils::*;

#[derive(Debug, PartialEq, Clone)]
struct InputData(Array2<u8>);

fn parse(input: &str) -> ParseResult<InputData> {
    use nom::{
        character::complete::{digit1, line_ending},
        combinator::map,
        multi::separated_list1,
    };
    let mut parser = map(separated_list1(line_ending, digit1), |c: Vec<&str>| {
        let mut arr = Array2::<u8>::default((c.len(), c[0].len()));
        for (mut row, s) in arr.axis_iter_mut(Axis(0)).zip(c.iter()) {
            for (col, byte) in row.iter_mut().zip(s.bytes()) {
                *col = byte - b'0';
            }
        }
        InputData(arr)
    });
    parser(input)
}

fn mask_ring(input: &InputData) -> Array2<bool> {
    let (height, width) = input.0.dim();
    let mut mask = Array2::<bool>::default(input.0.raw_dim());
    mask.fill(false);
    mask.row_mut(0).map_mut(|c| *c = true);
    mask.row_mut(height - 1).map_mut(|c| *c = true);
    mask.column_mut(0).map_mut(|r| *r = true);
    mask.column_mut(width - 1).map_mut(|r| *r = true);
    mask
}

fn mask_left(input: &InputData) -> Array2<bool> {
    let mut mask = Array2::<bool>::default(input.0.raw_dim());
    mask.fill(true);
    for (i, mut row) in mask.rows_mut().into_iter().enumerate() {
        let mut max: u8 = *input.0.get((i, 0)).unwrap_or(&0);
        for (j, col) in row.iter_mut().enumerate().skip(1) {
            let cur = *input.0.get((i, j)).unwrap_or(&0);
            *col = cur > max;
            max = cur.max(max);
        }
    }
    mask
}

fn mask_right(input: &InputData) -> Array2<bool> {
    let (_, width) = input.0.dim();
    let mut mask = Array2::<bool>::default(input.0.raw_dim());
    mask.fill(true);
    for (i, mut row) in mask.rows_mut().into_iter().enumerate() {
        let mut max: u8 = *input.0.get((i, width - 1)).unwrap_or(&0);
        for (j, col) in row.iter_mut().enumerate().rev().skip(1) {
            let cur = *input.0.get((i, j)).unwrap_or(&0);
            *col = cur > max;
            max = cur.max(max);
        }
    }
    mask
}

fn mask_down(input: &InputData) -> Array2<bool> {
    let mut mask = Array2::<bool>::default(input.0.raw_dim());
    mask.fill(true);
    for (j, mut col) in mask.columns_mut().into_iter().enumerate() {
        let mut max: u8 = *input.0.get((0, j)).unwrap_or(&0);
        for (i, row) in col.iter_mut().enumerate().skip(1) {
            let cur = *input.0.get((i, j)).unwrap_or(&0);
            *row = cur > max;
            max = cur.max(max);
        }
    }
    mask
}

fn mask_up(input: &InputData) -> Array2<bool> {
    let (_, width) = input.0.dim();
    let mut mask = Array2::<bool>::default(input.0.raw_dim());
    mask.fill(true);
    for (j, mut col) in mask.columns_mut().into_iter().enumerate() {
        let mut max: u8 = *input.0.get((width - 1, j)).unwrap_or(&0);
        for (i, row) in col.iter_mut().enumerate().rev().skip(1) {
            let cur = *input.0.get((i, j)).unwrap_or(&0);
            *row = cur > max;
            max = cur.max(max);
        }
    }
    mask
}

#[allow(clippy::unnecessary_wraps)]
fn part1(input: &InputData) -> AocResult<usize> {
    let ring = mask_ring(input);
    let left = mask_left(input);
    let right = mask_right(input);
    let up = mask_up(input);
    let down = mask_down(input);
    Ok((ring | left | right | up | down)
        .iter()
        .filter(|&&b| b)
        .count())
}

fn score_tree(input: &InputData, row: usize, col: usize) -> u64 {
    let cur_size = *input.0.get((row, col)).unwrap_or(&0);
    let (height, width) = input.0.dim();
    // check up
    let up = input
        .0
        .slice(s![0..row;-1, col])
        .iter()
        .fold_while(0u32, |acc, &i| {
            if cur_size > i {
                FoldWhile::Continue(acc + 1)
            } else {
                FoldWhile::Done(acc + 1)
            }
        })
        .into_inner();
    // check down
    let down = input
        .0
        .slice(s![row + 1..height, col])
        .iter()
        .fold_while(0u32, |acc, &i| {
            if cur_size > i {
                FoldWhile::Continue(acc + 1)
            } else {
                FoldWhile::Done(acc + 1)
            }
        })
        .into_inner();
    // check right
    let right = input
        .0
        .slice(s![row, 0..col; -1])
        .iter()
        .fold_while(0u32, |acc, &i| {
            if cur_size > i {
                FoldWhile::Continue(acc + 1)
            } else {
                FoldWhile::Done(acc + 1)
            }
        })
        .into_inner();
    // check left
    let left = input
        .0
        .slice(s![row, col + 1..width])
        .iter()
        .fold_while(0u32, |acc, &i| {
            if cur_size > i {
                FoldWhile::Continue(acc + 1)
            } else {
                FoldWhile::Done(acc + 1)
            }
        })
        .into_inner();
    let score = up * down * right * left;
    score as u64
}

#[allow(clippy::unnecessary_wraps)]
fn part2(input: &InputData) -> AocResult<u64> {
    Ok(input
        .0
        .indexed_iter()
        .map(|((row, col), _)| score_tree(input, row, col))
        .max()
        .unwrap_or(0))
}

aoc_main!(parse, part1, part2);

#[test]
fn test() {
    let input = "30373\n25512\n65332\n33549\n35390";
    assert_parser!(
        parse,
        input,
        InputData(array![
            [3, 0, 3, 7, 3],
            [2, 5, 5, 1, 2],
            [6, 5, 3, 3, 2],
            [3, 3, 5, 4, 9],
            [3, 5, 3, 9, 0],
        ])
    );
    assert_part!(parse, part1, input, 21);
    assert_part!(parse, part2, input, 8);
}
