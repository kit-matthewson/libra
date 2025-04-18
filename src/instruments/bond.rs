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

    pub fn dirty_price(
        &self,
        yield_to_maturity: f64,
        today: NaiveDate,
    ) -> Result<f64, InvalidDate> {
        let values = self
            .cash_flows()
            .iter()
            .filter(|c| c.date() >= today)
            .map(|c| c.compound_present_value(&today, yield_to_maturity, self.day_count))
            .collect::<Result<Vec<f64>, InvalidDate>>();

        match values {
            Ok(values) => Ok(values.iter().sum::<f64>()),
            Err(err) => Err(err),
        }
    }

    pub fn clean_price(
        &self,
        yield_to_maturity: f64,
        today: NaiveDate,
    ) -> Result<f64, InvalidDate> {
        if today > self.maturity_date {
            return Err(InvalidDate);
        }

        let cash_flows = self.cash_flows();

        let prev = match cash_flows.iter().position(|c| c.date() > today) {
            Some(i) => {
                if i > 0 {
                    cash_flows.get(i - 1)
                } else {
                    None
                }
            }
            None => return Err(InvalidDate),
        };

        let prev = match prev {
            Some(p) => p,
            None => return self.dirty_price(yield_to_maturity, today),
        };

        let accrued = prev.value() * self.day_count.year_frac(&prev.date(), &today)?;

        Ok(self.dirty_price(yield_to_maturity, today)? - accrued)
    }

    pub fn cash_flows(&self) -> Vec<CashFlow> {
        let mut cash_flows = Vec::new();

        match self.coupons {
            None => {}
            Some(coupons) => {
                cash_flows.append(
                    &mut coupons
                        .cash_flows(self.issue_date, self.maturity_date, self.principle)
                        .unwrap(),
                );
            }
        };

        cash_flows.push(CashFlow::new(self.principle, self.maturity_date));

        cash_flows
    }
}

impl fmt::Display for Bond {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Bond:\n Calendar: {}, Day Count: {}, Date Adjustment: {}\n Issued: {}, Maturity: {}\n Face: {}\n Coupon: {:?}", self.calendar, self.day_count, self.date_adjustment, self.issue_date, self.maturity_date, self.face_value, self.coupons)
    }
}
