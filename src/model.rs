//! Data types representing the source error.

pub use self::{
    error_code::ErrorCode, expr::Expr, expr_context::ExprContext, severity::Severity,
    source_error::SourceError, source_highlighted::SourceHighlighted,
    source_ref_hint::SourceRefHint, span::Span, suggestion::Suggestion,
};

mod error_code;
mod expr;
mod expr_context;
mod severity;
mod source_error;
mod source_highlighted;
mod source_ref_hint;
mod span;
mod suggestion;
