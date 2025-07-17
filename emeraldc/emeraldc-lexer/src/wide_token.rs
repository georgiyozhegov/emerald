use crate::LexerError;
use serde::{Serialize, Deserialize};

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

/// Отрезок, который обозначает местонахождение токена.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn join(self, right: Self) -> Self {
        assert!(self.end < right.start);
        Self::new(self.start, right.end)
    }
}
