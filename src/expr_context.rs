use crate::Span;

/// Line number and full line containing an expression.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExprContext<'source> {
    /// Start (inclusive) and end (exclusive) positions of the line in the source data.
    pub span_source: Span,
    /// Line number of the line within the source.
    pub line_number: usize,
    /// Full line containing the expression.
    pub line: &'source str,
}
