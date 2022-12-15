mod error;
use std::result;
pub type TAResult<T> = result::Result<T, error::TAError>;
pub mod linux;
pub mod user;
