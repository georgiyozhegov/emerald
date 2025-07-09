pub struct SourceBuffer {
    text: String,
    chars: Vec<char>,
    cursor: usize,
    length: usize,
}

// initialization
impl SourceBuffer {
    pub fn new(text: String) -> Self {
        let chars = text.chars().collect();
        let length = text.len();
        Self {
            text,
            chars,
            cursor: 0,
            length,
        }
    }
}

// public interface
impl SourceBuffer {
    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn cursor(&self) -> usize {
        self.cursor
    }

    pub fn length(&self) -> usize {
        self.length
    }
}

// logic
impl SourceBuffer {
    pub const EOF: char = '\0';

    pub fn advance_if(&mut self, f: impl Fn(char) -> bool) -> bool {
        let c = self.current();
        if f(c) {
            self.advance();
            true
        } else {
            false
        }
    }

    pub fn eat(&mut self) -> char {
        let c = self.current();
        self.advance();
        c
    }

    pub fn advance(&mut self) {
        self.cursor += 1;
    }

    pub fn current(&mut self) -> char {
        if self.is_eof() {
            Self::EOF
        } else {
            self.chars[self.cursor]
        }
    }

    fn is_eof(&self) -> bool {
        self.cursor == self.length
    }

    pub fn substring(&self, start: usize, end: usize) -> &str {
        &self.text[start..end]
    }
}
