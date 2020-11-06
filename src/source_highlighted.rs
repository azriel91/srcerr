use std::{borrow::Cow, path::Path};

use crate::ExprContext;

/// Relevant part of source data to highlight to the user.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceHighlighted<'path, 'source> {
    /// Path to the source that the source data comes from.
    pub path: Option<Cow<'path, Path>>,
    /// Context and expression that should be highlighted.
    pub expr_context: ExprContext<'source>,
}
