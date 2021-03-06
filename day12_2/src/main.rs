use aoc::aoc;

use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Clone, Copy, Ord, PartialOrd)]
enum Pot {
    Empty,
    Plant,
}

impl std::fmt::Debug for Pot {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Pot::Empty => f.write_str("."),
            Pot::Plant => f.write_str("#"),
        }
    }
}

impl std::fmt::Display for Pot {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Pot::Empty => f.write_str("."),
            Pot::Plant => f.write_str("#"),
        }
    }
}

impl From<char> for Pot {
    fn from(c: char) -> Self {
        match c {
            '#' => Pot::Plant,
            '.' => Pot::Empty,
            _ => panic!("Expected `#/.`. Got {}", c),
        }
    }
}

fn parse(s: &str) -> (HashSet<Vec<Pot>>, Vec<Pot>) {
    let mut lines = s.lines();

    let initial = lines.next().expect("Failed on line 28");
    let pots = initial.chars().skip(15).map(Pot::from).collect();

    let mut map = HashSet::new();

    for line in lines.skip(1) {
        let mut splitter = line.split(" => ");

        let rule = splitter
            .next()
            .expect("failed on 36")
            .chars()
            .map(Pot::from)
            .collect::<Vec<_>>();
        let outcome: Pot = splitter
            .next()
            .expect("failed on 37")
            .chars()
            .next()
            .unwrap()
            .into();

        if outcome != Pot::Empty {
            map.insert(rule);
        }
    }

    (map, pots)
}

fn sum_untill(gens: usize, mut curr: Vec<Pot>, rules: &HashSet<Vec<Pot>>) -> isize {
    let curlen = curr.len();

    for g in 1..gens {
        for _ in 0..4 {
            curr.insert(0, Pot::Empty);
            curr.push(Pot::Empty);
        }

        let mut next = Vec::new();

        for x in 2..(curr.len() - 2) {
            if rules.contains(&curr[x - 2..x + 3]) {
                next.push(Pot::Plant);
            } else {
                next.push(Pot::Empty);
            }
        }

        curr = next;
    }

    let diff = (curlen as isize - curr.len() as isize) / 2;

    curr.into_iter()
        .enumerate()
        .filter(|(_, pot)| *pot == Pot::Plant)
        .map(|(idx, _)| idx as isize + diff)
        .sum::<isize>()
}

#[aoc(2018, 12, 1)]
fn main(input: &str) -> usize {
    // Look at line 208 and beyon, it repeats. start at 112, 208 - 112 = 96. Sum of line 208 = 3400, difference with line 209 = 32
    (50000000000 - 96) * 32 + 3400
}

/*
..####....#...######.###.#...##....#.###.#.###.......###.##..##........##..#.#.#..##.##...####.#..##.#
.#.#..#....#.#.###..###..##.#..#....###..####.#.....#.#.##..#..#......#.....######..##.#.#.#...###...#
..###..#....#####..#.#..#...##..#..#.#..#.#...##.....####....#..#......#...#.###...#...######.#.#.#.#.
.#.#....#..#.##.....###..#.#.....#..###..###.#..#...#.#..#....#..#......#...###.#...#.#.###...########
..###....#..##.#...#.#....###.....##.#..#.#..##..#...###..#....#..#......#.#.#..##...#####.#.#.#####..
.#.#.#....##...##...###..#.#.#...#...##..####.....#.#.#....#....#..#......######..#.#.##...#######..#.
..#####..#..#.#..#.#.#....#####...#.#...#.#..#.....#####....#....#..#....#.###.....####.#.#.####.....#
.#.##.....#..###..#####..#.##..#...###...###..#...#.##..#....#....#..#....###.#...#.#...######..#.....
..##.#.....##.#..#.##.....##....#.#.#.#.#.#....#...##....#....#....#..#..#.#..##...###.#.###.....#....
.#...##...#...##..##.#...#..#....###########....#.#..#....#....#....#..#..####..#.#.#..####.#.....#...
..#.#..#...#.#...#...##...#..#..#.########..#....###..#....#....#....#..##.#.....######.#...##.....#..
...###..#...###...#.#..#...#..#..#######.....#..#.#....#....#....#....##...##...#.###...##.#..#.....#.
..#.#....#.#.#.#...###..#...#..##.####..#.....#..###....#....#....#..#..#.#..#...###.#.#...##..#.....#
...###....#######.#.#....#...##..###.....#.....##.#.#....#....#....#..#..###..#.#.#..####.#.....#.....
..#.#.#..#.####...####....#.#...#.#.#.....#...#...####....#....#....#..##.#....######.#...##.....#....
...#####..###..#.#.#..#....###...#####.....#...#.#.#..#....#....#....##...##..#.###...##.#..#.....#...
..#.##...#.#....#####..#..#.#.#.#.##..#.....#...#####..#....#....#..#..#.#.....###.#.#...##..#.....#..
...##.#...###..#.##.....#..########....#.....#.#.##.....#....#....#..#..###...#.#..####.#.....#.....#.
..#...##.#.#....##.#.....##.#####..#....#.....####.#.....#....#....#..##.#.#...####.#...##.....#.....#
...#.#...####..#...##...#..####.....#....#...#.#...##.....#....#....##...####.#.#...##.#..#.....#.....
....###.#.#.....#.#..#...##.#..#.....#....#...###.#..#.....#....#..#..#.#.#...####.#...##..#.....#....
...#.#..####.....###..#.#...##..#.....#....#.#.#..##..#.....#....#..#..#####.#.#...##.#.....#.....#...
....####.#..#...#.#....###.#.....#.....#....######.....#.....#....#..##.##...####.#...##.....#.....#..
...#.#...##..#...###..#.#..##.....#.....#..#.###..#.....#.....#....##..##.#.#.#...##.#..#.....#.....#.
....###.#.....#.#.#....####..#.....#.....#..###....#.....#.....#..#...#...######.#...##..#.....#.....#
...#.#..##.....#####..#.#.....#.....#.....##.#.#....#.....#.....#..#...#.#.###...##.#.....#.....#.....
....####..#...#.##.....###.....#.....#...#...####....#.....#.....#..#...#####.#.#...##.....#.....#....
...#.#.....#...##.#...#.#.#.....#.....#...#.#.#..#....#.....#.....#..#.#.##...####.#..#.....#.....#...
....###.....#.#...##...#####.....#.....#...#####..#....#.....#.....#..####.#.#.#...##..#.....#.....#..
...#.#.#.....###.#..#.#.##..#.....#.....#.#.##.....#....#.....#.....##.#...######.#.....#.....#.....#.
....#####...#.#..##..####....#.....#.....####.#.....#....#.....#...#...##.#.###...##.....#.....#.....#
...#.##..#...####...#.#..#....#.....#...#.#...##.....#....#.....#...#.#...####.#.#..#.....#.....#.....
....##....#.#.#..#...###..#....#.....#...###.#..#.....#....#.....#...###.#.#...####..#.....#.....#....
...#..#....#####..#.#.#....#....#.....#.#.#..##..#.....#....#.....#.#.#..####.#.#.....#.....#.....#...
....#..#..#.##.....#####....#....#.....######.....#.....#....#.....######.#...####.....#.....#.....#..
.....#..#..##.#...#.##..#....#....#...#.###..#.....#.....#....#...#.###...##.#.#..#.....#.....#.....#.
......#..##...##...##....#....#....#...###....#.....#.....#....#...###.#.#...####..#.....#.....#.....#
.......##..#.#..#.#..#....#....#....#.#.#.#....#.....#.....#....#.#.#..####.#.#.....#.....#.....#.....
......#.....###..###..#....#....#....#######....#.....#.....#....######.#...####.....#.....#.....#....
.......#...#.#..#.#....#....#....#..#.####..#....#.....#.....#..#.###...##.#.#..#.....#.....#.....#...
........#...###..###....#....#....#..###.....#....#.....#.....#..###.#.#...####..#.....#.....#.....#..
.........#.#.#..#.#.#....#....#....##.#.#.....#....#.....#.....##.#..####.#.#.....#.....#.....#.....#.
..........#####..#####....#....#..#...####.....#....#.....#...#...###.#...####.....#.....#.....#.....#
.........#.##...#.##..#....#....#..#.#.#..#.....#....#.....#...#.#.#..##.#.#..#.....#.....#.....#.....
..........##.#...##....#....#....#..#####..#.....#....#.....#...######...####..#.....#.....#.....#....
.........#...##.#..#....#....#....##.##.....#.....#....#.....#.#.###..#.#.#.....#.....#.....#.....#...
..........#.#...##..#....#....#..#..##.#.....#.....#....#.....#####....#####.....#.....#.....#.....#..
...........###.#.....#....#....#..##...##.....#.....#....#...#.##..#..#.##..#.....#.....#.....#.....#.
..........#.#..##.....#....#....##..#.#..#.....#.....#....#...##....#..##....#.....#.....#.....#.....#
...........####..#.....#....#..#.....###..#.....#.....#....#.#..#....##..#....#.....#.....#.....#.....
..........#.#.....#.....#....#..#...#.#....#.....#.....#....###..#..#.....#....#.....#.....#.....#....
...........###.....#.....#....#..#...###....#.....#.....#..#.#....#..#.....#....#.....#.....#.....#...
..........#.#.#.....#.....#....#..#.#.#.#....#.....#.....#..###....#..#.....#....#.....#.....#.....#..
...........#####.....#.....#....#..#######....#.....#.....##.#.#....#..#.....#....#.....#.....#.....#.
..........#.##..#.....#.....#....##.####..#....#.....#...#...####....#..#.....#....#.....#.....#.....#
...........##....#.....#.....#..#..###.....#....#.....#...#.#.#..#....#..#.....#....#.....#.....#.....
..........#..#....#.....#.....#..##.#.#.....#....#.....#...#####..#....#..#.....#....#.....#.....#....
...........#..#....#.....#.....##...####.....#....#.....#.#.##.....#....#..#.....#....#.....#.....#...
............#..#....#.....#...#..#.#.#..#.....#....#.....####.#.....#....#..#.....#....#.....#.....#..
.............#..#....#.....#...#..#####..#.....#....#...#.#...##.....#....#..#.....#....#.....#.....#.
..............#..#....#.....#...##.##.....#.....#....#...###.#..#.....#....#..#.....#....#.....#.....#
...............#..#....#.....#.#..##.#.....#.....#....#.#.#..##..#.....#....#..#.....#....#.....#.....
................#..#....#.....####...##.....#.....#....######.....#.....#....#..#.....#....#.....#....
.................#..#....#...#.#..#.#..#.....#.....#..#.###..#.....#.....#....#..#.....#....#.....#...
..................#..#....#...###..###..#.....#.....#..###....#.....#.....#....#..#.....#....#.....#..
...................#..#....#.#.#..#.#....#.....#.....##.#.#....#.....#.....#....#..#.....#....#.....#.
....................#..#....#####..###....#.....#...#...####....#.....#.....#....#..#.....#....#.....#
.....................#..#..#.##...#.#.#....#.....#...#.#.#..#....#.....#.....#....#..#.....#....#.....
......................#..#..##.#...#####....#.....#...#####..#....#.....#.....#....#..#.....#....#....
.......................#..##...##.#.##..#....#.....#.#.##.....#....#.....#.....#....#..#.....#....#...
........................##..#.#...###....#....#.....####.#.....#....#.....#.....#....#..#.....#....#..
.......................#.....###.#.#.#....#....#...#.#...##.....#....#.....#.....#....#..#.....#....#.
........................#...#.#..######....#....#...###.#..#.....#....#.....#.....#....#..#.....#....#
.........................#...####.###..#....#....#.#.#..##..#.....#....#.....#.....#....#..#.....#....
..........................#.#.#..###....#....#....######.....#.....#....#.....#.....#....#..#.....#...
...........................######.#.#....#....#..#.###..#.....#.....#....#.....#.....#....#..#.....#..
..........................#.###...####....#....#..###....#.....#.....#....#.....#.....#....#..#.....#.
...........................###.#.#.#..#....#....##.#.#....#.....#.....#....#.....#.....#....#..#.....#
..........................#.#..######..#....#..#...####....#.....#.....#....#.....#.....#....#..#.....
...........................####.###.....#....#..#.#.#..#....#.....#.....#....#.....#.....#....#..#....
..........................#.#..###.#.....#....#..#####..#....#.....#.....#....#.....#.....#....#..#...
...........................####.#..##.....#....##.##.....#....#.....#.....#....#.....#.....#....#..#..
..........................#.#...###..#.....#..#..##.#.....#....#.....#.....#....#.....#.....#....#..#.
...........................###.#.#....#.....#..##...##.....#....#.....#.....#....#.....#.....#....#..#
..........................#.#..####....#.....##..#.#..#.....#....#.....#.....#....#.....#.....#....#..
...........................####.#..#....#...#.....###..#.....#....#.....#.....#....#.....#.....#....#.
..........................#.#...##..#....#...#...#.#....#.....#....#.....#.....#....#.....#.....#....#
...........................###.#.....#....#...#...###....#.....#....#.....#.....#....#.....#.....#....
..........................#.#..##.....#....#...#.#.#.#....#.....#....#.....#.....#....#.....#.....#...
...........................####..#.....#....#...#######....#.....#....#.....#.....#....#.....#.....#..
..........................#.#.....#.....#....#.#.####..#....#.....#....#.....#.....#....#.....#.....#.
...........................###.....#.....#....#####.....#....#.....#....#.....#.....#....#.....#.....#
..........................#.#.#.....#.....#..#.##..#.....#....#.....#....#.....#.....#....#.....#.....
...........................#####.....#.....#..##....#.....#....#.....#....#.....#.....#....#.....#....
..........................#.##..#.....#.....##..#....#.....#....#.....#....#.....#.....#....#.....#...
...........................##....#.....#...#.....#....#.....#....#.....#....#.....#.....#....#.....#..
..........................#..#....#.....#...#.....#....#.....#....#.....#....#.....#.....#....#.....#.
...........................#..#....#.....#...#.....#....#.....#....#.....#....#.....#.....#....#.....#
............................#..#....#.....#...#.....#....#.....#....#.....#....#.....#.....#....#.....
*/
