mod error;
use std::result;
pub type TAResult<T> = result::Result<T, error::TAError>;
mod linux;
mod user;
