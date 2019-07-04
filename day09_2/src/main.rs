use aoc::aoc;

use std::collections::VecDeque;

fn parse(s: &str) -> (usize, usize) {
    let mut splitter = s.trim().split_whitespace();

    let players = splitter.next().unwrap().parse().unwrap();
    let last_marble = splitter.nth(5).unwrap().parse().unwrap();

    (players, last_marble)
}

fn normal_step<T>(marbles: &mut VecDeque<T>, marble: T) {
    marbles.rotate_right(1);
    marbles.push_front(marble);
}

fn special_step<T>(marbles: &mut VecDeque<T>) -> T {
    marbles.rotate_left(7);

    let score = marbles.pop_front().unwrap();

    marbles.rotate_right(1);

    score
}

#[aoc(2018, 9, 2)]
fn main(input: &str) -> Option<usize> {
    let (players, mut max_marble) = parse(input);
    max_marble *= 100;
    let mut scores = vec![0; players];

    let mut marbles = VecDeque::with_capacity(max_marble);
    marbles.push_front(0);

    for (marble, player) in (1..max_marble).zip((0..players).cycle()) {
        if marble % 23 != 0 {
            normal_step(&mut marbles, marble);
        } else {
            scores[player] += special_step(&mut marbles) + marble;
        }
    }

    scores.into_iter().max()
}
