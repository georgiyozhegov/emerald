/// Група символа.
pub enum ChGroup {
    /// Символы, содержашиеся в названиях.
    Alphabetic,
    /// Символы чисел.
    Numeric,
    /// Символы невидимых символов, например, пробел и таб.
    Invisible {
        newline: bool,
    },
    Comment,
    /// Другие символы, которые могут быть пунктуацией.
    MaybePunctuation,
}

impl From<char> for ChGroup {
    fn from(ch: char) -> Self {
        match ch {
            'a'..='z' | 'A'..='Z' | '_' => Self::Alphabetic,
            '0'..='9' => Self::Numeric,
            ' ' | '\t' => Self::Invisible { newline: false },
            '\n' => Self::Invisible { newline: true },
            '#' => Self::Comment,
            _ => Self::MaybePunctuation,
        }
    }
}
