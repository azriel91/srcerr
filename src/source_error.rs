use std::path::Path;

use crate::{Expr, Severity, Suggestion};

/// Information about an error from source data.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceError<'path, 'source> {
    /// Path to the source that the source data comes from.
    pub path: &'path Path,
    /// Expression that this error is for.
    pub expr: Expr<'source>,
    /// Suggestion or hint to provide to the user.
    pub suggestion: Option<Suggestion<'source>>,
    /// Whether this is a denied error or warning.
    pub severity: Severity,
}
