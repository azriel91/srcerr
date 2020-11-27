use std::borrow::Cow;

/// A value or section of the source data.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Expr<'source> {
    /// Line number of the expression within the source.
    pub line_number: usize,
    /// Column number of the expression within the source.
    pub col_number: usize,
    /// Value of the expression.
    pub value: Cow<'source, str>,
}
