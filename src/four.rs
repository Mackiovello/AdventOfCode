use super::get_input_vec;
use chrono::NaiveDateTime;
use chrono::prelude::*;
use regex::Regex;

use std::collections::HashMap;

fn split_into_date_and_action(records: Vec<&str>) -> Vec<(&str, &str)> {
    let re = Regex::new(r"^\[(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2})\] (.*)$").unwrap();
    let text = "[1518-11-01 00:00] Guard #10 begins shift";
    let caps = re.captures(text).unwrap();
    println!("{:?}", caps);
    Vec::new()
}

fn parse_into_entries(records: Vec<&str>) -> Vec<(DateTime<Utc>, String)> {
    let regex = Regex::new(r"^\[(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2})\] (.*)$").unwrap();

    records.iter().map(|entry| {
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

        (date_time, action)
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regex_words() {
        // Given
        // When
    
        // Then
    }


    #[test]
    fn converts_records_to_guards() {
        // Given
        let records = vec![
            "[1518-11-01 00:00] Guard #10 begins shift",
            "[1518-11-01 00:05] falls asleep",
            "[1518-11-01 00:25] wakes up",
            "[1518-11-01 00:30] falls asleep",
            "[1518-11-01 00:55] wakes up",
            "[1518-11-01 23:58] Guard #99 begins shift",
            "[1518-11-02 00:40] falls asleep",
            "[1518-11-02 00:50] wakes up",
        ];

        parse_into_entries(records); 
        assert!(false);
    }
}
