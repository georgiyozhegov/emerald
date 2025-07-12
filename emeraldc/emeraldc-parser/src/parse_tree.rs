use emeraldc_lexer::Span;

use crate::ParserError;

/// Обьявление конструкции верхнего уровня.
#[derive(Debug, Clone)]
pub enum Declaration {
    Function {
        name: ParsedNode<Identifier>,
        body: ParsedNode<FunctionBody>,
    },
}

/// Идентификатор, обозначающий имя чего-либо.
#[derive(Debug, Clone)]
pub struct Identifier {
    pub name_span: Span,
}

/// Тело функции
#[derive(Debug, Clone)]
pub struct FunctionBody {
    pub statements: Vec<ParsedNode<Statement>>,
}

/// Инструкция.
#[derive(Debug, Clone)]
pub enum Statement {
    Let {
        name: ParsedNode<Identifier>,
        value: ParsedNode<Expression>,
    },
}

/// Выражение.
#[derive(Debug, Clone)]
pub enum Expression {
    Integer { value_span: Span },
}

/// Возможно некорректная вершина.
pub type ParsedNode<T> = Result<T, ParserError>;
