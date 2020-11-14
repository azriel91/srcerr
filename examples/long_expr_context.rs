use std::{
    borrow::Cow,
    fs::File,
    io,
    io::{BufReader, Read},
    path::Path,
};

use srcerr::{
    ErrorCode, Expr, ExprHighlighted, PlainTextFormatter, Severity, SourceError, SourceHighlighted,
    Span, Suggestion,
};

fn main() -> Result<(), io::Error> {
    let path = Path::new(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/examples/long_expr_context.json"
    ));
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut content = String::new();
    buf_reader.read_to_string(&mut content)?;

    let value_out_of_range = value_out_of_range(&path, &content);

    println!("{}", PlainTextFormatter::fmt(&value_out_of_range));

    Ok(())
}

fn value_out_of_range<'path, 'source>(
    path: &'path Path,
    content: &'source str,
) -> SourceError<'path, 'source, ValueInvalid<'source>> {
    let error_code = ValueInvalid {
        value: &content[100..103],
    };
    let expr = {
        let inner = Expr {
            span: Span {
                start: 100,
                end: 103,
            },
            line_number: 1,
            col_number: 101,
            value: Cow::Borrowed(&content[100..103]),
        };
        ExprHighlighted { inner, hint: None }
    };
    let expr_context = {
        let inner = Expr {
            span: Span {
                start: 96,
                end: 104,
            },
            line_number: 1,
            col_number: 97,
            value: Cow::Borrowed(&content[96..104]),
        };
        ExprHighlighted { inner, hint: None }
    };
    let invalid_source = SourceHighlighted {
        path: Some(Cow::Borrowed(path)),
        expr_context,
        expr: Some(expr),
    };
    let suggestion_0 = Suggestion::Hint("expected value to be less than 26");
    let suggestions = vec![suggestion_0];

    let severity = Severity::Deny;

    SourceError {
        error_code,
        invalid_source,
        suggestions,
        severity,
    }
}

/// Error codes for the `long_expr_context` example.
#[derive(Debug)]
pub struct ValueInvalid<'source> {
    /// The invalid value.
    value: &'source str,
}

impl<'source> ErrorCode for ValueInvalid<'source> {
    const ERROR_CODE_MAX: usize = 1;
    const PREFIX: &'static str = "E";

    fn code(&self) -> usize {
        1
    }

    fn fmt_description<W>(&self, buffer: &mut W) -> Result<(), io::Error>
    where
        W: io::Write,
    {
        write!(buffer, "Value `{}` is invalid.", self.value)
    }
}
