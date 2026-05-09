#[derive(Debug)]
pub enum ScopaError {
    Player(String),
    Logic(String),
    Card(String),
    OutOfTurn,
    PuttingOnFullTable,
}

impl std::fmt::Display for ScopaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ScopaError::*;
        match self {
            Player(msg) => write!(f, "Player error: {}", msg),
            Logic(msg) => write!(f, "Logic error: {}", msg),
            Card(msg) => write!(f, "Card error: {}", msg),
            OutOfTurn => write!(f, "Player made a move out of turn"),
            PuttingOnFullTable => write!(f, "Trying to put a card on a full table"),
        }
    }
}

impl std::error::Error for ScopaError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

pub type Result<T> = std::result::Result<T, ScopaError>;
