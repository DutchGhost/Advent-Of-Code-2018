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

    power_level = (power_level / 100) % 10;

    power_level as isize - 5
}

fn area_power(
    x: usize,
    y: usize,
    size: usize,
    map: &[Vec<isize>],
    cache: &mut HashMap<(usize, usize), (isize)>,
) -> isize {
    if x + size > 300 || y + size > 300 {
        return 0;
    }

    match cache.entry((x, y)) {
        Entry::Occupied(value) => {
            let sum = value.into_mut();
            *sum += map[y + size - 1].iter().skip(x).take(size).sum::<isize>();

            *sum += map
                .iter()
                .skip(y)
                .take(size)
                .map(|row| row[x + size - 1])
                .sum::<isize>();

            *sum
        }
        Entry::Vacant(c) => {
            let sum = map
                .iter()
                .skip(y)
                .take(size)
                .flat_map(|row| row.iter().skip(x).take(size))
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

    let mut v = vec![vec![0; 300]; 300];

    for (y, row) in v.iter_mut().enumerate() {
        for (x, cell) in row.iter_mut().enumerate() {
            *cell = power(x, y, serial_number);
        }
    }

    let (x, y, _, size) = (1..300)
        .map(|size| {
            let (x, y, power) = coordinates((0, 0), (300, 300))
                .map(|(x, y)| {
                    let power = area_power(x, y, size, &v, &mut map);
                    (x, y, power)
                }).max_by_key(|&(_, _, power)| power)
                .unwrap();

            (x, y, power, size)
        }).max_by_key(|&(_, _, power, _)| power)
        .unwrap();

    (x, y, size)
}
