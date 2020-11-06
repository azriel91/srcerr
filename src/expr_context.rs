use std::borrow::Cow;

use crate::Span;

/// Line number and full line containing an expression.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExprContext<'source> {
    /// Location of the context in the source data.
    pub span_source: Span,
    /// Line number of the line within the source.
    pub line_number: usize,
    /// Full line containing the expression.
    pub line: Cow<'source, str>,
}
