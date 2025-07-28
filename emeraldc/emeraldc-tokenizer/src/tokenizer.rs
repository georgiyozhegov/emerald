use crate::{ChGroup, SourceBuffer, Token, TokenKind};

/// Токенизатор.
pub struct Tokenizer<'s> {
    source_buffer: SourceBuffer<'s>,
}

impl<'s> Tokenizer<'s> {
    /// Разделяет входную строку на отдельные токены.
    pub fn tokenize(source: &'s str) -> impl Iterator<Item = Token> {
        let mut tokenizer = Self::new(source);
        std::iter::from_fn(move || tokenizer.maybe_token())
    }
}

impl<'s> Tokenizer<'s> {
    fn new(source: &'s str) -> Self {
        let source_buffer = SourceBuffer::new(source);
        Self { source_buffer }
    }

    /// Вернёт токен, если в итераторе остались символы.
    fn maybe_token(&mut self) -> Option<Token> {
        self.source_buffer.peek()?;
        Some(self.token())
    }

    /// Извлекает один токен из входной строки.
    fn token(&mut self) -> Token {
        let ch = self.source_buffer.peek().unwrap(); // всегда имеет значение
        let group = ChGroup::from(ch);
        self.token_starts_with(group)
    }

    /// Извлекает токен из входной строки, основываясь на группе текущего символа.
    fn token_starts_with(&mut self, group: ChGroup) -> Token {
        match group {
            ChGroup::Alphabetic => self.identifier_or_keyword_token(),
            ChGroup::Numeric => self.integer_token(),
            ChGroup::Invisible { .. } => self.invisible_token(),
            ChGroup::Comment => self.comment(),
            ChGroup::MaybePunctuation => self.punctuation_or_unknown_token(),
        }
    }

    fn identifier_or_keyword_token(&mut self) -> Token {
        self.long_token_with_tracked_length(
            TokenKind::IdentifierOrKeyword,
            |cg| matches!(cg, ChGroup::Alphabetic),
        )
    }

    fn integer_token(&mut self) -> Token {
        self.long_token_with_tracked_length(TokenKind::Integer, |cg| {
            matches!(cg, ChGroup::Numeric)
        })
    }

    fn invisible_token(&mut self) -> Token {
        self.long_token_with_tracked_length(TokenKind::Invisible, |cg| {
            matches!(cg, ChGroup::Invisible { .. })
        })
    }

    fn comment(&mut self) -> Token {
        self.long_token_with_tracked_length(TokenKind::Comment, |cg| {
            !matches!(cg, ChGroup::Invisible { newline: true })
        })
    }
    /// Создаёт токен, включая в него длину.
    fn long_token_with_tracked_length(
        &mut self,
        kind: TokenKind,
        eat_while_predicate: impl Fn(ChGroup) -> bool,
    ) -> Token {
        self.source_buffer.mark_token_start();
        self.source_buffer.eat_while(&eat_while_predicate);
        Token::new(kind, self.source_buffer.token_length())
    }

    fn punctuation_or_unknown_token(&mut self) -> Token {
        self.source_buffer.mark_token_start();
        let kind = self.punctuation_or_unknown_token_kind();
        Token::new(kind, self.source_buffer.token_length())
    }

    fn punctuation_or_unknown_token_kind(&mut self) -> TokenKind {
        match self.source_buffer.eat() {
            '(' => TokenKind::OpenRound,
            ')' => TokenKind::CloseRound,
            '=' => TokenKind::Equal,
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Asterisk,
            '/' => TokenKind::Slash,
            _ => TokenKind::Unknown,
        }
    }
}
