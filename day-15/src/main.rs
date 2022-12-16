#![allow(clippy::cast_possible_wrap)]
use std::ops::RangeInclusive;

use itertools::Itertools;
#[allow(clippy::wildcard_imports)]
use utils::*;

const fn manhattan_distance((x1, y1): (i64, i64), (x2, y2): (i64, i64)) -> i64 {
    (x1.abs_diff(x2) + y1.abs_diff(y2)) as i64
}

#[derive(Debug, PartialEq)]
struct SensorBeaconPair {
    sensor: (i64, i64),
    beacon: (i64, i64),
}

impl SensorBeaconPair {
    fn parse(input: &str) -> ParseResult<Self> {
        use nom::{
            bytes::complete::tag,
            combinator::map,
            sequence::{preceded, separated_pair},
        };
        let sensor = preceded(tag("Sensor at "), xy_pair);
        let beacon = preceded(tag("closest beacon is at "), xy_pair);
        map(
            separated_pair(sensor, tag(": "), beacon),
            |(sensor, beacon)| Self { sensor, beacon },
        )(input)
    }

    fn range_in_row(&self, row: i64) -> Option<RangeInclusive<i64>> {
        let &Self {
            sensor: (sx, sy),
            beacon: _,
        } = self;
        let distance = manhattan_distance(self.sensor, self.beacon);
        if ((sy - distance)..=(sy + distance)).contains(&row) {
            let left = sx - distance + sy.abs_diff(row) as i64;
            let right = sx + distance - sy.abs_diff(row) as i64;
            Some(left..=right)
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq)]
struct InputData {
    pairs: Vec<SensorBeaconPair>,
}

impl InputData {
    fn covered_cells(&self, row: i64) -> Vec<i64> {
        self.pairs
            .iter()
            .filter_map(|p| p.range_in_row(row).map(itertools::Itertools::collect_vec))
            .flatten()
            .unique()
            .collect_vec()
    }

    fn is_covered(&self, row: i64) -> bool {
        let mut ranges = self
            .pairs
            .iter()
            .filter_map(|p| p.range_in_row(row))
            .collect_vec();
        ranges.sort_by_key(|r| *r.start());
        ranges
            .iter()
            .map(|r| Some(r.clone()))
            .reduce(|left, right| left.and_then(|l| collapse_range(l, right.unwrap())))
            .unwrap()
            .is_some()
    }
}

fn collapse_range(
    left: RangeInclusive<i64>,
    right: RangeInclusive<i64>,
) -> Option<RangeInclusive<i64>> {
    match (left.contains(right.start()), left.contains(right.end())) {
        (true, false) => Some(*left.start()..=*right.end()),
        (true, true) => Some(left),
        _ => None,
    }
}

fn xy_pair(input: &str) -> ParseResult<(i64, i64)> {
    use nom::{
        bytes::complete::tag,
        character::complete::i64,
        sequence::{preceded, separated_pair},
    };
    let x = preceded(tag("x="), i64);
    let y = preceded(tag("y="), i64);
    separated_pair(x, tag(", "), y)(input)
}

fn parse(input: &str) -> ParseResult<InputData> {
    use nom::{character::complete::line_ending, combinator::map, multi::separated_list1};
    map(
        separated_list1(line_ending, SensorBeaconPair::parse),
        |pairs| InputData { pairs },
    )(input)
}

fn calc_part1(input: &InputData, row: i64) -> usize {
    let beacons = input
        .pairs
        .iter()
        .map(|&SensorBeaconPair { sensor: _, beacon }| beacon)
        .collect_vec();
    input
        .covered_cells(row)
        .iter()
        .filter(|i| !beacons.contains(&(**i, row)))
        .count()
}

#[allow(clippy::unnecessary_wraps)]
fn part1(input: &InputData) -> AocResult<usize> {
    const ROW: i64 = 2_000_000;
    Ok(calc_part1(input, ROW))
}

fn calc_part2(input: &InputData, max: i64) -> i64 {
    const TUNING_FREQUENCY: i64 = 4_000_000;

    for i in 0..=max {
        if !input.is_covered(i) {
            let missing = input
                .covered_cells(i)
                .iter()
                .sorted()
                .tuple_windows()
                .find(|(&i, &j)| i.abs_diff(j) != 1)
                .map(|(&i, _)| i + 1)
                .unwrap();
            return missing * TUNING_FREQUENCY + i;
        }
    }
    0
}

#[allow(clippy::unnecessary_wraps)]
fn part2(input: &InputData) -> AocResult<i64> {
    Ok(calc_part2(input, 4_000_000))
}

aoc_main!(parse, part1, part2);

#[test]
fn test() {
    let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    let test_data = InputData {
        pairs: vec![
            SensorBeaconPair {
                sensor: (2, 18),
                beacon: (-2, 15),
            },
            SensorBeaconPair {
                sensor: (9, 16),
                beacon: (10, 16),
            },
            SensorBeaconPair {
                sensor: (13, 2),
                beacon: (15, 3),
            },
            SensorBeaconPair {
                sensor: (12, 14),
                beacon: (10, 16),
            },
            SensorBeaconPair {
                sensor: (10, 20),
                beacon: (10, 16),
            },
            SensorBeaconPair {
                sensor: (14, 17),
                beacon: (10, 16),
            },
            SensorBeaconPair {
                sensor: (8, 7),
                beacon: (2, 10),
            },
            SensorBeaconPair {
                sensor: (2, 0),
                beacon: (2, 10),
            },
            SensorBeaconPair {
                sensor: (0, 11),
                beacon: (2, 10),
            },
            SensorBeaconPair {
                sensor: (20, 14),
                beacon: (25, 17),
            },
            SensorBeaconPair {
                sensor: (17, 20),
                beacon: (21, 22),
            },
            SensorBeaconPair {
                sensor: (16, 7),
                beacon: (15, 3),
            },
            SensorBeaconPair {
                sensor: (14, 3),
                beacon: (15, 3),
            },
            SensorBeaconPair {
                sensor: (20, 1),
                beacon: (15, 3),
            },
        ],
    };
    assert_parser!(parse, input, test_data);
    assert_eq!(calc_part1(&test_data, 10), 26);
    assert_eq!(calc_part2(&test_data, 20), 56_000_011);
}
