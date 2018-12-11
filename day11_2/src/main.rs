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

    power_level /= 100;
    let power = if power_level == 0 {
        -5
    } else {
        ((power_level % 10) as isize) - 5
    };

    power
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Area {
    x: usize,
    y: usize,

    sizeof: usize,
}

fn area_power(
    x: usize,
    y: usize,
    serial_number: usize,
    size: usize,
    cache: &mut HashMap<Area, isize>,
) -> Option<isize> {
    let area = Area {
        x,
        y,
        sizeof: size - 1,
    };
    let v = match cache.get(&area) {
        Some(value) => {
            let mut sum = 0;
            if x + size > 300 || y + size > 300 {
                //  println!("{} {} with size of {} escapes", x, y, size);
                return None;
            }
            for xx in x..size + x {
                sum += power(xx, y + size - 1, serial_number);
            }

            for yy in y..size + y {
                sum += power(x + size - 1, yy, serial_number);
            }

            sum + value
        }
        None => {
            if x + size > 300 || y + size > 300 {
                //    println!("[*] need {} {} with size of {} escapes", x, y, size);
                return None;
            }
            coordinates((x, y), (x + (size - 1), y + (size - 1)))
                .map(|(x, y)| power(x, y, serial_number))
                .sum::<isize>()
        }
    };

    cache.insert(
        Area {
            x: x,
            y: y,
            sizeof: size,
        },
        v,
    );

    // for(k, v) in cache.iter() {
    //     println!("SIZE: {:?}, COORDS: {:?}, POWER: {:?}", size, k, v);
    // }

    Some(v)
}

#[aoc(2018, 11, 2)]
fn main(input: &str) {
    let serial_number = input.trim().parse::<usize>().unwrap();
    let mut map = HashMap::new();

    let (x, y, pow, size) = (1..300)
        .map(|size| {
            println!("at size {}", size);
            let (x, y, power) = coordinates((0, 0), (300, 300))
                .filter_map(|(x, y)| {
                    if let Some(power) = area_power(x, y, serial_number, size, &mut map) {
                        Some((x, y, power))
                    } else {
                        None
                    }
                }).max_by_key(|&(_, _, power)| power)
                .unwrap();

            (x, y, power, size)
        }).max_by_key(|&(x, y, power, size)| power)
        .unwrap();

    println!("{:?}", (x, y, size as usize))
}
