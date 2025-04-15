use std::fmt;

use super::different_variant::DifferentVariant;

/// An error indicating that a passed argument was outside the valid range.
#[derive(Debug, Clone)]
pub struct ArgumentRange {
    /// Name of the argument
    pub(crate) name: &'static str,
    /// The minimum allowed value (inclusive)
    pub(crate) min: i64,
    /// The maximum allowed value (inclusive)
    pub(crate) max: i64,
    /// The value that was provided
    pub(crate) value: i64,
    /// Given if the valid range is conditional on other parameters
    pub(crate) conditional_message: Option<String>, // TODO consider making this a &'static str, which would require a Box::Leak in some scenarios
}

impl fmt::Display for ArgumentRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} must be in the range {}..={}, but {} was provided",
            self.name, self.min, self.max, self.value
        )?;

        if let Some(msg) = &self.conditional_message {
            write!(f, ": {}", msg)?;
        }

        Ok(())
    }
}

impl std::error::Error for ArgumentRange {}

impl From<ArgumentRange> for crate::error::Error {
    fn from(value: ArgumentRange) -> Self {
        Self::ArgumentRange(value)
    }
}

impl TryFrom<super::Error> for ArgumentRange {
    type Error = DifferentVariant;

    fn try_from(value: super::Error) -> Result<Self, Self::Error> {
        match value {
            super::Error::ArgumentRange(err) => Ok(err),
            _ => Err(DifferentVariant),
        }
    }
}
