use aoc::aoc;

use chrono::{
    naive::{NaiveDate, NaiveDateTime, NaiveTime},
    Duration,
};

use lazy_static::lazy_static;
use regex::Regex;

use std::collections::HashMap;

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

lazy_static! {
    static ref MATCHER: Regex =
        Regex::new(r"(\d+)-(\d+)-(\d+) (\d+):(\d+)] [(\w?\s)|#(\d+)]+").unwrap();
    static ref INT_PARSE: Regex = Regex::new(r"#(\d+)").unwrap();
}

fn parse(s: &str) -> Action {
    let caps = MATCHER.captures(s).unwrap();

    let year = caps[1].parse().unwrap();
    let month = caps[2].parse().unwrap();
    let day = caps[3].parse().unwrap();
    let hour = caps[4].parse().unwrap();
    let min = caps[5].parse().unwrap();

    let date = NaiveDate::from_ymd(year, month, day).and_hms(hour, min, 0);

    if s.contains("falls") {
        Action::Asleep(date)
    } else if s.contains("wakes") {
        Action::Wake(date)
    } else {
        let nums = INT_PARSE.captures(s).unwrap();
        Action::Begin(date, nums[1].parse().unwrap())
    }
}

fn minutes_asleep(schedule: &Vec<Action>) -> Duration {
    sleep_periods(schedule)
        .map(|period| period.end - period.start)
        .fold(Duration::zero(), |current_min_asleep, slept| {
            current_min_asleep + slept
        })
}

#[derive(Debug)]
struct SleepPeriod {
    start: NaiveTime,
    end: NaiveTime,
}

fn actions_per_guard(actions: Vec<Action>) -> HashMap<usize, Vec<Action>> {
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

fn sleep_periods<'a>(v: &'a [Action]) -> impl Iterator<Item = SleepPeriod> + 'a {
    v.windows(2)
        .map(|actions| (actions[0], actions[1]))
        .filter(|(action1, action2)| action1.is_asleep() && action2.is_awake())
        .map(|(sleep, wake)| SleepPeriod {
            start: sleep.as_time(),
            end: wake.as_time(),
        })
}

#[aoc(2018, 4, 1)]
fn main(input: &str) -> usize {
    let mut v = input.lines().map(parse).collect::<Vec<_>>();

    v.sort_by_key(|e| match e {
        &Action::Begin(date, _) | &Action::Asleep(date) | &Action::Wake(date) => date,
    });

    let (guard, longest_sleep) = actions_per_guard(v)
        .into_iter()
        .max_by_key(|(_, v)| minutes_asleep(&v))
        .unwrap();

    let sleep_periods = sleep_periods(&longest_sleep).collect::<Vec<_>>();

    let minute = (0..60usize)
        .map(|min| {
            let time = NaiveTime::from_hms(0, min as u32, 0);

            (
                min,
                sleep_periods
                    .iter()
                    .filter(|span| time >= span.start && time < span.end)
                    .count(),
            )
        }).max_by_key(|&(_, count)| count)
        .map(|(min, _)| min)
        .unwrap();

    minute * guard
}
