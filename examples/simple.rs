use std::{borrow::Cow, io, ops::RangeInclusive, path::Path};

use srcerr::{
    DefaultFormatter, ErrorCode, Expr, ExprHighlighted, Severity, SourceError, SourceHighlighted,
    Span, Suggestion,
};

const SIMPLE_TOML: &str = include_str!("simple.toml");

fn main() {
    // Path to file containing error.
    let path = Path::new("examples/simple.toml");
    // Content from the file.
    let content = SIMPLE_TOML;

    let value_out_of_range = value_out_of_range(&path, content);
    let string_too_long = string_too_long(&path, content);

    println!("{}", DefaultFormatter::fmt(&value_out_of_range));
    println!("{}", DefaultFormatter::fmt(&string_too_long));
}

fn value_out_of_range<'path, 'source>(
    path: &'path Path,
    content: &'source str,
) -> SourceError<'path, 'source, SimpleErrorCode<'source>> {
    let range = 1..=3;
    let error_code = SimpleErrorCode::ValueOutOfRange {
        value: -1,
        range: range.clone(),
    };
    let expr = {
        let inner = Expr {
            span: Span { start: 21, end: 23 },
            line_number: 2,
            col_number: 13,
            value: Cow::Borrowed(&content[21..23]),
        };
        ExprHighlighted { inner, hint: None }
    };
    let expr_context = {
        let inner = Expr {
            span: Span { start: 9, end: 23 },
            line_number: 2,
            col_number: 1,
            value: Cow::Borrowed(&content[9..23]),
        };
        ExprHighlighted { inner, hint: None }
    };
    let invalid_source = SourceHighlighted {
        path: Some(Cow::Borrowed(path)),
        expr_context,
        expr: Some(expr),
    };
    let valid_exprs = range
        .map(|n| n.to_string())
        .map(Cow::Owned)
        .collect::<Vec<_>>();
    let suggestion_0 = Suggestion::ValidExprs(valid_exprs);
    let suggestions = vec![suggestion_0];
    let severity = Severity::Deny;

    SourceError {
        error_code,
        invalid_source,
        suggestions,
        severity,
    }
}

fn string_too_long<'path, 'source>(
    path: &'path Path,
    content: &'source str,
) -> SourceError<'path, 'source, SimpleErrorCode<'source>> {
    let error_code = SimpleErrorCode::StringTooLong {
        value: &content[40..47],
        limit: 5,
    };
    let expr = {
        let inner = Expr {
            span: Span { start: 39, end: 48 },
            line_number: 3,
            col_number: 16,
            value: Cow::Borrowed(&content[39..48]),
        };
        ExprHighlighted { inner, hint: None }
    };
    let expr_context = {
        let inner = Expr {
            span: Span { start: 24, end: 48 },
            line_number: 3,
            col_number: 1,
            value: Cow::Borrowed(&content[24..48]),
        };
        ExprHighlighted { inner, hint: None }
    };
    let invalid_source = SourceHighlighted {
        path: Some(Cow::Borrowed(path)),
        expr_context,
        expr: Some(expr),
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
        range: RangeInclusive<u32>,
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
            Self::ValueOutOfRange { value, range } => write!(
                buffer,
                "`{}` is out of the range: `{}..{}`.",
                value,
                range.start(),
                range.end()
            ),
            Self::StringTooLong { value, limit } => {
                write!(buffer, "`{}` exceeds the {} character limit.", value, limit)
            }
        }
    }
}
