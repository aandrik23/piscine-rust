use chrono::{Datelike, NaiveDate, Weekday};

pub fn middle_day(year: u32) -> Option<Weekday> {
    let first_day = NaiveDate::from_ymd_opt(year as i32, 1, 1)?;
    let last_day = NaiveDate::from_ymd_opt(year as i32, 12, 31)?;

    let total_days = last_day.ordinal();

    if total_days % 2 == 0 {
        return None;
    }

    let middle = (total_days / 2) + 1;

    let middle_day = NaiveDate::from_yo_opt(year as i32, middle)?;

    Some(middle_day.weekday())
}