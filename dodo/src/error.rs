#[derive(Debug)]
pub enum Error {
    Json(String),
    Io(String),
    Parse(String),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Json(msg) => write!(f, "Unable to parse json file: {msg}"),
            Self::Io(msg) => write!(f, "Unable to process cedict: {msg}"),
            Self::Parse(msg) => write!(f, "Unable to parse cedict file {msg}"),
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Json(err.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err.to_string())
    }
}
