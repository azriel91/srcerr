use std::{borrow::Cow, io, ops::Range, path::Path};

use srcerr::{
    ErrorCode, Expr, ExprContext, PlainTextFormatter, Severity, SourceError, SourceHighlighted,
    Span,
};

const SIMPLE_TOML: &str = include_str!("simple.toml");

fn main() {
    // Path to file containing error.
    let path = Path::new("examples/simple.toml");
    // Content from the file.
    let content = SIMPLE_TOML;

    let source_error = value_out_of_range(&path, content);

    println!("{}", PlainTextFormatter::fmt(&source_error));
}

fn value_out_of_range<'path, 'source>(
    path: &'path Path,
    content: &'source str,
) -> SourceError<'path, 'source, SimpleErrorCode<'source>> {
    let error_code = SimpleErrorCode::ValueOutOfRange {
        value: -5,
        range: -3..6,
    };
    let expr = Expr {
        span: Span { start: 21, end: 23 },
        line_number: 2,
        col_number: 13,
        value: Cow::Borrowed(&content[21..23]),
    };
    let expr_context = ExprContext {
        span: Span { start: 9, end: 23 },
        line_number: 2,
        col_number: 1,
        value: Cow::Borrowed(&content[9..23]),
        expr,
    };
    let invalid_source = SourceHighlighted {
        path: Some(Cow::Borrowed(path)),
        expr_context,
    };
    let suggestions = vec![];
    let severity = Severity::Deny;

    SourceError {
        error_code,
        invalid_source,
        suggestions,
        severity,
    }
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
    const ERROR_CODE_MAX: usize = 2;
    const PREFIX: &'static str = "E";

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
}
