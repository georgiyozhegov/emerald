use emeraldc_lexer::Lexer;
use emeraldc_parser::Parser;
use emeraldc_tokenizer::Tokenizer;

fn main() {
    env_logger::init();

    let source = std::fs::read_to_string("source.ed").unwrap();
    let tokens = Tokenizer::tokenize(source.as_str());
    let tokens = Lexer::lex(source.as_str(), tokens);
    let parse_tree = Parser::parse(tokens);
    let parse_tree = parse_tree.collect::<Vec<_>>();
    let json = serde_json::to_string_pretty(&parse_tree).unwrap();
    println!("{json}");

    /*
    let text = std::fs::read_to_string("source.ed").unwrap();
    let sb = emeraldc_lexer::SourceBuffer::new(text);
    let tokens = emeraldc_lexer::Lexer::new(sb).lex(&mut reporter);
    let tokens_iter = tokens.into_iter().peekable();
    */

    /*
    let source = ParserSource::new(tokens_iter);
    let pt = emeraldc_parser::Parser::new(source).parse();
    for declaration in pt.program.iter() {
        println!("{declaration:?}");
    }
    */
}
