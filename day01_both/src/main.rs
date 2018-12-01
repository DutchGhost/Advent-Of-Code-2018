#![feature(cell_update)]

extern crate itertools;

use itertools::Itertools;

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

    let part2 = loop {
        match cumulative_sums
            .iter_mut()
            .update(|n| {
                **n += finalsum;
            }).find(|n| set.contains(n))
        {
            Some(n) => break *n,
            _ => continue,
        }
    };
    println!("part1: {:?} part2: {:?}", finalsum, part2);
}
