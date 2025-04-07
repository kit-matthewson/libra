use core::fmt;

/// A day count convention.
pub enum DayCountConvention {
    Actual360,
    Actual365Fixed,
    ActualActual,
    Thirty360,
}

impl fmt::Display for DayCountConvention {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DayCountConvention::Actual360 => write!(f, "Actual / 360"),
            DayCountConvention::Actual365Fixed => write!(f, "Actual /365F"),
            DayCountConvention::ActualActual => write!(f, "Actual / Actual"),
            DayCountConvention::Thirty360 => write!(f, "30 / 360"),
        }
    }
}
