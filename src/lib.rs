#![deny(missing_docs, missing_debug_implementations)]

//! User friendly errors from source data.

pub use crate::{
    formatter::{PlainTextFormatter, SourceErrorFormatter, Styler},
    model::{
        ErrorCode, Expr, ExprHighlighted, HighlightLevel, Severity, SourceError, SourceHighlighted,
        SourceRefHint, Suggestion,
    },
};

#[cfg(feature = "ansi_color")]
pub use crate::formatter::AnsiColorFormatter;
#[cfg(feature = "ansi_color")]
pub use crate::formatter::AnsiColorFormatter as DefaultFormatter;
#[cfg(not(feature = "ansi_color"))]
pub use crate::formatter::PlainTextFormatter as DefaultFormatter;

pub mod formatter;
pub mod model;
