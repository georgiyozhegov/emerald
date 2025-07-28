use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LexerError {
    UnknownCharacter(char),
}

impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::UnknownCharacter(c) => write!(f, "found an unknown character: \x1b[3m{c:?}\x1b[m"),
        }
    }
}
impl std::error::Error for LexerError {}
