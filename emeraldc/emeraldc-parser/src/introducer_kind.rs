use emeraldc_lexer::WideTokenKind;

/// Kind of a construct introducer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntroducerKind {
    Declaration,
    Statement,
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
            _ => Self::Other,
        }
    }
}
