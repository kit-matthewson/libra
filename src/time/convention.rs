use crate::error;

use chrono::Datelike;
use chrono::NaiveDate;
use std::fmt;

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
    pub fn year_frac(&self, from: &NaiveDate, to: &NaiveDate) -> Result<f64, error::InvalidDate> {
        if from > to {
            return Err(error::InvalidDate);
        }

        match self {
            DayCountConvention::Actual360 => {
                let days = (*to - *from).num_days();
                Ok(days as f64 / 360.0)
            }
            DayCountConvention::Actual365Fixed => {
                let days = (*to - *from).num_days();
                Ok(days as f64 / 365.0)
            }
            DayCountConvention::ActualActual => {
                todo!()
            }
            DayCountConvention::Thirty360 => {
                // 30/360 US algorithm from ISDA 2006 Section 4.16(f)

                let mut d1 = from.day();
                let m1 = from.month();
                let y1 = from.year();

                let mut d2 = to.day();
                let m2 = to.month();
                let y2 = to.year();

                if d2 == 31 && (d1 == 30 || d1 == 31) {
                    d2 = 30;
                }

                if d1 == 31 {
                    d1 = 30;
                }

                let total_days_360 = ((y2 - y1) * 360
                    + (m2 as i32 - m1 as i32) * 30
                    + (d2 as i32 - d1 as i32)) as f64;

                Ok(total_days_360 / 360.0)
            }
        }
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
