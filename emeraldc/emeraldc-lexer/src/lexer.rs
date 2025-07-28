use emeraldc_span::{IntoSpanned, Span, Spanned};
use emeraldc_tokenizer::{Token, TokenKind};

use crate::{LexerError, WideToken};

/// Лексер.
///
/// Дополняет "тонкие" токены, производя полные, то есть "широкие" токены.
pub struct Lexer<'s> {
    source: &'s str,
    previous_token_end: usize,
}

impl<'s> Lexer<'s> {
    pub fn lex(
        source: &'s str,
        token_stream: impl Iterator<Item = Token>,
    ) -> impl Iterator<Item = Spanned<WideToken>> {
        let mut lexer = Self::new(source);
        token_stream.map(move |token| lexer.wide_token(token))
    }

    fn new(source: &'s str) -> Self {
        Self {
            source,
            previous_token_end: 0, // начало файла
        }
    }

    /// Создает полный токен из исходного.
    fn wide_token(&mut self, thin_token: Token) -> Spanned<WideToken> {
        let span = self.span(thin_token.length);
        let token = self.wide_kind(thin_token.kind, &span);
        token.into_spanned(span)
    }

    fn wide_kind(&mut self, thin_kind: TokenKind, span: &Span) -> WideToken {
        match thin_kind {
            TokenKind::IdentifierOrKeyword => {
                self.identifier_or_keyword_wide_kind(span)
            }
            TokenKind::Unknown => self.unknown_wide_kind(),
            same => self.same_wide_kind(same),
        }
    }

    fn identifier_or_keyword_wide_kind(&mut self, span: &Span) -> WideToken {
        let lexeme: &str = &self.source[span.start..span.end];
        if let Some(keyword) = self.maybe_keyword(lexeme) {
            keyword
        } else {
            WideToken::Identifier
        }
    }

    fn maybe_keyword(&self, lexeme: &str) -> Option<WideToken> {
        match lexeme {
            "function" => Some(WideToken::FunctionKeyword),
            "end" => Some(WideToken::EndKeyword),
            "let" => Some(WideToken::LetKeyword),
            _ => None,
        }
    }

    fn unknown_wide_kind(&mut self) -> WideToken {
        let error = LexerError::UnknownCharacter;
        WideToken::HadError(error)
    }

    /// Конвертирует типы, которые одинаковы и в токенизаторе, и в лексере.
    fn same_wide_kind(&self, thin_kind: TokenKind) -> WideToken {
        match thin_kind {
            TokenKind::Integer => WideToken::Integer,
            TokenKind::OpenRound => WideToken::OpenRound,
            TokenKind::CloseRound => WideToken::CloseRound,
            TokenKind::Equal => WideToken::Equal,
            TokenKind::Plus => WideToken::Plus,
            TokenKind::Minus => WideToken::Minus,
            TokenKind::Asterisk => WideToken::Asterisk,
            TokenKind::Slash => WideToken::Slash,
            TokenKind::Invisible => WideToken::Invisible,
            TokenKind::Comment => WideToken::Comment,
            _ => unreachable!(),
        }
    }

    fn span(&mut self, token_length: usize) -> Span {
        let start = self.previous_token_end;
        let end = start + token_length;
        self.previous_token_end = end;
        Span::new(start, end)
    }
}
