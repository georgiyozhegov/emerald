use core::str;

use emeraldc_lexer::{WideToken, WideTokenKind};

use crate::{error::ParserError, token_stream::TokenStream, tree};

pub struct Parser {
    token_stream: TokenStream,
}

impl Parser {
    pub fn parse(
        tokens: impl Iterator<Item = WideToken>,
    ) -> impl Iterator<Item = Result<tree::Declaration, ParserError>> {
        let mut parser = Self::new(tokens);
        log::trace!("running parser");
        std::iter::from_fn(move || parser.maybe_declaration())
    }

    fn new(tokens: impl Iterator<Item = WideToken>) -> Self {
        log::trace!("initializing parser");
        let token_stream = TokenStream::new(tokens);
        Self { token_stream }
    }

    fn maybe_declaration(
        &mut self,
    ) -> Option<Result<tree::Declaration, ParserError>> {
        if self.token_stream.is_eof() {
            log::trace!("[x] got eof, stopping parser");
            None
        } else {
            Some(self.parse_declaration())
        }
    }

    fn parse_declaration(&mut self) -> Result<tree::Declaration, ParserError> {
        log::trace!("[x] declaration start");
        let token = self.token_stream.peek()?;
        let node = match token.kind {
            WideTokenKind::FunctionKeyword => self.parse_function(),
            _ => self.invalid_declaration_err(),
        };
        log::trace!("[x] declaration end");
        node
    }

    fn invalid_declaration_err<T>(&mut self) -> Result<T, ParserError> {
        let token = self.token_stream.next()?;
        log::error!("[x] invalid declaration: {:?}", token.kind);
        Err(ParserError::InvalidDeclaration(token))
    }

    fn parse_function(&mut self) -> Result<tree::Declaration, ParserError> {
        log::trace!("=> function start");
        self.expect(WideTokenKind::FunctionKeyword)?;
        let identifier = self.parse_identifier();
        self.expect(WideTokenKind::OpenRound)?;
        self.expect(WideTokenKind::CloseRound)?;
        let body = self.parse_function_body();
        self.expect(WideTokenKind::EndKeyword)?;
        log::trace!("=> function end");
        Ok(tree::Declaration::Function { identifier, body })
    }

    fn parse_identifier(&mut self) -> Result<tree::Identifier, ParserError> {
        log::trace!("--> identifier");
        match self.token_stream.next()? {
            next if next.kind == WideTokenKind::Identifier => {
                let span = next.span;
                Ok(tree::Identifier { name: span })
            }
            _ => self.unexpected_token_err(),
        }
    }

    fn parse_function_body(
        &mut self,
    ) -> Vec<Result<tree::Statement, ParserError>> {
        log::trace!("=> function body start");
        let mut body = Vec::new();
        while !self.is_function_body_end() {
            let statement = self.parse_statement();
            body.push(statement);
        }
        log::trace!("=> function body end");
        body
    }

    fn is_function_body_end(&mut self) -> bool {
        self.token_stream
            .peek()
            .is_ok_and(|t| t.kind == WideTokenKind::EndKeyword)
    }

    fn parse_statement(&mut self) -> Result<tree::Statement, ParserError> {
        log::trace!("==> statement start");
        let node = match self.token_stream.peek()?.kind {
            WideTokenKind::LetKeyword => self.parse_let(),
            _ => self.unexpected_token_err(),
        };
        log::trace!("==> statement end");
        node
    }

    fn parse_let(&mut self) -> Result<tree::Statement, ParserError> {
        self.expect(WideTokenKind::LetKeyword)?;
        let identifier = self.parse_identifier();
        self.expect(WideTokenKind::Equal)?;
        let value = self.parse_integer();
        Ok(tree::Statement::Let { identifier, value })
    }

    fn parse_integer(&mut self) -> Result<tree::Expression, ParserError> {
        log::trace!("--> integer");
        match self.token_stream.next()? {
            next if next.kind == WideTokenKind::Integer => {
                Ok(tree::Expression::Integer(next.span))
            }
            _ => self.unexpected_token_err(),
        }
    }

    fn expect(
        &mut self,
        kind: WideTokenKind,
    ) -> Result<WideToken, ParserError> {
        match self.token_stream.next()? {
            next if next.kind == kind => {
                log::trace!("--> token: {:?}", next.kind);
                Ok(next)
            }
            _ => self.unexpected_token_err(),
        }
    }

    fn unexpected_token_err<T>(&mut self) -> Result<T, ParserError> {
        let previous = self.token_stream.take_previous()?;
        log::error!("[x] unexpected token: {:?}", previous.kind);
        Err(ParserError::UnexpectedToken(previous))
    }
}
