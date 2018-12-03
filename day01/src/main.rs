use aoc::aoc;

#[aoc(2018, 1, 1)]
fn main(input: &str) -> isize {
    input
        .lines()
        .filter_map(|s| s.parse::<isize>().ok())
        .sum::<isize>()
}