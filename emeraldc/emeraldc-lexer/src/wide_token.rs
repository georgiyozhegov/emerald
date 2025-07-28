use crate::LexerError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WideToken {
    Identifier,
    FunctionKeyword,
    EndKeyword,
    LetKeyword,
    Integer,
    OpenRound,
    CloseRound,
    Equal,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Invisible,
    Comment,
    /// Содержит случившуюся ошибку.
    HadError(LexerError),
}

impl WideToken {
    pub fn had_error(&self) -> bool {
        matches!(self, Self::HadError(_))
    }

    pub fn as_error(self) -> LexerError {
        match self {
            Self::HadError(error) => error,
            _ => panic!(),
        }
    }
}
