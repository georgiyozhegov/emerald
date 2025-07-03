fn main() {
    let source = std::fs::read_to_string("source.ed").unwrap();
    let output = emeraldc_lexer::lex(source);
    for token in output.iter() {
        println!("{token:?}");
    }
}
