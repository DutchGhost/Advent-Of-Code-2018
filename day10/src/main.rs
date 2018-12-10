use aoc::aoc;

use std::str::FromStr;

macro_rules! Struct2 {
    ($s:ident) => {
        #[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd)]
        struct $s {
            x: i64,
            y: i64,
        }
        
        impl FromStr for $s {
            type Err = ();
        
            #[inline]
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let (x, y) = to_nums(s.chars());
                Ok($s { x: x, y: y })
            }
        }
    };
}

#[inline]
fn to_nums<I: Iterator<Item = char>>(iter: I) -> (i64, i64) {
    let stringified = iter
        .filter(|c| c.is_digit(10) || c == &'-' || c == &',')
        .collect::<String>();

    let mut it = stringified.split(',');

    let n1 = it.next().unwrap().parse::<i64>().unwrap();
    let n2 = it.next().unwrap().parse::<i64>().unwrap();

    (n1, n2)
}

Struct2!(Position);
Struct2!(Velocity);

struct Point {
    position: Position,
    velocity: Velocity,
}

fn parse(s: &str) -> Vec<Point> {
    let mut points = Vec::new();

    for line in s.lines() {
        let mut split = line.split(" v");

        let position = split.next().unwrap().parse::<Position>().unwrap();
        let velocity = split.next().unwrap().parse::<Velocity>().unwrap();

        points.push(Point { position, velocity });
    }

    points
}

use std::cmp::{max, min};

#[aoc(2018, 10, 1)]
fn main(input: &str) {
    let points = parse(input);

    let (n, _) = (0..20000)
        .map(|n| {
            let mut minx = std::i64::MAX;
            let mut maxx = 0;
            let mut miny = std::i64::MAX;
            let mut maxy = 0;

            for p in points.iter() {
                let x = p.position.x + n * p.velocity.x;
                let y = p.position.y + n * p.velocity.y;

                minx = min(minx, x);
                maxx = max(maxx, x);
                miny = min(miny, y);
                maxy = max(maxy, y);
            }

            (n, maxx - minx + maxy - miny)
        }).min_by_key(|&(_, bounds)| bounds)
        .unwrap();

    let mut map = vec![vec![b' '; 300]; 400];

    for p in points.iter() {
        map[(p.position.y + n * p.velocity.y) as usize - 100]
            [(p.position.x + n * p.velocity.x) as usize - 150] = b'*';
    }

    for row in map.into_iter().filter(|row| row.contains(&b'*')) {
        println!("{}", std::str::from_utf8(&row).unwrap());
    }
}
