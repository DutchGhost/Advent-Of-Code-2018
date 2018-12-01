const PUZZLE: &str = include_str!("input.txt");

fn main() {
    let frequency = PUZZLE
        .lines()
        .filter_map(|s| s.parse::<isize>().ok())
        .sum::<isize>();

    println!("{}", frequency);
}