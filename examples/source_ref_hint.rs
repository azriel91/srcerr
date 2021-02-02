use std::{ops::Range, path::Path};

use srcerr::{
    codespan_reporting::{
        diagnostic::{Label, Severity},
        files::{Error, Files, SimpleFiles},
        term,
        term::termcolor::{ColorChoice, StandardStream},
    },
    ErrorCode, ErrorDetail, SourceError,
};

const SOURCE_REF_HINT_YAML: &str = include_str!("source_ref_hint.yaml");

fn main() -> Result<(), Error> {
    // Path to file containing error.
    let path = Path::new("examples/source_ref_hint.yaml");
    // Content from the file.
    let content = SOURCE_REF_HINT_YAML;

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
) -> SourceError<'f, SourceRefHintErrorCode, SourceRefHintErrorDetail, SimpleFiles<&'f str, &'f str>>
{
    let error_code = SourceRefHintErrorCode;
    let error_detail = SourceRefHintErrorDetail {
        file_id,
        value: content[45..48].to_string(),
        value_byte_indices: 44..49,
        valid_values: vec![content[20..23].to_string(), content[30..33].to_string()],
        valid_values_byte_indices: 4..34,
    };
    let severity = Severity::Error;

    SourceError::new(error_code, error_detail, severity)
}

/// Error codes for source_ref_hint example.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SourceRefHintErrorCode;

impl ErrorCode for SourceRefHintErrorCode {
    const ERROR_CODE_MAX: usize = 2;
    const PREFIX: &'static str = "E";

    fn code(self) -> usize {
        1
    }

    fn description(self) -> &'static str {
        "`chosen` value is invalid."
    }
}

/// Error detail for source_ref_hint example.
#[derive(Debug)]
pub struct SourceRefHintErrorDetail {
    /// ID of the file containing the invalid value.
    pub file_id: usize,
    /// The value that is too long.
    pub value: String,
    /// Byte begin and end indices where the value is defined.
    pub value_byte_indices: Range<usize>,
    /// Valid values available.
    pub valid_values: Vec<String>,
    /// Where the valid values are defined.
    pub valid_values_byte_indices: Range<usize>,
}

impl<'files> ErrorDetail<'files> for SourceRefHintErrorDetail {
    type Files = SimpleFiles<&'files str, &'files str>;

    fn labels(&self) -> Vec<Label<usize>> {
        let Self {
            file_id,
            value: _,
            value_byte_indices,
            valid_values: _,
            valid_values_byte_indices,
        } = self;

        vec![
            Label::primary(*file_id, value_byte_indices.clone())
                .with_message("invalid value specified"),
            Label::secondary(*file_id, valid_values_byte_indices.clone())
                .with_message("defined here"),
        ]
    }

    fn notes(&self, _files: &Self::Files) -> Vec<String> {
        vec![String::from(
            "`chosen` value must come from one of `available` values",
        )]
    }
}
