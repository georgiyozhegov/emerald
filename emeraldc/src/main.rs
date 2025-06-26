mod lexer;

fn main() {
    let source = std::fs::read_to_string("source.ed").unwrap();
    let iter = source.chars().collect::<Vec<_>>().into_iter();
    let lexer = lexer::Lexer::new(iter);
    for token in lexer {
        println!("{token:?}");
    }
}
