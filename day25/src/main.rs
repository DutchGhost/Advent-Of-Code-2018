use aoc::aoc;

//use std::cell::Cell;
use std::mem;

#[derive(Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
    q: i32,
}

impl Point {
    fn manhatten(&self, other: &Self) -> i32 {
        (self.x - other.x).abs()
            + (self.y - other.y).abs()
            + (self.z - other.z).abs()
            + (self.q - other.q).abs()
    }

    fn can_merge_with(&self, other: &ConstellationPoint) -> bool {
        match other {
            ConstellationPoint::Merged(v) => v.iter().any(|item| self.can_merge_with(item)),
            ConstellationPoint::Point(p) => self.manhatten(p) <= 3,
        }
    }
}

#[derive(Debug)]
enum ConstellationPoint {
    Merged(Vec<ConstellationPoint>),
    Point(Point),
}

impl ConstellationPoint {
    fn can_merge(&self, other: &Self) -> bool {
        match self {
            ConstellationPoint::Merged(v) => v.iter().any(|item| other.can_merge(item)),

            // We know `self` is a single Point, so call can_merge_with to recursively test what `other` is,
            ConstellationPoint::Point(p) => p.can_merge_with(other),
        }
    }
}

#[allow(clippy::many_single_char_names)]
fn parse(s: &str) -> Vec<Vec<ConstellationPoint>> {
    s.lines()
        .map(|line| {
            let mut line_splitter = line.split(',');

            let x = line_splitter.next().unwrap().parse().unwrap();
            let y = line_splitter.next().unwrap().parse().unwrap();
            let z = line_splitter.next().unwrap().parse().unwrap();
            let q = line_splitter.next().unwrap().parse().unwrap();

            vec![ConstellationPoint::Point(Point { x, y, z, q })]
        })
        .collect()
}

fn can_merge(constellation: &[ConstellationPoint], to_check: &[ConstellationPoint]) -> bool {
    constellation
        .iter()
        .any(|point| to_check.iter().any(|p| point.can_merge(p)))
}

#[aoc(2018, 25, 1)]
fn main(input: &str) -> usize {
    let parsed = parse(input);
    
    let mut constellations = parsed;

    let mut start_constellations = 0;
    
    // We use manual range loops here in combination with `[T]::split_at_mut`
    // in order to avoid borrow issues.
    // The slice is splitted in constellations that are already checked, and
    // thus possible merge candidates, and constellations that can be merged
    // into.
    //
    // Loop over the possible mergeable constellations, if they can merge with
    // the first constellation of the constellations that can be merged into,
    // merge them.
    // Merging:
    //  - Replace the mergeable constellation with an empty Vec (moving out)
    //  - Push the moved out constellation onto the constellation to merge into,
    //  - Swap the index of the merged constellation with the start of the mergeable
    //      constellations.
    //  - Increase the start of the mergeable constellations
    //
    //  This way, all the constellations merged from will pile up from the beginning.
    for idx in 0..constellations.len() {
        let (mergeable_candidates, merge_into) = constellations.split_at_mut(idx);
        
        let constell = &mut merge_into[0];
        
        for idx in start_constellations..idx {
            if can_merge(constell, &mergeable_candidates[idx]) {
            
                let found_constell = mem::replace(&mut mergeable_candidates[idx], Vec::new());
                constell.push(ConstellationPoint::Merged(found_constell));
                    

                mergeable_candidates.swap(start_constellations, idx);
                start_constellations += 1;
            }
        } 
    }
    constellations.len() - start_constellations
}
