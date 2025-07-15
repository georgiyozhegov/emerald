use emeraldc_lexer::WideTokenKind;

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

impl From<&WideTokenKind> for IntroducerKind {
    fn from(token_kind: &WideTokenKind) -> Self {
        match token_kind {
            WideTokenKind::FunctionKeyword => Self::Declaration,
            WideTokenKind::LetKeyword => Self::Statement,
            WideTokenKind::Identifier | WideTokenKind::Integer => {
                Self::Expression
            }
            _ => Self::Other,
        }
    }
}
