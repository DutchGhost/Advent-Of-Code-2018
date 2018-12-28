#![feature(cell_update, as_cell)]

use aoc::aoc;

use std::cell::Cell;

#[derive(Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
    q: i32,
}

impl Point {
    fn manhatten(&self, other: &Self) -> i32 {
        (self.x - other.x).abs()
            + (self.y - other.y).abs()
            + (self.z - other.z).abs()
            + (self.q - other.q).abs()
    }

    fn can_merge_with(&self, other: &ConstellationPoint) -> bool {
        match other {
            ConstellationPoint::Merged(v) => v.iter().any(|item| self.can_merge_with(item)),
            ConstellationPoint::Point(p) => self.manhatten(p) <= 3,
        }
    }
}

#[derive(Debug)]
enum ConstellationPoint {
    Merged(Vec<ConstellationPoint>),
    Point(Point),
}

impl ConstellationPoint {
    fn can_merge(&self, other: &Self) -> bool {
        match self {
            ConstellationPoint::Merged(v) => v.iter().any(|item| other.can_merge(item)),

            // We know `self` is a single Point, so call can_merge_with to recursively test what `other` is,
            ConstellationPoint::Point(ref p) => p.can_merge_with(other),
        }
    }
}

#[allow(clippy::many_single_char_names)]
fn parse(s: &str) -> Vec<Vec<ConstellationPoint>> {
    s.lines()
        .map(|line| {
            let mut line_splitter = line.split(',');

            let x = line_splitter.next().unwrap().parse().unwrap();
            let y = line_splitter.next().unwrap().parse().unwrap();
            let z = line_splitter.next().unwrap().parse().unwrap();
            let q = line_splitter.next().unwrap().parse().unwrap();

            vec![ConstellationPoint::Point(Point { x, y, z, q })]
        }).collect()
}

fn can_merge(constellation: &[ConstellationPoint], to_check: &[ConstellationPoint]) -> bool {
    constellation
        .iter()
        .any(|point| to_check.iter().any(|p| point.can_merge(p)))
}

#[aoc(2018, 25, 1)]
fn main(input: &str) -> usize {
    let mut parsed = parse(input);

    let slice = Cell::from_mut(parsed.as_mut_slice()).as_slice_of_cells();

    let mut start = 0;

    'outer: for (idx, constellation) in slice.iter().enumerate() {
        let mut constell = constellation.take();
        for to_check_constellation in slice[start..idx].iter() {
            let to_check_constell = to_check_constellation.take();

            if can_merge(&constell, &to_check_constell) {
                constell.push(ConstellationPoint::Merged(to_check_constell));

                to_check_constellation.swap(&slice[start]);
                start += 1;
            } else {
                to_check_constellation.set(to_check_constell);
            }
        }

        constellation.set(constell);
    }
    parsed.into_iter().filter(|v| !v.is_empty()).count()
}