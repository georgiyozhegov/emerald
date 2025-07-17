use emeraldc_lexer::{Span, WideTokenKind};
use serde::{Deserialize, Serialize};

use crate::NodeError;

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identifier;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Statement {
    Let {
        _introducer: ParsedNode<WideTokenKind>,
        identifier: ParsedNode<Identifier>,
        _equal: ParsedNode<WideTokenKind>,
        value: ParsedNode<Expression>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Expression {
    Integer,
    Variable(Identifier),
    Binary {
        left: Box<ParsedNode<Expression>>,
        operator: ParsedNode<BinaryOperator>,
        right: Box<ParsedNode<Expression>>,
    },
    Parenthesized {
        _open_round: ParsedNode<WideTokenKind>,
        inner: Box<ParsedNode<Expression>>,
        _close_round: ParsedNode<WideTokenKind>,
    },
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
