use chrono::NaiveDate;

use crate::error::LibraError;

pub trait ForwardCurve {
    /// Returns the constant forward rate for the period between two dates.
    ///
    /// # Arguments
    ///
    /// * `from` - The start date of the forward period.
    /// * `to` - The end date of the forward period.
    ///
    /// # Returns
    ///
    /// The forward rate.
    fn forward_rate(&self, from: NaiveDate, to: NaiveDate) -> Result<f64, LibraError>;
}

/// A forward yield term structure with a fixed rate.
pub struct FlatForward {
    rate: f64,
}

impl FlatForward {
    pub fn new(rate: f64) -> Self {
        Self { rate }
    }

    pub fn rate(&self) -> f64 {
        self.rate
    }
}

impl ForwardCurve for FlatForward {
    fn forward_rate(&self, _from: NaiveDate, _to: NaiveDate) -> Result<f64, LibraError> {
        Ok(self.rate)
    }
}

