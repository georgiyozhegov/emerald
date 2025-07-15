use emeraldc_lexer::WideTokenKind;

// i'm proud of this parser

use crate::{
    FatalParserError, IntroducerKind, NodeError, ParsedNode, Parser, Statement,
    Subparser,
};

pub struct StatementParser<'p> {
    parser: &'p mut Parser,
}

impl<'p> Subparser<'p, Statement> for StatementParser<'p> {
    fn parse(
        parser: &'p mut Parser,
    ) -> Result<ParsedNode<Statement>, FatalParserError> {
        let this = Self::new(parser);
        this.parse()
    }
}

impl<'p> StatementParser<'p> {
    fn new(parser: &'p mut Parser) -> Self {
        Self { parser }
    }

    fn parse(mut self) -> Result<ParsedNode<Statement>, FatalParserError> {
        match self.parser.token_introducer_kind() {
            IntroducerKind::Statement => self.parse_unchecked(),
            _ => self.invalid_introducer(),
        }
    }

    fn invalid_introducer(
        &mut self,
    ) -> Result<ParsedNode<Statement>, FatalParserError> {
        let token = self.parser.tokens.next().unwrap();
        let error = Err(NodeError::InvalidStatementIntroducer(token.kind));
        Ok(ParsedNode::new(error, token.span))
    }

    fn parse_unchecked(
        self,
    ) -> Result<ParsedNode<Statement>, FatalParserError> {
        match self.parser.tokens.peek().unwrap().kind {
            WideTokenKind::LetKeyword => self.parse_let(),
            _ => Err(FatalParserError::CompilerBug("unreachable variant")),
        }
    }

    fn parse_let(self) -> Result<ParsedNode<Statement>, FatalParserError> {
        let _introducer = self.parser.expect(WideTokenKind::LetKeyword)?;
        let introducer_span = _introducer.span.clone(); // todo: full span
        let identifier = self.parser.parse_identifier()?;
        let _equal = self.parser.expect(WideTokenKind::Equal)?;
        let value = self.parser.parse_expression()?;
        let value_span = value.span.clone();
        let node = Ok(Statement::Let {
            _introducer,
            identifier,
            _equal,
            value,
        });
        let span = introducer_span.join(value_span);
        Ok(ParsedNode::new(node, span))
    }
}
