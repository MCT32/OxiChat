use std::{
    //    error::Error,
    fmt::{self, Display},
};

#[derive(Debug)]
pub enum ArgumentParseError {
    InvalidArgumentCount,
    ParseIntError(std::num::ParseIntError),
    IoError(std::io::Error),
}

impl fmt::Display for ArgumentParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArgumentParseError::InvalidArgumentCount => {
                write!(f, "Invalid number of arguments provided")
            }
            ArgumentParseError::ParseIntError(e) => write!(f, "ParseIntError: {}", e),
            ArgumentParseError::IoError(e) => write!(f, "IoError: {}", e),
        }
    }
}

impl std::error::Error for ArgumentParseError {}

impl From<std::num::ParseIntError> for ArgumentParseError {
    fn from(err: std::num::ParseIntError) -> Self {
        ArgumentParseError::ParseIntError(err)
    }
}

impl From<std::io::Error> for ArgumentParseError {
    fn from(err: std::io::Error) -> Self {
        ArgumentParseError::IoError(err)
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum ClientError {
    ArgummentError,
}

impl Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClientError::ArgummentError => write!(f, "Error! Client arguments are invalid!"),
        }
    }
}
