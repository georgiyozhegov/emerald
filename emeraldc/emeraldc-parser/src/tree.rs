use emeraldc_lexer::WideToken;
use emeraldc_span::{Span, Spanned};
use serde::{Deserialize, Serialize};

use crate::NodeError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Declaration {
    Function(Function),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function {
    pub _introducer: Parsed<WideToken>,
    pub identifier: Parsed<Identifier>,
    pub _open_round: Parsed<WideToken>,
    pub _close_round: Parsed<WideToken>,
    pub body: Vec<Parsed<Statement>>,
    pub _end: Parsed<WideToken>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identifier;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Statement {
    Let(Let),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Let {
    pub _introducer: Parsed<WideToken>,
    pub identifier: Parsed<Identifier>,
    pub _equal: Parsed<WideToken>,
    pub value: Parsed<Expression>,
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
    pub left: Box<Parsed<Expression>>,
    pub operator: Parsed<BinaryOperator>,
    pub right: Box<Parsed<Expression>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parenthesized {
    pub _open_round: Parsed<WideToken>,
    pub inner: Box<Parsed<Expression>>,
    pub _close_round: Parsed<WideToken>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl BinaryOperator {
    pub fn from_token(kind: &WideToken) -> Option<Self> {
        match kind {
            WideToken::Plus => Some(Self::Add),
            WideToken::Minus => Some(Self::Subtract),
            WideToken::Asterisk => Some(Self::Multiply),
            WideToken::Slash => Some(Self::Divide),
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

pub type Parsed<T> = Result<Spanned<T>, Spanned<NodeError>>;

pub fn span_from_parsed<T>(parsed: &Parsed<T>) -> Span {
    match parsed {
        Ok(spanned) => spanned.span.clone(),
        Err(spanned) => spanned.span.clone(),
    }
}
