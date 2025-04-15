use super::{calendars::{self}, Date};

#[derive(Clone, Copy, Debug)]
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

impl std::fmt::Display for Calendar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Calendar::UnitedKingdom => write!(f, "United Kingdom"),
            Calendar::Basic => write!(f, "Basic"),
        }
    }
}

pub trait CalendarInterface {
    fn name(&self) -> &'static str;
    fn get_holiday(&self, date: &Date) -> Option<String>;
}

