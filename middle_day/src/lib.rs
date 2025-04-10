use chrono::{Datelike, NaiveDate,Duration};
use chrono::{ Weekday as wd};

pub fn middle_day(year: i32) -> Option<wd> {
    let is_leap_year = NaiveDate::from_ymd_opt(year, 2, 29).is_some();
    let total_days = if is_leap_year { 366 } else { 365 };
    if total_days % 2 == 0 {
        None
    } else {
        let middle_day_pos = total_days / 2 + 1;
     
        let jan_1 = NaiveDate::from_ymd_opt(year, 1, 1).unwrap();
        
        let middle_date = jan_1 + Duration::days((middle_day_pos - 1) as i64);
        
        Some(middle_date.weekday())
    }
}
