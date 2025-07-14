use std::iter::Peekable;

use emeraldc_lexer::{WideToken, WideTokenKind};

use crate::error::ParserError;

/// Stream of tokens used by the parser.
#[derive(Debug)]
pub struct TokenStream {
    iter: Peekable<std::vec::IntoIter<WideToken>>,
    previous: Option<Result<WideToken, ParserError>>,
}

impl TokenStream {
    pub fn new(iter: impl Iterator<Item = WideToken>) -> Self {
        let iter = iter.filter(|t| t.kind != WideTokenKind::Invisible);
        let iter = iter.collect::<Vec<_>>().into_iter();
        let iter = iter.peekable();
        Self {
            iter,
            previous: None,
        }
    }

    /// Takes the next token from the input iterator.
    ///
    /// Also, see [`Self::adapted_token`].
    pub fn next(&mut self) -> Result<WideToken, ParserError> {
        let token = Self::adapted_token(self.iter.next());
        self.previous = Some(token.clone());
        token
    }

    /// Takes the next token from the input iterator without consuming it.
    ///
    /// If it encounters an error, it will consume the error token to avoid infinite loop.
    ///
    /// Also, see [`Self::adapted_token`].
    pub fn peek(&mut self) -> Result<WideToken, ParserError> {
        let token = Self::adapted_token(self.iter.peek().cloned());
        if token.is_err() {
            return self.next();
        }
        token
    }

    /// Adapts token for parser needs.
    ///
    /// Handles unexpected EOF and converts [`WideTokenKind::HadError`] variant of token into
    /// result with parser error.
    fn adapted_token(
        token: Option<WideToken>,
    ) -> Result<WideToken, ParserError> {
        let token = Self::check_eof(token)?;
        Self::map_had_error(token)
    }

    /// Basically, returns an error if token is none.
    fn check_eof(option: Option<WideToken>) -> Result<WideToken, ParserError> {
        match option {
            Some(token) => Ok(token),
            None => Err(ParserError::UnexpectedEof),
        }
    }

    /// Maps the [`WideTokenKind::HadError`] variant to a result.
    fn map_had_error(token: WideToken) -> Result<WideToken, ParserError> {
        match token.kind {
            WideTokenKind::HadError(error) => Err(ParserError::Lexer(error)),
            _ => Ok(token),
        }
    }

    /// Checks if next token is none.
    pub fn is_eof(&mut self) -> bool {
        self.iter.peek().is_none()
    }

    /// Takes the previous saved token.
    ///
    /// Cannot be called twice for the same token!
    pub fn take_previous(&mut self) -> Result<WideToken, ParserError> {
        self.previous.take().unwrap()
    }
}
