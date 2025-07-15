use emeraldc_lexer::{LexerError, WideToken, WideTokenKind};

use crate::tree::ParsedNode;

/// Error that breaks parser.
#[derive(Debug, Clone)]
pub enum ParserError {
    UnexpectedEof,
    Lexer(LexerError),
    InvalidDeclaration(WideToken),
    UnexpectedToken(WideToken),
}

/// Error that found in a node and does not affect the whole parsing process.
#[derive(Debug, Clone)]
pub enum NodeError {
    UnexpectedToken(WideTokenKind),
}

pub type NodeResult<T> = Result<ParsedNode<T>, ParserError>;
