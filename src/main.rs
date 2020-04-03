extern crate chrono;

use std::fs::File;
use std::io::Read;
use std::ops::Add;

use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime, NaiveTime};

fn main() {
    let mut args = std::env::args();
    assert!(args.next().is_some());

    if let Some(file) = args.next() {
        let mut text = String::new();
        let _ = File::open(file).unwrap().read_to_string(&mut text).unwrap();

        let lines: Vec<&str> = text.split('\n').collect();
        let mut months: Vec<String> = Vec::new();

        let mut start = NaiveDateTime::new(NaiveDate::from_ymd(2000, 01, 01), NaiveTime::from_hms(0, 0, 0));

        let mut day = (start.date(), Duration::zero());
        let mut month = (start.date(), Duration::zero());
        let mut total_duration = Duration::zero();

        let mut first = true;

        for line in lines {
            if line.trim().is_empty() {
                continue;
            }
            let date_string: Vec<&str> = line.split(" ").collect();

            let date = NaiveDateTime::new(
                NaiveDate::parse_from_str(date_string[1], "%Y-%m-%d").unwrap(),
                NaiveTime::parse_from_str(&date_string[2][..date_string[2].find('+').unwrap()], "%H:%M:%S").unwrap(),
            );

            if line.starts_with("Enter") {
                start = date;

                if month.0.month() != date.month() {
                    months.push(format!("   {} => {}", month.0.format("%m.%Y"), format_duration(&month.1)));
                    month = (date.date(), Duration::zero());
                }

                if day.0.day() != date.day() {
                    if !first {
                        println!("{} => {}", day.0.format("%d.%m.%Y"), format_duration(&day.1));
                    } else {
                        first = false;
                    }
                    day = (date.date(), Duration::zero());
                }
            } else {
                let duration = date.signed_duration_since(start);
                total_duration = total_duration.add(duration);
                month.1 = month.1 + duration;
                day.1 = day.1 + duration;
            }
        }
        println!("{} => {}", day.0.format("%d.%m.%Y"), format_duration(&day.1));

        println!("=========================");
        months.remove(0);
        for duration in &months {
            println!("{}", duration);
        }
        println!("   {} => {}", month.0.format("%m.%Y"), format_duration(&month.1));
        println!("=========================");
        println!("       Sum => {}", format_duration(&total_duration));
    } else {
        println!("No file!");
    }
}


fn format_duration(dur: &Duration) -> String {
    format!("{:02}h {:02}m {:02}s", dur.num_hours(), dur.num_minutes() % 60, dur.num_seconds() % 60)
}
