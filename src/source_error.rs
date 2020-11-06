use crate::{Severity, SourceHighlighted, Suggestion};

/// Information about an error from source data.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceError<'path, 'source> {
    /// Source data that the erroneous value is found.
    pub invalid_source: SourceHighlighted<'path, 'source>,
    /// Suggestions or hints to provide to the user.
    pub suggestions: Vec<Suggestion<'path, 'source>>,
    /// Whether this is a denied error or warning.
    pub severity: Severity,
}
