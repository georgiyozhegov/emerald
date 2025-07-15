use std::iter::Peekable;

use emeraldc_lexer::{WideToken, WideTokenKind};

use crate::{
    declaration_parser::DeclarationParser,
    error::{FatalParserError, NodeError, NodeResult},
    introducer_kind::IntroducerKind,
    token_stream::TokenStream,
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
            let declaration = DeclarationParser::parse(&mut parser);
            Some(declaration)
        })
    }

    fn new(tokens: impl Iterator<Item = WideToken>) -> Self {
        log::trace!("initializing parser");
        let tokens = tokens.filter(|t| t.kind != WideTokenKind::Invisible);
        let tokens = tokens.collect::<Vec<_>>().into_iter();
        let tokens = tokens.peekable();
        Self { tokens }
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

    fn parse_statement(&mut self) -> Result<tree::Statement, FatalParserError> {
        todo!();
        /*
        log::trace!("==> statement start");
        assert!(self.token_introducer_kind() == IntroducerKind::Statement);
        let node = match self.token_stream.peek()?.kind {
            WideTokenKind::LetKeyword => self.parse_let(),
            _ => self.unexpected_token_err(),
        };
        log::trace!("==> statement end");
        log::trace!("==> node: {node:#?}");
        node
        */
    }

    fn parse_let(&mut self) -> Result<tree::Statement, FatalParserError> {
        log::trace!("--> let start");
        self.expect(WideTokenKind::LetKeyword)?;
        let identifier = self.parse_identifier();
        self.expect(WideTokenKind::Equal)?;
        let value = self.parse_expression();
        log::trace!("--> let end");
        todo!()
        // Ok(tree::Statement::Let { identifier, value })
    }

    fn parse_expression(
        &mut self,
    ) -> Result<tree::Expression, FatalParserError> {
        todo!();
        log::trace!("===> expression start");
        /*
        let node = match self.token_stream.peek()?.kind {
            WideTokenKind::Identifier => self
                .parse_identifier()
                .and_then(|i| Ok(tree::Expression::Variable(i))),
            WideTokenKind::Integer => self.parse_integer(),
            _ => self.unexpected_token_err(),
        };
        */
        log::trace!("===> expression end");
        // log::trace!("===> node: {node:#?}");
        todo!()
        // node
    }

    fn parse_integer(&mut self) -> NodeResult<Expression> {
        todo!();
        /*
        log::trace!("---> integer");
        match self.tokens.next()? {
            next if next.kind == WideTokenKind::Integer => {
                let node = Ok(Expression::Integer);
                Ok(ParsedNode::new(node, next.span))
            }
            next => {
                let error = Err(NodeError::UnexpectedToken(next.kind));
                Ok(ParsedNode::new(error, next.span))
            }
        }
        */
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
