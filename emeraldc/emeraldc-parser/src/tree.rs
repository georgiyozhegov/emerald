use emeraldc_lexer::WideTokenKind;
use emeraldc_span::Span;
use serde::{Deserialize, Serialize};

use crate::NodeError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Declaration {
    Function(Function),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function {
    pub _introducer: ParsedNode<WideTokenKind>,
    pub identifier: ParsedNode<Identifier>,
    pub _open_round: ParsedNode<WideTokenKind>,
    pub _close_round: ParsedNode<WideTokenKind>,
    pub body: Vec<ParsedNode<Statement>>,
    pub _end: ParsedNode<WideTokenKind>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identifier;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Statement {
    Let(Let),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Let {
    pub _introducer: ParsedNode<WideTokenKind>,
    pub identifier: ParsedNode<Identifier>,
    pub _equal: ParsedNode<WideTokenKind>,
    pub value: ParsedNode<Expression>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Expression {
    Integer,
    Variable(Identifier),
    Binary(Binary),
    Parenthesized(Parenthesized),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Binary {
    pub left: Box<ParsedNode<Expression>>,
    pub operator: ParsedNode<BinaryOperator>,
    pub right: Box<ParsedNode<Expression>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parenthesized {
    pub _open_round: ParsedNode<WideTokenKind>,
    pub inner: Box<ParsedNode<Expression>>,
    pub _close_round: ParsedNode<WideTokenKind>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl BinaryOperator {
    pub fn from_token(kind: &WideTokenKind) -> Option<Self> {
        match kind {
            WideTokenKind::Plus => Some(Self::Add),
            WideTokenKind::Minus => Some(Self::Subtract),
            WideTokenKind::Asterisk => Some(Self::Multiply),
            WideTokenKind::Slash => Some(Self::Divide),
            _ => None,
        }
    }

    pub fn precedence(&self) -> (u8, u8) {
        match self {
            Self::Add | Self::Subtract => (1, 2),
            Self::Multiply | Self::Divide => (3, 4),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedNode<T> {
    pub node: Result<T, NodeError>,
    pub span: Span,
}

impl<T> ParsedNode<T> {
    pub fn new(node: Result<T, NodeError>, span: Span) -> Self {
        Self { node, span }
    }
}
