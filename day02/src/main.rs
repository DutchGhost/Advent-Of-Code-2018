use aoc::aoc;

use std::collections::HashMap;

#[aoc(2018, 2, 1)]
fn main(input: &str) -> usize {
    let mut frequencies = HashMap::new();

    let mut twos = 0;
    let mut threes = 0;
    
    // use this as a bitarray!
    let mut current: u8 = 0;

    for line in input.lines() {
        for c in line.chars() {
            *frequencies.entry(c).or_insert(0) += 1;
        }

        for freq in frequencies
            .drain()
            .map(|(_, f)| f)
            .filter(|f| *f == 2 || *f == 3)
            .map(|f| f - 2)
        {
            current = (1 << freq) | current;
        }
        
        // 00000000 => nothing happened,
        // 00000001 => two's has incremented,
        // 00000010 => three's has incremented,
        // 00000011 => both two's and three's have incremented,
        match current {
            0 => {}
            1 => twos += 1,
            2 => threes += 1,
            3 => {
                twos += 1;
                threes += 1;
            }
            _ => unreachable!(),
        }
        current = 0;
    }

    twos * threes
}
