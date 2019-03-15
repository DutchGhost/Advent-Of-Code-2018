use aoc::aoc;

use hashbrown::{HashMap, HashSet};

fn parse(s: &str) -> (char, char) {
    let mut dependency = s.chars().skip(5);
    let mut task = s.chars().skip(36);

    (dependency.next().unwrap(), task.next().unwrap())
}

fn solve(mut tasks: HashMap<char, HashSet<char>>) -> String {
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

        tasks.remove(&task);

        for dependencies in tasks.values_mut() {
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
            .or_insert_with(HashSet::new)
            .insert(dependency);

        // Make the dependency a task as well (if it's not a task already)
        task_dependency_map
            .entry(dependency)
            .or_insert_with(HashSet::new);
    }

    solve(task_dependency_map)
}
