use itertools::izip;
use itertools::Itertools;
use ndarray::{Array2, Axis};
use pathfinding::prelude::bfs;
#[allow(clippy::wildcard_imports)]
use utils::*;

#[derive(Debug, PartialEq)]
struct InputData {
    start: Pos,
    end: Pos,
    arr: Array2<u8>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize);

impl Pos {
    fn up(&self) -> Option<Self> {
        Some(Self(self.0.checked_add(1)?, self.1))
    }

    fn down(&self) -> Option<Self> {
        Some(Self(self.0.checked_sub(1)?, self.1))
    }

    fn left(&self) -> Option<Self> {
        Some(Self(self.0, self.1.checked_sub(1)?))
    }

    fn right(&self) -> Option<Self> {
        Some(Self(self.0, self.1.checked_add(1)?))
    }

    fn walk_up(&self, map: &Array2<u8>) -> Vec<Self> {
        let &Self(x, y) = self;
        let &altitude = map.get((x, y)).unwrap();
        [self.up(), self.down(), self.left(), self.right()]
            .into_iter()
            .flatten()
            .filter(|Self(x, y)| {
                if let Some(&i) = map.get((*x, *y)) {
                    i <= altitude || altitude + 1 == i
                } else {
                    false
                }
            })
            .collect()
    }

    fn walk_down(&self, map: &Array2<u8>) -> Vec<Self> {
        let &Self(x, y) = self;
        let &altitude = map.get((x, y)).unwrap();
        [self.up(), self.down(), self.left(), self.right()]
            .into_iter()
            .flatten()
            .filter(|Self(x, y)| {
                if let Some(&i) = map.get((*x, *y)) {
                    altitude <= i || altitude == i + 1
                } else {
                    false
                }
            })
            .collect()
    }
}

#[allow(clippy::unnecessary_wraps)]
fn parse(input: &str) -> ParseResult<InputData> {
    let lines = input.lines().collect_vec();
    let mut data: InputData = InputData {
        start: Pos(0, 0),
        end: Pos(0, 0),
        arr: Array2::<u8>::default((lines.len(), lines[0].len())),
    };
    for (x, mut row, line) in izip!(0.., data.arr.axis_iter_mut(Axis(0)), lines) {
        for (y, col, byte) in izip!(0.., row.iter_mut(), line.as_bytes()) {
            match *byte {
                b'S' => {
                    *col = 0;
                    data.start = Pos(x, y);
                }
                b'E' => {
                    *col = 25;
                    data.end = Pos(x, y);
                }
                _ => *col = *byte - b'a',
            }
        }
    }
    Ok(("", data))
}

#[allow(clippy::unnecessary_wraps)]
fn part1(input: &InputData) -> AocResult<usize> {
    let result = bfs(&input.start, |p| p.walk_up(&input.arr), |p| p == &input.end);
    Ok(result.expect("no path found").len() - 1)
}

#[allow(clippy::unnecessary_wraps)]
fn part2(input: &InputData) -> AocResult<usize> {
    let result = bfs(
        &input.end,
        |p| p.walk_down(&input.arr),
        |Pos(x, y)| *input.arr.get((*x, *y)).unwrap() == 0,
    );
    Ok(result.expect("no path found").len() - 1)
}

aoc_main!(parse, part1, part2);

#[test]
#[allow(clippy::zero_prefixed_literal)]
fn test() {
    use ndarray::array;

    let input = "Sabqponm\nabcryxxl\naccszExk\nacctuvwj\nabdefghi";
    assert_parser!(
        parse,
        input,
        InputData {
            start: Pos(0, 0),
            end: Pos(2, 5),
            arr: array![
                [00, 00, 01, 16, 15, 14, 13, 12],
                [00, 01, 02, 17, 24, 23, 23, 11],
                [00, 02, 02, 18, 25, 25, 23, 10],
                [00, 02, 02, 19, 20, 21, 22, 09],
                [00, 01, 03, 04, 05, 06, 07, 08]
            ]
        }
    );
    assert_part!(parse, part1, input, 31);
    assert_part!(parse, part2, input, 29);
}
