use failure;
use std;

pub type StdResult<T, E> = std::result::Result<T, E>;
pub type Result<T> = StdResult<T, failure::Error>;
