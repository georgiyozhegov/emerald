use emeraldc_lexer::WideToken;
use emeraldc_span::IntoSpanned;

use crate::{
    Declaration, FatalParserError, Function, IntroducerKind, Parsed, Parser,
    Statement, Subparser, span_from_parsed,
};

pub struct DeclarationParser<'p> {
    parser: &'p mut Parser,
}

impl<'p> Subparser<'p, Declaration> for DeclarationParser<'p> {
    fn parse(
        parser: &'p mut Parser,
    ) -> Result<Parsed<Declaration>, FatalParserError> {
        let this = Self::new(parser);
        this.parse()
    }
}

impl<'p> DeclarationParser<'p> {
    fn new(parser: &'p mut Parser) -> Self {
        Self { parser }
    }

    fn parse(mut self) -> Result<Parsed<Declaration>, FatalParserError> {
        match self.parser.token_introducer_kind() {
            IntroducerKind::Declaration => self.parse_unchecked(),
            _ => self.invalid_introducer(),
        }
    }

    fn invalid_introducer<T>(&mut self) -> Result<T, FatalParserError> {
        let token = self.parser.tokens.next().unwrap();
        Err(FatalParserError::InvalidDeclarationIntroducer(token.value))
    }

    fn parse_unchecked(self) -> Result<Parsed<Declaration>, FatalParserError> {
        match self.parser.tokens.peek().unwrap().value {
            WideToken::FunctionKeyword => self.parse_function(),
            _ => Err(FatalParserError::CompilerBug("unreachable variant")),
        }
    }

    fn parse_function(
        mut self,
    ) -> Result<Parsed<Declaration>, FatalParserError> {
        let _introducer = self.parser.expect(WideToken::FunctionKeyword)?;
        let introducer_span = span_from_parsed(&_introducer);
        let identifier = self.parser.parse_identifier()?;
        let _open_round = self.parser.expect(WideToken::OpenRound)?;
        let _close_round = self.parser.expect(WideToken::CloseRound)?;
        let body = self.parse_function_body()?;
        let _end = self.parser.expect(WideToken::EndKeyword)?;
        let end_span = span_from_parsed(&_end);
        let function = Function {
            _introducer,
            identifier,
            _open_round,
            _close_round,
            body,
            _end,
        };
        let span = introducer_span.join(end_span);
        let parsed = Ok(Declaration::Function(function).into_spanned(span));
        Ok(parsed)
    }

    fn parse_function_body(
        &mut self,
    ) -> Result<Vec<Parsed<Statement>>, FatalParserError> {
        let mut body = Vec::new();
        while !self.is_function_body_end() {
            let statement = self.parser.parse_statement()?;
            body.push(statement);
        }
        Ok(body)
    }

    fn is_function_body_end(&mut self) -> bool {
        self.parser
            .tokens
            .peek()
            .is_some_and(|t| t.value == WideToken::EndKeyword)
    }
}
