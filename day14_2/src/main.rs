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
            if &recipes[recipes.len().saturating_sub(len)..] == pattern {
                return recipes.len() - len
            }
        }

        elf1 = (elf1 + recipes[elf1] as usize + 1) % recipes.len();
        elf2 = (elf2 + recipes[elf2] as usize + 1) % recipes.len();
    }
}
