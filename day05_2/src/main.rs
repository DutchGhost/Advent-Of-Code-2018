use aoc::aoc;

fn poly(mut to_react: String, reacting_units: &[String]) -> usize {
    (0..)
        .map(|_| reacting_units.iter())
        .map(|iter| {
            let len_before = to_react.len();
            iter.for_each(|unit| to_react = to_react.replace(unit, ""));
            let len_after = to_react.len();

            (len_after - len_before, len_after)
        }).take_while(|(len, _)| *len != 0)
        .last()
        .map(|(_, len)| len)
        .unwrap()
}

#[aoc(2018, 5, 1)]
fn main(input: &str) -> Option<usize> {
    let input = input.trim();
    let mut reacting_units = Vec::new();

    for c in (97..=122u8).map(|b| b as char) {
        let lower_upper = format!("{}{}", c, c.to_ascii_uppercase());
        let upper_lower = format!("{}{}", c.to_ascii_uppercase(), c);

        reacting_units.push(lower_upper);
        reacting_units.push(upper_lower);
    }

    (97..=122u8)
        .map(|b| b as char)
        .map(|c| input.replace(c, "").replace(c.to_ascii_uppercase(), ""))
        .map(|s| poly(s, &reacting_units))
        .min()
}
