use parser::{Ast, NodeId, ParserError};

mod lexer;
mod parser;

fn parse(source: String) -> Result<(Vec<NodeId>, Ast), ParserError> {
    let iter = source.chars().collect::<Vec<_>>().into_iter();
    let lexer = lexer::Lexer::new(iter);
    let parser = parser::Parser::new(lexer);
    let (program, ast) = parser.parse()?;
    for declaration in program.iter() {
        let node = &ast[*declaration];
        println!("{node:?}");
    }
    Ok((program, ast))
}

fn main() {
    let source = std::fs::read_to_string("source.ed").unwrap();
    let (program, ast) = parse(source).unwrap_or_else(|error| {
        eprintln!("error: {error}");
        std::process::exit(1);
    });
}
