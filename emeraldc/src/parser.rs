use std::{iter::Peekable, ops::Index};

use crate::lexer::{Lexer, TokenKind};

#[derive(Debug)]
pub struct Parser {
    lexer: Peekable<Lexer>,
    pub ast: Ast,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Self {
            lexer: lexer.peekable(),
            ast: Ast::new(),
        }
    }

    pub fn parse(mut self) -> (Vec<NodeId>, Ast) {
        let mut program = Vec::new();
        while let Some(declaration) = self.parse_declaration() {
            program.push(declaration);
        }
        (program, self.ast)
    }

    fn parse_declaration(&mut self) -> Option<NodeId> {
        let id = match self.lexer.peek()? {
            TokenKind::Function => self.parse_function(),
            _ => todo!("error handling"),
        };
        Some(id)
    }

    fn parse_function(&mut self) -> NodeId {
        self.expect(TokenKind::Function);
        let name = self.parse_name();
        self.expect(TokenKind::OpenRound);
        self.expect(TokenKind::CloseRound);
        let body = self.parse_declaration_body();
        self.expect(TokenKind::End);
        let node = Node::Function { name, body };
        self.ast.make(node)
    }

    fn parse_name(&mut self) -> NodeId {
        if let Some(TokenKind::Name(name)) = self.lexer.next() {
            let node = Node::Name(name);
            return self.ast.make(node);
        }
        todo!("error handling")
    }

    fn parse_declaration_body(&mut self) -> NodeId {
        let mut body = Vec::new();
        while !self.is_declaration_end() {
            let statement = self.parse_statement();
            body.push(statement);
        }
        let node = Node::DeclarationBody(body);
        self.ast.make(node)
    }

    fn is_declaration_end(&mut self) -> bool {
        matches!(self.lexer.peek(), Some(TokenKind::End))
    }

    fn parse_statement(&mut self) -> NodeId {
        match self.lexer.peek().expect("todo: error handling") {
            TokenKind::Let => self.parse_let(),
            _ => todo!("error handling"),
        }
    }

    fn parse_let(&mut self) -> NodeId {
        self.expect(TokenKind::Let);
        let name = self.parse_name();
        self.expect(TokenKind::Equal);
        let value = self.parse_expression();
        let node = Node::Let { name, value };
        self.ast.make(node)
    }

    fn parse_expression(&mut self) -> NodeId {
        let parser = ExpressionParser::new(self);
        parser.parse()
    }

    fn expect(&mut self, token: TokenKind) {
        let next = self.lexer.next();
        if next != Some(token) {
            todo!("error handling");
        }
    }
}

struct ExpressionParser<'p> {
    parser: &'p mut Parser,
}

// expression     → ...
// equality       → ... XXX
// comparison     → ...
// term           → ...
// factor         → ...
// unary          → ...
// primary        → ...

impl<'p> ExpressionParser<'p> {
    pub fn new(parser: &'p mut Parser) -> Self {
        Self { parser }
    }

    pub fn parse(mut self) -> NodeId {
        self.parse_comparision()
    }

    fn parse_comparision(&mut self) -> NodeId {
        let mut id = self.parse_term();
        while let Some(operator) = self.maybe_comparision_operator() {
            let right = self.parse_term();
            let node = Node::Binary {
                operator,
                left: id,
                right,
            };
            id = self.parser.ast.make(node);
        }
        id
    }

    fn maybe_comparision_operator(&mut self) -> Option<BinaryOperator> {
        const TOKENS: [TokenKind; 3] = [
            TokenKind::Question,
            TokenKind::RightAngle,
            TokenKind::LeftAngle,
        ];
        let token = self.parser.lexer.next_if(|t| TOKENS.contains(t))?;
        let operator = match token {
            TokenKind::Question => BinaryOperator::Equal,
            TokenKind::RightAngle => BinaryOperator::Greater,
            TokenKind::LeftAngle => BinaryOperator::Less,
            _ => unreachable!(),
        };
        Some(operator)
    }

    fn parse_term(&mut self) -> NodeId {
        let mut id = self.parse_factor();
        while let Some(operator) = self.maybe_term_operator() {
            let right = self.parse_factor();
            let node = Node::Binary {
                operator,
                left: id,
                right,
            };
            id = self.parser.ast.make(node);
        }
        id
    }

    fn maybe_term_operator(&mut self) -> Option<BinaryOperator> {
        const TOKENS: [TokenKind; 2] = [
            TokenKind::Plus,
            TokenKind::Minus,
        ];
        let token = self.parser.lexer.next_if(|t| TOKENS.contains(t))?;
        let operator = match token {
            TokenKind::Plus => BinaryOperator::Add,
            TokenKind::Minus => BinaryOperator::Subtract,
            _ => unreachable!(),
        };
        Some(operator)
    }

    fn parse_factor(&mut self) -> NodeId {
        let mut id = self.parse_unary();
        while let Some(operator) = self.maybe_factor_operator() {
            let right = self.parse_unary();
            let node = Node::Binary {
                operator,
                left: id,
                right,
            };
            id = self.parser.ast.make(node);
        }
        id
    }

    fn maybe_factor_operator(&mut self) -> Option<BinaryOperator> {
        const TOKENS: [TokenKind; 2] = [
            TokenKind::Star,
            TokenKind::Slash,
        ];
        let token = self.parser.lexer.next_if(|t| TOKENS.contains(t))?;
        let operator = match token {
            TokenKind::Star => BinaryOperator::Multiply,
            TokenKind::Slash => BinaryOperator::Divide,
            _ => unreachable!(),
        };
        Some(operator)
    }

    fn parse_unary(&mut self) -> NodeId {
        // todo: unary parsing
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> NodeId {
        let node = match self.parser.lexer.next().expect("todo: error handling") {
            TokenKind::Integer(value) => Node::Integer(value),
            TokenKind::Name(name) => Node::Name(name),
            _ => todo!("error handling"),
        };
        self.parser.ast.make(node)
    }
}

#[derive(Debug)]
pub struct Ast {
    tree: Vec<Node>,
}

impl Ast {
    pub fn new() -> Self {
        Self { tree: Vec::new() }
    }

    pub fn make(&mut self, node: Node) -> NodeId {
        let index = self.tree.len();
        self.tree.push(node);
        NodeId(index)
    }
}

#[derive(Debug)]
pub enum Node {
    Name(String),
    Integer(i128),
    Binary {
        operator: BinaryOperator,
        left: NodeId,
        right: NodeId,
    },
    Let {
        name: NodeId,
        value: NodeId,
    },
    DeclarationBody(Vec<NodeId>),
    Function {
        name: NodeId,
        body: NodeId,
    },
}

#[non_exhaustive]
#[derive(Debug)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equal,
    Greater,
    Less,
}

#[derive(Debug, Clone, Copy)]
pub struct NodeId(usize);

impl Index<NodeId> for Ast {
    type Output = Node;

    fn index(&self, id: NodeId) -> &Self::Output {
        &self.tree[id.0]
    }
}
