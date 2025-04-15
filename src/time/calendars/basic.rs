use crate::time::{calendar::CalendarInterface, Date, Month};

/// Basic calendar implementation for demonstration purposes.
/// Holidays are weekends, New Year's Day, and Christmas Day.
pub struct BasicCalendar;

impl CalendarInterface for BasicCalendar {
    fn name(&self) -> &'static str {
        "Basic Calendar"
    }

    fn get_holiday(&self, date: &Date) -> Option<String> {
        if date.weekday().is_weekend() {
            return Some("Weekend".to_string());
        }

        if date.day() == 1 && date.month() == Month::January {
            return Some("New Year's Day".to_string());
        }

        if date.day() == 25 && date.month() == Month::December {
            return Some("Christmas Day".to_string());
        }

        None
    }
}
