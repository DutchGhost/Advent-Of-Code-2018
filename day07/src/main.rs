use aoc::aoc;

use std::collections::HashSet;

mod dependencygraph;
use dependencygraph::{DependencyGraph, Dependents};

fn parse(s: &str) -> (char, char) {
    let mut dependency = s.chars().skip(5);
    let mut task = s.chars().skip(36);

    (dependency.next().unwrap(), task.next().unwrap())
}

fn solve(mut tasks: DependencyGraph<char, HashSet<char>>) -> String {
    let mut answer = String::new();
    let mut candidates: Vec<char> = Vec::new();

    while !tasks.is_empty() {
        let iter = tasks
            .iter()
            .filter(|(_, dependencies)| dependencies.is_empty())
            .map(|(task, _)| *task);

        candidates.extend(iter);

        candidates.sort();
        let task = candidates[0];
        candidates.clear();
        answer.push(task);

        tasks.remove_task(&task);

        for dependencies in tasks.values_mut() {
            dependencies.remove(&task);
        }
    }

    answer
}

#[aoc(2018, 7, 1)]
fn main(input: &str) -> String {
    let mut depgraph: DependencyGraph<char, HashSet<char>> = DependencyGraph::new();

    for line in input.lines() {
        let (dependency, task) = parse(line);
        depgraph.task(task).add_dependency(dependency);
        depgraph.task(dependency);
    }

    solve(depgraph)
}
