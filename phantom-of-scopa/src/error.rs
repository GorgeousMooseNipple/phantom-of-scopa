use std::error::Error;
use std::fmt;

pub type Result<T> = std::result::Result<T, BaseError>;

#[derive(Debug)]
pub enum BaseError {
    Gameplay(String),
    Io(std::io::Error),
    Toml(toml::de::Error),
}

impl std::fmt::Display for BaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use BaseError::*;
        match self {
            Gameplay(msg) => write!(f, "Gameplay error: {}", msg),
            Io(e) => e.fmt(f),
            Toml(e) => e.fmt(f),
        }
    }
}

impl Error for BaseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            BaseError::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for BaseError {
    fn from(value: std::io::Error) -> Self {
        BaseError::Io(value)
    }
}

impl From<toml::de::Error> for BaseError {
    fn from(value: toml::de::Error) -> Self {
        BaseError::Toml(value)
    }
}
