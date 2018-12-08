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
        if stack.ends_with(complement(c)) {
            stack.pop();
        } else {
            stack.push(c);
        }
    }

    stack.len()
}

fn is_unit_char(c: char, filter_char: char) -> bool {
    c == filter_char || c == filter_char.to_ascii_uppercase()
}

fn filter_units<'a>(s: &'a str, unit: char) -> impl Iterator<Item = char> + 'a {
    s.chars().filter(move |&c| !is_unit_char(c, unit))
}

#[aoc(2018, 5, 2)]
fn main(input: &str) -> usize {
    let input = input.trim();
    let mut len = std::usize::MAX;
    let mut buffer = String::new();

    for unit_char in (b'a'..=b'z').map(|c| c as char) {
        let iter = filter_units(input, unit_char);
        len = std::cmp::min(len, react(iter, &mut buffer));
        buffer.clear();
    }

    len
}
