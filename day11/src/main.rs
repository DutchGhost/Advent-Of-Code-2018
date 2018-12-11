use aoc::aoc;

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

fn area_power(x: usize, y: usize, serial_number: usize) -> isize {
    coordinates((x, y), (x + 2, y + 2))
        .map(|(x, y)| power(x, y, serial_number))
        .sum::<isize>()
}

#[aoc(2018, 11, 1)]
fn main(input: &str) -> (usize, usize) {
    let serial_number = input.trim().parse().unwrap();

    let (x, y, _) = coordinates((0, 0), (297, 297))
        .map(|(x, y)| (x, y, area_power(x, y, serial_number)))
        .max_by_key(|&(_, _, power)| power)
        .unwrap();

    (x, y)
}
