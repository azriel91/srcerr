use crate::{ExprContext, Span};

/// A token or value in the source data.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Expr<'source> {
    /// Line number and full line containing the expression.
    pub context: ExprContext<'source>,
    /// Start (inclusive) and end (exclusive) positions of the value in the source data.
    pub span_source: Span,
    /// Value of the expression.
    pub value: &'source str,
}
