use chrono::{Months, NaiveDate};

use crate::error;

use super::CashFlow;

#[derive(Debug, Clone, Copy)]
pub enum Coupons {
    Fixed(f64, Months),
}

impl Coupons {
    pub fn cash_flows(&self, issued: NaiveDate, maturity: NaiveDate, principle: f64) -> Result<Vec<CashFlow>, error::InvalidDate> {
        if issued > maturity {
            return Err(error::InvalidDate);
        }

        match self {
            Coupons::Fixed(rate, interval) => {
                let mut date = issued;
                let mut cash_flows = Vec::new();

                while date <= maturity {
                    cash_flows.push(CashFlow::new(*rate * principle, date));

                    date = match date.checked_add_months(*interval) {
                        Some(date) => date,
                        None => return Err(error::InvalidDate),
                    };
                }

                Ok(cash_flows)
            },
        }
    }
}
