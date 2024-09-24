use std::fmt::Debug;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MetalError {
    #[error("Couldn't find a system default device for Metal")]
    DeviceNotFound(),
    #[error("Couldn't create a new Metal Library: {0}")]
    LibraryError(String),
    #[error("Couldn't create a new Metal Function Object: {0}")]
    FunctionError(String),
    #[error("Couldn't create a new Metal Pipeline State: {0}")]
    PipelineError(String),
    #[error("Couldn't calculate {1} root of unity")]
    RootOfUnityError(String, u64),
    #[error("Input Length is {0}, which is not a power of two")]
    InputLengthError(usize),
}
