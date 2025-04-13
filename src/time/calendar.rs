use time::Date;

use super::{
    calendars::{self},
    DayCountConvention,
};

pub enum Calendar {
    UnitedKingdom,
    Basic,
}

impl Calendar {
    /// Gets the underlying `CalendarInterface` that this `Calendar` is pointing to.
    fn interface(&self) -> &dyn CalendarInterface {
        match self {
            Calendar::UnitedKingdom => &calendars::UnitedKingdom,
            Calendar::Basic => &calendars::BasicCalendar,
        }
    }

    /// The name of the calendar.
    pub fn name(&self) -> &'static str {
        self.interface().name()
    }

    /// Constructs a `Vec` of dates that are holidays between `from` and `to` (inclusive).
    pub fn construct_holiday_vec(&self, from: Date, to: Date) -> Vec<Date> {
        let mut holiday_dates = Vec::new();

        let mut date = from;
        while date <= to {
            if self.get_holiday(&date).is_some() {
                holiday_dates.push(date);
            }

            date = date.next_day().expect("could not increment day");
        }

        holiday_dates
    }

    /// Gets the holiday on `date`. Returns `Some(name)` if the day is a holiday, or `None` if it is not.
    pub fn get_holiday(&self, date: &Date) -> Option<String> {
        self.interface().get_holiday(date)
    }

    /// Returns `true` if `date` is a buisness day, or `false` otherwise.
    pub fn is_buisness_day(&self, date: &Date) -> bool {
        self.get_holiday(date).is_none()
    }
}

pub trait CalendarInterface {
    fn name(&self) -> &'static str;
    fn get_holiday(&self, date: &Date) -> Option<String>;
}

#[cfg(test)]
mod test_calendar {
    use time::macros::date;

    use crate::time::Calendar;

    #[test]
    fn test_is_buisness_day() {
        let calendar = Calendar::Basic;
        let date = date!(2023 - 12 - 25); // Christmas Day
        assert!(!calendar.is_buisness_day(&date));
    }
}
