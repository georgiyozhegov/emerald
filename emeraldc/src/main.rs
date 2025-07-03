use std::{fmt::format, process::CommandArgs, str};

use parser::{Ast, Node, NodeId, ParserError, Visit};

mod lexer;
mod parser;
mod tree;

fn parse(source: String) -> Result<(Vec<NodeId>, Ast<Node>), ParserError> {
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
    let str_ast = ast.accept(&mut PrettyPrint);
    for declaration in program.iter() {
        println!("{}", str_ast[*declaration]);
    }
}

pub struct PrettyPrint;

impl Visit<Node> for PrettyPrint {
    type Output = String;

    fn visit(&mut self, node: &Node, output: &Ast<Self::Output>) -> Self::Output {
        match node {
            Node::Name(name) => name.clone(),
            Node::Integer(value) => value.to_string(),
            Node::Binary {
                operator,
                left,
                right,
            } => {
                let left = &output[*left];
                let right = &output[*right];
                format!("{left} {operator} {right}")
            }
            Node::Let { name, value } => {
                let name = &output[*name];
                let value = &output[*value];
                format!("let {name} = {value}")
            }
            Node::Assign { name, value } => {
                let name = &output[*name];
                let value = &output[*value];
                format!("{name} = {value}")
            }
            Node::Else { body } => {
                let body = Self::indent(&output[*body], 4);
                format!(" else {{\n{body}\n}}")
            }
            Node::If {
                condition,
                body,
                else_,
            } => {
                let condition = &output[*condition];
                let body = Self::indent(&output[*body], 4);
                let else_ = else_.map(|id| &output[id]).map_or("", |v| v);
                format!("if {condition} {{\n{body}\n}}{else_}",)
            }
            Node::While { condition, body } => {
                let condition = &output[*condition];
                let body = Self::indent(&output[*body], 4);
                format!("while {condition} {{\n{body}\n}}")
            }
            Node::StatementBody(body) => {
                let mut string = String::new();
                for statement in body {
                    let line = &output[*statement];
                    string.push_str(&format!("{line}\n"));
                }
                string
            }
            Node::Function { name, body } => {
                let name = &output[*name];
                let body = Self::indent(&output[*body], 4);
                format!("function {name}()\n{body}\nend")
            }
            Node::DeclarationBody(body) => {
                let mut string = String::new();
                for statement in body {
                    let line = &output[*statement];
                    string.push_str(&format!("{line}\n"));
                }
                string
            }
        }
    }
}

impl PrettyPrint {
    fn indent(string: &str, n: usize) -> String {
        let tab = " ".repeat(n);
        string
            .lines()
            .map(|line| format!("{tab}{line}"))
            .collect::<Vec<_>>()
            .join("\n")
    }
}
