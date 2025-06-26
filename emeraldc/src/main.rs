mod lexer;
mod parser;

fn main() {
    let source = std::fs::read_to_string("source.ed").unwrap();
    let iter = source.chars().collect::<Vec<_>>().into_iter();
    let lexer = lexer::Lexer::new(iter);
    let parser = parser::Parser::new(lexer);
    let (program, ast) = parser.parse();
    for declaration in program {
        let node = &ast[declaration];
        println!("{node:?}");
    }
}
