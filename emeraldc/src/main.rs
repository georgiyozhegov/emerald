use std::iter::Peekable;

fn main() {
    let source = std::fs::read_to_string("source.ed").unwrap();
    let iter = source.chars().collect::<Vec<_>>().into_iter();
    let lexer = Lexer::new(iter);
    for token in lexer {
        println!("{token:?}");
    }
}

#[derive(Debug)]
struct Lexer {
    buffer: SourceBuffer,
}

impl Lexer {
    pub fn new(iter: std::vec::IntoIter<char>) -> Self {
        let buffer = SourceBuffer::new(iter);
        Self { buffer }
    }
}

impl Iterator for Lexer {
    type Item = TokenKind;

    fn next(&mut self) -> Option<Self::Item> {
        self.take_while(|c| matches!(c, ' ' | '\n'));
        self.lex_token()
    }
}

impl Lexer {
    fn lex_token(&mut self) -> Option<TokenKind> {
        let next = self.buffer.peek()?;
        match next {
            '0'..'9' => self.lex_integer(),
            'a'..'z' | 'A'..'Z' => self.lex_name_or_keyword(),
            _ => self.lex_punctuation(),
        }
    }

    fn lex_integer(&mut self) -> Option<TokenKind> {
        let text = self.take_while(|c| matches!(c, '0'..'9' | '_'));
        let value = text.parse::<i128>().unwrap();
        Some(TokenKind::Integer(value))
    }

    fn lex_name_or_keyword(&mut self) -> Option<TokenKind> {
        let text = self.take_while(|c| matches!(c, 'a'..'z' | 'A'..'Z' | '0'..'9' | '_'));
        let token = match text.as_str() {
            "function" => TokenKind::Function,
            "end" => TokenKind::End,
            "let" => TokenKind::Let,
            _ => TokenKind::Name(text),
        };
        Some(token)
    }

    fn lex_punctuation(&mut self) -> Option<TokenKind> {
        let first = self.buffer.next()?;
        let token = match first {
            '=' => TokenKind::Equal,
            '(' => TokenKind::OpenRound,
            ')' => TokenKind::CloseRound,
            '{' => TokenKind::OpenCurly,
            '}' => TokenKind::CloseCurly,
            _ => todo!("error handling"),
        };
        Some(token)
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
#[derive(Debug)]
pub enum TokenKind {
    // literals
    Integer(i128),
    Name(String),
    // keywords
    Function,
    End,
    Let,
    // pucntuation
    Equal,
    OpenRound,
    CloseRound,
    OpenCurly,
    CloseCurly,
}

#[derive(Debug)]
struct SourceBuffer {
    iter: Peekable<std::vec::IntoIter<char>>,
    pub cursor: usize,
}

impl SourceBuffer {
    pub fn new(iter: std::vec::IntoIter<char>) -> Self {
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
