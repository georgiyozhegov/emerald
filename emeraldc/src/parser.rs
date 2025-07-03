use std::fmt;
use std::iter::Peekable;

use crate::lexer::{Lexer, LexerError, TokenKind};

#[derive(Debug)]
pub struct Parser {
    lexer: Peekable<Lexer>,
    pub ast: Par,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Self {
            lexer: lexer.peekable(),
            ast: Ast::new(),
        }
    }

    pub fn parse(mut self) -> Result<(Vec<NodeId>, Ast<Node>), ParserError> {
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
                let error =
                    ParserError::unexpected_token("declaration", got.clone());
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
        while !self.is_declaration_body_end()? {
            let statement = self.parse_statement()?;
            body.push(statement);
        }
        let node = Node::DeclarationBody(body);
        Ok(self.ast.make(node))
    }

    fn is_declaration_body_end(&mut self) -> Result<bool, ParserError> {
        Ok(matches!(self.peek_token()?, Some(TokenKind::End)))
    }

    fn parse_statement(&mut self) -> Result<NodeId, ParserError> {
        match self.peek_token()? {
            Some(TokenKind::Let) => self.parse_let(),
            Some(TokenKind::Name(..)) => self.parse_assign(),
            Some(TokenKind::If) => self.parse_if(),
            Some(TokenKind::While) => self.parse_while(),
            Some(got) => {
                let error =
                    ParserError::unexpected_token("statement", got.clone());
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

    fn parse_assign(&mut self) -> Result<NodeId, ParserError> {
        let name = self.parse_name()?;
        self.expect(TokenKind::Equal, "=")?;
        let value = self.parse_expression()?;
        let node = Node::Assign { name, value };
        Ok(self.ast.make(node))
    }

    fn parse_if(&mut self) -> Result<NodeId, ParserError> {
        self.expect(TokenKind::If, "if")?;
        let condition = self.parse_expression()?;
        self.expect(TokenKind::OpenCurly, "'{' after if condition")?;
        let body = self.parse_statement_body()?;
        self.expect(TokenKind::CloseCurly, "'}' after if body")?;
        let else_ = self.maybe_else()?;
        let node = Node::If {
            condition,
            body,
            else_,
        };
        Ok(self.ast.make(node))
    }

    fn maybe_else(&mut self) -> Result<Option<NodeId>, ParserError> {
        matches!(self.peek_token()?, Some(TokenKind::Else))
            .then(|| self.parse_else())
            .transpose()
    }

    fn parse_else(&mut self) -> Result<NodeId, ParserError> {
        self.expect(TokenKind::Else, "'else' after if body")?;
        self.expect(TokenKind::OpenCurly, "'{' after else")?;
        let body = self.parse_statement_body()?;
        self.expect(TokenKind::CloseCurly, "'}' after else body")?;
        let node = Node::Else { body };
        Ok(self.ast.make(node))
    }

    fn parse_while(&mut self) -> Result<NodeId, ParserError> {
        self.expect(TokenKind::While, "while")?;
        let condition = self.parse_expression()?;
        self.expect(TokenKind::OpenCurly, "'{' after while condition")?;
        let body = self.parse_statement_body()?;
        self.expect(TokenKind::CloseCurly, "'}' after while body")?;
        let node = Node::While { condition, body };
        Ok(self.ast.make(node))
    }

    fn parse_statement_body(&mut self) -> Result<NodeId, ParserError> {
        let mut body = Vec::new();
        while !self.is_statement_body_end()? {
            let statement = self.parse_statement()?;
            body.push(statement);
        }
        let node = Node::StatementBody(body);
        Ok(self.ast.make(node))
    }

    fn is_statement_body_end(&mut self) -> Result<bool, ParserError> {
        Ok(matches!(self.peek_token()?, Some(TokenKind::CloseCurly)))
    }

    fn parse_expression(&mut self) -> Result<NodeId, ParserError> {
        let parser = ExpressionParser::new(self);
        parser.parse()
    }

    fn expect(
        &mut self,
        token: TokenKind,
        message: &'static str,
    ) -> Result<(), ParserError> {
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

    fn maybe_comparision_operator(
        &mut self,
    ) -> Result<Option<BinaryOperator>, ParserError> {
        let next = self.parser.peek_token()?;
        matches!(
            next,
            Some(TokenKind::Question)
                | Some(TokenKind::RightAngle)
                | Some(TokenKind::LeftAngle)
        )
        .then(|| self.parse_binary_operator())
        .transpose()
    }

    fn parse_term(&mut self) -> Result<NodeId, ParserError> {
        self.parse_binary(Self::parse_factor, Self::maybe_term_operator)
    }

    fn maybe_term_operator(
        &mut self,
    ) -> Result<Option<BinaryOperator>, ParserError> {
        let next = self.parser.peek_token()?;
        matches!(next, Some(TokenKind::Plus) | Some(TokenKind::Minus))
            .then(|| self.parse_binary_operator())
            .transpose()
    }

    fn parse_factor(&mut self) -> Result<NodeId, ParserError> {
        self.parse_binary(Self::parse_unary, Self::maybe_factor_operator)
    }

    fn maybe_factor_operator(
        &mut self,
    ) -> Result<Option<BinaryOperator>, ParserError> {
        let next = self.parser.peek_token()?;
        matches!(next, Some(TokenKind::Star) | Some(TokenKind::Slash))
            .then(|| self.parse_binary_operator())
            .transpose()
    }

    fn parse_binary(
        &mut self,
        parse_higher: impl Fn(&mut Self) -> Result<NodeId, ParserError>,
        maybe_operator: impl Fn(
            &mut Self,
        )
            -> Result<Option<BinaryOperator>, ParserError>,
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

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Lexer(error) => write!(f, "{error}"),
            Self::UnexpectedToken { expected, got } => {
                write!(f, "expected {expected}, got '{got}'")
            }
            Self::UnexpectedEof => write!(f, "unexpected end of file"),
        }
    }
}

pub type ParseTree = Vec<ParseNode>;
pub type ParseNodeId = usize;

#[non_exhaustive]
#[derive(Debug)]
pub enum ParseNode {
    // expressions
    Name(String),
    Integer(i128),
    Binary {
        operator: BinaryOperator,
        left: ParseNodeId,
        right: ParseNodeId,
    },
    // statements
    Let {
        name: ParseNodeId,
        value: ParseNodeId,
    },
    Assign {
        name: ParseNodeId,
        value: ParseNodeId,
    },
    Else {
        body: ParseNodeId,
    },
    If {
        condition: ParseNodeId,
        body: ParseNodeId,
        else_: Option<ParseNodeId>,
    },
    While {
        condition: ParseNodeId,
        body: ParseNodeId,
    },
    StatementBody(Vec<ParseNodeId>),
    // declarations
    Function {
        name: ParseNodeId,
        body: ParseNodeId,
    },
    DeclarationBody(Vec<ParseNodeId>),
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

impl fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let token = match self {
            Self::Add => TokenKind::Plus,
            Self::Subtract => TokenKind::Minus,
            Self::Multiply => TokenKind::Star,
            Self::Divide => TokenKind::Slash,
            Self::Equal => TokenKind::Question,
            Self::Greater => TokenKind::RightAngle,
            Self::Less => TokenKind::LeftAngle,
        };
        write!(f, "{token}")
    }
}
