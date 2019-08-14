#![feature(cell_update)]

use aoc::aoc;
use std::cell::Cell;
use std::collections::HashSet;

#[aoc(2018, 1, 3)]
fn main(input: &str) -> (isize, Option<isize>) {
    let mut cumulative_sums = input
        .lines()
        .filter_map(|s| s.parse::<isize>().ok())
        .scan(Cell::new(0), |freq, n| Some(freq.update(|old| old + n)))
        .collect::<Vec<_>>();

    let finalsum = *cumulative_sums.last().unwrap();

    let set = cumulative_sums.iter().cloned().collect::<HashSet<_>>();

    let slice = Cell::from_mut(cumulative_sums.as_mut_slice()).as_slice_of_cells();

    let part2 = slice
        .iter()
        .cycle()
        .map(|n| n.update(|old| old + finalsum))
        .find(|n| set.contains(&n));

    (finalsum, part2)
}
