use chrono::{Months, NaiveDate};

use super::cashflow::CashFlow;

pub enum Coupon {
    Fixed(f64, Months),
    Zero,
    Vec(Vec<CashFlow>),
}

impl Coupon {
    pub fn cash_flows(&self, _start_date: NaiveDate) -> Vec<CashFlow> {
        todo!()
    }
}
