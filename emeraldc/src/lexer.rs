use std::iter::Peekable;
use std::vec;
use std::fmt;

#[derive(Debug)]
pub struct Lexer {
    buffer: SourceBuffer,
}

impl Lexer {
    pub fn new(iter: vec::IntoIter<char>) -> Self {
        let buffer = SourceBuffer::new(iter);
        Self { buffer }
    }
}

impl Iterator for Lexer {
    type Item = Result<TokenKind, LexerError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.take_while(|c| matches!(c, ' ' | '\n'));
        self.lex_token()
    }
}

impl Lexer {
    fn lex_token(&mut self) -> Option<Result<TokenKind, LexerError>> {
        let next = self.buffer.peek()?;
        match next {
            '0'..'9' => Some(self.lex_integer()),
            'a'..'z' | 'A'..'Z' | '_' => Some(Ok(self.lex_name_or_keyword())),
            _ => self.lex_punctuation(),
        }
    }

    fn lex_integer(&mut self) -> Result<TokenKind, LexerError> {
        let text = self.take_while(|c| matches!(c, '0'..'9' | '_'));
        let value = text
            .parse::<i128>()
            .map_err(|_| LexerError::IntegerTooLarge)?;
        Ok(TokenKind::Integer(value))
    }

    fn lex_name_or_keyword(&mut self) -> TokenKind {
        let text = self.take_while(|c| matches!(c, 'a'..'z' | 'A'..'Z' | '_' | '0'..'9'));
        let token = match text.as_str() {
            "function" => TokenKind::Function,
            "end" => TokenKind::End,
            "let" => TokenKind::Let,
            "if" => TokenKind::If,
            _ => TokenKind::Name(text),
        };
        token
    }

    fn lex_punctuation(&mut self) -> Option<Result<TokenKind, LexerError>> {
        let first = self.buffer.next()?;
        let token = match first {
            '=' => TokenKind::Equal,
            '(' => TokenKind::OpenRound,
            ')' => TokenKind::CloseRound,
            '{' => TokenKind::OpenCurly,
            '}' => TokenKind::CloseCurly,
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Star,
            '/' => TokenKind::Slash,
            '?' => TokenKind::Question,
            '>' => TokenKind::RightAngle,
            '<' => TokenKind::LeftAngle,
            c => {
                return Some(Err(LexerError::UnknownCharacter(c)));
            }
        };
        Some(Ok(token))
    }

    fn take_while(&mut self, predicate: impl Fn(char) -> bool) -> String {
        let mut buffer = String::new();
        while let Some(c) = self.buffer.next_if(&predicate) {
            buffer.push(c);
        }
        buffer
    }
}

#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenKind {
    // literals
    Integer(i128),
    Name(String),
    // keywords
    Function,
    End,
    Let,
    If,
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

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Integer(value) => write!(f, "{value}"),
            Self::Name(name) => write!(f, "{name}"),
            Self::Function => write!(f, "function"),
            Self::End => write!(f, "end"),
            Self::Let => write!(f, "let"),
            Self::If => write!(f, "if"),
            Self::Equal => write!(f, "="),
            Self::OpenRound => write!(f, "("),
            Self::CloseRound => write!(f, ")"),
            Self::OpenCurly => write!(f, "{{"),
            Self::CloseCurly => write!(f, "}}"),
            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::Star => write!(f, "*"),
            Self::Slash => write!(f, "/"),
            Self::Question => write!(f, "/"),
            Self::RightAngle => write!(f, ">"),
            Self::LeftAngle => write!(f, "<"),
        }
    }
}

#[derive(Debug)]
struct SourceBuffer {
    iter: Peekable<vec::IntoIter<char>>,
    pub cursor: usize,
}

impl SourceBuffer {
    pub fn new(iter: vec::IntoIter<char>) -> Self {
        Self {
            iter: iter.peekable(),
            cursor: 0,
        }
    }

    pub fn next(&mut self) -> Option<char> {
        let c = self.iter.next()?;
        self.cursor += 1;
        Some(c)
    }

    pub fn peek(&mut self) -> Option<char> {
        self.iter.peek().cloned()
    }

    pub fn next_if(&mut self, predicate: impl Fn(char) -> bool) -> Option<char> {
        let c = self.iter.next_if(|c: &char| predicate(*c))?;
        self.cursor += 1;
        Some(c)
    }
}

#[derive(Debug, Clone)]
pub enum LexerError {
    UnknownCharacter(char),
    IntegerTooLarge,
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::UnknownCharacter(c) => write!(f, "unknown character '{c}'"),
            Self::IntegerTooLarge => write!(f, "integer is too large"),
        }
    }
}
