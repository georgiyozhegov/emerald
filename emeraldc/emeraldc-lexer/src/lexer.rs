use crate::{
    CharClassifier, CharGroup, SourceBuffer, Token, TokenFactory,
    TokenFactoryError,
};

pub struct Lexer {
    source: SourceBuffer,
}

// initialization
impl Lexer {
    pub fn new(source: SourceBuffer) -> Self {
        Self { source }
    }
}

// iterator implementation
impl Iterator for Lexer {
    type Item = Result<Token, LexerError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.maybe_token()
    }
}

// logic
impl Lexer {
    fn maybe_token(&mut self) -> Option<Result<Token, LexerError>> {
        let c = self.source.current();
        if !CharClassifier::group(c).is_eof() {
            let token = self.lex_token();
            Some(token)
        } else {
            None
        }
    }

    fn lex_token(&mut self) -> Result<Token, LexerError> {
        self.skip_invisible();
        let c = self.source.current();
        let group = CharClassifier::group(c);
        match group {
            CharGroup::Alphabetic => self.lex_alphabetic(),
            CharGroup::Numeric => self.lex_numeric(),
            CharGroup::MaybePunctuation => self.lex_punctuation(),
            CharGroup::Invisible | CharGroup::Eof => unreachable!(),
        }
    }

    fn skip_invisible(&mut self) {
        let _invisible = self.take_while(|c| {
            let group = CharClassifier::group(c);
            matches!(group, CharGroup::Invisible)
        });
    }

    fn lex_alphabetic(&mut self) -> Result<Token, LexerError> {
        let buffer = self.take_while(|c| {
            let group = CharClassifier::group(c);
            matches!(group, CharGroup::Alphabetic | CharGroup::Numeric)
        });
        let token = TokenFactory::from_alphabetic(buffer);
        Ok(token)
    }

    fn lex_numeric(&mut self) -> Result<Token, LexerError> {
        let buffer = self.take_while(|c| {
            let group = CharClassifier::group(c);
            matches!(group, CharGroup::Numeric)
        });
        let token = TokenFactory::from_numeric(buffer)
            .map_err(|e| LexerError::TokenFactory(e))?;
        Ok(token)
    }

    fn lex_punctuation(&mut self) -> Result<Token, LexerError> {
        let c = self.source.eat();
        let token = TokenFactory::from_punctuation(c)
            .map_err(|e| LexerError::TokenFactory(e))?;
        Ok(token)
    }

    fn take_while(&mut self, predicate: impl Fn(char) -> bool) -> &str {
        let start = self.source.cursor();
        while predicate(self.source.current()) {
            self.source.advance();
        }
        let end = self.source.cursor();
        self.source.substring(start, end)
    }
}

#[derive(Debug, Clone)]
pub enum LexerError {
    TokenFactory(TokenFactoryError),
}
