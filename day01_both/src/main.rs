#![feature(cell_update)]

const PUZZLE: &str = include_str!("input.txt");

use std::{cell::Cell, collections::HashSet};

fn main() {
    let mut cumulative_sums = PUZZLE
        .lines()
        .filter_map(|s| s.parse::<isize>().ok())
        .scan(Cell::new(0), |freq, n| Some(freq.update(|old| old + n)))
        .collect::<Vec<_>>();

    let finalsum = *cumulative_sums.last().unwrap();

    let set = cumulative_sums.iter().cloned().collect::<HashSet<_>>();
    println!("part1: {:?} part2: {:?}", finalsum, iterate(&mut cumulative_sums, &set, finalsum));
}

fn iterate(sums: &mut [isize], set: &HashSet<isize>, finalsum: isize) -> isize {
    sums.iter_mut().for_each(|n| *n += finalsum);

    match sums.iter().find(|n| set.contains(n)) {
        Some(n) => *n,
        None => iterate(sums, set, finalsum),
    }
}