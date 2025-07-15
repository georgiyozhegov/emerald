//! Этот модуль ответственен только за разделение входного буфера на отдельные токены. Дальнейший анализ проводится лексером.
//!
//! Очень много принципов позаимствовано из [лексера rustc](https://github.com/rust-lang/rust/tree/master/compiler/rustc_lexer).

mod ch_group;
mod source_buffer;
mod token;
mod tokenizer;
use ch_group::*;
use source_buffer::*;
pub use token::*;
pub use tokenizer::*;
