use serde::{Serialize, Deserialize};

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
