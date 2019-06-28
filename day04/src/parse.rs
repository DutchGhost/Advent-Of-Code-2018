use chrono::naive::NaiveDateTime;

use std::iter::IntoIterator;
use std::ops::Deref;
use std::vec::IntoIter;

/// An UnknownEvent is an event,
/// but the `event-type` is unknown.
#[derive(Debug, Clone, Copy)]
pub struct UnknownEvent<'a>(&'a str);

impl Deref for UnknownEvent<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

/// An Observation is just a simple observation
/// containing
#[derive(Debug, Clone, Copy)]
pub struct Observation<'a> {
    timestamp: NaiveDateTime,
    event: UnknownEvent<'a>,
}

impl Observation<'_> {
    pub fn event(&self) -> UnknownEvent {
        self.event
    }

    pub fn timestamp(&self) -> NaiveDateTime {
        self.timestamp
    }
}

impl<'a, S: AsRef<str> + ?Sized> From<&'a S> for Observation<'a> {
    fn from(observation: &'a S) -> Self {
        let observation = observation.as_ref();

        let (timestamp, event) = observation.split_at(18);
        let timestamp = NaiveDateTime::parse_from_str(timestamp, "[%Y-%m-%d %H:%M]").unwrap();

        Self {
            timestamp,
            event: UnknownEvent(event),
        }
    }
}

#[derive(Debug)]
pub struct Observations<'a> {
    observations: Vec<Observation<'a>>,
}

impl<'a> Observations<'a> {
    pub fn new(observations: &'a str) -> Self {
        let mut observations = observations
            .lines()
            .map(Observation::from)
            .collect::<Vec<_>>();

        observations.sort_by_key(|observation| observation.timestamp);
        Self { observations }
    }
}

impl<'a> IntoIterator for Observations<'a> {
    type Item = Observation<'a>;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.observations.into_iter()
    }
}
