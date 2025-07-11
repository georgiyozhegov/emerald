use emeraldc_lexer::WideToken;

pub struct Parser {
    token_stream: std::vec::IntoIter<WideToken>,
}

impl Parser {
    pub fn parse(token_stream: impl Iterator<Item = WideToken>) {
        let parser = Self::new(token_stream);
        todo!()
    }

    fn new(token_stream: impl Iterator<Item = WideToken>) -> Self {
        let token_stream = token_stream.collect::<Vec<_>>().into_iter();
        Self { token_stream }
    }
}
