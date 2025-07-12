use crate::LexerError;

/// Полный токен.
///
/// Содержит полный тип токена и его местонахождение.
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WideTokenKind {
    Identifier,
    FunctionKeyword,
    EndKeyword,
    LetKeyword,
    Integer,
    OpenRound,
    CloseRound,
    Equal,
    Invisible,
    /// Содержит случившуюся ошибку.
    HadError(LexerError),
}

/// Отрезок, который обозначает местонахождение токена.
#[derive(Debug, Clone)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}
