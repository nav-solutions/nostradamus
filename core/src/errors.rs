use thiserror::Error;

#[cfg(feature = "sp3")]
use sp3::prelude::Error as SP3Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("process or object is not OD compatible")]
    ODIncompatible,

    #[error("incompatible time-axis definitions")]
    IncompatibleTimeAxis,

    #[error("undefined time-axis")]
    UndefinedTimeAxis,

    #[cfg(feature = "sp3")]
    #[error("sp3 model error: {0}")]
    SP3Error(#[from] SP3Error),
}
