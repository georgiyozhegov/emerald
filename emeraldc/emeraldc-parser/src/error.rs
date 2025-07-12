use emeraldc_lexer::WideTokenKind;

#[derive(Debug, Clone)]
pub enum ParserError {
    UnexpectedEof,
    UnexpectedToken {
        expected: WideTokenKind,
        got: WideTokenKind,
    },
    UnexpectedTokenStr {
        expected: &'static str,
        got: WideTokenKind,
    },
    ToDo,
}
