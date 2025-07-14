use emeraldc_lexer::{LexerError, WideToken};

#[derive(Debug, Clone)]
pub enum ParserError {
    UnexpectedEof,
    Lexer(LexerError),
    InvalidDeclaration(WideToken),
    UnexpectedToken(WideToken),
}
