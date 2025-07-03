use crate::ParserError;

#[derive(Debug, Clone)]
pub struct ParseTree {
    pub program: Vec<Result<DeclarationNode, ParserError>>,
}

impl ParseTree {
    pub fn new() -> Self {
        Self {
            program: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum DeclarationNode {
    Function {
        identifier: IdentifierNode,
        body: BodyNode,
    },
}

#[derive(Debug, Clone)]
pub struct IdentifierNode {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct BodyNode {
    pub body: Vec<StatementNode>,
}

#[derive(Debug, Clone)]
pub enum StatementNode {
    Let {
        identifier: IdentifierNode,
        value: ExpressionNode,
    },
    If {
        condition: ExpressionNode,
        then: BodyNode,
        otherwise: Option<ElseNode>,
    },
    While {
        condition: ExpressionNode,
        body: BodyNode,
    },
}

#[derive(Debug, Clone)]
pub struct ElseNode {
    pub body: BodyNode,
}

#[derive(Debug, Clone)]
pub enum ExpressionNode {
    Integer(i128),
    Identifier(IdentifierNode),
    Binary {
        operator: BinaryOperatorNode,
        left: Box<ExpressionNode>,
        right: Box<ExpressionNode>,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum BinaryOperatorNode {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equal,
    Greater,
    Less,
}
