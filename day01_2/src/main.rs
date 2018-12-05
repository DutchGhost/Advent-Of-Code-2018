#![feature(cell_update)]

use aoc::aoc;

use std::{cell::Cell, collections::HashSet};

#[aoc(2018, 1, 2)]
fn main(input: &str) -> Option<isize> {
    let mut set = HashSet::new();

    input
        .lines()
        .filter_map(|s| s.parse::<isize>().ok())
        .cycle()
        .scan(Cell::new(0), |frequency, n| {
            Some(frequency.update(|old| old + n))
        }).find(|n| !set.insert(*n))
}
