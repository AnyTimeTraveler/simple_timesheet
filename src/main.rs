extern crate chrono;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::ops::Add;

use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime, NaiveTime};

fn main() {
    let mut args = std::env::args();
    assert!(args.next().is_some());

    if let Some(arg) = args.next() {
        parse_times(arg);
    } else {
        println!("No file!");
    }
}

fn parse_times(file: String) {
    let mut text = String::new();
    let _ = File::open(file).unwrap().read_to_string(&mut text).unwrap();

    let lines: Vec<&str> = text.split('\n').collect();
    let mut durations: HashMap<NaiveDate, Duration> = HashMap::new();
    let mut start = NaiveDateTime::new(NaiveDate::from_ymd(2000, 01, 01), NaiveTime::from_hms(0, 0, 0));
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let date_string: Vec<&str> = line.split(" ").collect();

        let date = NaiveDateTime::new(
            NaiveDate::parse_from_str(date_string[1], "%Y-%m-%d").unwrap(),
            NaiveTime::parse_from_str(&date_string[2][..date_string[2].find('+').unwrap()], "%H:%M:%S").unwrap()
        );

        if line.starts_with("Enter") {
            start = date;
        } else {
            let duration = date.signed_duration_since(start);

            if durations.contains_key(&start.date()) {
                *durations.get_mut(&start.date()).unwrap() = durations.get(&start.date()).unwrap().add(duration);
            } else {
                durations.insert(start.date(), duration);
            }
        }
    }

    for (date, duration) in &durations {
        println!("{} => {}", date.format("%d.%m.%Y"), format_duration(&duration));
    }
    println!("=========================");
    let mut sum = Duration::zero();
    let mut last_date = durations.keys().next().unwrap();
    for (date, dur) in &durations {
        if date.month() == last_date.month() {
            sum = sum.add(*dur);
        } else {
            println!("   {:02}.{} => {}", last_date.month(), last_date.year(), format_duration(&sum));
            sum = Duration::zero();
            last_date = &date;
        }
    }
    println!("   {:02}.{} => {}", last_date.month(), last_date.year(), format_duration(&sum));
    println!("=========================");
    let mut total = Duration::zero();
    for duration in durations.values() {
        total = total.add(*duration);
    }
    println!("       Sum => {}", format_duration(&total));
}

fn format_duration(dur: &Duration) -> String {
    format!("{:02}h {:02}m {:02}s", dur.num_hours(), dur.num_minutes() % 60, dur.num_seconds() % 60)
}
