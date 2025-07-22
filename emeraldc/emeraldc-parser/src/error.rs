use emeraldc_lexer::{LexerError, WideTokenKind};
use serde::{Deserialize, Serialize};

/// Error that breaks parser.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FatalParserError {
    InvalidDeclarationIntroducer(WideTokenKind),
    CompilerBug(&'static str),
    UnexpectedEof,
}

impl std::fmt::Display for FatalParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::InvalidDeclarationIntroducer(token) => {
                write!(f, "invalid declaration introducer: {token:?}")
            }
            Self::CompilerBug(message) => {
                write!(f, "critical compiler bug: {message}")
            }
            Self::UnexpectedEof => write!(f, "unexpected eof"),
        }
    }
}

impl std::error::Error for FatalParserError {}

/// Error found in a node and does not affect the whole parsing process.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeError {
    UnexpectedToken(WideTokenKind),
    InvalidStatementIntroducer(WideTokenKind),
    InvalidExpressionIntroducer(WideTokenKind),
    Lexer(LexerError),
}

impl NodeError {
    pub fn is_had_error(token_kind: &WideTokenKind) -> bool {
        matches!(token_kind, WideTokenKind::HadError(_))
    }
}

impl std::fmt::Display for NodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::UnexpectedToken(token) => {
                write!(f, "unexpected token: {token:?}")
            }
            Self::InvalidStatementIntroducer(token) => {
                write!(f, "invalid statement introducer: {token:?}")
            }
            Self::InvalidExpressionIntroducer(token) => {
                write!(f, "invalid expression introducer: {token:?}")
            }
            Self::Lexer(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for NodeError {}
