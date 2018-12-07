use aoc::aoc;

use std::collections::{HashMap, HashSet};

fn parse(s: &str) -> (char, char) {
    let mut dependency = s.chars().skip(5);
    let mut task = s.chars().skip(36);

    (dependency.next().unwrap(), task.next().unwrap())
}

fn solve(mut tasks: HashMap<char, HashSet<char>>) -> String {
    let mut answer = String::new();
    let mut candidates: Vec<char> = Vec::new();

    while !tasks.is_empty() {
        candidates.extend(
            tasks
                .iter()
                .filter(|(_, dependencies)| dependencies.is_empty())
                .map(|(task, _)| task.clone()),
        );

        candidates.sort();
        let task = candidates[0];
        candidates.clear();
        answer.push(task);

        tasks.remove(&task);

        for (_, dependencies) in &mut tasks {
            dependencies.remove(&task);
        }
    }

    answer
}

#[aoc(2018, 7, 1)]
fn main(input: &str) -> String {
    let mut task_dependency_map = HashMap::new();

    // add tasks and dependencies to the task_dependency_map
    for line in input.lines() {
        let (dependency, task) = parse(line);

        task_dependency_map
            .entry(task)
            .or_insert(HashSet::new())
            .insert(dependency);
    }

    let mut are_dependencies_only = Vec::new();

    // find the things which are not tasks, but *only* a dependency
    for (_, dependencies) in task_dependency_map.iter() {
        for dependency in dependencies.iter() {
            if !task_dependency_map.contains_key(dependency) {
                are_dependencies_only.push(*dependency);
            }
        }
    }

    // The ones which are dependency only, but not a task, depend on an empty dependency list.
    for dependency in are_dependencies_only {
        task_dependency_map
            .entry(dependency)
            .or_insert(HashSet::new());
    }

    solve(task_dependency_map)
}
