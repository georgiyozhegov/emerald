use emeraldc_tokenizer::{Token, TokenKind};

use crate::{
    LexerError,
    wide_token::{Span, WideToken, WideTokenKind},
};

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
    ) -> impl Iterator<Item = WideToken> {
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
    fn wide_token(&mut self, thin_token: Token) -> WideToken {
        let span = self.span(thin_token.length);
        let kind = self.wide_kind(thin_token.kind, &span);
        WideToken::new(kind, span)
    }

    fn wide_kind(
        &mut self,
        thin_kind: TokenKind,
        span: &Span,
    ) -> WideTokenKind {
        match thin_kind {
            TokenKind::IdentifierOrKeyword => {
                self.identifier_or_keyword_wide_kind(span)
            }
            TokenKind::Unknown => self.unknown_wide_kind(),
            same => self.same_wide_kind(same),
        }
    }

    fn identifier_or_keyword_wide_kind(
        &mut self,
        span: &Span,
    ) -> WideTokenKind {
        let lexeme: &str = &self.source[span.start..span.end];
        if let Some(keyword_kind) = self.maybe_keyword(lexeme) {
            keyword_kind
        } else {
            WideTokenKind::Identifier
        }
    }

    fn maybe_keyword(&self, lexeme: &str) -> Option<WideTokenKind> {
        match lexeme {
            "function" => Some(WideTokenKind::FunctionKeyword),
            "end" => Some(WideTokenKind::EndKeyword),
            "let" => Some(WideTokenKind::LetKeyword),
            _ => None,
        }
    }

    fn unknown_wide_kind(&mut self) -> WideTokenKind {
        let error = LexerError::UnknownCharacter;
        WideTokenKind::HadError(error)
    }

    /// Конвертирует типы, которые одинаковы и в токенизаторе, и в лексере.
    fn same_wide_kind(&self, thin_kind: TokenKind) -> WideTokenKind {
        match thin_kind {
            TokenKind::Integer => WideTokenKind::Integer,
            TokenKind::OpenRound => WideTokenKind::OpenRound,
            TokenKind::CloseRound => WideTokenKind::CloseRound,
            TokenKind::Equal => WideTokenKind::Equal,
            TokenKind::Invisible => WideTokenKind::Invisible,
            _ => unreachable!(),
        }
    }

    /// Создает спан и обновляет конец токена.
    fn span(&mut self, token_length: usize) -> Span {
        let start = self.previous_token_end;
        let end = start + token_length;
        self.previous_token_end = end;
        Span::new(start, end)
    }
}
