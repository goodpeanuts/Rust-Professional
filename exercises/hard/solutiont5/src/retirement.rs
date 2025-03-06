#![allow(dead_code)]

fn str_to_num(s: &str) -> u32 {
    let tmp = s.as_bytes();
    let mut number = 0;
    for num in tmp.iter() {
        number = number * 10 + (*num - b'0') as u32
    }
    number
}

fn calculate_retire_time(
    year: u32,
    month: u32,
    base_years: u32,
    retired_years: u32,
    max_delay_months: u32,
) -> String {
    if year < base_years {
        return format!(
            "{}-{:02},{},{}",
            year + retired_years,
            month,
            retired_years,
            0
        );
    }

    let mut delay_months = ((year - base_years) * 12 + (month - 1)) / 4 + 1;
    if delay_months > max_delay_months {
        delay_months = max_delay_months;
    }

    let mut tmp = delay_months;
    let mut year = year + retired_years + tmp / 12;
    tmp %= 12;
    let mut month = month + tmp;
    if month > 12 {
        year += 1;
        month -= 12;
    }

    if delay_months % 12 == 0 {
        format!(
            "{}-{:02},{},{}",
            year,
            month,
            retired_years + delay_months / 12,
            delay_months
        )
    } else {
        format!(
            "{}-{:02},{:.2},{}",
            year,
            month,
            retired_years as f32 + delay_months as f32 / 12_f32,
            delay_months
        )
    }
}

pub fn retire_time(time: &str, tp: &str) -> String {
    let tmp: Vec<&str> = time.split('-').collect();
    let year = str_to_num(tmp[0]);
    let month = str_to_num(tmp[1]);

    match tp {
        "原法定退休年龄50周岁女职工" => {
            calculate_retire_time(year, month, 1975, 50, 60)
        }
        "原法定退休年龄55周岁女职工" => {
            calculate_retire_time(year, month, 1970, 55, 36)
        }
        "男职工" => calculate_retire_time(year, month, 1965, 60, 36),
        _ => panic!("retire_time error"),
    }
}
