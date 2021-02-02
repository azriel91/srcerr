use std::{
    fs::File,
    io::{BufReader, Read},
    ops::Range,
    path::Path,
};

use srcerr::{
    codespan_reporting::{
        diagnostic::{Label, Severity},
        files::{Error, Files, SimpleFiles},
        term,
        term::termcolor::{ColorChoice, StandardStream},
    },
    ErrorCode, ErrorDetail, SourceError,
};

// Truncate long lines is pending <https://github.com/brendanzab/codespan/issues/228>
fn main() -> Result<(), Error> {
    let path = Path::new(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/examples/long_expr_context.json"
    ));
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut content = String::new();
    buf_reader.read_to_string(&mut content)?;

    let mut files = SimpleFiles::new();
    let path_display = path.display().to_string();
    let file_id = files.add(path_display.as_str(), content);
    let content = files
        .source(file_id)
        .expect("Expected to get file content.");

    let invalid_value = invalid_value(file_id, content);

    let writer = StandardStream::stderr(ColorChoice::Always);
    let config = term::Config::default();
    term::emit(
        &mut writer.lock(),
        &config,
        &files,
        &invalid_value.as_diagnostic(&files),
    )?;

    Ok(())
}

fn invalid_value<'f>(
    file_id: usize,
    content: &str,
) -> SourceError<
    'f,
    LongExprContextErrorCode,
    LongExprContextErrorDetail,
    SimpleFiles<&'f str, String>,
> {
    let error_code = LongExprContextErrorCode;
    let error_detail = LongExprContextErrorDetail {
        file_id,
        value: content[100..103].to_string(),
        value_byte_indices: 100..103,
    };
    let severity = Severity::Error;

    SourceError::new(error_code, error_detail, severity)
}

/// Error codes for `long_expr_context` example.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LongExprContextErrorCode;

impl ErrorCode for LongExprContextErrorCode {
    const ERROR_CODE_MAX: usize = 2;
    const PREFIX: &'static str = "E";

    fn code(self) -> usize {
        1
    }

    fn description(self) -> &'static str {
        "`chosen` value is invalid."
    }
}

/// Error detail for `long_expr_context` example.
#[derive(Debug)]
pub struct LongExprContextErrorDetail {
    /// ID of the file containing the invalid value.
    pub file_id: usize,
    /// The value that is too long.
    pub value: String,
    /// Byte begin and end indices where the value is defined.
    pub value_byte_indices: Range<usize>,
}

impl<'files> ErrorDetail<'files> for LongExprContextErrorDetail {
    type Files = SimpleFiles<&'files str, String>;

    fn labels(&self) -> Vec<Label<usize>> {
        let Self {
            file_id,
            value: _,
            value_byte_indices,
        } = self;

        vec![
            Label::primary(*file_id, value_byte_indices.clone())
                .with_message("expected value to be less than 26"),
        ]
    }

    fn notes(&self, _files: &Self::Files) -> Vec<String> {
        vec![String::from(
            "`chosen` value must come from one of `available` values",
        )]
    }
}
