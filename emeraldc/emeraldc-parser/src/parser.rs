use std::iter::Peekable;
use std::vec;

use emeraldc_lexer::{LexerError, Token};

use crate::{
    BodyNode, DeclarationNode, DeclarationParser, DummyToken, ExpressionNode,
    ExpressionParser, IdentifierNode, ParseTree, StatementNode,
    StatementParser,
};

/// Constructs a parse tree from tokens.
#[derive(Debug)]
pub struct Parser {
    pub(crate) source: ParserSource,
}

impl Parser {
    pub fn new(source: ParserSource) -> Self {
        Self { source }
    }

    /// Parse a program.
    pub fn parse(mut self) -> ParseTree {
        let mut pt = ParseTree::new();
        while let Some(declaration) = self.parse_declaration() {
            pt.program.push(declaration);
        }
        pt
    }

    /// Parse a single declaration.
    fn parse_declaration(
        &mut self,
    ) -> Option<Result<DeclarationNode, ParserError>> {
        (!self.source.is_eof()).then(|| {})?;
        let parser = DeclarationParser::new(self);
        let node = parser.parse();
        Some(node)
    }

    /// Parse an identifier.
    pub(crate) fn parse_identifier(
        &mut self,
    ) -> Result<IdentifierNode, ParserError> {
        let Token::Name(name) = self.expect(DummyToken::Name)? else {
            unreachable!()
        };
        let node = IdentifierNode { name };
        Ok(node)
    }

    /// Parse a single expression.
    pub(crate) fn parse_expression(
        &mut self,
    ) -> Result<ExpressionNode, ParserError> {
        let parser = ExpressionParser::new(self);
        dbg!(parser.parse())
    }

    /// Parse a declaration/statement body.
    pub(crate) fn parse_body(&mut self) -> Result<BodyNode, ParserError> {
        let mut body = Vec::new();
        while self.source.peek()? != DummyToken::End {
            let statement = self.parse_statement()?;
            body.push(statement);
        }
        let node = BodyNode { body };
        Ok(node)
    }

    /// Parse a single statement.
    fn parse_statement(&mut self) -> Result<StatementNode, ParserError> {
        let parser = StatementParser::new(self);
        dbg!(parser.parse())
    }

    /// Check if next token matches the expected one.
    pub fn expect(&mut self, token: DummyToken) -> Result<Token, ParserError> {
        let next = self.source.next()?;
        let as_dummy = DummyToken::from(&next);
        if as_dummy == token {
            Ok(next)
        } else {
            let error = ParserError::UnexpectedToken {
                expected: token,
                got: next,
            };
            Err(error)
        }
    }
}

#[derive(Debug)]
pub struct ParserSource {
    iter: Peekable<vec::IntoIter<Result<Token, LexerError>>>,
}

impl ParserSource {
    pub fn new(
        iter: Peekable<vec::IntoIter<Result<Token, LexerError>>>,
    ) -> Self {
        Self { iter }
    }

    /// Consume the next token.
    pub(crate) fn next(&mut self) -> Result<Token, ParserError> {
        let maybe_next = self.iter.next();
        let next = maybe_next.ok_or(ParserError::UnexpectedEof)?;
        next.map_err(|e| ParserError::Lexer(e))
    }

    /// Peek the next token.
    pub(crate) fn peek(&mut self) -> Result<DummyToken, ParserError> {
        let next = self.iter.peek();
        if let Some(token) = next {
            token
                .as_ref()
                .map_err(|e: &LexerError| ParserError::Lexer(e.clone()))
                .map(|t| DummyToken::from(t))
        } else {
            Ok(DummyToken::Eof)
        }
    }

    pub(crate) fn is_eof(&mut self) -> bool {
        self.peek().is_ok_and(|t| t == DummyToken::Eof)
    }
}
#[derive(Debug, Clone)]
pub enum ParserError {
    Lexer(LexerError),
    UnexpectedEof,
    UnexpectedToken {
        expected: DummyToken,
        got: Token,
    },
    UnexpectedTokenStr {
        expected: &'static str,
        got: DummyToken,
    },
    ExpectedDeclaration {
        got: DummyToken,
    },
    ExpectedStatement {
        got: DummyToken,
    },
}
