use std::io;

#[derive(Debug)]
pub enum Error {
    Io(String),
    Serialize(String),
    Process(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(msg) => write!(f, "Error while doing I/O operations: {msg}"),
            Error::Serialize(msg) => write!(f, "Unable to serialize items due to: {msg}"),
            Error::Process(msg) => write!(f, "Unable to process cedict file due to: {msg}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Serialize(err.to_string())
    }
}

impl From<csv::Error> for Error {
    fn from(err: csv::Error) -> Self {
        Error::Serialize(err.to_string())
    }
}
