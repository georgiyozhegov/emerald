use crate::{
    Parser,
    error::ParserError,
    parser::Subparser,
    tree::{Declaration, ParsedNode},
};

pub struct DeclarationParser<'p> {
    parser: &'p mut Parser,
}

impl<'p> Subparser<'p, Declaration> for DeclarationParser<'p> {
    fn parse(
        parser: &'p mut Parser,
    ) -> Result<ParsedNode<Declaration>, ParserError> {
        let this = Self::new(parser);
        this._parse()
    }
}

impl<'p> DeclarationParser<'p> {
    fn new(parser: &'p mut Parser) -> Self {
        Self { parser }
    }

    fn _parse(mut self) -> Result<ParsedNode<Declaration>, ParserError> {
        todo!()
    }
}
