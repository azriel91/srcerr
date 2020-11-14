use std::borrow::Cow;

use crate::Expr;

/// Expression to highlight and an optional hint.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExprHighlighted<'source> {
    /// The expression value to highlight.
    pub inner: Expr<'source>,
    /// Optional hint to display alongside the expression.
    pub hint: Option<Cow<'source, str>>,
}
