use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("process or object is not OD compatible")]
    ODIncompatible,
}
