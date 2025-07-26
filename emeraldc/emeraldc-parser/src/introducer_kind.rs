use emeraldc_lexer::WideToken;

/// Kind of a construct introducer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntroducerKind {
    Declaration,
    Statement,
    /// Identifier might not be an expression, but we don't care about it, because we're simply
    /// detecting possible expression tokens.
    Expression,
    /// Other variants that don't fall in categories above.
    ///
    /// Could be errors, literals and other non-introducer tokens.
    Other,
}

impl From<&WideToken> for IntroducerKind {
    fn from(token_kind: &WideToken) -> Self {
        match token_kind {
            WideToken::FunctionKeyword => Self::Declaration,
            WideToken::LetKeyword => Self::Statement,
            WideToken::Identifier
            | WideToken::Integer
            | WideToken::OpenRound => Self::Expression,
            _ => Self::Other,
        }
    }
}
