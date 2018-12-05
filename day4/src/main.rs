use std::env;
use std::fmt;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::collections::HashMap;

enum ShiftState {
    START,
    AWAKE,
    ASLEEP,
}

#[derive(Eq,PartialEq,PartialOrd,Ord)]
struct Timestamp {
    year: usize,
    month: usize,
    day: usize,
    hour: usize,
    min: usize,
}

struct LogEntry {
    guard_id: usize,
    event: ShiftState,
    timestamp: Timestamp,
}

struct Guard {
    guard_id: usize,
    minutes_slept: u32,
    minutes_slept_by_minute: [u32; 60],
}

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{:02}-{:02} {:02}:{:02}", self.year, self.month, self.day, self.hour, self.min)
    }
}

impl fmt::Display for LogEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let event_str = match self.event {
            ShiftState::START => "start",
            ShiftState::AWAKE => "awake",
            ShiftState::ASLEEP => "asleep",
        };
        write!(f, "[{}] #{} {}", self.timestamp, self.guard_id, event_str)
    }
}

impl fmt::Display for Guard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let minutes_slept_by_minute: String = self.minutes_slept_by_minute.
            iter().
            map(|m| m.to_string()).collect::<Vec<String>>().join(":");
        write!(f, "{}, {}, {}", self.guard_id, self.minutes_slept, minutes_slept_by_minute)
    }
}

impl LogEntry {
    fn parse(entry: &str) -> LogEntry {
        let tokens: Vec<&str> = entry.split_whitespace().collect();

        let date_values: Vec<usize> = tokens[0].trim_start_matches('[').
            split('-').
            map(|t| t.parse().expect("Bad date!"))
            .collect();

        let time_values: Vec<usize> = tokens[1].trim_end_matches(']').
            split(':').
            map(|t| t.parse().expect("Bad time!"))
            .collect();

        let guard_id: usize;
        let event: ShiftState;

        if tokens[2] == "Guard" {
            guard_id = tokens[3].trim_start_matches('#').parse().expect("Invalid guard id!");
            event = ShiftState::START;
        } else if tokens[2] == "falls" {
            guard_id = 0;
            event = ShiftState::ASLEEP;
        } else {
            guard_id = 0;
            event = ShiftState::AWAKE;
        }

        LogEntry {
            guard_id: guard_id,
            event: event,
            timestamp: Timestamp {
                year: date_values[0],
                month: date_values[1],
                day: date_values[2],
                hour: time_values[0],
                min: time_values[1],
            },
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 { panic!("Too few arguments!") }

    let f = File::open(&args[1]).expect("File not found!");
    let reader = BufReader::new(&f);

    let part: u32 = args[2].parse().expect("Invalid part!");

    let mut entries: Vec<LogEntry> = reader.
        lines().
        map(|l| LogEntry::parse(l.unwrap().trim())).
        collect();

    // Sort by timestamp
    entries.sort_unstable_by(|a,b| { a.timestamp.cmp(&b.timestamp) });

    // Accumulate a hash map of guard ids to guards
    let mut guards = HashMap::new();
    let mut guard_id = 0;

    for entry in &mut entries {
        if entry.guard_id != 0 {
            guard_id = entry.guard_id;
            guards.entry(entry.guard_id).
                or_insert(Guard {
                    guard_id:  entry.guard_id,
                    minutes_slept: 0,
                    minutes_slept_by_minute: [0; 60]
                });
        } else {
            entry.guard_id = guard_id;
        }
    }

    // Find sleepiest guard
    let mut sleepiest_guard_id = 0;
    let mut most_minutes_slept = 0;

    let mut most_consistent_guard_id = 0;
    let mut minute_with_most_sleeps = 0;
    let mut most_sleeps_in_minute = 0;

    let mut sleep_start_min = 0;
    let mut last_guard_id = 0;
    let mut last_event_was_sleep = false;

    let last_entry_id = entries.len() - 1;
    for entry_id in 0..=last_entry_id {
        let entry = &entries[entry_id];
        let update_sleepiness = match entry.event {
            ShiftState::ASLEEP => {
                sleep_start_min = entry.timestamp.min;
                last_guard_id = entry.guard_id;
                last_event_was_sleep = true;
                entry_id == last_entry_id
            },
            _ => { last_event_was_sleep }
        };

        if update_sleepiness {
            let mut guard = guards.get_mut(&last_guard_id).unwrap();
            let sleep_end_min = entry.timestamp.min;
            let minutes_slept = sleep_end_min - sleep_start_min;
            guard.minutes_slept += minutes_slept as u32;
            for min in sleep_start_min..sleep_end_min {
                guard.minutes_slept_by_minute[min] += 1;
            }

            if most_minutes_slept < guard.minutes_slept {
                sleepiest_guard_id = guard.guard_id;
                most_minutes_slept = guard.minutes_slept;
            }

            for min in 0..60 {
                if most_sleeps_in_minute < guard.minutes_slept_by_minute[min] {
                    most_sleeps_in_minute = guard.minutes_slept_by_minute[min];
                    minute_with_most_sleeps = min;
                    most_consistent_guard_id = last_guard_id;
                }
            }

            last_event_was_sleep = false;
        }
    }


    if part == 1 {
        let sleepiest_guard = guards.get(&sleepiest_guard_id).unwrap();
        let mut sleepiest_minute_count = 0;
        let mut sleepiest_minute = 0;
        for min in 0..60 {
            if sleepiest_minute_count < sleepiest_guard.minutes_slept_by_minute[min] {
                sleepiest_minute = min;
                sleepiest_minute_count = sleepiest_guard.minutes_slept_by_minute[min];
            }
        }
        println!("guard_id: {}, min: {}", sleepiest_guard_id, sleepiest_guard.minutes_slept_by_minute[sleepiest_minute]);
        println!("{}", sleepiest_guard_id * sleepiest_minute);
    } else {
        println!("guard_id: {}, min: {}", most_consistent_guard_id, most_sleeps_in_minute);
        println!("{}", most_consistent_guard_id * minute_with_most_sleeps);
    }
}
