use aoc::aoc;

fn complement(c: char) -> char {
    if c.is_uppercase() {
        c.to_ascii_lowercase()
    } else {
        c.to_ascii_uppercase()
    }
}

fn react(input: impl Iterator<Item = char>) -> usize {
    let mut stack = String::new();
    for c in input {
        if stack.ends_with(complement(c)) {
            stack.pop();
        } else {
            stack.push(c)
        }
    }

    stack.len()
}

#[aoc(2018, 5, 1)]
fn main(input: &str) -> usize {
    react(input.trim().chars())
}
