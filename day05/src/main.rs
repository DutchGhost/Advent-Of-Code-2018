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
        match stack.chars().last().filter(|last| *last == complement(c)) {
            // Need the drop here to make the match arm happy.
            Some(_) => drop(stack.pop()),
            None => stack.push(c),
        }
    }

    stack.len()
}

#[aoc(2018, 5, 1)]
fn main(input: &str) -> usize {
    react(input.trim().chars())
}
