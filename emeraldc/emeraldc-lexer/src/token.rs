use core::fmt;

pub(crate) struct TokenFactory;

impl TokenFactory {
    pub fn from_alphabetic(buffer: &str) -> Token {
        match buffer {
            "function" => Token::Function,
            "end" => Token::End,
            "let" => Token::Let,
            "if" => Token::If,
            "else" => Token::Else,
            "while" => Token::While,
            _name => Token::Name(buffer.to_owned()),
        }
    }

    pub fn from_numeric(buffer: &str) -> Result<Token, TokenFactoryError> {
        let value: i128 =
            buffer
                .parse()
                .map_err(|_e| TokenFactoryError::IntegerTooBig {
                    buffer: buffer.to_owned(),
                })?;
        let token = Token::Integer(value);
        Ok(token)
    }

    pub fn from_punctuation(c: char) -> Result<Token, TokenFactoryError> {
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
                let error = TokenFactoryError::UnknownChar(unknown);
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

#[derive(Debug, Clone)]
pub enum TokenFactoryError {
    IntegerTooBig { buffer: String },
    UnknownChar(char),
    UnexpectedEof,
}
