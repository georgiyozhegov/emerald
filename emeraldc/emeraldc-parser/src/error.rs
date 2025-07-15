use emeraldc_lexer::{LexerError, WideTokenKind};

/// Error that breaks parser.
#[derive(Debug, Clone)]
pub enum FatalParserError {
    InvalidDeclarationIntroducer,
    CompilerBug(&'static str),
    UnexpectedEof,
    UnexpectedToken, // deprecate
    Lexer(LexerError),
}

/// Error found in a node and does not affect the whole parsing process.
#[derive(Debug, Clone)]
pub enum NodeError {
    UnexpectedToken(WideTokenKind),
    InvalidStatementIntroducer(WideTokenKind),
    InvalidExpressionIntroducer,
}
