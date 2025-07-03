mod char_group;
use char_group::*;
mod lexer;
pub use lexer::*;
mod token;
pub use token::*;

/// Perform lexing.
pub fn lex(source: String) -> LexerOutput {
    let char_vec = source.chars().collect::<Vec<_>>();
    let owned_iter = char_vec.into_iter();
    let peekable_owned_iter = owned_iter.peekable();
    let lexer = Lexer::new(peekable_owned_iter);
    let output = lexer.lex();
    output
}
