//! This module implements particular type of error needed for the application
use std::fmt::Display;

///Define Error structure with the two needed types of errors
#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    NoColon,
    ReqwestError(reqwest::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error")
    }
}

///Implements from trait for io type errors
impl From<std::io::Error> for Error {
    fn from(x: std::io::Error) -> Self {
        Error::IoError(x)
    }
}
///Implements from trait for reqwest type errors
impl From<reqwest::Error> for Error {
    fn from(x: reqwest::Error) -> Self {
        Error::ReqwestError(x)
    }
}

impl std::error::Error for Error {}
