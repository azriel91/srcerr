use crate::{Severity, SourceHighlighted};

/// Reference a different part of the source data as a hint.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceRefHint<'path, 'source> {
    /// Source data that the hint appears in.
    pub source_ref: SourceHighlighted<'path, 'source>,
    /// Whether this is a denied error or warning.
    pub severity: Severity,
}
