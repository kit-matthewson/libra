use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{name} must be in the range {min}=..{max}, but {value} was provided")]
    ArgumentRange {
        name: String,
        min: i64,
        max: i64,
        value: i64,
    },

    #[error("{name} must be in the range {min}=..{max}, but {value} was provided: {conditional_message}")]
    ConditionalArgumentRange {
        name: String,
        min: i64,
        max: i64,
        value: i64,
        conditional_message: String,
    },

    #[error("value was of a different variant than required")]
    DifferentVariant,
}

