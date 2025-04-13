use core::fmt;

use time::Date;

/// A day count convention.
pub enum DayCountConvention {
    Actual360,
    Actual365Fixed,
    ActualActual,
    Thirty360,
}

impl DayCountConvention {
    /// Returns the year fraction between `from` and `to` using `day_count_convention`.
    pub fn year_frac(&self, from: &Date, to: &Date) -> f64 {
        let year_diff = (to.year() - from.year()) as f64;

        let year_days = match self {
            DayCountConvention::Actual360 => 360.0,
            DayCountConvention::Actual365Fixed => 365.0,
            DayCountConvention::Thirty360 => 360.0,
            DayCountConvention::ActualActual => {
                let start_of_year =
                    Date::from_calendar_date(from.year(), time::Month::January, 1).unwrap();
                let end_of_year =
                    Date::from_calendar_date(from.year(), time::Month::December, 31).unwrap();
                (end_of_year - start_of_year).whole_days() as f64 + 1.0
            }
        };

        let days = (year_diff * year_days) - (from.ordinal() as f64) + (to.ordinal() as f64);

        days / year_days
    }
}

impl fmt::Display for DayCountConvention {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DayCountConvention::Actual360 => write!(f, "Actual / 360"),
            DayCountConvention::Actual365Fixed => write!(f, "Actual / 365F"),
            DayCountConvention::ActualActual => write!(f, "Actual / Actual"),
            DayCountConvention::Thirty360 => write!(f, "30 / 360"),
        }
    }
}

pub enum DateAdjustment {
    Following,
    Preceding,
    ModifiedFollowing,
    ModifiedPreceding,
}
