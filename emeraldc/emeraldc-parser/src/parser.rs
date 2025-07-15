use std::iter::Peekable;

use emeraldc_lexer::{WideToken, WideTokenKind};

use crate::{
    declaration_parser::DeclarationParser,
    error::{FatalParserError, NodeError, NodeResult},
    introducer_kind::IntroducerKind,
    statement_parser::StatementParser,
    tree::{self, Declaration, Expression, Identifier, ParsedNode, Statement},
};

pub struct Parser {
    pub(crate) tokens: Peekable<std::vec::IntoIter<WideToken>>,
}

impl Parser {
    pub fn parse(
        tokens: impl Iterator<Item = WideToken>,
    ) -> impl Iterator<Item = Result<ParsedNode<Declaration>, FatalParserError>>
    {
        let mut parser = Self::new(tokens);
        log::trace!("running parser");
        std::iter::from_fn(move || {
            parser.tokens.peek()?;
            Some(parser.parse_declaration())
        })
    }

    fn new(tokens: impl Iterator<Item = WideToken>) -> Self {
        log::trace!("initializing parser");
        let tokens = tokens.filter(|t| t.kind != WideTokenKind::Invisible);
        let tokens = tokens.collect::<Vec<_>>().into_iter();
        let tokens = tokens.peekable();
        Self { tokens }
    }

    pub(crate) fn parse_declaration(
        &mut self,
    ) -> Result<ParsedNode<Declaration>, FatalParserError> {
        DeclarationParser::parse(self)
    }

    pub(crate) fn parse_statement(
        &mut self,
    ) -> Result<ParsedNode<Statement>, FatalParserError> {
        StatementParser::parse(self)
    }

    pub(crate) fn parse_expression(
        &mut self,
    ) -> Result<ParsedNode<Expression>, FatalParserError> {
        self.parse_integer()
    }

    pub(crate) fn token_introducer_kind(&mut self) -> IntroducerKind {
        match self.tokens.peek() {
            Some(token) => IntroducerKind::from(&token.kind),
            None => IntroducerKind::Other,
        }
    }

    pub(crate) fn parse_identifier(
        &mut self,
    ) -> Result<ParsedNode<Identifier>, FatalParserError> {
        log::trace!("> identifier");
        match self.tokens.next() {
            Some(token) if token.kind == WideTokenKind::Identifier => {
                let node = Ok(Identifier);
                Ok(ParsedNode::new(node, token.span))
            }
            Some(token) => {
                let error = Err(NodeError::UnexpectedToken(token.kind));
                Ok(ParsedNode::new(error, token.span))
            }
            None => Err(FatalParserError::UnexpectedEof),
        }
    }

    fn parse_integer(&mut self) -> Result<ParsedNode<Expression>, FatalParserError> {
        match self.tokens.next() {
            Some(token) if token.kind == WideTokenKind::Integer => {
                let node = Ok(Expression::Integer);
                Ok(ParsedNode::new(node, token.span))
            }
            Some(token) => {
                let error = Err(NodeError::UnexpectedToken(token.kind));
                Ok(ParsedNode::new(error, token.span))
            }
            None => Err(FatalParserError::UnexpectedEof),
        }
    }

    pub(crate) fn expect(
        &mut self,
        kind: WideTokenKind,
    ) -> Result<ParsedNode<WideTokenKind>, FatalParserError> {
        match self.tokens.next() {
            Some(token) if token.kind == kind => {
                let node = Ok(token.kind);
                Ok(ParsedNode::new(node, token.span))
            }
            Some(token) => {
                let error = Err(NodeError::UnexpectedToken(token.kind));
                Ok(ParsedNode::new(error, token.span))
            }
            None => Err(FatalParserError::UnexpectedEof),
        }
    }

    pub(crate) fn _expect(
        &mut self,
        kind: WideTokenKind,
    ) -> Result<WideToken, FatalParserError> {
        match self.tokens.next() {
            Some(next) if next.kind == kind => {
                log::trace!("> token: {:?}", next.kind);
                Ok(next)
            }
            _ => self.unexpected_token_err(),
        }
    }

    fn unexpected_token_err<T>(&mut self) -> Result<T, FatalParserError> {
        log::error!("[x] unexpected token");
        Err(FatalParserError::UnexpectedToken)
    }
}

pub trait Subparser<'p, T> {
    fn parse(parser: &'p mut Parser)
    -> Result<ParsedNode<T>, FatalParserError>;
}
