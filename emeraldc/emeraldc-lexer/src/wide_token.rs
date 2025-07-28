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

impl std::fmt::Display for WideToken {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Identifier => write!(f, "an identifier"),
            Self::FunctionKeyword=> write!(f, "\x1b[3m'function'\x1b[m keyword"),
            Self::EndKeyword=> write!(f, "\x1b[3m'end'\x1b[m keyword"),
            Self::LetKeyword=> write!(f, "\x1b[3m'let'\x1b[m keyword"),
            Self::Integer=> write!(f, "an integer"),
            Self::OpenRound=> write!(f, "\x1b[3m'('\x1b[m"),
            Self::CloseRound=> write!(f, "\x1b[3m')'\x1b[m"),
            Self::Equal=> write!(f, "\x1b[3m'='\x1b[m"),
            Self::Plus=> write!(f, "\x1b[3m'+'\x1b[m"),
            Self::Minus=> write!(f, "\x1b[3m'-'\x1b[m"),
            Self::Asterisk => write!(f, "\x1b[3m'*'\x1b[m"),
            Self::Slash => write!(f, "\x1b[3m'/'\x1b[m"),
            Self::Invisible => write!(f, "an invisible symbol"),
            Self::Comment => write!(f, "a comment"),
            _ => unreachable!(),
        }
    }
}
