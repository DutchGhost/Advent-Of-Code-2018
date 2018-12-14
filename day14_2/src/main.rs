#![feature(test)]
extern crate test;
use aoc::aoc;

struct Digits {
    n: u8,
    divisor: u8,
}

impl Digits {

    fn new(n: u8) -> Self {
        let mut divisor = 1;
        while n >= divisor * 10 {
            divisor *= 10;
        }

        Self { n, divisor }
    }
}

impl Iterator for Digits {
    type Item = u8;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.divisor == 0 {
            None
        } else {
            let v = Some(self.n / self.divisor);
            self.n %= self.divisor;
            self.divisor /= 10;
            v
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (1, Some(2))
    }
}

impl std::iter::ExactSizeIterator for Digits {
    
    #[inline]
    fn len(&self) -> usize {
        if self.n > 10 { 2 } else if self.n > 0 { 1 } else { 0 }
    }
}

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

fn solve_fast(input: &str) -> usize {
    let iter = input.trim().chars().rev().map(|c| c as u8 - 48).enumerate();

    let mut target = 0;
    let mut mask = 0;
    let len = input.len();
    
    for (idx, byte) in iter {
        target |= (byte as u64) << idx * 4;
        mask |= 15 << idx * 4;
    }

    let mut recipes = Vec::with_capacity(20_000_000);
    recipes.push(3usize);
    recipes.push(7);

    let mut end = (3u64 << 4) | 7;

    let mut elf1 = 0;
    let mut elf2 = 1;

    loop {
        let recipe_elf1 = unsafe { *recipes.get_unchecked(elf1) as usize };
        let recipe_elf2 = unsafe { *recipes.get_unchecked(elf2) as usize };

        elf1 = elf1 + (1 + recipe_elf1) as usize;
        elf2 = elf2 + (1 + recipe_elf2) as usize;

        for digit in unsafe { *DIGITS.get_unchecked(recipe_elf1 + recipe_elf2) } {
            recipes.push(*digit);

            end = ((end << 4 ) & mask) | *digit as u64;

            if end == target {
                return recipes.len() - len
            }
        }

        while elf1 >= recipes.len() { elf1 -= recipes.len() };
        while elf2 >= recipes.len() { elf2 -= recipes.len() };

    }
}

fn solve_table(input: &str) -> usize {
    let pattern = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect::<Vec<_>>();

    let pattern = pattern.as_slice();
    let len = pattern.len();

    let mut recipes = Vec::with_capacity(20_000_00);
    recipes.push(3u8);
    recipes.push(7);

    let mut elf1 = 0;
    let mut elf2 = 1;

    loop {
        let recipe_elf1 = unsafe { *recipes.get_unchecked(elf1) as usize };
        let recipe_elf2 = unsafe { *recipes.get_unchecked(elf2) as usize };

        for digit in unsafe { *DIGITS.get_unchecked(recipe_elf1 + recipe_elf2) } {
            recipes.push(*digit as u8);
            if recipes.ends_with(pattern) {
                return recipes.len() - len;
            }
        }
        
        elf1 = elf1 + (1 + recipe_elf1) as usize;
        while elf1 >= recipes.len() { elf1 -= recipes.len() };
        elf2 = elf2 + (1 + recipe_elf2) as usize;
        while elf2 >= recipes.len() { elf2 -= recipes.len() };
    }
}

fn solve_digits(input: &str) -> usize {
    let pattern = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect::<Vec<_>>();

    let pattern = pattern.as_slice();
    let len = pattern.len();

    let mut recipes = Vec::with_capacity(20_000_00);
    recipes.push(3u8);
    recipes.push(7);

    let mut elf1 = 0;
    let mut elf2 = 1;

    loop {
        let new_recipe = recipes[elf1] + recipes[elf2];

        for digit in Digits::new(new_recipe) {
            recipes.push(digit);
            if recipes.ends_with(pattern) {
                return recipes.len() - len;
            }
        }

        elf1 = (elf1 + recipes[elf1] as usize + 1) % recipes.len();
        elf2 = (elf2 + recipes[elf2] as usize + 1) % recipes.len();
    }
}

#[aoc(2018, 14, 2)]
fn main(input: &str) -> usize {
    solve_fast(input);
    let pattern = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect::<Vec<_>>();

    let pattern = pattern.as_slice();
    let len = pattern.len();

    let mut recipes = vec![3, 7];

    let mut elf1 = 0;
    let mut elf2 = 1;

    loop {
        let new_recipe = recipes[elf1] + recipes[elf2];

        for digit in Digits::new(new_recipe) {
            recipes.push(digit);
            if recipes.ends_with(pattern)  {
                return recipes.len() - len;
            }
        }

        elf1 = (elf1 + recipes[elf1] as usize + 1) % recipes.len();
        elf2 = (elf2 + recipes[elf2] as usize + 1) % recipes.len();
    }
}

#[cfg(test)]
mod tests {
    use super::test::Bencher;
    use super::test::black_box;
    use super::*;

    #[bench]
    fn solve_table_bench(b: &mut Bencher) {
        b.iter(|| {
            assert_eq!(black_box(solve_table("51589")), 9);
            assert_eq!(black_box(solve_table("01245")), 5);
            assert_eq!(black_box(solve_table("92510")), 18);
            assert_eq!(black_box(solve_table("59414")), 2018);
           //assert_eq!(solve_table("598701"), 20331097)
        });
    }

    #[bench]
    fn solve_digits_bench(b: &mut Bencher) {
        b.iter(|| {
            assert_eq!(black_box(solve_digits("51589")), 9);
            assert_eq!(black_box(solve_digits("01245")), 5);
            assert_eq!(black_box(solve_digits("92510")), 18);
            assert_eq!(black_box(solve_digits("59414")), 2018);
 //           assert_eq!(solve_digits("598701"), 20331097)
        });
    }

    #[bench]
    fn solve_u64_bench(b: &mut Bencher) {
        b.iter(|| {
            assert_eq!(black_box(solve_fast("51589")), 9);
            assert_eq!(black_box(solve_fast("01245")), 5);
            assert_eq!(black_box(solve_fast("92510")), 18);
            assert_eq!(black_box(solve_fast("59414")), 2018);
          //  assert_eq!(solve_fast("598701"), 20331097);
        });
    }
}