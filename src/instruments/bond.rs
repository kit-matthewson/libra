use std::fmt;

use chrono::NaiveDate;

use crate::cashflows::{CashFlow, Coupon};

use crate::time::{Calendar, DateAdjustment, DayCountConvention};

#[derive(Clone, Debug)]
pub struct Bond {
    calendar: Calendar,
    day_count: DayCountConvention,
    date_adjustment: DateAdjustment,
    issue_date: NaiveDate,
    maturity_date: NaiveDate,
    face_value: f64,
    coupons: Vec<CashFlow>,
}

impl Bond {
    pub fn new(
        calendar: Calendar,
        day_count: DayCountConvention,
        date_adjustment: DateAdjustment,
        issue_date: NaiveDate,
        maturity_date: NaiveDate,
        face_value: f64,
        coupons: Coupon,
    ) -> Self {
        let coupons = coupons.cash_flows(issue_date);

        Bond {
            calendar,
            day_count,
            date_adjustment,
            issue_date,
            maturity_date,
            face_value,
            coupons,
        }
    }

    pub fn present_value(&self) -> f64 {
        todo!()
    }
}

impl fmt::Display for Bond {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Bond:\n Calendar: {}, Day Count: {}, Date Adjustment: {}\n Issued: {}, Maturity: {}\n Face: {}\n Coupon: {:?}", self.calendar, self.day_count, self.date_adjustment, self.issue_date, self.maturity_date, self.face_value, self.coupons)
    }
}
