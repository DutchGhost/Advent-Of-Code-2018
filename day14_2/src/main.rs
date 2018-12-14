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
}

fn solve_fast(input: &str) -> usize {
    let iter = input.trim().chars().rev().map(|c| c.to_digit(10).unwrap() as u8).enumerate();

    let mut target = 0;
    let mut mask = 0;
    let len = input.len();
    
    for (idx, byte) in iter {
        target |= (byte as u64) << idx * 8;
        mask |= 255 << idx * 8;
    }

    let mut recipes = vec![3u8,7];
    let mut end = (3u64 << 8) | 7;

    let mut elf1 = 0;
    let mut elf2 = 1;

    loop {
        let new_recipe = recipes[elf1] + recipes[elf2];
        for digit in Digits::new(new_recipe) {
            recipes.push(digit);

            end = ((end << 8 ) & mask) | digit as u64;

            if end == target {
                return recipes.len() - len
            }
        }

        elf1 = (elf1 + (1 + recipes[elf1]) as usize) % recipes.len();
        elf2 = (elf2 + (1 + recipes[elf2]) as usize) % recipes.len();
    }
}

fn solve(input: &str) -> usize {
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
    use super::*;

    #[bench]
    fn solve_slow(b: &mut Bencher) {
        b.iter(|| {
            assert_eq!(solve("51589"), 9);
            assert_eq!(solve("01245"), 5);
            assert_eq!(solve("92510"), 18);
            assert_eq!(solve("59414"), 2018);
           assert_eq!(solve_fast("598701"), 20331097)
        })
    }

    #[bench]
    fn solve_usize(b: &mut Bencher) {
        b.iter(|| {
            assert_eq!(solve_fast("51589"), 9);
            assert_eq!(solve_fast("01245"), 5);
            assert_eq!(solve_fast("92510"), 18);
            assert_eq!(solve_fast("59414"), 2018);
            assert_eq!(solve_fast("598701"), 20331097);
        })
    }
}