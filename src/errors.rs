use std::error::Error;
use std::fmt::{Debug, Display, Error as FmtErr, Formatter};
use std::io::Error as IOError;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
pub enum ThreadPoolError {}

// TODO: this is a placeholder implementation
impl Display for ThreadPoolError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtErr> {
        write!(f, "ThreadPoolError")
    }
}

impl Error for ThreadPoolError {}

#[derive(Debug, PartialEq)]
pub enum ServerError {
    Internal,
    NotImplemented,
}

impl Error for ServerError {}

impl Display for ServerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtErr> {
        match self {
            Self::Internal => write!(f, "500 Internal Server Error"),
            Self::NotImplemented => write!(f, "501 Not Implemented"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ClientError {
    NotFound,
    BadRequest,
}

impl Error for ClientError {}

impl Display for ClientError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtErr> {
        match self {
            Self::NotFound => write!(f, "404 Not Found"),
            Self::BadRequest => write!(f, "400 Bad Request"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum AppError {
    Client(ClientError),
    Server(ServerError),
    IO(String),
    Parse(ParseIntError),
    ThreadPool(ThreadPoolError),
}

impl Error for AppError {}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtErr> {
        write!(f, "{:?}", self)
    }
}

impl From<ClientError> for AppError {
    fn from(error: ClientError) -> Self {
        Self::Client(error)
    }
}

impl From<ServerError> for AppError {
    fn from(error: ServerError) -> Self {
        Self::Server(error)
    }
}

impl From<IOError> for AppError {
    fn from(error: IOError) -> Self {
        Self::IO(error.to_string())
    }
}

impl From<ParseIntError> for AppError {
    fn from(error: ParseIntError) -> Self {
        Self::Parse(error)
    }
}

impl From<ThreadPoolError> for AppError {
    fn from(error: ThreadPoolError) -> Self {
        Self::ThreadPool(error)
    }
}
