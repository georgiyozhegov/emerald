use emeraldc_parser::ParserSource;

fn main() {
    env_logger::init();

    let source = std::fs::read_to_string("source.ed").unwrap();
    let tokens = emeraldc_lexer::lex(source);
    let tokens_iter = tokens.into_iter().peekable();
    let source = ParserSource::new(tokens_iter);
    let parser = emeraldc_parser::Parser::new(source);
    let pt = parser.parse();
    for declaration in pt.program.iter() {
        println!("{declaration:?}");
    }
}
