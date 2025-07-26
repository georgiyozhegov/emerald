use emeraldc_span::Span;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LexerError {
    UnknownCharacter(Span),
}

impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::UnknownCharacter(_) => write!(f, "found an unknown character"),
        }
    }
}
impl std::error::Error for LexerError {}
