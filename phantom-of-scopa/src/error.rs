use std::error::Error;
use std::fmt;

pub type Result<T> = std::result::Result<T, ScopaError>;

#[derive(Debug)]
pub enum ScopaError {
    Gameplay(String),
    Io(std::io::Error),
    TomlDeserialize(toml::de::Error),
    TomlSer(toml::ser::Error),
}

impl std::fmt::Display for ScopaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ScopaError::*;
        match self {
            Gameplay(msg) => write!(f, "{}", msg),
            Io(e) => e.fmt(f),
            TomlDeserialize(e) => e.fmt(f),
            TomlSer(e) => e.fmt(f),
        }
    }
}

impl Error for ScopaError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ScopaError::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for ScopaError {
    fn from(value: std::io::Error) -> Self {
        ScopaError::Io(value)
    }
}

impl From<toml::de::Error> for ScopaError {
    fn from(value: toml::de::Error) -> Self {
        ScopaError::TomlDeserialize(value)
    }
}

impl From<toml::ser::Error> for ScopaError {
    fn from(value: toml::ser::Error) -> Self {
        ScopaError::TomlSer(value)
    }
}
