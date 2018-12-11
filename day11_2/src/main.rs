use aoc::aoc;

use hashbrown::{hash_map::Entry, HashMap};
use rayon::prelude::*;

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

/// Computes the `power` of an (x, y) coordinate on grid `map`.
/// Uses a lookup table, so for each increase of a size,
/// only the most bottem row, and most right column need to be added to the current `power` of the coordinate.
///
/// For example:
///     All powers of chunks of size 3 are computed,
///     We look up coordinate x = 0, y = 0 for size 4:
///
///     The map contains an entry, <(0, 0), 27>.
///     
///     We compute the bottem row, and most right column:
///         [1,    2,   3,  (5)]
///         [5,    4,   3,  (1)]
///         [2,    3,   4,  (1)]
///         [(5), (4), (2), (6)]
///         
///         5 + 4 + 2 + 5 + 1 + 1 + 6 = 24.
///
///     The entry is updated to <(0, 0), 51>
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
                .take(size - 1)
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

/// finds the top left (x, y) and size that produced the highest value,
/// where the size is in range of `start` to `end`.
pub fn largest_between_size(
    start: usize,
    end: usize,
    v: &[Vec<isize>],
) -> (usize, usize, isize, usize) {
    let mut map = HashMap::new();

    (start..end)
        .map(|size| {
            let (x, y, power) = coordinates((0, 0), (300, 300))
                .map(|(x, y)| {
                    let power = area_power(x, y, size, v, &mut map);
                    (x, y, power)
                }).max_by_key(|&(_, _, power)| power)
                .unwrap();

            (x, y, power, size)
        }).max_by_key(|&(_, _, power, _)| power)
        .unwrap()
}

const THREADS: usize = 4;
const CHUNK_FACTOR: usize = 300 / THREADS;

#[aoc(2018, 11, 2)]
fn main(input: &str) -> (usize, usize, usize) {
    let serial_number = input.trim().parse::<usize>().unwrap();

    let grid = (0..300)
        .map(|y| (0..300).map(|x| power(x, y, serial_number)).collect())
        .collect::<Vec<_>>();

    let (x, y, _, size) = (0..THREADS)
        .into_par_iter()
        .map(|n| largest_between_size(CHUNK_FACTOR * n, CHUNK_FACTOR * (n + 1), &grid))
        .max_by_key(|&(_, _, power, _)| power)
        .unwrap();

    (x, y, size)
}
