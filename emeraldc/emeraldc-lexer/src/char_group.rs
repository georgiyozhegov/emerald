/// Classifies characters into lexical groups.
pub(crate) struct CharClassifier;

impl CharClassifier {
    pub fn group(c: &char) -> CharGroup {
        match c {
            'a'..'z' | 'A'..'Z' | '_' => CharGroup::Alphabetic,
            '0'..'9' => CharGroup::Numeric,
            ' ' | '\t' | '\n' => CharGroup::Invisible,
            _other => CharGroup::MaybePunctuation,
        }
    }
}

/// A lexical group.
#[derive(Debug)]
pub(crate) enum CharGroup {
    Alphabetic,
    Numeric,
    Invisible,
    MaybePunctuation,
}
