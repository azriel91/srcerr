use std::{borrow::Cow, io, path::Path};

use srcerr::{
    ErrorCode, Expr, ExprHighlighted, PlainTextFormatter, Severity, SourceError, SourceHighlighted,
    SourceRefHint, Span, Suggestion,
};

const SOURCE_REF_HINT_YAML: &str = include_str!("source_ref_hint.yaml");

fn main() {
    // Path to file containing error.
    let path = Path::new("examples/source_ref_hint.yaml");
    // Content from the file.
    let content = SOURCE_REF_HINT_YAML;

    let value_out_of_range = value_out_of_range(&path, content);

    println!("{}", PlainTextFormatter::fmt(&value_out_of_range));
}

fn value_out_of_range<'path, 'source>(
    path: &'path Path,
    content: &'source str,
) -> SourceError<'path, 'source, SourceRefHintErrorCode<'source>> {
    let error_code = SourceRefHintErrorCode::InvalidValue {
        value: &content[45..48],
    };
    let expr = {
        let inner = Expr {
            span: Span { start: 44, end: 49 },
            line_number: 6,
            col_number: 9,
            value: Cow::Borrowed(&content[44..49]),
        };
        ExprHighlighted { inner, hint: None }
    };
    let expr_context = {
        let inner = Expr {
            span: Span { start: 36, end: 49 },
            line_number: 6,
            col_number: 1,
            value: Cow::Borrowed(&content[36..49]),
        };
        ExprHighlighted { inner, hint: None }
    };
    let invalid_source = SourceHighlighted {
        path: Some(Cow::Borrowed(path)),
        expr_context,
        expr: Some(expr),
    };
    let suggestion_0 = {
        let valid_exprs = [&content[20..23], &content[30..33]]
            .iter()
            .map(|s| Cow::Borrowed(*s))
            .collect::<Vec<_>>();
        Suggestion::ValidExprs(valid_exprs)
    };
    let suggestion_1 = {
        let expr_context = {
            let inner = Expr {
                span: Span { start: 4, end: 34 },
                line_number: 2,
                col_number: 1,
                value: Cow::Borrowed(&content[4..34]),
            };
            ExprHighlighted { inner, hint: None }
        };
        let expr = {
            let inner = Expr {
                span: Span { start: 4, end: 14 },
                line_number: 2,
                col_number: 1,
                value: Cow::Borrowed(&content[4..14]),
            };
            ExprHighlighted {
                inner,
                hint: Some(Cow::Borrowed("first defined here")),
            }
        };
        let source_ref = SourceHighlighted {
            path: Some(Cow::Borrowed(path)),
            expr_context,
            expr: Some(expr),
        };
        Suggestion::SourceRefHint(SourceRefHint {
            source_ref,
            description: String::from("`chosen` value must come from one of `available` values"),
        })
    };
    let suggestions = vec![suggestion_0, suggestion_1];
    let severity = Severity::Deny;

    SourceError {
        error_code,
        invalid_source,
        suggestions,
        severity,
    }
}

/// Error codes for source_ref_hint example.
#[derive(Debug)]
pub enum SourceRefHintErrorCode<'source> {
    /// Error when a value is invalid.
    InvalidValue {
        /// The invalid value.
        value: &'source str,
    },
}

impl<'source> ErrorCode for SourceRefHintErrorCode<'source> {
    const ERROR_CODE_MAX: usize = 1;
    const PREFIX: &'static str = "E";

    fn code(&self) -> usize {
        match self {
            Self::InvalidValue { .. } => 1,
        }
    }

    fn fmt_description<W>(&self, buffer: &mut W) -> Result<(), io::Error>
    where
        W: io::Write,
    {
        match self {
            Self::InvalidValue { value } => {
                write!(buffer, "`chosen` value `{}` is invalid.", value)
            }
        }
    }
}
