use time::Date;

use crate::time::{Calendar, DayCountConvention};

pub struct CashFlow {
    value: f64,
    date: Date,
}

impl CashFlow {
    pub fn new(value: f64, date: Date) -> Self {
        CashFlow { value, date}
    }

    /// The present value of this cash flow using simple interest: PV = FV / (1 + rt)
    pub fn simple_present_value(&self, today: &Date, rate: f64, convention: DayCountConvention) -> f64 {
        let year_frac = convention.year_frac(today, &self.date);
        let pv = self.value / (1.0 + (rate * year_frac));

        pv
    }
}
