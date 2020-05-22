use crate::error::Error;

pub mod provider;
pub mod blob;
pub mod error;

pub type Result<T> = std::result::Result<T, Error>;
