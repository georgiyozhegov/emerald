use emeraldc_lexer::Lexer;

fn main() {
    let source = std::fs::read_to_string("source.ed").unwrap();
    let char_vec = source.chars().collect::<Vec<_>>();
    let owned_char_iter = char_vec.into_iter();
    let lexer = Lexer::new(owned_char_iter);
    let output = lexer.lex();
    for token in output.iter() {
        println!("{token:?}");
    }
}
