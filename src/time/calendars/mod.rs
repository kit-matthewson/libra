use super::calendar::Calendar;

mod united_kingdom;

pub use united_kingdom::UnitedKingdom;

/// Basic calendar implementation for demonstration purposes.
/// Holidays are weekends, New Year's Day, and Christmas Day.
pub struct BasicCalendar;

impl Calendar for BasicCalendar {
    fn new() -> Self {
        BasicCalendar
    }

    fn name(&self) -> &'static str {
        "Basic Calendar"
    }

    fn get_holiday(&self, date: &time::Date) -> Option<String> {
        if date.weekday() == time::Weekday::Saturday || date.weekday() == time::Weekday::Sunday {
            return Some("Weekend".to_string());
        }

        if date.day() == 1 && date.month() == time::Month::January {
            return Some("New Year's Day".to_string());
        }

        if date.day() == 25 && date.month() == time::Month::December {
            return Some("Christmas Day".to_string());
        }

        None
    }
}
