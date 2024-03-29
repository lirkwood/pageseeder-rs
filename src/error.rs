use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum PSError {
    CommunicationError { msg: String },
    ParseError { msg: String },
    ServerError { msg: String },
    TokenError { msg: String },
}

pub type PSResult<T> = Result<T, PSError>;

impl Error for PSError {}
impl Display for PSError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CommunicationError { msg } => {
                write!(f, "Error communicating with server: {}", msg)
            }
            Self::ParseError { msg } => {
                write!(f, "Error parsing server response: {}", msg)
            }
            Self::ServerError { msg } => {
                write!(f, "Operation failed on the server: {}", msg)
            }
            Self::TokenError { msg } => {
                write!(f, "Error using token: {}", msg)
            }
        }
    }
}
