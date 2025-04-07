use time::Date;

use crate::time::{calendar::Calendar, period::Period, schedule::Schedule};

pub struct FixedRateBond<C: Calendar> {
    calendar: C,
    issue_date: Date,
    maturity_date: Date,
    schedule: Schedule<C>,
    coupon_rate: f64,
    face_amount: f64,
    frequency: Period,
}

impl FixedRateBond<calendars::UnitedKingdom> {
    pub fn new<C: Calendar>(
        issue_date: Date,
        maturity_date: Date,
        coupon_rate: f64,
        face_amount: f64,
    ) -> Self {
        let schedule = Schedule::new::<C>(issue_date, maturity_date, Period::Year(1));
        FixedRateBond {
            calendar: C::new(),
            issue_date,
            maturity_date,
            schedule,
            coupon_rate,
            face_amount,
            frequency: Period::Year(1),
        }
    }
}
