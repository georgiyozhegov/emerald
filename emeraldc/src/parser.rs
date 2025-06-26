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
        if let Some(TokenKind::Integer(value)) = self.lexer.next() {
            let node = Node::Integer(value);
            return self.ast.make(node);
        }
        todo!("error handling and expression parsing")
    }

    fn expect(&mut self, token: TokenKind) {
        let next = self.lexer.next();
        if next != Some(token) {
            todo!("error handling");
        }
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
    Let { name: NodeId, value: NodeId },
    DeclarationBody(Vec<NodeId>),
    Function { name: NodeId, body: NodeId },
}

#[derive(Debug, Clone, Copy)]
pub struct NodeId(usize);

impl Index<NodeId> for Ast {
    type Output = Node;

    fn index(&self, id: NodeId) -> &Self::Output {
        &self.tree[id.0]
    }
}
