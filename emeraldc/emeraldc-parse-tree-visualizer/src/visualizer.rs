use emeraldc_parser::{Declaration, FatalParserError, ParsedNode};
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

    fn generate_declaration(
        &mut self,
        node: Result<ParsedNode<Declaration>, FatalParserError>,
    ) {
        self.output.push(VisualizedNode::new(
            "function".into(),
            VisualizedNodeKind::Keyword,
        ));
        self.output.push(VisualizedNode::new(
            " ".into(),
            VisualizedNodeKind::Whitespace,
        ));
        self.output.push(VisualizedNode::new(
            "name".into(),
            VisualizedNodeKind::Identifier,
        ));
        self.output.push(VisualizedNode::new(
            "(".into(),
            VisualizedNodeKind::Punctuation,
        ));
        self.output.push(VisualizedNode::new(
            ")".into(),
            VisualizedNodeKind::Punctuation,
        ));
        self.output.push(VisualizedNode::new(
            " ".into(),
            VisualizedNodeKind::Newline,
        ));

        self.output.push(VisualizedNode::new(
            "\t".into(),
            VisualizedNodeKind::Whitespace,
        ));
        self.output.push(VisualizedNode::new(
            "let".into(),
            VisualizedNodeKind::Keyword,
        ));
        self.output.push(VisualizedNode::new(
            " ".into(),
            VisualizedNodeKind::Whitespace,
        ));
        self.output.push(VisualizedNode::new(
            "=".into(),
            VisualizedNodeKind::Punctuation,
        ));
        self.output.push(VisualizedNode::new(
            " ".into(),
            VisualizedNodeKind::Whitespace,
        ));
        self.output.push(VisualizedNode::new(
            "(".into(),
            VisualizedNodeKind::Punctuation,
        ));
        self.output.push(VisualizedNode::new(
            "9".into(),
            VisualizedNodeKind::Integer,
        ));
        self.output.push(VisualizedNode::new(
            " ".into(),
            VisualizedNodeKind::Whitespace,
        ));
        self.output.push(VisualizedNode::new(
            "+".into(),
            VisualizedNodeKind::Punctuation,
        ));
        self.output.push(VisualizedNode::new(
            " ".into(),
            VisualizedNodeKind::Whitespace,
        ));
        self.output.push(VisualizedNode::new(
            "28".into(),
            VisualizedNodeKind::Integer,
        ));
        self.output.push(VisualizedNode::new(
            ")".into(),
            VisualizedNodeKind::Punctuation,
        ));
        self.output.push(VisualizedNode::new(
            " ".into(),
            VisualizedNodeKind::Newline,
        ));
        self.output.push(VisualizedNode::new(
            "end".into(),
            VisualizedNodeKind::Keyword,
        ));
    }
}

#[derive(serde::Serialize)]
struct VisualizedNode {
    text: String,
    kind: VisualizedNodeKind,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "snake_case")]
enum VisualizedNodeKind {
    Identifier,
    Integer,
    Keyword,
    Punctuation,
    Whitespace,
    Newline,
}

impl VisualizedNode {
    pub fn new(text: String, kind: VisualizedNodeKind) -> Self {
        Self { text, kind }
    }
}
