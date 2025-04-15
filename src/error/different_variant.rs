use std::fmt;

/// An error indicating that a [`TryFrom`](core::convert::TryFrom) call failed because the original value was of a different variant.
#[derive(Debug, Clone, Copy)]
pub struct DifferentVariant;

impl fmt::Display for DifferentVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "value was of a different variant than required")
    }
}

impl std::error::Error for DifferentVariant {}

impl From<DifferentVariant> for super::Error {
    fn from(value: DifferentVariant) -> Self {
        Self::DifferentVariant(value)
    }
}

impl TryFrom<super::Error> for DifferentVariant {
    type Error = Self;

    fn try_from(value: super::Error) -> Result<Self, Self::Error> {
        match value {
            super::Error::DifferentVariant(err) => Ok(err),
            _ => Err(Self),
        }
    }
}
