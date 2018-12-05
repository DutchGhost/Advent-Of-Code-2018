use aoc::aoc;

#[aoc(2018, 5, 1)]
fn main(input: &str) -> usize {
    let mut input = input.trim().to_owned();
    let mut reacting_units = Vec::new();

    for c in (97..=122u8).map(|b| b as char) {
        let lower_upper = format!("{}{}", c, c.to_ascii_uppercase());
        let upper_lower = format!("{}{}", c.to_ascii_uppercase(), c);

        reacting_units.push(lower_upper);
        reacting_units.push(upper_lower);
    }

    (0..)
        .map(|_| reacting_units.iter())
        .map(|iter| {
            let len_before = input.len();
            iter.for_each(|unit| input = input.replace(unit, ""));
            let len_after = input.len();

            (len_after - len_before, len_after)
        }).take_while(|(len, _)| *len != 0)
        .last()
        .map(|(_, len)| len)
        .unwrap()
}
