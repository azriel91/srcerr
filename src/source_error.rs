use crate::{RelevantSource, Severity, Suggestion};

/// Information about an error from source data.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceError<'path, 'source> {
    /// Source data that the erroneous value is found.
    pub invalid_source: RelevantSource<'path, 'source>,
    /// Suggestion or hint to provide to the user.
    pub suggestion: Option<Suggestion<'source>>,
    /// Whether this is a denied error or warning.
    pub severity: Severity,
}
