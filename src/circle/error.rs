use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum CircleError {
    ResponseStatusCodeError(reqwest::StatusCode),
}

impl Display for CircleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "CircleError({:?})", self)
    }
}

impl Error for CircleError {}
