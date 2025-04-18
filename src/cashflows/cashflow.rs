use chrono::NaiveDate;

use crate::{error, time::DayCountConvention};

use super::InterestType;

/// A simple cash flow.
#[derive(Clone, Copy, Debug)]
pub struct CashFlow {
    /// The value of this cash flow.
    value: f64,
    /// The date at which this cash flow occurs.
    date: NaiveDate,
}

impl CashFlow {
    pub fn new(value: f64, date: NaiveDate) -> Self {
        CashFlow { value, date }
    }

    pub fn date(&self) -> NaiveDate {
        self.date
    }

    /// The present value of this cash flow using simple interest: PV = FV / (1 + rt)
    ///
    /// # Arguments
    ///
    /// * `today` - Today's date to discount back to.
    /// * `rate` - Annualized interest rate.
    /// * `convention` - The day count convention to use.
    ///
    /// # Returns
    ///
    /// The simple present value of this cash flow.
    pub fn simple_present_value(
        &self,
        today: &NaiveDate,
        rate: f64,
        convention: DayCountConvention,
    ) -> Result<f64, error::InvalidDate> {
        let year_frac = convention.year_frac(today, &self.date)?;
        let pv = self.value / (1.0 + (rate * year_frac));

        Ok(pv)
    }

    /// The present value of this cash flow using compound interest: PV = FV / (1 + r)^t
    ///
    /// # Arguments
    ///
    /// * `today` - Today's date to discount back to.
    /// * `rate` - Annualized interest rate.
    /// * `convention` - The day count convention to use.
    ///
    /// # Returns
    ///
    /// The compound present value of this cash flow.
    pub fn compound_present_value(
        &self,
        today: &NaiveDate,
        rate: f64,
        convention: DayCountConvention,
    ) -> Result<f64, error::InvalidDate> {
        let year_frac = convention.year_frac(today, &self.date)?;
        let pv = self.value / (1.0 + rate).powf(year_frac);

        Ok(pv)
    }

    /// The present value of this cash flow using continuous compounding interest: PV = FV * e^(-rt)
    ///
    /// # Arguments
    ///
    /// * `today` - Today's date to discount back to.
    /// * `rate` - Annualized interest rate.
    /// * `convention` - The day count convention to use.
    ///
    /// # Returns
    ///
    /// The continuously compounded present value of this cash flow.
    pub fn continuous_present_value(
        &self,
        today: &NaiveDate,
        rate: f64,
        convention: DayCountConvention,
    ) -> Result<f64, error::InvalidDate> {
        let year_frac = convention.year_frac(today, &self.date)?;
        let pv = self.value * (-rate * year_frac).exp();

        Ok(pv)
    }

    /// Calculates the present value of this cash flow based on the specified interest rate type.
    ///
    /// # Arguments
    ///
    /// * `today` - Today's date to discount back to.
    /// * `rate` - Annualized interest rate.
    /// * `convention` - The day count convention to use.
    /// * `interest_type` - The type of interest rate to use for the calculation.
    ///
    /// # Returns
    ///
    /// The present value of this cash flow.
    pub fn present_value(
        &self,
        today: &NaiveDate,
        rate: f64,
        convention: DayCountConvention,
        interest_type: InterestType,
    ) -> Result<f64, error::InvalidDate> {
        match interest_type {
            InterestType::Simple => self.simple_present_value(today, rate, convention),
            InterestType::Compound => self.compound_present_value(today, rate, convention),
            InterestType::Continuous => self.continuous_present_value(today, rate, convention),
        }
    }
}

impl std::fmt::Display for CashFlow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cash Flow: {} on {}", self.value, self.date)
    }
}
