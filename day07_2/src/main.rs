use aoc::aoc;

use hashbrown::{HashMap, HashSet};

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
            default_ticks,
        }
    }

    fn spawn(&mut self, c: char) {
        self.work = Some(c);
        self.ticks_left = (c as u8 - b'A') as usize + 1 + self.default_ticks;
    }

    pub fn poll(&mut self) -> Option<Poll<char>> {
        self.ticks_left = self.ticks_left.saturating_sub(1);
        if self.ticks_left == 0 {
            self.work.take().map(Poll::Ready)
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
        let mut idles: Vec<&mut Worker> = Vec::new();
        
        for worker in workers.iter_mut() {
            match worker.poll() {
                // Some task was completed.
                // Push it to the output,
                // Remove it as a dependency for all tasks that depend on the just completed task.
                // This worker is now idle, push it to the idle workers.
                Some(Poll::Ready(task_complete)) => {
                    output.push(task_complete);

                    for dependencies in tasks.values_mut() {
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

            let min_ticks_left = workers.iter().map(|task| task.ticks_left).min().unwrap();
            for mut task in &mut workers {
                task.ticks_left -= min_ticks_left;
            }
            tick += min_ticks_left;
            // nll know's about this continue...
            // it sees we are wrapping around to the next iteration of the loop {},
            // so therefore we can borrow `workers` here as we please.
            // (normally this would conflict because `idles` still holds references to items of `workers`,
            // but in this entire if-block, `idles` isn't used.)
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

    tick - 1
}

#[aoc(2018, 7, 2)]
fn main(input: &str) -> usize {
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

    solve(task_dependency_map, 5, 60)
}
