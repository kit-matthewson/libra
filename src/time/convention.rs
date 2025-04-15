use crate::time::Month;
use std::fmt;

use super::Date;

/// A day count convention.
#[derive(Clone, Copy, Debug)]
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
                    Date::from_parts(1, Month::January, from.year()).unwrap();
                let end_of_year =
                    Date::from_parts(31, Month::December, from.year()).unwrap();
                (end_of_year - start_of_year).whole_days() as f64 + 1.0
            }
        };

        let days = (year_diff * year_days) - (from.day_of_year() as f64) + (to.day_of_year() as f64);

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

#[derive(Clone, Copy, Debug)]
pub enum DateAdjustment {
    Following,
    Preceding,
    ModifiedFollowing,
    ModifiedPreceding,
}

impl std::fmt::Display for DateAdjustment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DateAdjustment::Following => write!(f, "Following"),
            DateAdjustment::Preceding => write!(f, "Preceding"),
            DateAdjustment::ModifiedFollowing => write!(f, "Modified Following"),
            DateAdjustment::ModifiedPreceding => write!(f, "Modified Preceding"),
        }
    }
}
