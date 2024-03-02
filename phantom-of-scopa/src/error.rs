use std::error::Error;
use std::fmt;

pub type Result<T> = std::result::Result<T, BaseError>;

#[derive(Debug)]
pub enum BaseError {
    GameplayError(String),
}

impl std::fmt::Display for BaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use BaseError::*;
        match self {
            GameplayError(msg) => write!(f, "Gameplay error: {}", msg),
        }
    }
}

impl Error for BaseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            _ => None,
        }
    }
}

impl From<BaseError> for String {
    fn from(value: BaseError) -> Self {
        use BaseError::*;
        match value {
            GameplayError(msg) => msg,
        }
    }
}
