use aoc::aoc;

use chrono::{
    naive::{NaiveDateTime, NaiveTime},
    Duration,
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

#[derive(Debug)]
struct SleepPeriod {
    start: NaiveTime,
    end: NaiveTime,
}

impl SleepPeriod {
    fn contains(&self, other: NaiveTime) -> bool {
        other >= self.start && other < self.end
    }

    fn duration(&self) -> Duration {
        self.end - self.start
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

fn minutes_asleep(schedule: &[Action]) -> Duration {
    sleep_pattern(schedule).fold(Duration::zero(), |current_min_asleep, period| {
        current_min_asleep + period.duration()
    })
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

#[aoc(2018, 4, 1)]
fn main(input: &str) -> usize {
    let mut v: Vec<Action> = input
        .lines()
        .map(Action::from_str)
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    v.sort_by_key(|e| match e {
        &Action::Begin(date, _) | &Action::Asleep(date) | &Action::Wake(date) => date,
    });

    let (sleepiest_guard, sleeping_schedule) = build_sleep_schedules(v)
        .into_iter()
        .max_by_key(|(_, v)| minutes_asleep(&v))
        .unwrap();

    let minute = (1..60)
        .map(|minute| {
            let time = NaiveTime::from_hms(0, minute, 0);
            let frequency = sleep_pattern(&sleeping_schedule)
                .filter(|span| span.contains(time))
                .count();

            (minute, frequency)
        }).max_by_key(|&(_, frequency)| frequency)
        .map(|(minute, _)| minute as usize)
        .unwrap();

    minute * sleepiest_guard
}
