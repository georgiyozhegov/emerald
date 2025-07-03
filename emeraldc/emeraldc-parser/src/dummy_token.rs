use emeraldc_lexer::Token;

/// A token without any data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DummyToken {
    // literals
    Integer,
    Name,
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
    // only dummy variant
    Eof,
}

// for expect function
impl From<Token> for DummyToken {
    fn from(owned_token: Token) -> Self {
        let borrowed = &owned_token;
        Self::from(borrowed)
    }
}

impl From<&Token> for DummyToken {
    fn from(token: &Token) -> Self {
        match token {
            Token::Integer(..) => Self::Integer,
            Token::Name(..) => Self::Name,
            Token::Function => Self::Function,
            Token::End => Self::End,
            Token::Let => Self::Let,
            Token::If => Self::If,
            Token::Else => Self::Else,
            Token::While => Self::While,
            Token::OpenRound => Self::OpenRound,
            Token::CloseRound => Self::CloseRound,
            Token::OpenCurly => Self::OpenCurly,
            Token::CloseCurly => Self::CloseCurly,
            Token::Plus => Self::Plus,
            Token::Minus => Self::Minus,
            Token::Star => Self::Star,
            Token::Slash => Self::Slash,
            Token::Question => Self::Question,
            Token::RightAngle => Self::RightAngle,
            Token::LeftAngle => Self::LeftAngle,
            uncovered => todo!("{uncovered:?}"),
        }
    }
}
