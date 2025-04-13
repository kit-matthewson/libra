use time::Date;

use crate::time::Period;

use super::cashflow::CashFlow;


pub enum Coupon {
    Fixed(f64, Period),
    Zero,
    Vec(Vec<f64>),
}

impl Coupon {
    pub fn cash_flows(&self, start_date: Date) -> Vec<CashFlow> {
        todo!()
    }
}
