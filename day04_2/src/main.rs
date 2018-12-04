use aoc::aoc;

use chrono::{
    naive::{NaiveDateTime, NaiveTime},
};

use std::{collections::HashMap, str::FromStr};

#[derive(Debug, Hash, Copy, Clone)]
enum Action {
    Begin(NaiveDateTime, usize),
    Asleep(NaiveDateTime),
    Wake(NaiveDateTime),
}

impl Action {
    fn is_asleep(&self) -> bool {
        match self {
            &Action::Asleep(_) => true,
            _ => false,
        }
    }

    fn is_awake(&self) -> bool {
        match self {
            &Action::Wake(_) => true,
            _ => false,
        }
    }

    fn as_time(&self) -> NaiveTime {
        match self {
            &Action::Begin(time, _) | &Action::Asleep(time) | &Action::Wake(time) => time.time(),
        }
    }
}

impl FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (date, action) = s.split_at(18);
        let date = NaiveDateTime::parse_from_str(date, "[%Y-%m-%d %H:%M]").unwrap();

        Ok(match &action[1..] {
            "falls asleep" => Action::Asleep(date),
            "wakes up" => Action::Wake(date),
            _ => {
                let guard_id = action.split_whitespace().nth(1).unwrap()[1..]
                    .parse()
                    .unwrap();
                Action::Begin(date, guard_id)
            }
        })
    }
}

#[derive(Debug)]
struct SleepPeriod {
    start: NaiveTime,
    end: NaiveTime,
}

impl SleepPeriod {
    fn contains(&self, other: NaiveTime) -> bool {
        other >= self.start && other < self.end
    }
}

fn most_frequent_minute(schedule: &Vec<Action>, buffer: &mut Vec<SleepPeriod>) -> (usize, usize) {
    buffer.extend(sleep_pattern(schedule));

    (0..60usize)
        .map(|min| {
            let time = NaiveTime::from_hms(0, min as u32, 0);
            let frequency = buffer.iter().filter(|span| span.contains(time)).count();
            (min, frequency)
        }).max_by_key(|&(_, frequency)| frequency)
        .unwrap()
}

fn build_sleep_schedules(actions: Vec<Action>) -> HashMap<usize, Vec<Action>> {
    let mut schedule = HashMap::new();

    let mut current: &mut Vec<Action> = &mut Vec::new();

    for action in actions {
        match action {
            Action::Begin(_, guard) => {
                current = schedule.entry(guard).or_insert(Vec::new());
                current.push(action);
            }
            _ => current.push(action),
        }
    }

    schedule
}

fn sleep_pattern<'a>(v: &'a [Action]) -> impl Iterator<Item = SleepPeriod> + 'a {
    v.windows(2)
        .map(|actions| (actions[0], actions[1]))
        .filter(|(action1, action2)| action1.is_asleep() && action2.is_awake())
        .map(|(sleep, wake)| SleepPeriod {
            start: sleep.as_time(),
            end: wake.as_time(),
        })
}

#[aoc(2018, 4, 2)]
fn main(input: &str) -> usize {
    let mut v = input
        .lines()
        .map(Action::from_str)
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    v.sort_by_key(|e| match e {
        &Action::Begin(date, _) | &Action::Asleep(date) | &Action::Wake(date) => date,
    });

    let mut buffer = Vec::new();

    let (sleepiest_guard, sleepiest_minute) = build_sleep_schedules(v)
        .into_iter()
        .map(|(guard, scheds)| {
            let (sleepiest_minute, frequency) = most_frequent_minute(&scheds, &mut buffer);
            buffer.clear();
            (guard, sleepiest_minute, frequency)
        })
        .max_by_key(|&(_, _, most_frequent)| most_frequent)
        .map(|(guard, sleepiest_minute, _)| (guard, sleepiest_minute))
        .unwrap();

    sleepiest_guard * sleepiest_minute
}
