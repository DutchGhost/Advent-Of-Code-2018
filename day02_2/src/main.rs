const PUZZLE: &str = include_str!("input.txt");

use std::{
    collections::HashMap,
};

fn main() {
    let boxes = PUZZLE.lines().collect::<Vec<_>>();

    let mut common = String::new();

    'outer: for (idx, b1) in boxes.iter().enumerate() {
        for b2 in boxes[idx..].iter() {
            let mut faults = 0;

            for(c1, c2) in b1.chars().zip(b2.chars()) {
                if c1 != c2 {
                    faults += 1;
                } else {
                    common.push(c1);
                }

                if faults > 1 {
                    break;
                }
            }

            if faults == 1 {
                break 'outer;
            }
            common.clear();
        }
    }

    println!("{}", common);
}