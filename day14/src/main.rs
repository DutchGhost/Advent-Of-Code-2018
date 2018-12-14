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

#[aoc(2018, 14, 1)]
fn main(input: &str) -> String {
    let input = input.trim().parse::<usize>().unwrap();

    let mut recipes = vec![3u8, 7];

    let mut elf1 = 0;
    let mut elf2 = 1;

    while recipes.len() < input + 10 {
        let new_recipe = recipes[elf1] + recipes[elf2];
        let digits = Digits::new(new_recipe);

        for digit in digits {
            recipes.push(digit);
        }

        elf1 = (elf1 + recipes[elf1] as usize + 1) % recipes.len();
        elf2 = (elf2 + recipes[elf2] as usize + 1) % recipes.len();
    }

    recipes
        .into_iter()
        .skip(input)
        .take(10)
        .map(|n| std::char::from_digit(n as u32, 10).unwrap())
        .collect::<String>()
}
