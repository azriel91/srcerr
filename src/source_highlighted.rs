use std::path::Path;

use crate::Expr;

/// Relevant part of source data to highlight to the user.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceHighlighted<'path, 'source> {
    /// Path to the source that the source data comes from.
    pub path: &'path Path,
    /// Expression that should be highlighted.
    pub expr: Expr<'source>,
}
