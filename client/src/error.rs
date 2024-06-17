use std::{
    error::Error,
    fmt::{write, Display},
};

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
