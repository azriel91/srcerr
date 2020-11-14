#![deny(missing_docs, missing_debug_implementations)]

//! User friendly errors from source data.

pub use crate::{
    formatter::PlainTextFormatter,
    model::{
        ErrorCode, Expr, ExprHighlighted, Severity, SourceError, SourceHighlighted, SourceRefHint,
        Span, Suggestion,
    },
};

pub mod formatter;
pub mod model;
