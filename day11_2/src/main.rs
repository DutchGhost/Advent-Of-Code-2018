use aoc::aoc;

use hashbrown::{hash_map::Entry, HashMap};

fn coordinates(
    (start_x, start_y): (usize, usize),
    (end_x, end_y): (usize, usize),
) -> impl Iterator<Item = (usize, usize)> {
    (start_y..=end_y).flat_map(move |y| (start_x..=end_x).map(move |x| (x, y)))
}

fn power(x: usize, y: usize, serial_number: usize) -> isize {
    let rack_id = x + 10;

    let mut power_level = rack_id * y;
    power_level += serial_number;
    power_level *= rack_id;

    power_level /= 100;
    if power_level == 0 {
        -5
    } else {
        ((power_level % 10) as isize) - 5
    }
}

fn area_power(
    x: usize,
    y: usize,
    serial_number: usize,
    size: usize,
    cache: &mut HashMap<(usize, usize), (isize)>,
) -> isize {
    if x + size > 300 || y + size > 300 {
        return 0;
    }

    match cache.entry((x, y)) {
        Entry::Occupied(value) => {
            let sum = value.into_mut();
            *sum += (x..size + x)
                .map(|x| power(x, y + size - 1, serial_number))
                .sum::<isize>();

            *sum += (y..size + y)
                .map(|y| power(x + size - 1, y, serial_number))
                .sum::<isize>();
            *sum
        }
        Entry::Vacant(c) => {
            let sum = coordinates((x, y), (x + (size - 1), y + (size - 1)))
                .map(|(x, y)| power(x, y, serial_number))
                .sum::<isize>();

            c.insert(sum);
            sum
        }
    }
}

#[aoc(2018, 11, 2)]
fn main(input: &str) -> (usize, usize, usize) {
    let serial_number = input.trim().parse::<usize>().unwrap();
    let mut map = HashMap::new();

    let (x, y, _, size) = (1..300)
        .map(|size| {
            let (x, y, power) = coordinates((0, 0), (300, 300))
                .map(|(x, y)| {
                    let power = area_power(x, y, serial_number, size, &mut map);
                    (x, y, power)
                }).max_by_key(|&(_, _, power)| power)
                .unwrap();

            (x, y, power, size)
        }).max_by_key(|&(_, _, power, _)| power)
        .unwrap();

    (x, y, size)
}
