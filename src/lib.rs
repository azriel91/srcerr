#![deny(missing_docs, missing_debug_implementations)]

//! User friendly errors from source data.

pub use self::model::{
    ErrorCode, Expr, ExprContext, Severity, SourceError, SourceHighlighted, SourceRefHint, Span,
    Suggestion,
};

pub mod model;
