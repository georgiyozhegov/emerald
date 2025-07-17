use emeraldc_lexer::WideTokenKind;

// i'm proud of this parser

use crate::{
    Binary, BinaryOperator, Expression, FatalParserError, IntroducerKind, NodeError, Parenthesized, ParsedNode, Parser, Subparser
};

pub struct ExpressionParser<'p> {
    parser: &'p mut Parser,
}

impl<'p> Subparser<'p, Expression> for ExpressionParser<'p> {
    fn parse(
        parser: &'p mut Parser,
    ) -> Result<ParsedNode<Expression>, FatalParserError> {
        let this = Self::new(parser);
        this.parse()
    }
}

impl<'p> ExpressionParser<'p> {
    fn new(parser: &'p mut Parser) -> Self {
        Self { parser }
    }

    fn parse(mut self) -> Result<ParsedNode<Expression>, FatalParserError> {
        self.parse_with_precedence(0)
    }

    fn parse_with_precedence(
        &mut self,
        minimal_precedence: u8,
    ) -> Result<ParsedNode<Expression>, FatalParserError> {
        let mut left = self.parse_primary()?;
        while let Some(operator) = self.peek_binary_operator() {
            let (left_precedence, right_precedence) = operator.precedence();
            if left_precedence < minimal_precedence {
                break;
            }
            let operator = self.parse_binary_operator(operator)?;
            let right = self.parse_with_precedence(right_precedence)?;
            let span = left.span.clone().join(right.span.clone());
            let binary = Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
            let node = Ok(Expression::Binary(binary));
            left = ParsedNode::new(node, span);
        }
        Ok(left)
    }

    fn peek_binary_operator(&mut self) -> Option<BinaryOperator> {
        let token = self.parser.tokens.peek();
        token.and_then(|t| BinaryOperator::from_token(&t.kind))
    }

    fn parse_binary_operator(
        &mut self,
        peeked_operator: BinaryOperator,
    ) -> Result<ParsedNode<BinaryOperator>, FatalParserError> {
        let token = self.parser.tokens.next().unwrap();
        Ok(ParsedNode::new(Ok(peeked_operator), token.span))
    }

    fn parse_primary(
        &mut self,
    ) -> Result<ParsedNode<Expression>, FatalParserError> {
        match self.parser.token_introducer_kind() {
            IntroducerKind::Expression => self.parse_primary_unchecked(),
            _ => self.invalid_primary(),
        }
    }

    fn invalid_primary(
        &mut self,
    ) -> Result<ParsedNode<Expression>, FatalParserError> {
        match self.parser.tokens.next() {
            Some(token) => {
                let error = Err(NodeError::UnexpectedToken(token.kind));
                Ok(ParsedNode::new(error, token.span))
            }
            None => Err(FatalParserError::UnexpectedEof),
        }
    }

    fn parse_primary_unchecked(
        &mut self,
    ) -> Result<ParsedNode<Expression>, FatalParserError> {
        match self.parser.tokens.peek() {
            Some(token) if token.kind == WideTokenKind::Integer => {
                let token = self.parser.tokens.next().unwrap();
                let node = Ok(Expression::Integer);
                Ok(ParsedNode::new(node, token.span))
            }
            Some(token) if token.kind == WideTokenKind::Identifier => {
                let identifier = self.parser.parse_identifier()?;
                let node =
                    identifier.node.and_then(|n| Ok(Expression::Variable(n)));
                Ok(ParsedNode::new(node, identifier.span))
            }
            Some(token) if token.kind == WideTokenKind::OpenRound => {
                self.parse_parenthesized()
            }
            _ => Err(FatalParserError::CompilerBug("unreachable variant")),
        }
    }

    fn parse_parenthesized(
        &mut self,
    ) -> Result<ParsedNode<Expression>, FatalParserError> {
        let _open_round = self.parser.expect(WideTokenKind::OpenRound)?;
        let inner = self.parser.parse_expression()?;
        let _close_round = self.parser.expect(WideTokenKind::CloseRound)?;
        let span = _open_round.span.clone().join(_close_round.span.clone());
        let parenthesized = Parenthesized {
            _open_round,
            inner: Box::new(inner),
            _close_round,
        };
        let node = Ok(Expression::Parenthesized(parenthesized));
        Ok(ParsedNode::new(node, span))
    }
}
