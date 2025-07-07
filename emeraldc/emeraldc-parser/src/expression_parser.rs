use emeraldc_lexer::Token;
use log::trace;

use crate::{
    BinaryOperatorNode, DummyToken, ExpressionNode, Parser, ParserError,
};

/// Parses expressions.
#[derive(Debug)]
pub struct ExpressionParser<'p> {
    parser: &'p mut Parser,
}

impl<'p> ExpressionParser<'p> {
    pub fn new(parser: &'p mut Parser) -> Self {
        Self { parser }
    }

    /// Parse a single expression.
    pub fn parse(mut self) -> Result<ExpressionNode, ParserError> {
        self.parse_comparison()
    }

    /// Parse a single comparison binary expression.
    fn parse_comparison(&mut self) -> Result<ExpressionNode, ParserError> {
        let mut node = self.parse_term()?;
        while let Some(operator) =
            BinaryOperatorFactory::from_comparison(self.parser.source.peek()?)
        {
            self.parser.source.next().unwrap(); // always ok
            let right = self.parse_term()?;
            node = ExpressionNode::Binary {
                operator,
                left: Box::new(node),
                right: Box::new(right),
            };
        }
        log::trace!("comparison: {node:?}");
        Ok(node)
    }

    /// Parse a single term binary expression.
    fn parse_term(&mut self) -> Result<ExpressionNode, ParserError> {
        let mut node = self.parse_factor()?;
        while let Some(operator) =
            BinaryOperatorFactory::from_term(self.parser.source.peek()?)
        {
            self.parser.source.next().unwrap(); // always ok
            let right = self.parse_factor()?;
            node = ExpressionNode::Binary {
                operator,
                left: Box::new(node),
                right: Box::new(right),
            };
        }
        log::trace!("term: {node:?}");
        Ok(node)
    }

    /// Parse a single factor binary expression.
    fn parse_factor(&mut self) -> Result<ExpressionNode, ParserError> {
        let mut node = self.parse_primary()?;
        while let Some(operator) =
            BinaryOperatorFactory::from_factor(self.parser.source.peek()?)
        {
            self.parser.source.next().unwrap(); // always ok
            let right = self.parse_primary()?;
            node = ExpressionNode::Binary {
                operator,
                left: Box::new(node),
                right: Box::new(right),
            };
        }
        log::trace!("factor: {node:?}");
        Ok(node)
    }

    fn parse_primary(&mut self) -> Result<ExpressionNode, ParserError> {
        match self.parser.source.peek()? {
            DummyToken::Name => {
                let identifier = self.parser.parse_identifier()?;
                Ok(ExpressionNode::Identifier(identifier))
            }
            DummyToken::Integer => {
                let Token::Integer(value) = self.parser.source.next()? else {
                    unreachable!()
                };
                Ok(ExpressionNode::Integer(value))
            }
            got => {
                log::error!("expected identifier or literal, got {got:?}");
                let error = ParserError::UnexpectedTokenStr {
                    expected: "identifier or literal",
                    got,
                };
                Err(error)
            }
        }
    }
}

struct BinaryOperatorFactory;

impl BinaryOperatorFactory {
    pub fn from_comparison(token: DummyToken) -> Option<BinaryOperatorNode> {
        match token {
            DummyToken::Equal => Some(BinaryOperatorNode::Equal),
            DummyToken::RightAngle => Some(BinaryOperatorNode::Greater),
            DummyToken::LeftAngle => Some(BinaryOperatorNode::Less),
            _ => None,
        }
    }

    pub fn from_term(token: DummyToken) -> Option<BinaryOperatorNode> {
        match token {
            DummyToken::Star => Some(BinaryOperatorNode::Multiply),
            DummyToken::Slash => Some(BinaryOperatorNode::Divide),
            _ => None,
        }
    }

    pub fn from_factor(token: DummyToken) -> Option<BinaryOperatorNode> {
        match token {
            DummyToken::Plus => Some(BinaryOperatorNode::Add),
            DummyToken::Minus => Some(BinaryOperatorNode::Subtract),
            _ => None,
        }
    }
}
