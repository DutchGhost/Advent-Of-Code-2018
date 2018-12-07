use aoc::aoc;

use std::collections::{HashSet, BTreeMap, HashMap};

fn parse(s: &str) -> (char, char) {
    let mut pre = s.chars().skip(5);
    let mut post = s.chars().skip(36);

    (pre.next().unwrap(), post.next().unwrap())
}

fn solve(mut instructions: Vec<Step>) {
    let mut posts = HashSet::new();
    let mut pres = HashSet::new();
    
    let mut answer = String::new();

    while !instructions.is_empty() {
        for instruction in instructions.iter() {
            posts.extend(instruction.posts.iter().cloned());
            pres.insert(instruction.pre);

        }
        
        let mut lasts = pres.difference(&posts).cloned().collect::<Vec<_>>();
        lasts.sort();
 
        let last = lasts[0];
        answer.push(last);

        for ins in instructions.iter_mut() {
            ins.posts.retain(|x| x != &last);
        }

        instructions.retain(|ins| ins.pre != last);

        posts.clear();
        pres.clear();
        println!();
        println!("LAST {:?}", last);

        for ins in instructions.iter() {
            println!("{:?}", ins);
        }

    }

   // answer.extend(instructions[0].posts.drain(..));
    println!("{:?}", instructions);
    println!("{:?}", answer);
}

#[derive(Debug)]
struct Step {
    pre: char,

    posts: Vec<char>,
}

#[aoc(2018, 7, 1)]
fn main(input2: &str) {
    let input = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin";
    let mut map = BTreeMap::new();

    for line in input.lines() {
        let (pre, post) = parse(line);

        map.entry(pre).or_insert(HashSet::new()).insert(post);
    }

    let mut instructions = Vec::new();

    for (pre, mut posts) in map {
        let posts = posts.into_iter().collect::<Vec<_>>();
        instructions.push(Step { pre, posts });
    }

    for instruction in instructions.iter() {
        println!("{:?}", instruction);
    }

    solve(instructions);
}
