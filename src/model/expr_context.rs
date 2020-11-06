use std::borrow::Cow;

use crate::{Expr, Span};

/// Line number and full line containing an expression.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExprContext<'source> {
    /// Location of the expression context in the source data.
    pub span: Span,
    /// Line number of the context within the source.
    pub line_number: usize,
    /// Full line containing the expression.
    pub line: Cow<'source, str>,
    /// Actual token or value of interest within this context.
    pub expr: Expr<'source>,
}
