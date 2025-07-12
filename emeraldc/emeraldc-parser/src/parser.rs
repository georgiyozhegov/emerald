use std::iter::Peekable;

use emeraldc_lexer::{KeywordKind, WideToken, WideTokenKind};

use crate::ParserError;
use crate::parse_tree::{
    Declaration, Expression, FunctionBody, Identifier, ParsedNode, Statement,
};

pub struct Parser {
    token_stream: Peekable<std::vec::IntoIter<WideToken>>,
}

impl Parser {
    pub fn parse(
        token_stream: impl Iterator<Item = WideToken>,
    ) -> impl Iterator<Item = ParsedNode<Declaration>> {
        let mut parser = Self::new(token_stream);
        std::iter::from_fn(move || parser.maybe_declaration())
    }

    fn new(token_stream: impl Iterator<Item = WideToken>) -> Self {
        let token_stream = Self::filter_invisible(token_stream);
        let token_stream = token_stream.collect::<Vec<_>>().into_iter();
        let token_stream = token_stream.peekable();
        Self { token_stream }
    }

    fn filter_invisible(
        token_stream: impl Iterator<Item = WideToken>,
    ) -> impl Iterator<Item = WideToken> {
        token_stream.filter(|t| t.kind != WideTokenKind::Invisible)
    }

    /// Парсит объявление, если в потоке токенов ещё есть токены.
    fn maybe_declaration(&mut self) -> Option<ParsedNode<Declaration>> {
        self.has_next_token().then(|| self.parse_declaration())
    }

    /// Проверяет, что входные токены не закончились.
    fn has_next_token(&mut self) -> bool {
        self.token_stream.peek().is_some()
    }

    fn parse_declaration(&mut self) -> ParsedNode<Declaration> {
        match self.token_stream.peek().unwrap().kind.clone() {
            WideTokenKind::Keyword(KeywordKind::Function) => {
                self.parse_function()
            }
            token => {
                self.token_stream.next().unwrap();
                Err(ParserError::UnexpectedTokenStr {
                    expected: "declaration",
                    got: token.to_owned(),
                })
            }
        }
    }

    fn parse_function(&mut self) -> ParsedNode<Declaration> {
        self.expect_keyword(KeywordKind::Function)?;
        let name = self.parse_identifier();
        self.expect(WideTokenKind::OpenRound)?;
        self.expect(WideTokenKind::CloseRound)?;
        let body = self.parse_function_body();
        self.expect_keyword(KeywordKind::End)?;
        let node = Declaration::Function { name, body };
        Ok(node)
    }

    fn parse_identifier(&mut self) -> ParsedNode<Identifier> {
        let token = self.expect(WideTokenKind::Identifier)?;
        Ok(Identifier {
            name_span: token.span,
        })
    }

    fn parse_function_body(&mut self) -> ParsedNode<FunctionBody> {
        let mut statements = Vec::new();
        while !self.is_function_body_end() {
            let statement = self.parse_statement();
            if statement.is_err() {
                self.synchronize_statement();
            }
            statements.push(statement);
        }
        let node = FunctionBody { statements };
        Ok(node)
    }

    fn is_function_body_end(&mut self) -> bool {
        let end_kind = WideTokenKind::Keyword(KeywordKind::End);
        self.token_stream.peek().is_some_and(|t| t.kind == end_kind)
    }

    /// Пропускает токены до начала новой инструкции.
    fn synchronize_statement(&mut self) {
        while !self.is_statement_introducer() && !self.is_function_body_end() {
            self.token_stream.next();
        }
    }

    fn is_statement_introducer(&mut self) -> bool {
        const STATEMENT_INTRODUCERS: [WideTokenKind; 1] =
            [WideTokenKind::Keyword(KeywordKind::Let)];
        self.token_stream
            .peek()
            .is_some_and(|t| STATEMENT_INTRODUCERS.contains(&t.kind))
    }

    fn parse_statement(&mut self) -> ParsedNode<Statement> {
        match self.token_stream.peek().unwrap().kind.clone() {
            WideTokenKind::Keyword(KeywordKind::Let) => self.parse_let(),
            token => {
                self.token_stream.next().unwrap();
                Err(ParserError::UnexpectedTokenStr {
                    expected: "statement",
                    got: token,
                })
            }
        }
    }

    fn parse_let(&mut self) -> ParsedNode<Statement> {
        self.expect_keyword(KeywordKind::Let)?;
        let name = self.parse_identifier();
        self.expect(WideTokenKind::Equal)?;
        let value = self.parse_integer(); // todo: parse_expression
        let node = Statement::Let { name, value };
        Ok(node)
    }

    fn parse_integer(&mut self) -> ParsedNode<Expression> {
        let token = self.expect(WideTokenKind::Integer)?;
        Ok(Expression::Integer {
            value_span: token.span,
        })
    }

    fn expect_keyword(
        &mut self,
        kind: KeywordKind,
    ) -> Result<WideToken, ParserError> {
        let wide_token_kind = WideTokenKind::Keyword(kind);
        self.expect(wide_token_kind)
    }

    /// Проверяет, что следующий токен равен ожидаемому, и возрвращает полученый токен.
    fn expect(
        &mut self,
        kind: WideTokenKind,
    ) -> Result<WideToken, ParserError> {
        if let Some(next) = self.token_stream.next() {
            if next.kind == kind {
                return Ok(next);
            }
            return Err(ParserError::UnexpectedToken {
                expected: kind,
                got: next.kind,
            });
        };
        Err(ParserError::UnexpectedEof)
    }
}
