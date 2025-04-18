mod cashflow;
mod coupons;

pub use cashflow::*;
pub use coupons::*;

/// Represents a type of interest.
#[derive(Clone, Copy, Debug)]
pub enum InterestType {
    /// Simple interest
    Simple,
    /// Compound interest
    Compound,
    /// Continuously compounding interest
    Continuous,
}
