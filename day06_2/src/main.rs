use aoc::aoc;

#[derive(Debug, Copy, Clone)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn from_str(s: &str) -> Self {
        let mut splitter = s.split(", ");

        let x = splitter.next().unwrap().parse().unwrap();
        let y = splitter.next().unwrap().parse().unwrap();

        Self { x, y }
    }
}

fn coordinates(x: isize, y: isize) -> impl Iterator<Item = (isize, isize)> {
    (0..=x).flat_map(move |x| (0..=y).map(move |y| (x, y)))
}

#[aoc(2018, 6, 2)]
fn main(input: &str) -> usize {
    let points = input.lines().map(Point::from_str).collect::<Vec<_>>();

    let max_y = points.iter().map(|p| p.y).max().unwrap();
    let max_x = points.iter().map(|p| p.x).max().unwrap();

    coordinates(max_x, max_y)
        .map(|(x, y)| {
            points
                .iter()
                .map(|point| {
                    let dist_x = (point.x - x).abs();
                    let dist_y = (point.y - y).abs();

                    dist_x + dist_y
                })
                .sum::<isize>()
        })
        .filter(|&total_distance| total_distance < 10000)
        .count()
}

/*
    Collect the points into a vec,

    for each coordinate in the list,
        sum all the manhattendistances between the coordinate and all points
        if the sum was less than 10000, add 1 to the area's size
*/
