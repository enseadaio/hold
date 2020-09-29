use crate::error::Error;

pub mod blob;
pub mod error;
pub mod provider;

pub type Result<T> = std::result::Result<T, Error>;
