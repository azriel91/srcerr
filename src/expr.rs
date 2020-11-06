use std::borrow::Cow;

use crate::{ExprContext, Span};

/// A token or value in the source data.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Expr<'source> {
    /// Line number and full line containing the expression.
    pub context: ExprContext<'source>,
    /// Location of the expression in the source data.
    pub span_source: Span,
    /// Value of the expression.
    pub value: Cow<'source, str>,
}
