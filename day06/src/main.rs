use aoc::aoc;

use std::collections::{HashMap, HashSet};

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

/// For a given slice of Points, and an (x, y) coordinate,
/// finds the Point that is closest in manhattendistance to the coordinate.
///
/// Whenever 2 or more points tie on the closest distance, None is returned.
#[inline(always)]
fn closest_point<'a>(
    points: &'a [Point],
    x: isize,
    y: isize,
    buffer: &mut Vec<(usize, isize)>,
) -> Option<usize> {
    let iter = points.iter().enumerate().map(|(idx, p)| {
        let dist_x = (p.x - x).abs();
        let dist_y = (p.y - y).abs();

        (idx, dist_x + dist_y)
    });

    buffer.extend(iter);
    buffer.sort_by_key(|&(_, dist)| dist);

    if buffer[0].1 == buffer[1].1 {
        None
    } else {
        Some(buffer[0].0)
    }
}

#[aoc(2018, 6, 1)]
fn main(input: &str) -> Option<usize> {
    let points = input.lines().map(Point::from_str).collect::<Vec<_>>();

    let max_x = points.iter().map(|point| point.x).max().unwrap();
    let max_y = points.iter().map(|point| point.y).max().unwrap();

    let mut areas = HashMap::new();
    let mut infinites = HashSet::new();

    let escapes_grid = |x: isize, y: isize| x == 0 || x == max_x || y == 0 || y == max_y;

    let mut buffer = Vec::new();

    for (x, y) in coordinates(max_x, max_y) {
        buffer.clear();

        if let Some(point) = closest_point(&points, x, y, &mut buffer) {
            if escapes_grid(x, y) {
                infinites.insert(point);
            }

            *areas.entry(point).or_insert(0) += 1;
        }
    }

    areas
        .iter()
        .filter(|(k, _)| !infinites.contains(k))
        .map(|(_, size)| size)
        .max()
        .map(|n| *n)
}

/*
    Collect the points into a vec,

    for each coordinate in the list,
        for each point:
            1) compute the manhatten distance between the coordinate and the point,
            2) find the minimum manhattendistance, if there are ties: go to the next coordinate
                If there are no ties, grab the point out
        
        If the coordinate escapes the grid, add it to the map of inifintes.
        From the locationsmap, increment the points area counter.
    Lastly, iterate over the locationsmap, filter out the locations that are infinite, find the max, and return it.
*/
