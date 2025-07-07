use crate::{DummyToken, Parser, ParserError, StatementNode};

/// Parses statements.
#[derive(Debug)]
pub struct StatementParser<'p> {
    parser: &'p mut Parser,
}

impl<'p> StatementParser<'p> {
    pub fn new(parser: &'p mut Parser) -> Self {
        Self { parser }
    }

    /// Parse a single statement.
    pub fn parse(mut self) -> Result<StatementNode, ParserError> {
        match self.parser.source.peek()? {
            DummyToken::Let => self.parse_let(),
            got => {
                let error = ParserError::ExpectedStatement { got };
                Err(error)
            }
        }
    }

    /// Parse a let statement.
    fn parse_let(&mut self) -> Result<StatementNode, ParserError> {
        self.parser.expect(DummyToken::Let)?;
        let identifier = self.parser.parse_identifier()?;
        self.parser.expect(DummyToken::Equal)?;
        let value = self.parser.parse_expression()?;
        let node = StatementNode::Let { identifier, value };
        Ok(node)
    }
}
