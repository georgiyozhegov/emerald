use emeraldc_lexer::{LexerError, WideToken};
use serde::{Deserialize, Serialize};

/// Error that breaks parser.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FatalParserError {
    InvalidDeclarationIntroducer(WideToken),
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
    UnexpectedToken(WideToken),
    InvalidStatementIntroducer(WideToken),
    InvalidExpressionIntroducer(WideToken),
    Lexer(LexerError),
}

impl NodeError {
    pub fn is_had_error(token_kind: &WideToken) -> bool {
        matches!(token_kind, WideToken::HadError(_))
    }
}

impl std::fmt::Display for NodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::UnexpectedToken(token) => {
                write!(f, "unexpected token: {token}")
            }
            Self::InvalidStatementIntroducer(token) => {
                write!(f, "invalid statement introducer: {token}")
            }
            Self::InvalidExpressionIntroducer(token) => {
                write!(f, "invalid expression introducer: {token}")
            }
            Self::Lexer(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for NodeError {}
