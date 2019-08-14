use std::collections::HashMap;

use chrono::{
    naive::{NaiveDateTime, NaiveTime},
    Duration,
};

type Guard = u64;

use crate::parse::Observations;

#[derive(Debug)]
struct SleepPeriod {
    start: NaiveTime,
    end: NaiveTime,
}

impl SleepPeriod {
    pub fn new(period: &[Record]) -> Option<Self> {
        if period.len() != 2 {
            return None;
        }

        if period[0].event.is_awake() {
            return None;
        }

        if period[1].event.is_asleep() {
            return None;
        }

        Some(Self {
            start: period[0].time(),
            end: period[1].time(),
        })
    }
}

impl SleepPeriod {
    fn contains(&self, other: NaiveTime) -> bool {
        other >= self.start && other < self.end
    }

    fn duration(&self) -> Duration {
        self.end - self.start
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum Event {
    Asleep,
    Awake,
}

impl Event {
    fn is_awake(&self) -> bool {
        match *self {
            Event::Awake => true,
            _ => false,
        }
    }

    fn is_asleep(&self) -> bool {
        !self.is_awake()
    }
}

#[derive(Debug, Hash)]
pub struct Record {
    event: Event,
    timestamp: NaiveDateTime,
}

impl Record {
    fn new(event: Event, timestamp: NaiveDateTime) -> Self {
        Self { event, timestamp }
    }

    fn time(&self) -> NaiveTime {
        self.timestamp.time()
    }
}

#[derive(Debug, Hash)]
pub struct SleepPattern {
    guard: Guard,
    begin: NaiveDateTime,
    pattern: Vec<Record>,
}

impl SleepPattern {
    const fn new(guard: u64, begin: NaiveDateTime, pattern: Vec<Record>) -> Self {
        Self {
            guard,
            begin,
            pattern,
        }
    }

    fn sleep_periods<'a>(&'a self) -> impl Iterator<Item = SleepPeriod> + 'a + Clone {
        self.pattern.windows(2).filter_map(SleepPeriod::new)
    }

    fn minutes_asleep(&self) -> Duration {
        self.sleep_periods()
            .fold(Duration::zero(), |current_min_asleep, period| {
                current_min_asleep + period.duration()
            })
    }

    pub const fn guard(&self) -> Guard {
        self.guard
    }

    pub fn sleepiest_minute(&self) -> u64 {
        (0..60)
            .map(|minute| {
                let time = NaiveTime::from_hms(0, minute, 0);
                let frequency = self
                    .sleep_periods()
                    .filter(|span| span.contains(time))
                    .count();

                (minute, frequency)
            })
            .max_by_key(|&(_, frequency)| frequency)
            .map(|(minute, _)| u64::from(minute))
            .unwrap()
    }
}

#[derive(Debug)]
pub struct SleepPatterns {
    patterns: HashMap<Guard, SleepPattern>,
}

impl SleepPatterns {
    pub fn sleepiest_guard(self) -> SleepPattern {
        self.patterns
            .into_iter()
            .map(|(_, v)| v)
            .max_by_key(|pattern| pattern.minutes_asleep())
            .unwrap()
    }
}

impl From<Observations<'_>> for SleepPatterns {
    fn from(observations: Observations<'_>) -> Self {
        let mut patterns = HashMap::new();

        let mut pattern = &mut Vec::new();

        for observation in observations {
            let timestamp = observation.timestamp();
            let observation = observation.event();

            match &observation[1..] {
                "falls asleep" => {
                    pattern.push(Record::new(Event::Asleep, timestamp));
                }
                "wakes up" => {
                    pattern.push(Record::new(Event::Awake, timestamp));
                }

                _ => {
                    let guard = observation.split_whitespace().nth(1).unwrap()[1..]
                        .parse()
                        .unwrap();
                    pattern = &mut patterns
                        .entry(guard)
                        .or_insert_with(|| SleepPattern::new(guard, timestamp, Vec::new()))
                        .pattern;
                }
            }
        }

        Self { patterns }
    }
}
