use std::{borrow::Cow, io, ops::Range, path::Path};

use srcerr::{
    ErrorCode, Expr, ExprContext, PlainTextFormatter, Severity, SourceError, SourceHighlighted,
    Span,
};

const SIMPLE_TOML: &str = include_str!("simple.toml");

fn main() {
    let error_code = SimpleErrorCode::ValueOutOfRange {
        value: -5,
        range: -3..6,
    };
    let path = Path::new("examples/simple.toml");
    let expr = Expr {
        span: Span { start: 21, end: 23 },
        line_number: 2,
        col_number: 13,
        value: Cow::Borrowed(&SIMPLE_TOML[21..23]),
    };
    let expr_context = ExprContext {
        span: Span { start: 9, end: 23 },
        line_number: 2,
        col_number: 1,
        value: Cow::Borrowed(&SIMPLE_TOML[9..23]),
        expr,
    };
    let invalid_source = SourceHighlighted {
        path: Some(Cow::Borrowed(path)),
        expr_context,
    };
    let suggestions = vec![];
    let severity = Severity::Deny;

    let source_error = SourceError {
        error_code,
        invalid_source,
        suggestions,
        severity,
    };

    println!("{}", PlainTextFormatter::fmt(&source_error));
}

/// Error codes for simple example.
#[derive(Debug)]
pub enum SimpleErrorCode<'source> {
    /// Error when a value is out of range.
    ValueOutOfRange {
        /// The value.
        value: i32,
        /// Range that the value must be within.
        range: Range<i32>,
    },
    /// Error when a string is too long.
    StringTooLong {
        /// The value that is too long.
        value: &'source str,
        /// Maximum length allowed for the string.
        limit: usize,
    },
}

impl<'source> ErrorCode for SimpleErrorCode<'source> {
    fn code(&self) -> usize {
        match self {
            Self::ValueOutOfRange { .. } => 1,
            Self::StringTooLong { .. } => 2,
        }
    }

    fn fmt_description<W>(&self, buffer: &mut W) -> Result<(), io::Error>
    where
        W: io::Write,
    {
        match self {
            Self::ValueOutOfRange { value, .. } => write!(buffer, "`{}` is out of range.", value),
            Self::StringTooLong { value, .. } => write!(buffer, "`{}` is too long.", value),
        }
    }

    fn error_count() -> usize {
        2
    }
}
