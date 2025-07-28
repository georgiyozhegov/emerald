use std::str;

use emeraldc_lexer::WideToken;
use emeraldc_parser::{Binary, BinaryOperator, Declaration, Expression, FatalParserError, Function, Identifier, Let, NodeError, Parenthesized, Parsed, Statement};
use emeraldc_span::{IntoSpanned, Spanned};

pub struct TreeChecker {}

impl TreeChecker {
    pub fn check(
        tree: impl Iterator<Item = Result<Parsed<Declaration>, FatalParserError>>,
    ) -> impl Iterator<Item = Report> {
        let mut pool = Vec::new();
        for result in tree {
            result.check(&mut pool);
        }
        pool.into_iter()
    }

    fn new() -> Self {
        Self {}
    }
}

trait Check {
    fn check(self, pool: &mut Vec<Report>);
}

impl<T> Check for Result<Parsed<T>, FatalParserError>
where
    Spanned<T>: Check,
{
    fn check(self, pool: &mut Vec<Report>) {
        match self {
            Ok(parsed) => parsed.check(pool),
            Err(error) => error.check(pool),
        }
    }
}

impl<T> Check for Parsed<T>
where
    Spanned<T>: Check,
{
    fn check(self, pool: &mut Vec<Report>) {
        match self {
            Ok(spanned_node) => spanned_node.check(pool),
            Err(spanned_error) => spanned_error.check(pool),
        }
    }
}

impl Check for Spanned<Declaration> {
    fn check(self, pool: &mut Vec<Report>) {
        match self.value {
            Declaration::Function(function) => function.into_spanned(self.span).check(pool),
        }
    }
}

impl Check for Spanned<Function> {
    fn check(self, pool: &mut Vec<Report>) {
        let this = self.value;
        this._introducer.check(pool);
        this.identifier.check(pool);
        this._open_round.check(pool);
        this._close_round.check(pool);
        this.body.check(pool);
        this._end.check(pool);
    }
}

impl Check for Vec<Parsed<Statement>> {
    fn check(self, pool: &mut Vec<Report>) {
        for statement in self {
            statement.check(pool);
        }
    }
}

impl Check for Spanned<Statement> {
    fn check(self, pool: &mut Vec<Report>) {
        match self.value {
            Statement::Let(let_) => let_.into_spanned(self.span).check(pool),
        }
    }
}

impl Check for Spanned<Let> {
    fn check(self, pool: &mut Vec<Report>) {
        let this = self.value;
        this._introducer.check(pool);
        this.identifier.check(pool);
        this._equal.check(pool);
        this.value.check(pool);
    }
}

impl Check for Spanned<Expression> {
    fn check(self, pool: &mut Vec<Report>) {
        match self.value {
            Expression::Integer => {},
            Expression::Variable(identifier) => identifier.into_spanned(self.span).check(pool),
            Expression::Binary(binary) => binary.into_spanned(self.span).check(pool),
            Expression::Parenthesized(parenthesized) => parenthesized.into_spanned(self.span).check(pool),
        }
    }
}

impl Check for Spanned<Binary> {
    fn check(self, pool: &mut Vec<Report>) {
        let this = self.value;
        this.left.check(pool);
        this.operator.check(pool);
        this.right.check(pool);
    }
}

impl Check for Spanned<Parenthesized> {
    fn check(self, pool: &mut Vec<Report>) {
        let this = self.value;
        this._open_round.check(pool);
        this.inner.check(pool);
        this._close_round.check(pool);
    }
}

impl Check for Spanned<BinaryOperator> {
    fn check(self, pool: &mut Vec<Report>) {}
}

impl Check for Spanned<Identifier> {
    fn check(self, pool: &mut Vec<Report>) {}
}

impl Check for Spanned<WideToken> {
    fn check(self, pool: &mut Vec<Report>) {}
}

impl Check for Spanned<NodeError> {
    fn check(self, pool: &mut Vec<Report>) {
        pool.push(Report::Node(self));
    }
}

impl Check for FatalParserError {
    fn check(self, pool: &mut Vec<Report>) {
        pool.push(Report::Fatal(self));
    }
}

#[derive(Debug)]
pub enum Report {
    Node(Spanned<NodeError>),
    Fatal(FatalParserError),
}

impl std::fmt::Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Node(spanned_error) => {
                write!(f, "\x1b[31merror\x1b[m[{}] {}", spanned_error.span, spanned_error.value)
            }
            Self::Fatal(error) => {
                write!(f, "\x1b[31merror\x1b[m[?] {error}")
            }
        }
    }
}

impl Report {
    pub fn preview(&self, source: &str) -> String {
        match self {
            Self::Node(spanned_error) => {
                let span = &spanned_error.span;
                let start = source[..span.start]
                    .rfind('\n')
                    .map(|i| i + 1)
                    .unwrap_or(0);
                let end = source[span.end..]
                    .find('\n')
                    .map(|i| span.end + i)
                    .unwrap_or_else(|| source.len());
                source[start..end].to_string()
            }
            _ => unimplemented!(),
        }
    }
}
