use crate::SourceBuffer;

pub(crate) struct CharClassifier;

impl CharClassifier {
    pub fn group(c: char) -> CharGroup {
        match c {
            'a'..='z' | 'A'..='Z' | '_' => CharGroup::Alphabetic,
            '0'..='9' => CharGroup::Numeric,
            ' ' | '\t' | '\n' => CharGroup::Invisible,
            SourceBuffer::EOF => CharGroup::Eof,
            _other => CharGroup::MaybePunctuation,
        }
    }
}

#[derive(Debug)]
pub(crate) enum CharGroup {
    Alphabetic,
    Numeric,
    Invisible,
    MaybePunctuation,
    Eof,
}

impl CharGroup {
    pub fn is_eof(&self) -> bool {
        matches!(self, Self::Eof)
    }
}
