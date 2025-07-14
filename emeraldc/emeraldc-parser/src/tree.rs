use emeraldc_lexer::Span;

use crate::error::ParserError;

#[derive(Debug, Clone)]
pub enum Declaration {
    Function {
        identifier: Result<Identifier, ParserError>,
        body: Vec<Result<Statement, ParserError>>,
    },
}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub name: Span,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Let {
        identifier: Result<Identifier, ParserError>,
        value: Result<Expression, ParserError>,
    },
}

#[derive(Debug, Clone)]
pub enum Expression {
    Integer(Span),
}
