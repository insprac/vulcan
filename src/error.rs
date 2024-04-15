use std::fmt;

#[derive(Debug)]
pub enum Error {
    Serde(serde_json::Error),
    Provider(String),
    Tool(String),
    String(String),
}

impl From<String> for Error {
    fn from(s: String) -> Error {
        Error::String(s)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Error {
        Error::Serde(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Serde(e) => write!(f, "Serde error: {}", e),
            Error::Provider(e) => write!(f, "Provider error: {}", e),
            Error::Tool(e) => write!(f, "Tool error: {}", e),
            Error::String(e) => write!(f, "String error: {}", e),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
