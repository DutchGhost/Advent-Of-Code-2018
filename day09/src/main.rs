use aoc::aoc;

use std::collections::VecDeque;

fn parse(s: &str) -> (usize, usize) {
    let mut splitter = s.trim().split_whitespace();

    let players = splitter.next().unwrap().parse().unwrap();
    let last_marble = splitter.nth(5).unwrap().parse().unwrap();

    (players, last_marble)
}

fn rotate_clockwise<T>(marbles: &mut VecDeque<T>) {
    if let Some(popped_back) = marbles.pop_back() {
        marbles.push_front(popped_back);
    }
}

fn rotate_counter_clockwise<T>(marbles: &mut VecDeque<T>) {
    if let Some(popped_front) = marbles.pop_front() {
        marbles.push_back(popped_front);
    }
}

fn normal_step<T>(marbles: &mut VecDeque<T>, marble: T) {
    rotate_clockwise(marbles);
    marbles.push_front(marble);
}

fn special_step<T>(marbles: &mut VecDeque<T>) -> T {
    (0..7).for_each(|_| rotate_counter_clockwise(marbles));

    let score = marbles.pop_front().unwrap();

    rotate_clockwise(marbles);
    score
}

#[aoc(2018, 9, 1)]
fn main(input: &str) -> Option<usize> {
    let (players, max_marble) = parse(input);

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
