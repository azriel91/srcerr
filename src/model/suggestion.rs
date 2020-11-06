use crate::SourceRefHint;

/// Suggestions to provide to the user.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Suggestion<'path, 'source> {
    /// Suggestions to provide as valid exprs.
    ValidExprs(Vec<&'source str>),
    /// Reference a different part of the source data as a hint.
    SourceRefHint(SourceRefHint<'path, 'source>),
    /// Simple message to give to the user.
    Hint(&'source str),
}
