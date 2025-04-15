use crate::time::{Date, DayCountConvention};

/// A simple cash flow.
#[derive(Clone, Copy, Debug)]
pub struct CashFlow {
    /// The value of this cash flow.
    value: f64,
    /// The date at which this cash flow occurs.
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

impl std::fmt::Display for CashFlow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cash Flow: {} on {}", self.value, self.date)
    }
}
