use emeraldc_lexer::WideToken;
use emeraldc_span::IntoSpanned;

use crate::{
    FatalParserError, IntroducerKind, Let, NodeError, Parsed, Parser,
    Statement, Subparser, span_from_parsed,
};

pub struct StatementParser<'p> {
    parser: &'p mut Parser,
}

impl<'p> Subparser<'p, Statement> for StatementParser<'p> {
    fn parse(
        parser: &'p mut Parser,
    ) -> Result<Parsed<Statement>, FatalParserError> {
        let this = Self::new(parser);
        this.parse()
    }
}

impl<'p> StatementParser<'p> {
    fn new(parser: &'p mut Parser) -> Self {
        Self { parser }
    }

    fn parse(mut self) -> Result<Parsed<Statement>, FatalParserError> {
        match self.parser.token_introducer_kind() {
            IntroducerKind::Statement => self.parse_unchecked(),
            _ => self.invalid_introducer(),
        }
    }

    fn invalid_introducer(
        &mut self,
    ) -> Result<Parsed<Statement>, FatalParserError> {
        match self.parser.tokens.next() {
            Some(token) if token.value.had_error() => {
                let error = Err(NodeError::Lexer(token.value.as_error())
                    .into_spanned(token.span));
                Ok(error)
            }
            Some(token) => {
                let error =
                    Err(NodeError::InvalidStatementIntroducer(token.value)
                        .into_spanned(token.span));
                Ok(error)
            }
            None => Err(FatalParserError::UnexpectedEof),
        }
    }

    fn parse_unchecked(self) -> Result<Parsed<Statement>, FatalParserError> {
        match self.parser.tokens.peek().unwrap().value {
            WideToken::LetKeyword => self.parse_let(),
            _ => Err(FatalParserError::CompilerBug("unreachable variant")),
        }
    }

    fn parse_let(self) -> Result<Parsed<Statement>, FatalParserError> {
        let _introducer = self.parser.expect(WideToken::LetKeyword)?;
        let introducer_span = span_from_parsed(&_introducer); // todo: full span
        let identifier = self.parser.parse_identifier()?;
        let _equal = self.parser.expect(WideToken::Equal)?;
        let value = self.parser.parse_expression()?;
        let value_span = span_from_parsed(&value);
        let let_ = Let {
            _introducer,
            identifier,
            _equal,
            value,
        };
        let span = introducer_span.join(value_span);
        let parsed = Ok(Statement::Let(let_).into_spanned(span));
        Ok(parsed)
    }
}
