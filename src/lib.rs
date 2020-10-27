#![deny(missing_docs, missing_debug_implementations)]

//! User friendly errors from source data.

pub use crate::{
    expr::Expr, expr_context::ExprContext, severity::Severity, source_error::SourceError,
    source_highlighted::SourceHighlighted, span::Span, suggestion::Suggestion,
};

mod expr;
mod expr_context;
mod severity;
mod source_error;
mod source_highlighted;
mod span;
mod suggestion;
