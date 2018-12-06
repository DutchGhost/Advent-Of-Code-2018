use aoc::aoc;

use std::collections::{HashMap, HashSet};

#[derive(Debug, Copy, Clone, Default, Hash)]
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

#[aoc(2018, 6, 1)]
fn main(input: &str) -> Option<usize> {
    let points = input.lines().map(Point::from_str).collect::<Vec<_>>();

    let max_y = points.iter().map(|p| p.y).max().unwrap();
    let max_x = points.iter().map(|p| p.x).max().unwrap();

    let mut locations = HashMap::new();
    let mut infinites = HashSet::new();

    let mut manhatten_distances = Vec::new();
    for (x, y) in coordinates(max_x, max_y) {
        let iter = points.iter().enumerate().map(|(idx, p)| {
            let dist_x = (p.x - x).abs();
            let dist_y = (p.y - y).abs();

            (idx, dist_x + dist_y)
        });

        manhatten_distances.clear();
        manhatten_distances.extend(iter);
        manhatten_distances.sort_by_key(|&(_, dist)| dist);

        let (p, potential_closest_dist) = manhatten_distances[0];

        // there was a tie
        if manhatten_distances
            .iter()
            .take_while(|(_, dist)| &potential_closest_dist == dist)
            .count()
            != 1
        {
            continue;
        }

        if x == 0 || x == max_x {
            infinites.insert(p);
        }

        if y == 0 || y == max_y {
            infinites.insert(p);
        }

        *locations.entry(p).or_insert(0) += 1;
    }

    locations
        .iter()
        .filter(|(k, _)| !infinites.contains(k))
        .map(|(_, v)| v)
        .max()
        .map(|n| *n)
}
