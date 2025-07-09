#[derive(Debug, Clone, Copy)]
pub enum LexerError {
    /// Найден неизвестный символ, который не может быть обработан.
    UnknownCharacter,
}

impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::UnknownCharacter => write!(f, "found an unknown character"),
        }
    }
}
impl std::error::Error for LexerError {}
