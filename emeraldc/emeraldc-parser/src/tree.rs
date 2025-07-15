use emeraldc_lexer::Span;

use crate::error::NodeError;

#[derive(Debug, Clone)]
pub enum Declaration {
    Function {
        identifier: ParsedNode<Identifier>,
        body: Vec<ParsedNode<Statement>>,
    },
}

#[derive(Debug, Clone)]
pub struct Identifier;

#[derive(Debug, Clone)]
pub enum Statement {
    Let {
        identifier: ParsedNode<Identifier>,
        value: ParsedNode<Expression>,
    },
}

#[derive(Debug, Clone)]
pub enum Expression {
    Integer,
    Variable(Identifier),
}

#[derive(Debug, Clone)]
pub struct ParsedNode<T> {
    pub node: Result<T, NodeError>,
    pub span: Span,
}

impl<T> ParsedNode<T> {
    pub fn new(node: Result<T, NodeError>, span: Span) -> Self {
        Self { node, span }
    }
}
