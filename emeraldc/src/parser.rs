use std::{fmt::Debug, iter::Peekable, ops::Index};

use crate::lexer::{Lexer, LexerError, TokenKind};

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

    pub fn parse(mut self) -> Result<(Vec<NodeId>, Ast), ParserError> {
        let mut program = Vec::new();
        while let Some(declaration) = self.parse_declaration()? {
            program.push(declaration);
        }
        Ok((program, self.ast))
    }

    fn parse_declaration(&mut self) -> Result<Option<NodeId>, ParserError> {
        let id = match self.peek_token()? {
            Some(TokenKind::Function) => self.parse_function(),
            Some(got) => {
                let error = ParserError::unexpected_token("declaration", got.clone());
                Err(error)
            }
            _eof => {
                return Ok(None);
            }
        };
        Some(id).transpose()
    }

    fn parse_function(&mut self) -> Result<NodeId, ParserError> {
        self.expect(TokenKind::Function, "function")?;
        let name = self.parse_name()?;
        self.expect(TokenKind::OpenRound, "'(' after function name")?;
        self.expect(TokenKind::CloseRound, "')' after function parameters")?;
        let body = self.parse_declaration_body()?;
        self.expect(TokenKind::End, "'end' after function body")?;
        let node = Node::Function { name, body };
        Ok(self.ast.make(node))
    }

    fn parse_name(&mut self) -> Result<NodeId, ParserError> {
        if let TokenKind::Name(name) = self.next_token()? {
            let node = Node::Name(name);
            return Ok(self.ast.make(node));
        }
        todo!("error handling")
    }

    fn parse_declaration_body(&mut self) -> Result<NodeId, ParserError> {
        let mut body = Vec::new();
        while !self.is_declaration_end()? {
            let statement = self.parse_statement()?;
            body.push(statement);
        }
        let node = Node::DeclarationBody(body);
        Ok(self.ast.make(node))
    }

    fn is_declaration_end(&mut self) -> Result<bool, ParserError> {
        Ok(matches!(self.peek_token()?, Some(TokenKind::End)))
    }

    fn parse_statement(&mut self) -> Result<NodeId, ParserError> {
        match self.peek_token()? {
            Some(TokenKind::Let) => self.parse_let(),
            Some(got) => {
                let error = ParserError::unexpected_token("statement", got.clone());
                Err(error)
            }
            _eof => Err(ParserError::UnexpectedEof),
        }
    }

    fn parse_let(&mut self) -> Result<NodeId, ParserError> {
        self.expect(TokenKind::Let, "let")?;
        let name = self.parse_name()?;
        self.expect(TokenKind::Equal, "=")?;
        let value = self.parse_expression()?;
        let node = Node::Let { name, value };
        Ok(self.ast.make(node))
    }

    fn parse_expression(&mut self) -> Result<NodeId, ParserError> {
        let parser = ExpressionParser::new(self);
        parser.parse()
    }

    fn expect(&mut self, token: TokenKind, message: &'static str) -> Result<(), ParserError> {
        let next = self.next_token()?;
        if next != token {
            let error = ParserError::unexpected_token(message, next);
            return Err(error);
        }
        Ok(())
    }

    fn next_token(&mut self) -> Result<TokenKind, ParserError> {
        if let Some(next) = self.lexer.next() {
            return next.map_err(|e| ParserError::Lexer(e));
        }
        Err(ParserError::UnexpectedEof)
    }

    fn peek_token(&mut self) -> Result<Option<&TokenKind>, ParserError> {
        match self.lexer.peek() {
            Some(Ok(token)) => Ok(Some(token)),
            Some(Err(e)) => Err(ParserError::Lexer(e.clone())),
            None => Ok(None),
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

    pub fn parse(mut self) -> Result<NodeId, ParserError> {
        self.parse_comparision()
    }

    fn parse_comparision(&mut self) -> Result<NodeId, ParserError> {
        self.parse_binary(Self::parse_term, Self::maybe_comparision_operator)
    }

    fn maybe_comparision_operator(&mut self) -> Result<Option<BinaryOperator>, ParserError> {
        let next = self.parser.peek_token()?;
        matches!(
            next,
            Some(TokenKind::Question) | Some(TokenKind::RightAngle) | Some(TokenKind::LeftAngle)
        )
        .then(|| self.parse_binary_operator())
        .transpose()
    }

    fn parse_term(&mut self) -> Result<NodeId, ParserError> {
        self.parse_binary(Self::parse_factor, Self::maybe_term_operator)
    }

    fn maybe_term_operator(&mut self) -> Result<Option<BinaryOperator>, ParserError> {
        let next = self.parser.peek_token()?;
        matches!(next, Some(TokenKind::Plus) | Some(TokenKind::Minus))
            .then(|| self.parse_binary_operator())
            .transpose()
    }

    fn parse_factor(&mut self) -> Result<NodeId, ParserError> {
        self.parse_binary(Self::parse_unary, Self::maybe_factor_operator)
    }

    fn maybe_factor_operator(&mut self) -> Result<Option<BinaryOperator>, ParserError> {
        let next = self.parser.peek_token()?;
        matches!(next, Some(TokenKind::Star) | Some(TokenKind::Slash))
            .then(|| self.parse_binary_operator())
            .transpose()
    }

    fn parse_binary(
        &mut self,
        parse_higher: impl Fn(&mut Self) -> Result<NodeId, ParserError>,
        maybe_operator: impl Fn(&mut Self) -> Result<Option<BinaryOperator>, ParserError>,
    ) -> Result<NodeId, ParserError> {
        let mut id = parse_higher(self)?;
        while let Some(operator) = maybe_operator(self)? {
            let right = parse_higher(self)?;
            let node = Node::Binary {
                operator,
                left: id,
                right,
            };
            id = self.parser.ast.make(node);
        }
        Ok(id)
    }

    fn parse_binary_operator(&mut self) -> Result<BinaryOperator, ParserError> {
        let operator = match self.parser.next_token()? {
            TokenKind::Plus => BinaryOperator::Add,
            TokenKind::Minus => BinaryOperator::Subtract,
            TokenKind::Star => BinaryOperator::Multiply,
            TokenKind::Slash => BinaryOperator::Divide,
            TokenKind::Question => BinaryOperator::Equal,
            TokenKind::RightAngle => BinaryOperator::Greater,
            TokenKind::LeftAngle => BinaryOperator::Less,
            got => {
                let error = ParserError::unexpected_token("operator", got);
                return Err(error);
            }
        };
        Ok(operator)
    }

    fn parse_unary(&mut self) -> Result<NodeId, ParserError> {
        // todo: unary parsing
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<NodeId, ParserError> {
        let node = match self.parser.next_token()? {
            TokenKind::Integer(value) => Node::Integer(value),
            TokenKind::Name(name) => Node::Name(name),
            got => {
                let error = ParserError::unexpected_token("literal", got);
                return Err(error);
            }
        };
        Ok(self.parser.ast.make(node))
    }
}

#[derive(Debug)]
pub enum ParserError {
    Lexer(LexerError),
    UnexpectedToken { expected: String, got: TokenKind },
    UnexpectedEof,
}

impl ParserError {
    pub fn unexpected_token(expected: impl ToString, got: TokenKind) -> Self {
        Self::UnexpectedToken {
            expected: expected.to_string(),
            got,
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
