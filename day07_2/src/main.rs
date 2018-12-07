use aoc::aoc;

use std::collections::{HashMap, HashSet};

fn parse(s: &str) -> (char, char) {
    let mut dependency = s.chars().skip(5);
    let mut task = s.chars().skip(36);

    (dependency.next().unwrap(), task.next().unwrap())
}

#[derive(Debug)]
struct Worker {
    work: Option<char>,

    ticks_left: usize,
    default_ticks: usize,
}

pub enum Poll<T> {
    Ready(T),
    NotReady,
}

impl Worker {
    fn new(default_ticks: usize) -> Self {
        Self {
            work: None,
            ticks_left: 0,
            default_ticks: default_ticks,
        }
    }

    fn spawn(&mut self, c: char) {
        self.work = Some(c);
        self.ticks_left = (c as u8 - b'A') as usize + 1 + self.default_ticks;
    }

    pub fn poll(&mut self) -> Option<Poll<char>> {
        self.ticks_left = self.ticks_left.saturating_sub(1);
        if self.ticks_left == 0 {
            self.work.take().map(|w| Poll::Ready(w))
        } else {
            Some(Poll::NotReady)
        }
    }
}

fn solve(
    mut tasks: HashMap<char, HashSet<char>>,
    number_of_workers: usize,
    default_time: usize,
) -> usize {
    let mut workers = (0..number_of_workers)
        .map(|_| Worker::new(default_time))
        .collect::<Vec<_>>();

    let mut output = String::new();
    let mut spawnable_tasks = Vec::new();

    let mut tick = 0;

    loop {
        tick += 1;
        let mut idles = Vec::new();

        for worker in workers.iter_mut() {
            match worker.poll() {
                // Some task was completed.
                // Push it to the output,
                // Remove it as a dependency for all tasks that depend on the just completed task.
                // This worker is now idle, push it to the idle workers.
                Some(Poll::Ready(task_complete)) => {
                    output.push(task_complete);

                    for (_, dependencies) in &mut tasks {
                        dependencies.remove(&task_complete);
                    }

                    idles.push(worker);
                }

                // this worker was idle, push it to the idles.
                None => {
                    idles.push(worker);
                }

                // this worker is still bussy. Let it do it's job
                Some(Poll::NotReady) => {}
            }
        }

        // No more tasks, all workers are idle: We're done.
        if tasks.is_empty() && idles.len() == number_of_workers {
            break;
        }

        // We can't spawn new work yet!
        if idles.is_empty() {
            continue;
        }

        // Find the next tasks to be spawned.
        // A task can be spawned if all its dependency's are satisfied.
        spawnable_tasks.extend(
            tasks
                .iter()
                .filter(|(_, dependencies)| dependencies.is_empty())
                .map(|(task, _)| *task),
        );

        spawnable_tasks.sort();

        // Spawn the tasks on the workers.
        // Remove the task from the tasks, because we started working on it.
        for (worker, task) in idles.into_iter().zip(spawnable_tasks.drain(..)) {
            tasks.remove(&task);
            worker.spawn(task);
        }
    }

    return tick - 1;
}

#[aoc(2018, 7, 2)]
fn main(input: &str) -> usize {
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

    solve(task_dependency_map, 5, 60)
}
