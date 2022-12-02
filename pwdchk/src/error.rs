//! This module implements particular type of error needed for the application
use std::fmt::Display;

///Define Error structure with the two needed types of errors
#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    NoColon,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error")
    }
}

impl From<std::io::Error> for Error {
    fn from(x: std::io::Error) -> Self {
        Error::IoError(x)
    }
}

impl std::error::Error for Error {}
