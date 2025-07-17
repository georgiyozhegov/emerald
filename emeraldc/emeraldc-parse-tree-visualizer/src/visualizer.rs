use std::collections::hash_map::Values;

use emeraldc_lexer::{Span, WideTokenKind};
use emeraldc_parser::{
    BinaryOperator, Declaration, Expression, FatalParserError, Identifier,
    NodeError, ParsedNode, Statement,
};
use tera::Tera;

pub struct Visualizer<'s> {
    source: &'s str,
    output: Vec<VisualizedNode>,
    templater: Tera,
}

impl<'s> Visualizer<'s> {
    pub fn visualize(
        source: &'s str,
        parse_tree: impl Iterator<
            Item = Result<ParsedNode<Declaration>, FatalParserError>,
        >,
    ) -> String {
        let mut visualizer = Self::new(source);
        visualizer.load_template();
        for node in parse_tree {
            visualizer.generate_declaration(node);
        }
        visualizer.strip_newline();
        visualizer.render_template()
    }

    fn new(source: &'s str) -> Self {
        Self {
            source,
            output: Vec::new(),
            templater: Tera::new("../*.html").unwrap(),
        }
    }

    fn load_template(&mut self) {
        let template = include_str!("../index.html");
        self.templater.add_raw_template("index", &template).unwrap();
    }

    fn render_template(self) -> String {
        let mut context = tera::Context::new();
        context.insert("parse_tree", &self.output);
        self.templater.render("index", &context).unwrap()
    }

    fn strip_newline(&mut self) {
        while self.output.last().is_some_and(|n| n.kind == VisualizedNodeKind::Newline) {
            self.output.pop();
        }
    }

    fn generate_declaration(
        &mut self,
        node: Result<ParsedNode<Declaration>, FatalParserError>,
    ) {
        match node {
            Ok(node) => self.generate_parsed_declaration(node),
            Err(error) => self.generate_fatal_error(error),
        }
    }

    fn generate_parsed_declaration(&mut self, node: ParsedNode<Declaration>) {
        match node.node {
            Ok(node) => self.generate_ok_declaration(node),
            Err(error) => self.generate_node_error(error, node.span),
        }
    }

    fn generate_ok_declaration(&mut self, node: Declaration) {
        match node {
            Declaration::Function {
                _introducer,
                identifier,
                _open_round,
                _close_round,
                body,
                _end,
            } => {
                self.generate_token(_introducer);
                self.push_whitespace();
                self.generate_identifier(identifier);
                self.push_whitespace();
                self.generate_token(_open_round);
                self.generate_token(_close_round);
                self.push_newline();
                for statement in body {
                    self.generate_statement(statement);
                }
                self.generate_token(_end);
                self.push_newline();
                self.push_newline();
            }
        }
    }

    fn generate_token(&mut self, token: ParsedNode<WideTokenKind>) {
        match token.node {
            Ok(token) => {
                match token {
                    // keywords
                    WideTokenKind::FunctionKeyword => {
                        self.push_keyword("function")
                    }
                    WideTokenKind::EndKeyword => self.push_keyword("end"),
                    WideTokenKind::LetKeyword => self.push_keyword("let"),
                    // punctuation
                    WideTokenKind::OpenRound => self.push_punctuation("("),
                    WideTokenKind::CloseRound => self.push_punctuation(")"),
                    WideTokenKind::Equal => self.push_punctuation("="),
                    // other tokens are handled by other functions
                    _ => unreachable!(),
                }
            }
            Err(error) => self.generate_node_error(error, token.span),
        }
    }

    fn generate_identifier(&mut self, node: ParsedNode<Identifier>) {
        let span = node.span;
        let text = &self.source[span.start..span.end];
        match node.node {
            Ok(_) => self.push_identifier(text),
            Err(error) => self.generate_node_error(error, span),
        }
    }

    fn generate_statement(&mut self, node: ParsedNode<Statement>) {
        match node.node {
            Ok(node) => self.generate_ok_statement(node),
            Err(error) => self.generate_node_error(error, node.span),
        }
    }

    fn generate_ok_statement(&mut self, node: Statement) {
        self.push_tab();
        match node {
            Statement::Let {
                _introducer,
                identifier,
                _equal,
                value,
            } => {
                self.generate_token(_introducer);
                self.push_whitespace();
                self.generate_identifier(identifier);
                self.push_whitespace();
                self.generate_token(_equal);
                self.push_whitespace();
                self.generate_expression(value);
                self.push_newline();
            }
        }
    }

    fn generate_expression(&mut self, node: ParsedNode<Expression>) {
        let text = &self.source[node.span.start..node.span.end];
        match node.node {
            Ok(node) => self.generate_ok_expression(node, text),
            Err(error) => self.generate_node_error(error, node.span),
        }
    }

    fn generate_ok_expression(&mut self, node: Expression, text: &str) {
        match node {
            Expression::Integer => self.push_integer(text),
            Expression::Variable(_) => self.push_identifier(text),
            Expression::Binary {
                left,
                operator,
                right,
            } => {
                self.generate_expression(*left);
                self.push_whitespace();
                self.generate_binary_operator(operator);
                self.push_whitespace();
                self.generate_expression(*right);
            }
            Expression::Parenthesized {
                _open_round,
                inner,
                _close_round,
            } => {
                self.generate_token(_open_round);
                self.generate_expression(*inner);
                self.generate_token(_close_round);
            }
        }
    }

    fn generate_binary_operator(&mut self, node: ParsedNode<BinaryOperator>) {
        match node.node {
            Ok(operator) => {
                let text = match operator {
                    BinaryOperator::Add => "+",
                    BinaryOperator::Subtract => "-",
                    BinaryOperator::Multiply => "*",
                    BinaryOperator::Divide => "/",
                };
                self.push_punctuation(text);
            }
            Err(error) => self.generate_node_error(error, node.span),
        }
    }

    fn generate_node_error(&mut self, error: NodeError, span: Span) {
        let text = &self.source[span.start..span.end];
        let message = match error {
            NodeError::UnexpectedToken(token) => {
                let token = match token {
                    WideTokenKind::HadError(error) => format!("{error}"),
                    token => format!("{token:?}"),
                };
                format!("unexpected token: {token}")
            }
            NodeError::InvalidStatementIntroducer(token) => {
                format!("invalid statement introducer: {token:?}")
            }
            NodeError::InvalidExpressionIntroducer => {
                format!("invalid expression introducer")
            }
        };
        self.output.push(VisualizedNode::new(
            text.into(),
            VisualizedNodeKind::Error,
            message,
        ));
    }

    fn generate_fatal_error(&mut self, error: FatalParserError) {
        let message = match error {
            FatalParserError::InvalidDeclarationIntroducer => {
                format!("invalid declaration introducer")
            }
            FatalParserError::CompilerBug(message) => {
                format!("compiler bug: {message}")
            }
            FatalParserError::UnexpectedEof => format!("unexpected eof"),
            FatalParserError::Lexer(error) => format!("lexical error: {error}"),
            FatalParserError::UnexpectedToken => todo!("deprecate this error"),
        };
        self.output.push(VisualizedNode::new(
            "!?".into(),
            VisualizedNodeKind::Error,
            message,
        ));
    }

    fn push_identifier(&mut self, text: &str) {
        self.output.push(VisualizedNode::new(
            text.into(),
            VisualizedNodeKind::Identifier,
            "".into(),
        ));
    }

    fn push_integer(&mut self, text: &str) {
        self.output.push(VisualizedNode::new(
            text.into(),
            VisualizedNodeKind::Integer,
            "".into(),
        ));
    }

    fn push_keyword(&mut self, text: &str) {
        self.output.push(VisualizedNode::new(
            text.into(),
            VisualizedNodeKind::Keyword,
            "".into(),
        ));
    }

    fn push_punctuation(&mut self, text: &str) {
        self.output.push(VisualizedNode::new(
            text.into(),
            VisualizedNodeKind::Punctuation,
            "".into(),
        ));
    }

    fn push_newline(&mut self) {
        self.output.push(VisualizedNode::new(
            "↵".into(),
            VisualizedNodeKind::Newline,
            "<newline>".into(),
        ));
    }

    fn push_whitespace(&mut self) {
        self.output.push(VisualizedNode::new(
            "·".into(),
            VisualizedNodeKind::Whitespace,
            "<whitespace>".into(),
        ));
    }

    fn push_tab(&mut self) {
        self.output.push(VisualizedNode::new(
            "\t".into(),
            VisualizedNodeKind::Whitespace,
            "<tab>".into(),
        ));
    }
}

#[derive(serde::Serialize)]
struct VisualizedNode {
    text: String,
    kind: VisualizedNodeKind,
    message: String,
}

#[derive(serde::Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum VisualizedNodeKind {
    Identifier,
    Integer,
    Keyword,
    Punctuation,
    Whitespace,
    Newline,
    Error,
}

impl VisualizedNode {
    pub fn new(
        text: String,
        kind: VisualizedNodeKind,
        message: String,
    ) -> Self {
        Self {
            text: Self::unescape(text),
            kind,
            message: Self::unescape(message),
        }
    }

    fn unescape(text: String) -> String {
        text.replace("<", "&lt;").replace(">", "&gt;")
    }
}
