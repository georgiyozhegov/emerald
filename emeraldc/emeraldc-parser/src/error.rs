use emeraldc_lexer::{LexerError, WideTokenKind};
use serde::{Deserialize, Serialize};

/// Error that breaks parser.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FatalParserError {
    InvalidDeclarationIntroducer,
    CompilerBug(&'static str),
    UnexpectedEof,
}

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
