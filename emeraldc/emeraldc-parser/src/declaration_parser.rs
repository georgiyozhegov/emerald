use emeraldc_lexer::WideTokenKind;

// i'm proud of this parser

use crate::{
    Declaration, FatalParserError, Function, IntroducerKind, ParsedNode,
    Parser, Statement, Subparser,
};

pub struct DeclarationParser<'p> {
    parser: &'p mut Parser,
}

impl<'p> Subparser<'p, Declaration> for DeclarationParser<'p> {
    fn parse(
        parser: &'p mut Parser,
    ) -> Result<ParsedNode<Declaration>, FatalParserError> {
        let this = Self::new(parser);
        this.parse()
    }
}

impl<'p> DeclarationParser<'p> {
    fn new(parser: &'p mut Parser) -> Self {
        Self { parser }
    }

    fn parse(mut self) -> Result<ParsedNode<Declaration>, FatalParserError> {
        match self.parser.token_introducer_kind() {
            IntroducerKind::Declaration => self.parse_unchecked(),
            _ => self.invalid_introducer(),
        }
    }

    fn invalid_introducer<T>(&mut self) -> Result<T, FatalParserError> {
        self.parser.tokens.next();
        Err(FatalParserError::InvalidDeclarationIntroducer)
    }

    fn parse_unchecked(
        self,
    ) -> Result<ParsedNode<Declaration>, FatalParserError> {
        match self.parser.tokens.peek().unwrap().kind {
            WideTokenKind::FunctionKeyword => self.parse_function(),
            _ => Err(FatalParserError::CompilerBug("unreachable variant")),
        }
    }

    fn parse_function(
        mut self,
    ) -> Result<ParsedNode<Declaration>, FatalParserError> {
        let _introducer = self.parser.expect(WideTokenKind::FunctionKeyword)?;
        let introducer_span = _introducer.span.clone();
        let identifier = self.parser.parse_identifier()?;
        let _open_round = self.parser.expect(WideTokenKind::OpenRound)?;
        let _close_round = self.parser.expect(WideTokenKind::CloseRound)?;
        let body = self.parse_function_body()?;
        let _end = self.parser.expect(WideTokenKind::EndKeyword)?;
        let end_span = _end.span.clone();
        let function = Function {
            _introducer,
            identifier,
            _open_round,
            _close_round,
            body,
            _end,
        };
        let node = Ok(Declaration::Function(function));
        let span = introducer_span.join(end_span);
        Ok(ParsedNode::new(node, span))
    }

    fn parse_function_body(
        &mut self,
    ) -> Result<Vec<ParsedNode<Statement>>, FatalParserError> {
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
            .is_some_and(|t| t.kind == WideTokenKind::EndKeyword)
    }
}
