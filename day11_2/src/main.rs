use aoc::aoc;

use std::ops::Range;

fn coordinates(xrange: Range<usize>, yrange: Range<usize>) -> impl Iterator<Item = (usize, usize)> {
    yrange.flat_map(move |y| xrange.clone().map(move |x| (x, y)))
}

fn power(x: usize, y: usize, serial_number: usize) -> isize {
    let rack_id = x + 10;

    let mut power_level = rack_id * y;
    power_level += serial_number;
    power_level *= rack_id;

    power_level = (power_level / 100) % 10;

    power_level as isize - 5
}

const SIZE: usize = 301;

/// A chunk of x, y with a size of `size`, having a total sum of `sum`
#[derive(Debug)]
struct Chunk {
    x: usize,
    y: usize,
    size: usize,
    area_sum: isize,
}

impl Chunk {
    fn new(x: usize, y: usize, size: usize, area_sum: isize) -> Self {
        Self {
            x,
            y,
            size,
            area_sum,
        }
    }

    fn area_sum(&self) -> isize {
        self.area_sum
    }
}

fn sum_of_area(x: usize, y: usize, size: usize, table: &[Vec<isize>]) -> isize {
    table[y][x] + table[y + size][x + size] - table[y][x + size] - table[y + size][x]
}

fn chunks_by_size<'a>(size: usize, grid: &'a [Vec<isize>]) -> impl Iterator<Item = Chunk> + 'a {
    let range = 0..SIZE - size;

    coordinates(range.clone(), range.clone()).map(move |(x, y)| {
        let area_sum = sum_of_area(x, y, size, grid);
        Chunk::new(x + 1, y + 1, size, area_sum)
    })
}

/// Implementation based on a summed area table: https://en.wikipedia.org/wiki/Summed-area_table
#[aoc(2018, 11, 2)]
fn main(input: &str) -> Chunk {
    let serial_number = input.trim().parse::<usize>().unwrap();

    let mut table = vec![vec![0; SIZE]; SIZE];

    // Build the table.
    // Sum up the previous numbers for the current one.
    // No need to treat the first row/column special, since its padded by 1 with a 0 row and 0 column.
    for y in 1..SIZE {
        for x in 1..SIZE {
            table[y][x] = power(x, y, serial_number) + table[y][x - 1] + table[y - 1][x]
                - table[y - 1][x - 1];
        }
    }

    (0..SIZE)
        .flat_map(|size| chunks_by_size(size, &table))
        .max_by_key(|sumchunk| sumchunk.area_sum())
        .unwrap()
}
