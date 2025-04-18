use core::error;
use std::fmt;

use chrono::NaiveDate;

use crate::cashflows::{CashFlow, Coupons};

use crate::error::InvalidDate;
use crate::time::{Calendar, DateAdjustment, DayCountConvention};

#[derive(Clone, Debug)]
pub struct Bond {
    calendar: Calendar,
    day_count: DayCountConvention,
    date_adjustment: DateAdjustment,
    issue_date: NaiveDate,
    maturity_date: NaiveDate,
    face_value: f64,
    principle: f64,
    coupons: Option<Coupons>,
}

impl Bond {
    pub fn new(
        calendar: Calendar,
        day_count: DayCountConvention,
        date_adjustment: DateAdjustment,
        issue_date: NaiveDate,
        maturity_date: NaiveDate,
        face_value: f64,
        principle: f64,
        coupons: Option<Coupons>,
    ) -> Self {
        Bond {
            calendar,
            day_count,
            date_adjustment,
            issue_date,
            maturity_date,
            face_value,
            principle,
            coupons,
        }
    }

    pub fn present_value(
        &self,
        yield_to_maturity: f64,
        today: NaiveDate,
    ) -> Result<f64, InvalidDate> {
        match self.coupons {
            None => {
                let cashflow = CashFlow::new(self.principle, self.maturity_date);
                cashflow.compound_present_value(&today, yield_to_maturity, self.day_count)
            }

            Some(coupons) => {
                let mut present_value = CashFlow::new(self.principle, self.maturity_date)
                    .compound_present_value(&today, yield_to_maturity, self.day_count)?;

                // Try and sum all the present values of future coupons
                let coupon_values: Result<Vec<f64>, InvalidDate> = coupons
                    .cash_flows(self.issue_date, self.maturity_date, self.principle)?
                    .iter()
                    .filter(|c| c.date() >= today)
                    .map(|c| c.compound_present_value(&today, yield_to_maturity, self.day_count))
                    .collect();

                match coupon_values {
                    Ok(values) => present_value += values.iter().sum::<f64>(),
                    Err(err) => return Err(err),
                }

                Ok(present_value)
            }
        }
    }
}

impl fmt::Display for Bond {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Bond:\n Calendar: {}, Day Count: {}, Date Adjustment: {}\n Issued: {}, Maturity: {}\n Face: {}\n Coupon: {:?}", self.calendar, self.day_count, self.date_adjustment, self.issue_date, self.maturity_date, self.face_value, self.coupons)
    }
}
