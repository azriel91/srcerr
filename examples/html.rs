use std::{
    borrow::Cow,
    fs::File,
    io,
    io::{BufReader, Read},
    path::Path,
};

use srcerr::{
    ErrorCode, Expr, ExprHighlighted, Severity, SourceError, SourceHighlighted, Span, Suggestion,
};

use crate::formatter::HtmlFormatter;

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

    println!(
        "<div style=\"font-family: monospace;\">{}</div>",
        HtmlFormatter::fmt(&value_out_of_range)
    );

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

mod formatter {
    use std::io;

    use srcerr::{SourceErrorFormatter, Styler};

    /// Formats a [`SourceError`] with HTML tags.
    ///
    /// [`SourceError`]: `srcerr::SourceError`
    pub type HtmlFormatter<W> = SourceErrorFormatter<W, HtmlStyler>;

    /// Provides HTML styling to the formatted text.
    #[derive(Debug)]
    pub struct HtmlStyler;

    const BLUE_BOLD: &str = r#"<span style="font-weight: bold; color: #5b7fa6;">"#;
    const RED_BOLD: &str = r#"<span style="font-weight: bold; color: #bf2121;">"#;
    const YELLOW_BOLD: &str = r#"<span style="font-weight: bold; color: #9d8000;">"#;
    const BOLD: &str = r#"<span style="font-weight: bold;">"#;
    const SPAN_CLOSE: &str = "</span>";

    impl<W> Styler<W> for HtmlStyler
    where
        W: io::Write,
    {
        const NEWLINE: &'static str = "<br />\n";
        const SPACE: &'static str = "&nbsp;";

        fn margin_begin(buffer: &mut W) -> Result<(), io::Error> {
            write!(buffer, "{}", BLUE_BOLD)
        }

        fn margin_end(buffer: &mut W) -> Result<(), io::Error> {
            write!(buffer, "{}", SPAN_CLOSE)
        }

        fn error_code_begin(buffer: &mut W) -> Result<(), io::Error> {
            write!(buffer, "{}", RED_BOLD)
        }

        fn error_code_end(buffer: &mut W) -> Result<(), io::Error> {
            write!(buffer, "{}", SPAN_CLOSE)
        }

        fn error_tag_begin(buffer: &mut W) -> Result<(), io::Error> {
            write!(buffer, "{}", RED_BOLD)
        }

        fn error_tag_end(buffer: &mut W) -> Result<(), io::Error> {
            write!(buffer, "{}", SPAN_CLOSE)
        }

        fn error_description_begin(buffer: &mut W) -> Result<(), io::Error> {
            write!(buffer, "{}", RED_BOLD)
        }

        fn error_description_end(buffer: &mut W) -> Result<(), io::Error> {
            write!(buffer, "{}", SPAN_CLOSE)
        }

        fn error_marker_begin(buffer: &mut W) -> Result<(), io::Error> {
            write!(buffer, "{}", RED_BOLD)
        }

        fn error_marker_end(buffer: &mut W) -> Result<(), io::Error> {
            write!(buffer, "{}", SPAN_CLOSE)
        }

        fn hint_marker_begin(buffer: &mut W) -> Result<(), io::Error> {
            write!(buffer, "{}", BLUE_BOLD)
        }

        fn hint_marker_end(buffer: &mut W) -> Result<(), io::Error> {
            write!(buffer, "{}", SPAN_CLOSE)
        }

        fn hint_error_begin(buffer: &mut W) -> Result<(), io::Error> {
            write!(buffer, "{}", BLUE_BOLD)
        }

        fn hint_error_end(buffer: &mut W) -> Result<(), io::Error> {
            write!(buffer, "{}", SPAN_CLOSE)
        }

        fn hint_info_begin(buffer: &mut W) -> Result<(), io::Error> {
            write!(buffer, "{}", BLUE_BOLD)
        }

        fn hint_info_end(buffer: &mut W) -> Result<(), io::Error> {
            write!(buffer, "{}", SPAN_CLOSE)
        }

        fn number_begin(buffer: &mut W) -> Result<(), io::Error> {
            write!(buffer, "{}", BOLD)
        }

        fn number_end(buffer: &mut W) -> Result<(), io::Error> {
            write!(buffer, "{}", SPAN_CLOSE)
        }

        fn path_begin(buffer: &mut W) -> Result<(), io::Error> {
            write!(buffer, "{}", BOLD)
        }

        fn path_end(buffer: &mut W) -> Result<(), io::Error> {
            write!(buffer, "{}", SPAN_CLOSE)
        }

        fn warning_marker_begin(buffer: &mut W) -> Result<(), io::Error> {
            write!(buffer, "{}", YELLOW_BOLD)
        }

        fn warning_marker_end(buffer: &mut W) -> Result<(), io::Error> {
            write!(buffer, "{}", YELLOW_BOLD)
        }

        fn warning_tag_begin(buffer: &mut W) -> Result<(), io::Error> {
            write!(buffer, "{}", YELLOW_BOLD)
        }

        fn warning_tag_end(buffer: &mut W) -> Result<(), io::Error> {
            write!(buffer, "{}", YELLOW_BOLD)
        }
    }
}
