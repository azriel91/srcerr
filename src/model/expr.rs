use std::borrow::Cow;

use crate::Span;

/// A token or value in the source data.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Expr<'source> {
    /// Location of the expression in the source data.
    pub span: Span,
    /// Line number of the expression within the source.
    pub line_number: usize,
    /// Column number of the expression within the source.
    pub col_number: usize,
    /// Value of the expression.
    pub value: Cow<'source, str>,
}
