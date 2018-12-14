use aoc::aoc;

const DIGITS: [&[usize]; 20] = [
    &[0],
    &[1],
    &[2],
    &[3],
    &[4],
    &[5],
    &[6],
    &[7],
    &[8],
    &[9],
    &[1, 0],
    &[1, 1],
    &[1, 2],
    &[1, 3],
    &[1, 4],
    &[1, 5],
    &[1, 6],
    &[1, 7],
    &[1, 8],
    &[1, 9],
];

#[aoc(2018, 14, 2)]
fn main(input: &str) -> usize {
    let pattern = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect::<Vec<_>>();

    let pattern = pattern.as_slice();
    let len = pattern.len();

    let mut recipes = Vec::with_capacity(25_000_000);
    recipes.push(3);
    recipes.push(7);

    let mut elf1 = 0;
    let mut elf2 = 1;

    loop {
        let recipe_elf1 = recipes[elf1] as usize;
        let recipe_elf2 = recipes[elf2] as usize;

        for digit in DIGITS[recipe_elf1 + recipe_elf2] {
            recipes.push(*digit as u8);
            if recipes.ends_with(pattern) {
                return recipes.len() - len;
            }
        }

        let l = recipes.len();
        elf1 = (elf1 + 1 + recipe_elf1) % l;
        elf2 = (elf2 + 1 + recipe_elf2) % l;
    }
}
