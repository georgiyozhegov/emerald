use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        assert!(start < end);
        Self { start, end }
    }

    pub fn join(self, right: Self) -> Self {
        assert!(self.end < right.start);
        Self::new(self.start, right.end)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Spanned<T> {
    pub value: T,
    pub span: Span,
}

impl<T> Spanned<T> {
    pub fn new(value: T, span: Span) -> Self {
        Self { value, span }
    }
}

pub trait IntoSpanned<T> {
    fn into_spanned(self, span: Span) -> Spanned<T>;
}

impl<T> IntoSpanned<T> for T {
    fn into_spanned(self, span: Span) -> Spanned<T> {
        Spanned::new(self, span)
    }
}
