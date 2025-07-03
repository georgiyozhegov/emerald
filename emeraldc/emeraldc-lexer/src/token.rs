/// A token factory.
pub(crate) struct TokenFactory;

impl TokenFactory {
    /// Creates a new token from an alphabetic buffer.
    pub fn from_alphabetic(buffer: String) -> Token {
        match buffer.as_str() {
            "function" => Token::Function,
            "end" => Token::End,
            "let" => Token::Let,
            "if" => Token::If,
            "else" => Token::Else,
            "while" => Token::While,
            _name => Token::Name(buffer),
        }
    }

    /// Creates a new token from a numeric buffer.
    pub fn from_numeric(buffer: String) -> Result<Token, TokenFactoryError> {
        let value: i128 = buffer
            .as_str()
            .parse()
            .map_err(|_e| TokenFactoryError::IntegerTooBig { buffer })?;
        let token = Token::Integer(value);
        Ok(token)
    }

    /// Creates a new token from a single character.
    pub fn from_punctuation(c: &char) -> Result<Token, TokenFactoryError> {
        match c {
            '=' => Ok(Token::Equal),
            '(' => Ok(Token::OpenRound),
            ')' => Ok(Token::CloseRound),
            '{' => Ok(Token::OpenCurly),
            '}' => Ok(Token::CloseCurly),
            '+' => Ok(Token::Plus),
            '-' => Ok(Token::Minus),
            '*' => Ok(Token::Star),
            '/' => Ok(Token::Slash),
            '?' => Ok(Token::Question),
            '>' => Ok(Token::RightAngle),
            '<' => Ok(Token::LeftAngle),
            unknown => {
                let error = TokenFactoryError::UnknownChar(*unknown);
                Err(error)
            }
        }
    }
}

/// A lexical token.
#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum Token {
    // literals
    Integer(i128),
    Name(String),
    // keywords
    Function,
    End,
    Let,
    If,
    Else,
    While,
    // punctuation
    Equal,
    OpenRound,
    CloseRound,
    OpenCurly,
    CloseCurly,
    // operators
    Plus,
    Minus,
    Star,
    Slash,
    Question,
    RightAngle,
    LeftAngle,
}

/// An error that may occur in token factory.
#[derive(Debug, Clone)]
pub enum TokenFactoryError {
    IntegerTooBig { buffer: String },
    UnknownChar(char),
}
