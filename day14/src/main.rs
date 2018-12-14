use aoc::aoc;

const DIGITS: [&[u8]; 20] = [
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

#[aoc(2018, 14, 1)]
fn main(input: &str) -> String {
    let input = input.trim().parse::<usize>().unwrap();

    let mut recipes = vec![3u8, 7];

    let mut elf1 = 0;
    let mut elf2 = 1;

    while recipes.len() < input + 10 {
        let new_recipe = recipes[elf1] + recipes[elf2];
        recipes.extend_from_slice(DIGITS[new_recipe as usize]);

        elf1 = (elf1 + recipes[elf1] as usize + 1) % recipes.len();
        elf2 = (elf2 + recipes[elf2] as usize + 1) % recipes.len();
    }

    recipes.drain(0..input);
    recipes.iter_mut().for_each(|n| *n += 48);

    String::from_utf8(recipes).unwrap()
}
