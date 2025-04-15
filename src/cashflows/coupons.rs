use crate::time::{Date, Period};

use super::cashflow::CashFlow;

pub enum Coupon {
    Fixed(f64, Period),
    Zero,
    Vec(Vec<CashFlow>),
}

impl Coupon {
    pub fn cash_flows(&self, start_date: Date) -> Vec<CashFlow> {
        match self {
            Coupon::Fixed(rate, period) => {
                // TODO

                vec![CashFlow::new(*rate, start_date.increment(period).unwrap())]
            }
            Coupon::Zero => Vec::new(),
            Coupon::Vec(v) => v.to_vec(),
        }
    }
}
