use thiserror::Error;

pub enum LibraError {
    ArgumentRange(ArgumentRange),
    ConditionalArgumentRange(ConditionalArgumentRange),
    InvalidDate(InvalidDate),
    DifferentVariant(DifferentVariant),
}

#[derive(Error, Debug)]
#[error("{name} must be in the range {min}=..{max}, but {value} was provided")]
pub struct ArgumentRange {
    name: String,
    min: i64,
    max: i64,
    value: i64,
}

#[derive(Error, Debug)]
#[error(
    "{name} must be in the range {min}=..{max}, but {value} was provided: {conditional_message}"
)]
pub struct ConditionalArgumentRange {
    name: String,
    min: i64,
    max: i64,
    value: i64,
    conditional_message: String,
}

#[derive(Error, Debug)]
#[error("an invalid date was provided")]
pub struct InvalidDate;

#[derive(Error, Debug)]
#[error("value was of a different variant than required")]
pub struct DifferentVariant;
