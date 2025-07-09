//! Этот модуль ответственен только за разделение входного буфера на отдельные токены. Дальнейший анализ проводится лексером.
//!
//! Очень много принципов позаимствовано из [лексера rustc](https://github.com/rust-lang/rust/tree/master/compiler/rustc_lexer).

mod ch_group;
mod source_buffer;
mod token;
mod tokenizer;
pub use token::{Token, TokenKind};
pub use tokenizer::Tokenizer;
