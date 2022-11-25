use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    NoColon,
}   

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"error")
    }
}

impl From<std::io::Error> for Error {
    fn from(x: std::io::Error) -> Self {
        Error::IoError(x)
    }
}

impl std::error::Error for Error {

}
