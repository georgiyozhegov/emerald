use crate::{DeclarationNode, DummyToken, Parser, ParserError};

/// Parses declarations.
#[derive(Debug)]
pub struct DeclarationParser<'p> {
    parser: &'p mut Parser,
}

impl<'p> DeclarationParser<'p> {
    pub fn new(parser: &'p mut Parser) -> Self {
        Self { parser }
    }

    /// Parse a single declaration.
    pub fn parse(mut self) -> Result<DeclarationNode, ParserError> {
        match self.parser.source.peek()? {
            DummyToken::Function => self.parse_function(),
            got => {
                self.parser.source.next()?;
                log::error!("expected declaration, got {got:?}");
                let error = ParserError::ExpectedDeclaration { got };
                Err(error)
            }
        }
    }

    /// Parse a function declaration.
    fn parse_function(&mut self) -> Result<DeclarationNode, ParserError> {
        self.parser.expect(DummyToken::Function)?;
        let identifier = self.parser.parse_identifier()?;
        self.parser.expect(DummyToken::OpenRound)?;
        self.parser.expect(DummyToken::CloseRound)?;
        let body = self.parser.parse_body()?;
        let node = DeclarationNode::Function { identifier, body };
        log::trace!("function: {node:?}");
        Ok(node)
    }
}
