use emeraldc_lexer::{Span, WideTokenKind};

use crate::error::NodeError;

#[derive(Debug, Clone)]
pub enum Declaration {
    Function {
        _introducer: ParsedNode<WideTokenKind>,
        identifier: ParsedNode<Identifier>,
        _open_round: ParsedNode<WideTokenKind>,
        _close_round: ParsedNode<WideTokenKind>,
        body: Vec<ParsedNode<Statement>>,
        _end: ParsedNode<WideTokenKind>,
    },
}

#[derive(Debug, Clone)]
pub struct Identifier;

#[derive(Debug, Clone)]
pub enum Statement {
    Let {
        _introducer: ParsedNode<WideTokenKind>,
        identifier: ParsedNode<Identifier>,
        _equal: ParsedNode<WideTokenKind>,
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
