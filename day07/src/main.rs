use aoc::aoc;

use std::collections::{HashMap, HashSet};

fn parse(s: &str) -> (char, char) {
    let mut pre = s.chars().skip(5);
    let mut post = s.chars().skip(36);

    (pre.next().unwrap(), post.next().unwrap())
}

fn solve(mut instructions: HashMap<char, HashSet<char>>) -> String {
    let mut answer = String::new();

    let mut steps = HashSet::new();

    for (pre, posts) in instructions.iter() {
        steps.insert(*pre);
        steps.extend(posts.iter().cloned());
    }

    let mut posts = HashSet::new();
    let mut candidates: Vec<char> = Vec::new();

    while !steps.is_empty() {
        for (_, post) in instructions.iter() {
            posts.extend(post.iter().cloned());
        }

        candidates.extend(steps.difference(&posts).cloned());
        candidates.sort();
        let n = candidates[0];
        candidates.clear();
        answer.push(n);

        instructions.remove(&n);

        steps.remove(&n);
        posts.clear();
    }

    answer
}

#[aoc(2018, 7, 1)]
fn main(input: &str) -> String {
    let mut map = HashMap::new();

    for line in input.lines() {
        let (pre, post) = parse(line);

        map.entry(pre).or_insert(HashSet::new()).insert(post);
    }
    solve(map)
}
