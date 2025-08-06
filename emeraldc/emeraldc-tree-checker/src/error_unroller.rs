use std::str;

use emeraldc_lexer::WideToken;
use emeraldc_parser::{
    Binary, BinaryOperator, Declaration, Expression, FatalParserError,
    Function, Identifier, Let, NodeError, Parenthesized, Parsed, Statement,
};
use emeraldc_span::{IntoSpanned, Span, Spanned};

pub struct ErrorUnroller {}

impl ErrorUnroller {
    pub fn unroll(
        tree: impl Iterator<Item = Result<Parsed<Declaration>, FatalParserError>>,
    ) -> impl Iterator<Item = Report> {
        let mut pool = Vec::new();
        for result in tree {
            result.unroll(&mut pool);
        }
        pool.into_iter()
    }
}

trait Unroll {
    fn unroll(self, pool: &mut Vec<Report>);
}

impl<T> Unroll for Result<Parsed<T>, FatalParserError>
where
    Spanned<T>: Unroll,
{
    fn unroll(self, pool: &mut Vec<Report>) {
        match self {
            Ok(parsed) => parsed.unroll(pool),
            Err(error) => error.unroll(pool),
        }
    }
}

impl<T> Unroll for Parsed<T>
where
    Spanned<T>: Unroll,
{
    fn unroll(self, pool: &mut Vec<Report>) {
        match self {
            Ok(spanned_node) => spanned_node.unroll(pool),
            Err(spanned_error) => spanned_error.unroll(pool),
        }
    }
}

impl Unroll for Spanned<Declaration> {
    fn unroll(self, pool: &mut Vec<Report>) {
        match self.value {
            Declaration::Function(function) => {
                function.into_spanned(self.span).unroll(pool)
            }
        }
    }
}

impl Unroll for Spanned<Function> {
    fn unroll(self, pool: &mut Vec<Report>) {
        let this = self.value;
        this._introducer.unroll(pool);
        this.identifier.unroll(pool);
        this._open_round.unroll(pool);
        this._close_round.unroll(pool);
        this.body.unroll(pool);
        this._end.unroll(pool);
    }
}

impl Unroll for Vec<Parsed<Statement>> {
    fn unroll(self, pool: &mut Vec<Report>) {
        for statement in self {
            statement.unroll(pool);
        }
    }
}

impl Unroll for Spanned<Statement> {
    fn unroll(self, pool: &mut Vec<Report>) {
        match self.value {
            Statement::Let(let_) => let_.into_spanned(self.span).unroll(pool),
        }
    }
}

impl Unroll for Spanned<Let> {
    fn unroll(self, pool: &mut Vec<Report>) {
        let this = self.value;
        this._introducer.unroll(pool);
        this.identifier.unroll(pool);
        this._equal.unroll(pool);
        this.value.unroll(pool);
    }
}

impl Unroll for Spanned<Expression> {
    fn unroll(self, pool: &mut Vec<Report>) {
        match self.value {
            Expression::Integer => {}
            Expression::Variable(identifier) => {
                identifier.into_spanned(self.span).unroll(pool)
            }
            Expression::Binary(binary) => {
                binary.into_spanned(self.span).unroll(pool)
            }
            Expression::Parenthesized(parenthesized) => {
                parenthesized.into_spanned(self.span).unroll(pool)
            }
        }
    }
}

impl Unroll for Spanned<Binary> {
    fn unroll(self, pool: &mut Vec<Report>) {
        let this = self.value;
        this.left.unroll(pool);
        this.operator.unroll(pool);
        this.right.unroll(pool);
    }
}

impl Unroll for Spanned<Parenthesized> {
    fn unroll(self, pool: &mut Vec<Report>) {
        let this = self.value;
        this._open_round.unroll(pool);
        this.inner.unroll(pool);
        this._close_round.unroll(pool);
    }
}

impl Unroll for Spanned<BinaryOperator> {
    fn unroll(self, _pool: &mut Vec<Report>) {}
}

impl Unroll for Spanned<Identifier> {
    fn unroll(self, _pool: &mut Vec<Report>) {}
}

impl Unroll for Spanned<WideToken> {
    fn unroll(self, _pool: &mut Vec<Report>) {}
}

impl Unroll for Spanned<NodeError> {
    fn unroll(self, pool: &mut Vec<Report>) {
        pool.push(Report::Node(self));
    }
}

impl Unroll for FatalParserError {
    fn unroll(self, pool: &mut Vec<Report>) {
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
                write!(
                    f,
                    "\x1b[31merror\x1b[m[{}] {}",
                    spanned_error.span, spanned_error.value
                )
            }
            Self::Fatal(error) => {
                write!(f, "\x1b[31merror\x1b[m[?] {error}")
            }
        }
    }
}

impl Report {
    pub fn with_preview<'s>(self, source: &'s str) -> FullReport<'s> {
        FullReport::new(self, source)
    }
}

#[derive(Debug)]
pub struct FullReport<'s> {
    pub report: Report,
    pub source: &'s str,
}

impl<'s> FullReport<'s> {
    pub fn new(report: Report, source: &'s str) -> Self {
        Self { report, source }
    }
}

impl std::fmt::Display for FullReport<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.report)?;
        match &self.report {
            Report::Node(spanned_error) => {
                writeln!(f)?;
                let span = &spanned_error.span;
                let (start, end) = self.find_line_boundaries(span);
                writeln!(f, "\x1b[31m|\x1b[m")?;
                writeln!(
                    f,
                    "\x1b[31m|\x1b[m {}",
                    self.source[start..end].to_string()
                )?;
                let pointer_line = self.pointer_line(span, start);
                write!(f, "\x1b[31m|\x1b[m {pointer_line}")
            }
            _ => Ok(()),
        }
    }
}

impl FullReport<'_> {
    fn find_line_boundaries(&self, span: &Span) -> (usize, usize) {
        let start = self.source[..span.start]
            .rfind('\n')
            .map(|i| i + 1)
            .unwrap_or(0);
        let end = self.source[span.end..]
            .find('\n')
            .map(|i| span.end + i)
            .unwrap_or_else(|| self.source.len());
        (start, end)
    }

    fn pointer_line(
        &self,
        span: &Span,
        start: usize,
    ) -> impl std::fmt::Display {
        let pointer_start = span.start - start;
        let pointer_length = span.end - span.start;
        " ".repeat(pointer_start)
            + "\x1b[33m"
            + &"^".repeat(pointer_length)
            + " here\x1b[m"
    }
}
