use crate::cashflows::{CashFlow, Coupon};
use time::Date;

use crate::time::{Calendar, DateAdjustment, DayCountConvention};

pub struct Bond {
    calendar: Calendar,
    day_count: DayCountConvention,
    date_adjustment: DateAdjustment,
    issue_date: Date,
    maturity_date: Date,
    face_value: f64,
    coupons: Vec<CashFlow>,
}

impl Bond {
    pub fn new(
        calendar: Calendar,
        day_count: DayCountConvention,
        date_adjustment: DateAdjustment,
        issue_date: Date,
        maturity_date: Date,
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

