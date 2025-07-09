/// Група символа.
pub enum ChGroup {
    /// Символы, содержашиеся в названиях.
    Alphabetic,
    /// Символы чисел.
    Numeric,
    /// Символы невидимых символов, например, пробел и таб.
    Invisible,
    /// Другие символы, которые могут быть пунктуацией.
    MaybePunctuation,
}

impl From<char> for ChGroup {
    fn from(ch: char) -> Self {
        match ch {
            'a'..='z' | 'A'..='Z' | '_' => Self::Alphabetic,
            '0'..='9' => Self::Numeric,
            ' ' | '\t' | '\n' => Self::Invisible,
            _ => Self::MaybePunctuation,
        }
    }
}
