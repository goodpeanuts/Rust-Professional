#![allow(dead_code)]
use chrono::{Datelike, NaiveDate};

pub fn time_info(time: &str) -> String {
    let date = NaiveDate::parse_from_str(time, "%Y-%m-%d").unwrap();

    let year = date.year();
    let month = date.month() as i32;
    let day = date.day() as i32;

    let week = date.iso_week().week();

    let weekday = date.weekday().number_from_monday();
    let weekday = if weekday == 7 { 7 } else { weekday };

    let day_of_year = date.ordinal();

    let days_in_year = if date.leap_year() { 366 } else { 365 };
    let days_remaining = days_in_year - day_of_year;

    let spring_festival = if year == 2025 {
        NaiveDate::from_ymd_opt(2025, 1, 29).unwrap()
    } else {
        NaiveDate::from_ymd_opt(2026, 2, 17).unwrap()
    };

    let days_to_spring_festival = if date <= spring_festival {
        (spring_festival - date).num_days()
    } else {
        let next_spring_festival = NaiveDate::from_ymd_opt(2026, 2, 17).unwrap();
        (next_spring_festival - date).num_days()
    };

    let days_to_next_a_share = days_to_next_a_share_opening(year, month, day);

    format!(
        "{},{},{},{},{},{}",
        week, weekday, day_of_year, days_remaining, days_to_spring_festival, days_to_next_a_share
    )
}

fn is_leap_year(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

fn day_of_year(year: i32, month: i32, day: i32) -> i32 {
    NaiveDate::from_ymd_opt(year, month as u32, day as u32)
        .unwrap()
        .ordinal() as i32
}

fn day_of_week(year: i32, month: i32, day: i32) -> i32 {
    let date = NaiveDate::from_ymd_opt(year, month as u32, day as u32).unwrap();
    let weekday = date.weekday().number_from_monday();
    if weekday == 7 {
        7
    } else {
        weekday as i32
    }
}

fn days_to_next_a_share_opening(year: i32, month: i32, day: i32) -> i32 {
    let new_year_day_open = day_of_year(year, 1, 2);
    let spring_year_day_open = day_of_year(year, 2, 5);
    let qingming_day_open = day_of_year(year, 4, 7);
    let labor_day_open = day_of_year(year, 5, 6);
    let zongzi_day_open = day_of_year(year, 6, 3);
    let autumn_day_open = day_of_year(year, 10, 9);
    let next_new_year_day_open = day_of_year(year + 1, 1, 1);

    if month == 1 && day == 1 {
        return new_year_day_open - day_of_year(year, month, day) - 1;
    }
    if (month == 1 && (28..=31).contains(&day)) || (month == 2 && (1..=4).contains(&day)) {
        return spring_year_day_open - day_of_year(year, month, day) - 1;
    }
    if month == 4 && (4..=6).contains(&day) {
        return qingming_day_open - day_of_year(year, month, day) - 1;
    }
    if month == 5 && (1..=5).contains(&day) {
        return labor_day_open - day_of_year(year, month, day) - 1;
    }
    if (month == 5 && day == 31) || (month == 6 && (1..=2).contains(&day)) {
        return zongzi_day_open - day_of_year(year, month, day) - 1;
    }

    if month == 10 && (1..=8).contains(&day) {
        return autumn_day_open - day_of_year(year, month, day) - 1;
    }

    if month == 12 && day == 31 {
        let days = if is_leap_year(year) { 366 } else { 365 };
        return next_new_year_day_open - day_of_year(year, month, day) + days;
    }

    let weekday = day_of_week(year, month, day);
    match weekday {
        7 => 0,
        6 => 1,
        5 => 2,
        _ => 0,
    }
}
