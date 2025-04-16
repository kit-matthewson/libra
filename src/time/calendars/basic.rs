use chrono::{Datelike, Month, NaiveDate, Weekday};

use crate::time::calendar::CalendarInterface;

/// Basic calendar implementation for demonstration purposes.
/// Holidays are weekends, New Year's Day, and Christmas Day.
pub struct BasicCalendar;

impl CalendarInterface for BasicCalendar {
    fn name(&self) -> &'static str {
        "Basic Calendar"
    }

    fn get_holiday(&self, date: &NaiveDate) -> Option<String> {
        if date.weekday() == Weekday::Sat || date.weekday() == Weekday::Sun {
            return Some("Weekend".to_string());
        }

        if date.day() == 1 && date.month() == Month::January as u32 {
            return Some("New Year's Day".to_string());
        }

        if date.day() == 25 && date.month() == Month::December as u32 {
            return Some("Christmas Day".to_string());
        }

        None
    }
}
