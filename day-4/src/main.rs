use regex::Regex;
//use itertools::Itertools;
use std::collections::BTreeMap;

static INPUT: &str = include_str!("../input.txt");

type Error = Box<std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Timestamp {
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
}

impl Timestamp {
    fn minutes_until<'a>(&'a self, other: &'a Self) -> impl Iterator<Item = u32> + 'a {
        // all asleep/awake times are during the midnight hour
        self.minute..other.minute
    }
}

#[derive(Debug, Copy, Clone)]
enum Event {
    Wake,
    Sleep,
    Start(u32),
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum State {
    Asleep,
    Awake,
}

#[derive(Debug, Copy, Clone)]
struct LogEntry {
    timestamp: Timestamp,
    event: Event,
}

fn main() -> Result<()> {
    let mut log_entries = log_entries()?;
    log_entries.sort_by_key(|e| e.timestamp);

    let mut time = BTreeMap::new();

    let mut current_guard = None;

    use self::Event::*;
    use self::State::*;

    for entry in log_entries {
        match entry.event {
            Start(id) => {
                current_guard = Some((id, Awake, entry.timestamp));
            }
            Sleep => {
                let (id, state, _timestamp) = current_guard.ok_or("No current guard")?;
                assert_eq!(state, Awake, "guard {}, is already asleep", id);
                current_guard = Some((id, Asleep, entry.timestamp));
            }
            Wake => {
                let (id, state, timestamp) = current_guard.ok_or("No current guard")?;
                assert_eq!(state, Asleep, "guard {}, is already awake", id);

                let minutes = time.entry(id).or_insert_with(BTreeMap::new);
                for minute in timestamp.minutes_until(&entry.timestamp) {
                    *minutes.entry(minute).or_insert(0) += 1;
                }

                current_guard = Some((id, Awake, entry.timestamp));
            }
        }
    }

    let sleepiest_guard = time
        .iter()
        .max_by_key(|(_, x)| x.values().sum::<u32>())
        .map(|(id, _)| id);

    if let Some(sleepiest_guard) = sleepiest_guard {
        let sleepiest_minute = time[&sleepiest_guard]
            .iter()
            .max_by_key(|(_, &count)| count)
            .map(|(minute, _)| minute)
            .expect("Must be some time they are asleep");

        println!(
            "The sleepiest guard is {} at {} ({})",
            sleepiest_guard,
            sleepiest_minute,
            sleepiest_guard * sleepiest_minute
        );
    }

    Ok(())
}

fn log_entries() -> Result<Vec<LogEntry>> {
    // [1518-11-07 00:21] falls asleep
    // wakes up
    // Guard #1823 begins shift
    let log_entry_regex = Regex::new(
        r"(?x)
        \[
        (?P<year>\d+)
        -
        (?P<month>\d+)
        -
        (?P<day>\d+)
        \s+
        (?P<hour>\d+)
        :
        (?P<minute>\d+)
        \]
        \s+
        (?:
        (?P<sleeps>falls\s+asleep)
        |
        (?P<wakes>wakes\s+up)
        |
        (Guard\s+\#(?P<starts>\d+)\s+begins\s+shift)
        )
    ",
    )
    .unwrap();

    INPUT
        .lines()
        .map(|l| {
            let captures = log_entry_regex
                .captures(l)
                .ok_or_else(|| "No matching captures")?;

            let year = captures.name("year").ok_or_else(|| "No Year")?;
            let year = year.as_str().parse()?;

            let month = captures.name("month").ok_or_else(|| "No Month")?;
            let month = month.as_str().parse()?;

            let day = captures.name("day").ok_or_else(|| "No Day")?;
            let day = day.as_str().parse()?;

            let hour = captures.name("hour").ok_or_else(|| "No Hour")?;
            let hour = hour.as_str().parse()?;

            let minute = captures.name("minute").ok_or_else(|| "No Minute")?;
            let minute = minute.as_str().parse()?;

            let timestamp = Timestamp {
                year,
                month,
                day,
                hour,
                minute,
            };

            let event = match (
                captures.name("sleeps"),
                captures.name("wakes"),
                captures.name("starts"),
            ) {
                (Some(_), _, _) => Event::Sleep,
                (_, Some(_), _) => Event::Wake,
                (_, _, Some(s)) => Event::Start(s.as_str().parse()?),
                _ => return Err(Error::from("Unknown event type")),
            };

            Ok(LogEntry { timestamp, event })
        })
        .collect()
}
