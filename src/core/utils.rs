use std::str::FromStr;
use chrono::{Duration, Utc};

pub fn parse_relative_time(input: &str) -> Option<chrono::DateTime<Utc>> {
    let mut parts = input.split_whitespace();
    if let (
        Some(amount_str),
        Some(unit_str),
        Some(direction_str)
    ) = (parts.next(), parts.next(), parts.next()) {
        if let Ok(amount) = i64::from_str(amount_str) {
            let duration = match unit_str {
                "day" | "days" => Duration::days(amount),
                "hour" | "hours" => Duration::hours(amount),
                "minute" | "minutes" => Duration::minutes(amount),
                _ => return None,
            };
            return match direction_str {
                "ago" => Some(Utc::now() - duration),
                "later" => {
                    println!("{:?}   {:?}", duration, direction_str);
                    Some(Utc::now() + duration)
                },
                _ => None,
            };
        }
    }
    None
}