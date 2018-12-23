use aoc::aoc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
enum Cell {
    Sand,
    Clay,
    WaterSpring,
    DryWater,
    Water,
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '+' => Cell::WaterSpring,
            '#' => Cell::Clay,
            '.' => Cell::Sand,
            _ => unreachable!("bad, reall bad"),
        }
    }
}

fn print_all(min_x: usize, grid: &[Vec<Cell>]) {
    for row in grid {
        for cell in row.iter().skip(min_x) {
            match cell {
                Cell::Sand => print!("."),
                Cell::Clay => print!("#"),
                Cell::WaterSpring => print!("+"),
                Cell::DryWater => print!("|"),
                Cell::Water => print!("~"),
            }
        }
        println!()
    }
}

#[derive(Debug)]
pub enum Vein {
    Horizontal(usize, usize, usize),
    Vertical(usize, usize, usize),
}

fn parse(s: &str) -> Vec<Vein> {
    let mut veins = Vec::new();

    for line in s.lines() {
        let char_to_look_for = if line.starts_with('x') { 'y' } else { 'x' };

        let end = line.chars().position(|c| c == ',').unwrap();
        let fst = line[2..end].parse().expect("NOO");

        let start_scnd = line.chars().position(|c| c == char_to_look_for).unwrap();
        let end_scnd_num = line.chars().position(|c| c == '.').unwrap();
        let scnd =
            line[start_scnd + 2..end_scnd_num]
                .parse()
                .expect(&format!(r"{}\{}", file!(), line!()));
        let last = line[end_scnd_num + 2..].parse().unwrap();

        if char_to_look_for == 'y' {
            veins.push(Vein::Vertical(fst, scnd, last));
        } else {
            veins.push(Vein::Horizontal(fst, scnd, last));
        }
    }

    veins
}

use std::cmp::{max, min};

#[aoc(2018, 17, 1)]
fn main(input: &str) {
    //     let input = "x=495, y=2..7
    // y=7, x=495..501
    // x=501, y=3..7
    // x=498, y=2..4
    // x=506, y=1..2
    // x=498, y=10..13
    // x=504, y=10..13
    // y=13, x=498..504";
    let veins = parse(input);

    let mut max_x = 0;
    let mut max_y = 0;
    let mut min_x = std::usize::MAX;

    for vein in veins.iter() {
        match vein {
            Vein::Horizontal(y, x1, x2) => {
                max_x = max(max_x, *x2);
                max_y = max(max_y, *y);
                min_x = min(min_x, *x1);
            }

            Vein::Vertical(x, y1, y2) => {
                max_x = max(max_x, *x);
                max_y = max(max_y, *y2);
                min_x = min(min_x, *x);
            }
        }
    }

    let mut grid = vec![vec![Cell::Sand; max_x + 2]; max_y + 2];

    for vein in veins {
        match vein {
            Vein::Horizontal(y, x1, x2) => {
                for item in grid[y].iter_mut().skip(x1).take(x2 - x1) {
                    *item = Cell::Clay;
                }
            }
            Vein::Vertical(x, y1, y2) => {
                for row in y1..=y2 {
                    grid[row][x] = Cell::Clay;
                }
            }
        }
    }

    grid[0][500] = Cell::WaterSpring;
    print_all(min_x - 2, &grid);

    println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");

    for _ in 0..10_000 {
        //let water_before = grid.iter().flat_map(|v| v.into_iter()).filter(|&&c| c == Cell::Water || c == Cell::DryWater).count();

        for y in 0..max_y {
            for x in min_x - 2..max_x {
                match grid[y][x] {
                    // found the spring, go down straight untill we reach something that isn't sand
                    Cell::WaterSpring => {
                        for going_down_y in (y + 1..) {
                            if grid[going_down_y][x] != Cell::Sand {
                                break;
                            }
                            grid[going_down_y][x] = Cell::DryWater;
                        }
                    }

                    // found a '|'. Check if we can flow.
                    // flowing is allowed if the cell underneath the current is a `#` or a `~`.
                    Cell::DryWater => {
                        if has_ground(x, y, &grid) {
                            match (can_flood_left(x, y, &grid), can_flood_right(x, y, &grid)) {
                                (Some(x_offset_left), Some(x_offset_right)) => {
                                    grid[y][x] = Cell::Water;
                                    for item in grid[y][x_offset_left..x_offset_right].iter_mut() {
                                        *item = Cell::Water;
                                    }
                                }

                                (Some(x_offset_left), None) => {
                                    for item in grid[y][x_offset_left..x].iter_mut() {
                                        *item = Cell::DryWater;
                                    }
                                }

                                (None, Some(x_offset_right)) => {
                                    for item in grid[y][x..x_offset_right].iter_mut() {
                                        if *item != Cell::Clay {
                                            *item = Cell::DryWater;
                                        }
                                    }
                                }

                                (None, None) => {}
                            }
                        }
                        for going_down_y in (y + 1..max_y + 1) {
                            if grid[going_down_y][x] != Cell::Sand {
                                break;
                            }
                            grid[going_down_y][x] = Cell::DryWater;
                        }
                    }
                    Cell::Sand => {
                        if x > 0 && grid[y][x - 1] == Cell::DryWater {
                            if has_ground(x - 1, y, &grid) {
                                grid[y][x] = Cell::DryWater;
                            } else if has_ground(x, y, &grid) {
                                grid[y][x] = Cell::DryWater;
                            }
                        }

                        if x + 1 < grid[0].len() && grid[y][x + 1] == Cell::DryWater {
                            if has_ground(x + 1, y, &grid) {
                                grid[y][x] = Cell::DryWater;
                            } else if has_ground(x, y, &grid) {
                                grid[y][x] = Cell::DryWater;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        // let water_after = grid.iter().flat_map(|v| v.into_iter()).filter(|&&c| c == Cell::Water || c == Cell::DryWater).count();

        // if water_before == water_after {
        //     break;
        // }
    }

    print_all(min_x - 2, &grid);

    let count = grid
        .into_iter()
        .flat_map(|v| v.into_iter())
        .filter(|&c| c == Cell::Water || c == Cell::DryWater)
        .count();

    println!("{:?}", count);
}

fn is_sand_and_not_water(cell: &Cell) -> bool {
    *cell != Cell::Sand && !(*cell == Cell::DryWater || *cell == Cell::Water)
}

fn can_flood_left(x: usize, y: usize, grid: &[Vec<Cell>]) -> Option<usize> {
    let mut ret = x;
    for going_left_x in (0..=x).rev() {
        if is_sand_and_not_water(&grid[y][going_left_x]) {
            return Some(ret);
        }

        if !has_ground(going_left_x, y, &grid) {
            return None;
        }

        ret = going_left_x;
    }

    return None;
}

fn can_flood_right(x: usize, y: usize, grid: &[Vec<Cell>]) -> Option<usize> {
    let mut ret = x;
    for going_right_x in x..grid[0].len() {
        if is_sand_and_not_water(&grid[y][going_right_x]) {
            return Some(ret);
        }

        if !has_ground(going_right_x, y, &grid) {
            return None;
        }
        ret = going_right_x
    }

    return None;
}

fn has_wall(x: usize, y: usize, grid: &[Vec<Cell>]) -> bool {
    for going_to_left_x in (0..x).rev() {
        if grid[y][going_to_left_x] == Cell::Clay {
            return true;
        }
    }

    for going_to_right_x in x + 1..grid[0].len() {
        if grid[y][going_to_right_x] == Cell::Clay {
            return true;
        }
    }

    false
}

fn has_2_walls(x: usize, y: usize, grid: &[Vec<Cell>]) -> bool {
    let mut walls = 0;
    for going_to_left_x in (0..x).rev() {
        if grid[y][going_to_left_x] == Cell::Clay {
            walls += 1;
            break;
        }
    }

    for going_to_right_x in x + 1..grid[0].len() {
        if grid[y][going_to_right_x] == Cell::Clay {
            walls += 1;
            break;
        }
    }

    walls == 2
}
fn has_ground(x: usize, y: usize, grid: &[Vec<Cell>]) -> bool {
    if y > grid.len() {
        return false;
    }
    match grid[y + 1][x] {
        Cell::Water | Cell::Clay => true,
        _ => false,
    }
}
