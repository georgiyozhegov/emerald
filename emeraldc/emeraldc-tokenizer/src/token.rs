/// Токен.
///
/// Содержит свой тип и длину.
///
/// Подразумевается, что для получения текста токена нужно вычислить спаны и получить подстроки
/// исходного буфера.
#[derive(Debug, Clone, Copy)]
pub struct Token {
    pub kind: TokenKind,
    /// Длина используется для вычисления спанов.
    pub length: usize,
}

impl Token {
    pub fn new(kind: TokenKind, length: usize) -> Self {
        Self { kind, length }
    }
}

/// Тип токена.
///
/// Не содержит никаких других данных кроме самого типа.
///
/// Можно дёшево копировать, так как перечесление не содержит никаких дополнительных данных.
#[derive(Debug, Clone, Copy)]
pub enum TokenKind {
    IdentifierOrKeyword,
    Integer,
    OpenRound,
    CloseRound,
    Equal,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Invisible,
    Comment,
    Unknown(char),
}
