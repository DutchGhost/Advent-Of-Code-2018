use aoc::aoc;

fn complement(c: char) -> char {
    if c.is_uppercase() {
        c.to_ascii_lowercase()
    } else {
        c.to_ascii_uppercase()
    }
}

fn react(input: impl Iterator<Item = char>, stack: &mut String) -> usize {
    for c in input {
        match stack.chars().last().filter(|last| *last == complement(c)) {
            // Need the drop here to make the match arm happy.
            Some(_) => drop(stack.pop()),
            None => stack.push(c),
        }
    }

    stack.len()
}

fn is_filter_char(c: char, filter_char: char) -> bool {
    c == filter_char || c == filter_char.to_ascii_uppercase()
}

#[aoc(2018, 5, 1)]
fn main(input: &str) -> usize {
    let input = input.trim();
    let mut len = std::usize::MAX;
    let mut buffer = String::new();

    for filter_char in (97..=122u8).map(|c| c as char) {
        let iter = input.chars().filter(|c| !is_filter_char(*c, filter_char));
        len = std::cmp::min(len, react(iter, &mut buffer));
        buffer.clear();
    }

    len
}
