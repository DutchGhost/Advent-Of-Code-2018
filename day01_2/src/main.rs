const PUZZLE: &str = include_str!("input.txt");
use std::collections::HashSet;

fn main() {
    let mut set = HashSet::new();

    let mut frequency = 0;
    for number in PUZZLE.lines().flat_map(|s| s.parse::<isize>().ok()).cycle() {
        frequency += number;

        if set.contains(&frequency) {
            break;
        }

        set.insert(frequency);
    }

    println!("{:?}", frequency);
}
