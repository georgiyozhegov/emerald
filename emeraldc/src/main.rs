use emeraldc_lexer::Lexer;
use emeraldc_parser::{Declaration, FatalParserError, Parsed, Parser};
use emeraldc_tokenizer::Tokenizer;
use emeraldc_tree_checker::{ErrorUnroller, Report};

fn parse_tree(
    source: &str,
) -> impl Iterator<Item = Result<Parsed<Declaration>, FatalParserError>> {
    let thin_tokens = Tokenizer::tokenize(source);
    let tokens = Lexer::lex(source, thin_tokens);
    let parse_tree = Parser::parse(tokens);
    parse_tree
}

fn main() {
    env_logger::init();

    let source = std::fs::read_to_string("source.ed").unwrap();
    let pt = parse_tree(&source);
    for report in ErrorUnroller::unroll(pt) {
        let report = report.with_preview(&source);
        eprintln!("{report}");
    }
}
