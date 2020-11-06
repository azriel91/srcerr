use crate::{Severity, SourceHighlighted, Suggestion};

/// Information about an error from source data.
///
/// # Type Parameters
///
/// * `E`: [`ErrorCode`] type.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceError<'path, 'source, E> {
    /// Code within the [`ErrorCode`] this error corresponds to.
    pub error_code: E,
    /// Source data that the erroneous value is found.
    pub invalid_source: SourceHighlighted<'path, 'source>,
    /// Suggestions or hints to provide to the user.
    pub suggestions: Vec<Suggestion<'path, 'source>>,
    /// Whether this is a denied error or warning.
    pub severity: Severity,
}
