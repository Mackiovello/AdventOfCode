use super::get_input_vec;
use chrono::prelude::*;
use std::collections::HashMap;
use regex::Regex;
use chrono::Duration;

// previous answer: 36454
// second answer was right: 125444 
pub fn problem_four_part_one() -> u32 {
    let input = get_input_vec("four.txt");
    let input_refs = input.iter().map(AsRef::as_ref).collect();
    let entries = parse_into_entries(input_refs);
    let guards = group_entries_by_guard(entries);
    let most_sleep = get_guard_with_most_sleep(guards);
    let most_slept_minute = most_sleep.get_most_slept_minute();
    most_sleep.id * most_slept_minute.0 as u32
}

// first try: 11728 - too low
// second try: 18325 - correct
pub fn problem_four_part_two() -> u32 {
    let input = get_input_vec("four.txt");
    let input_refs = input.iter().map(AsRef::as_ref).collect();
    let entries = parse_into_entries(input_refs);
    let guards = group_entries_by_guard(entries);

    let most_slept_minute_with_guard_id = guards
        .iter()
        .map(|g| {
            let most_slept_minute_with_frequency = g.get_most_slept_minute(); 
            // id, most slept minute, freq of most slept minute
            (g.id, most_slept_minute_with_frequency.0, most_slept_minute_with_frequency.1)
        })
        .max_by(|a, b| a.2.cmp(&b.2))
        .unwrap();

    most_slept_minute_with_guard_id.0 * most_slept_minute_with_guard_id.1
}

#[derive(Debug, PartialEq)]
struct Guard {
    id: u32,
    entries: Vec<ParsedEntry>
}

#[derive(Debug, PartialEq, Clone)]
struct ParsedEntry {
    timestamp: DateTime<Utc>,
    action: GuardAction
}

impl ParsedEntry {
    fn from(entry: &Entry) -> ParsedEntry {
        ParsedEntry {
            timestamp: entry.timestamp,
            action: parse_action(&entry.action)
        }
    }
}

#[derive(Debug)]
struct Entry {
    timestamp: DateTime<Utc>,
    action: String
}

#[derive(Debug, PartialEq, Clone)]
enum GuardAction {
    WakesUp,
    FallsAsleep
}

impl Guard {
    fn get_minutes_of_sleep(&self) -> u32 {
        let awake = self.entries.iter().filter(|e| e.action == GuardAction::WakesUp);
        let asleep = self.entries.iter().filter(|e| e.action == GuardAction::FallsAsleep);

        awake.zip(asleep)
            .map(|e| e.0.timestamp - e.1.timestamp)
            .fold(Duration::zero(), |a, b| a + b)
            .num_minutes() as u32
    }

    fn get_most_slept_minute(&self) -> (u32, u32) {
        let mut slept_minutes: Vec<u32> = Vec::new();

        let awake = self.entries.iter().filter(|e| e.action == GuardAction::WakesUp);
        let asleep = self.entries.iter().filter(|e| e.action == GuardAction::FallsAsleep);

        awake.zip(asleep).for_each(|e| {
            let woke_up = e.0.timestamp.minute();
            let fell_asleep = e.1.timestamp.minute();

            let mut minutes = (fell_asleep..woke_up).collect::<Vec<u32>>();
            slept_minutes.append(&mut minutes);
        });

        let mut minute_count: HashMap<u32, u32> = HashMap::new();

        slept_minutes.iter().for_each(|c| {
            let v = minute_count.entry(*c).or_insert(0);
            *v += 1;
        });

        minute_count.into_iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap_or((0,0))
    }
}

fn parse_action(action: &str) -> GuardAction {
    if action.starts_with("falls") {
        GuardAction::FallsAsleep
    } else if action.starts_with("wakes") {
        GuardAction::WakesUp
    } else {
        panic!("Invalid action");
    }
}

fn get_guard_with_most_sleep(guards: Vec<Guard>) -> Guard {
    guards
        .into_iter()
        .max_by(|a, b| a.get_minutes_of_sleep().cmp(&b.get_minutes_of_sleep()))
        .unwrap()
}

fn group_entries_by_guard(entries: Vec<Entry>) -> Vec<Guard> {
    let mut guards: HashMap<u32, Vec<ParsedEntry>> = HashMap::new();
    let mut current_id = 0;

    let extract_id_regex = Regex::new(r"^Guard #(\d{1,})").unwrap();

    for entry in entries {
        if let Some(captured) = extract_id_regex.captures(&entry.action) {
            let id = captured[1].parse::<u32>().unwrap();

            if !guards.contains_key(&id) {
                guards.insert(id, vec![]);
            }
            current_id = id;
        } else {
            guards.get_mut(&current_id).unwrap().push(ParsedEntry::from(&entry))
        }
    };

    guards.iter().map(|e| Guard { id: *e.0, entries: e.1.to_vec() } ).collect()
}


fn parse_into_entries(records: Vec<&str>) -> Vec<Entry> {
    let regex = Regex::new(r"^\[(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2})\] (.*)$").unwrap();

    let mut entries = records.iter().map(|entry| {
        let components = regex.captures(entry).unwrap();

        let mut relevant_components = components
            .iter()
            .skip(1)
            .map(|c| c.unwrap().to_owned())
            .collect::<Vec<String>>();

        let action = relevant_components.pop().unwrap();

        let date_time = Utc
            .ymd(
                relevant_components[0].parse::<i32>().unwrap(), 
                relevant_components[1].parse::<u32>().unwrap(), 
                relevant_components[2].parse::<u32>().unwrap())
            .and_hms(
                relevant_components[3].parse::<u32>().unwrap(), 
                relevant_components[4].parse::<u32>().unwrap(), 
                0); 

        Entry { 
            timestamp: date_time, 
            action: action 
        }
    }).collect::<Vec<Entry>>();
    
    entries.sort_unstable_by(|a, b| a.timestamp.cmp(&b.timestamp));

    entries
}