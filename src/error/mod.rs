mod argument_range;
mod different_variant;

pub use argument_range::ArgumentRange;
pub use different_variant::DifferentVariant;

pub type Result<T> = core::result::Result<T, Error>;

/// A unified Error type for any errors returned by a method in the libra crate.
///
/// All error types should implement:
/// * [`Display`](core::fmt::Display)
/// * [`Error`](std::error::Error)
/// * [`From<T> for Error`](core::convert::From)
/// * [`TryFrom`](core::convert::TryFrom)
#[derive(Debug)]
pub enum Error {
    ArgumentRange(ArgumentRange),
    DifferentVariant(DifferentVariant),
}

