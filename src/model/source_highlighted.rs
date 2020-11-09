use std::{borrow::Cow, path::Path};

use crate::Expr;

/// Relevant part of source data to highlight to the user.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceHighlighted<'path, 'source> {
    /// Path to the source that the source data comes from.
    pub path: Option<Cow<'path, Path>>,
    /// Context that the expression resides in.
    ///
    /// This includes additional text around the expression, so that it is
    /// easier for a person to recognize where the value comes from.
    pub expr_context: Expr<'source>,
    /// Actual token or value of interest within this context.
    ///
    /// If the whole expression context is of interest, then this may be `None`.
    pub expr: Option<Expr<'source>>,
}
