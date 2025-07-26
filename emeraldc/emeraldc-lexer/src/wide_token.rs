use crate::LexerError;
use serde::{Deserialize, Serialize};
use emeraldc_span::Span;

/// Полный токен.
///
/// Содержит полный тип токена и его местонахождение.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WideToken {
    pub kind: WideTokenKind,
    pub span: Span,
}

impl WideToken {
    pub fn new(kind: WideTokenKind, span: Span) -> Self {
        Self { kind, span }
    }
}

/// Тип полного токена.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WideTokenKind {
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
    /// Содержит случившуюся ошибку.
    HadError(LexerError),
}

impl WideTokenKind {
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
