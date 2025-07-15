use emeraldc_lexer::{LexerError, WideTokenKind};

use crate::tree::ParsedNode;

/// Error that breaks parser.
#[derive(Debug, Clone)]
pub enum FatalParserError {
    InvalidDeclarationIntroducer,
    CompilerBug(&'static str),
    UnexpectedEof,
    UnexpectedToken, // deprecate
    Lexer(LexerError),
}

/// Error that found in a node and does not affect the whole parsing process.
#[derive(Debug, Clone)]
pub enum NodeError {
    UnexpectedToken(WideTokenKind),
    InvalidStatementIntroducer(WideTokenKind),
    InvalidExpressionIntroducer,
}

pub type NodeResult<T> = Result<ParsedNode<T>, FatalParserError>;
