use std::iter::Peekable;
use std::vec;

use crate::{
    CharClassifier, CharGroup, Token, TokenFactory, TokenFactoryError,
};

pub type LexerSource = Peekable<vec::IntoIter<char>>;
pub type LexerOutput = Vec<Result<Token, LexerError>>;

/// Splits a source buffer into tokens.
pub struct Lexer {
    source: LexerSource,
}

impl Lexer {
    pub fn new(iter: vec::IntoIter<char>) -> Self {
        let peekable = iter.peekable();
        Self { source: peekable }
    }

    /// Perform lexing.
    pub fn lex(mut self) -> LexerOutput {
        let mut output = Vec::new();
        while let Some(token) = self.lex_token() {
            output.push(token);
        }
        output
    }

    /// Lex a single token.
    fn lex_token(&mut self) -> Option<Result<Token, LexerError>> {
        self.skip_invisible();
        let next = self.source.peek()?;
        let group = CharClassifier::group(next);
        let token = match group {
            CharGroup::Alphabetic => self.lex_alphabetic(),
            CharGroup::Numeric => self.lex_numeric(),
            CharGroup::MaybePunctuation => self.lex_punctuation(),
            _ => todo!(),
        };
        Some(token)
    }

    /// Skip invisible characters.
    fn skip_invisible(&mut self) {
        let _invisible = self.take_while(|c| {
            let group = CharClassifier::group(c);
            matches!(group, CharGroup::Invisible)
        });
    }

    /// Lex an alphabetic token.
    fn lex_alphabetic(&mut self) -> Result<Token, LexerError> {
        let buffer = self.take_while(|c| {
            let group = CharClassifier::group(c);
            matches!(group, CharGroup::Alphabetic | CharGroup::Numeric)
        });
        let token = TokenFactory::from_alphabetic(buffer);
        Ok(token)
    }

    /// Lex a numeric token.
    fn lex_numeric(&mut self) -> Result<Token, LexerError> {
        let buffer = self.take_while(|c| {
            let group = CharClassifier::group(c);
            matches!(group, CharGroup::Numeric)
        });
        let token = TokenFactory::from_numeric(buffer)
            .map_err(|e| LexerError::TokenFactory(e))?;
        Ok(token)
    }

    /// Lex a punctuation token.
    fn lex_punctuation(&mut self) -> Result<Token, LexerError> {
        let c = self.source.peek().unwrap(); // always is some
        let token = TokenFactory::from_punctuation(c)
            .map_err(|e| LexerError::TokenFactory(e))?;
        self.source.next().unwrap(); // skip current
        Ok(token)
    }

    /// Collect characters in a buffer while predicate is true.
    fn take_while(&mut self, predicate: impl Fn(&char) -> bool) -> String {
        let mut buffer = String::new();
        while let Some(c) = self.source.next_if(&predicate) {
            buffer.push(c);
        }
        buffer
    }
}

#[derive(Debug, Clone)]
pub enum LexerError {
    TokenFactory(TokenFactoryError),
}
