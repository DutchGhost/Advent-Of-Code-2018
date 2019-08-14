use aoc::aoc;
mod parse;
mod pattern;

use parse::Observations;
use pattern::SleepPatterns;

#[aoc(2018, 4, 1)]
fn main(input: &str) -> usize {
    let sleep_patterns: SleepPatterns = Observations::new(input).into();
    let sleepiest_guard = sleep_patterns.sleepiest_guard();
    let guard_id = sleepiest_guard.guard();
    let sleepiest_minute = sleepiest_guard.sleepiest_minute();
    (guard_id * sleepiest_minute) as usize
}
