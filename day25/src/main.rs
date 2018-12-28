#![feature(cell_update, as_cell)]

use aoc::aoc;

use std::cell::Cell;

#[derive(Clone)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
    q: i32,
}

impl Point {
    fn can_merge_with(&self, other: &ConstellationPoint) -> bool {
        match other {
            ConstellationPoint::Merged(v) => v.iter().any(|item| self.can_merge_with(item)),
            ConstellationPoint::Point(p) => self.manhatten(p) <= 3,
        }
    }
}

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

impl Point {
    fn manhatten(&self, other: &Self) -> i32 {
        (self.x - other.x).abs()
            + (self.y - other.y).abs()
            + (self.z - other.z).abs()
            + (self.q - other.q).abs()
    }
}

#[allow(clippy::many_single_char_names)]
fn parse(s: &str) -> Vec<Vec<ConstellationPoint>> {
    s.lines()
        .map(|line| {
            let mut line_spliiter = line.split(',');

            let x = line_spliiter.next().unwrap().parse().unwrap();
            let y = line_spliiter.next().unwrap().parse().unwrap();
            let z = line_spliiter.next().unwrap().parse().unwrap();
            let q = line_spliiter.next().unwrap().parse().unwrap();

            vec![ConstellationPoint::Point(Point { x, y, z, q })]
        }).collect()
}

#[aoc(2018, 25, 1)]
fn main(input: &str) -> usize {
    let mut parsed = parse(input);

    let slice = Cell::from_mut(parsed.as_mut_slice()).as_slice_of_cells();

    for (idx, constellation) in slice.iter().enumerate() {
        let mut constell = constellation.take();
        for constellation2 in slice[..idx].iter() {
            let constell2 = constellation2.take();

            // If not mergable, set constellation2's constellation back to what it was.
            if constell
                .iter()
                .any(|p| constell2.iter().any(|point| point.can_merge(p)))
            {
                constell.push(ConstellationPoint::Merged(constell2));
            } else {
                constellation2.set(constell2);
            }
        }

        constellation.set(constell);
    }

    parsed.into_iter().filter(|v| !v.is_empty()).count()
}
