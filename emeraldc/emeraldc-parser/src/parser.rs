use std::iter::Peekable;

use emeraldc_lexer::WideToken;
use emeraldc_span::{IntoSpanned, Spanned};

use crate::{
    Declaration, DeclarationParser, Expression, ExpressionParser,
    FatalParserError, Identifier, IntroducerKind, NodeError, Parsed, Statement,
    StatementParser,
};

pub struct Parser {
    pub(crate) tokens: Peekable<std::vec::IntoIter<Spanned<WideToken>>>,
}

impl Parser {
    pub fn parse(
        tokens: impl Iterator<Item = Spanned<WideToken>>,
    ) -> impl Iterator<Item = Result<Parsed<Declaration>, FatalParserError>>
    {
        let mut parser = Self::new(tokens);
        std::iter::from_fn(move || {
            parser.tokens.peek()?;
            Some(parser.parse_declaration())
        })
    }

    fn new(tokens: impl Iterator<Item = Spanned<WideToken>>) -> Self {
        let tokens = tokens.filter(|t| {
            !matches!(t.value, WideToken::Invisible | WideToken::Comment)
        });
        let tokens = tokens.collect::<Vec<_>>().into_iter();
        let tokens = tokens.peekable();
        Self { tokens }
    }

    pub(crate) fn parse_declaration(
        &mut self,
    ) -> Result<Parsed<Declaration>, FatalParserError> {
        let declaration = DeclarationParser::parse(self);
        self.synchronize(|this| {
            this.tokens.peek().is_none()
                || this.token_introducer_kind() == IntroducerKind::Declaration
        });
        declaration
    }

    pub(crate) fn parse_statement(
        &mut self,
    ) -> Result<Parsed<Statement>, FatalParserError> {
        let statement = StatementParser::parse(self);
        self.synchronize(|this| {
            this.tokens.peek().is_none()
                || this.token_introducer_kind() == IntroducerKind::Statement
                || this
                    .tokens
                    .peek()
                    .is_some_and(|s| s.value == WideToken::EndKeyword)
        });
        statement
    }

    pub(crate) fn parse_expression(
        &mut self,
    ) -> Result<Parsed<Expression>, FatalParserError> {
        ExpressionParser::parse(self)
    }

    pub(crate) fn token_introducer_kind(&mut self) -> IntroducerKind {
        match self.tokens.peek() {
            Some(token) => IntroducerKind::from(&token.value),
            None => IntroducerKind::Other,
        }
    }

    pub(crate) fn parse_identifier(
        &mut self,
    ) -> Result<Parsed<Identifier>, FatalParserError> {
        match self.tokens.next() {
            Some(token) if token.value == WideToken::Identifier => {
                let parsed = Ok(Identifier.into_spanned(token.span));
                Ok(parsed)
            }
            Some(token) if token.value.had_error() => {
                let error = Err(NodeError::Lexer(token.value.as_error())
                    .into_spanned(token.span));
                Ok(error)
            }
            Some(token) => {
                let error = Err(NodeError::UnexpectedToken(token.value)
                    .into_spanned(token.span));
                Ok(error)
            }
            None => Err(FatalParserError::UnexpectedEof),
        }
    }

    pub(crate) fn expect(
        &mut self,
        kind: WideToken,
    ) -> Result<Parsed<WideToken>, FatalParserError> {
        match self.tokens.next() {
            Some(token) if token.value == kind => {
                let parsed = Ok(token.value.into_spanned(token.span));
                Ok(parsed)
            }
            Some(token) if token.value.had_error() => {
                let error = Err(NodeError::Lexer(token.value.as_error())
                    .into_spanned(token.span));
                Ok(error)
            }
            Some(token) => {
                let error = Err(NodeError::UnexpectedToken(token.value)
                    .into_spanned(token.span));
                Ok(error)
            }
            None => Err(FatalParserError::UnexpectedEof),
        }
    }

    fn synchronize(&mut self, stop: impl Fn(&mut Self) -> bool) {
        while !stop(self) {
            self.tokens.next().unwrap();
        }
    }
}

pub trait Subparser<'p, T> {
    fn parse(parser: &'p mut Parser) -> Result<Parsed<T>, FatalParserError>;
}
