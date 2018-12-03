use aoc::aoc;

use std::{
    collections::HashMap,
};

#[aoc(2018, 2, 1)]
fn main(input: &str) -> usize {
    let mut frequencies = HashMap::new();
    let mut v = Vec::new();

    let mut twos = 0;
    let mut threes = 0;

    for line in input.lines() {
        for c in line.chars() {
            *frequencies.entry(c).or_insert(0) += 1;
        }

        v.extend(frequencies.drain().map(|(_, v)| v));

        v.retain(|n| *n == 2 || *n == 3);
        v.dedup();

        for n in v.drain(..) {
            if n == 2 { twos += 1 } else { threes += 1 }
        }
    }
    
    twos * threes
}
