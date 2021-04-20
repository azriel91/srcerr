use std::{borrow::Cow, ops::RangeInclusive, path::Path};

use srcerr::{
    codespan::{FileId, Files, Span},
    codespan_reporting::{
        diagnostic::{Label, Severity},
        files::Error,
        term,
        term::termcolor::{ColorChoice, StandardStream},
    },
    fmt::Note,
    ErrorCode, ErrorDetail, SourceError,
};

const SIMPLE_TOML: &str = include_str!("simple.toml");

fn main() -> Result<(), Error> {
    // Path to file containing error.
    let path = Path::new("examples/simple.toml");
    // Content from the file.
    let content = SIMPLE_TOML;

    let mut files = Files::<Cow<'_, str>>::new();
    let path_display = path.display().to_string();
    let file_id = files.add(path_display.as_str(), Cow::Borrowed(content));
    let content = files.source(file_id);

    let value_out_of_range = value_out_of_range(file_id);
    let value_out_of_range = value_out_of_range.as_diagnostic(&files);
    let string_too_long = string_too_long(file_id, content);
    let string_too_long = string_too_long.as_diagnostic(&files);

    let writer = StandardStream::stderr(ColorChoice::Always);
    let config = term::Config::default();
    term::emit(&mut writer.lock(), &config, &files, &value_out_of_range)?;
    term::emit(&mut writer.lock(), &config, &files, &string_too_long)?;

    Ok(())
}

fn value_out_of_range<'f>(
    file_id: FileId,
) -> SourceError<'f, SimpleErrorCode, SimpleErrorDetail, Files<Cow<'f, str>>> {
    let error_code = SimpleErrorCode::ValueOutOfRange;
    let error_detail = SimpleErrorDetail::ValueOutOfRange {
        file_id,
        value: -1,
        value_byte_indices: Span::from(21..23),
        range: 1..=3,
    };
    let severity = Severity::Error;

    SourceError::new(error_code, error_detail, severity)
}

fn string_too_long<'f>(
    file_id: FileId,
    content: &str,
) -> SourceError<'f, SimpleErrorCode, SimpleErrorDetail, Files<Cow<'f, str>>> {
    let error_code = SimpleErrorCode::StringTooLong;
    let error_detail = SimpleErrorDetail::StringTooLong {
        file_id,
        value: content[40..47].to_string(),
        value_byte_indices: Span::from(39..48),
        limit: 5,
    };
    let severity = Severity::Error;

    SourceError::new(error_code, error_detail, severity)
}

/// Error codes for simple example.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SimpleErrorCode {
    /// Error when a value is out of range.
    ValueOutOfRange,
    /// Error when a string is too long.
    StringTooLong,
}

impl ErrorCode for SimpleErrorCode {
    const ERROR_CODE_MAX: usize = 2;
    const PREFIX: &'static str = "E";

    fn code(self) -> usize {
        match self {
            Self::ValueOutOfRange => 1,
            Self::StringTooLong => 2,
        }
    }

    fn description(self) -> &'static str {
        match self {
            Self::ValueOutOfRange => "Value out of range.",
            Self::StringTooLong => "String provided is too long.",
        }
    }
}

/// Error detail for simple example.
#[derive(Debug)]
pub enum SimpleErrorDetail {
    /// Error when a value is out of range.
    ValueOutOfRange {
        /// ID of the file containing the invalid value.
        file_id: FileId,
        /// The value.
        value: i32,
        /// Byte begin and end indices where the value is defined.
        value_byte_indices: Span,
        /// Range that the value must be within.
        range: RangeInclusive<u32>,
    },
    /// Error when a string is too long.
    StringTooLong {
        /// ID of the file containing the invalid value.
        file_id: FileId,
        /// The value that is too long.
        value: String,
        /// Byte begin and end indices where the value is defined.
        value_byte_indices: Span,
        /// Maximum length allowed for the string.
        limit: usize,
    },
}

impl<'files> ErrorDetail<'files> for SimpleErrorDetail {
    type Files = Files<Cow<'files, str>>;

    fn labels(&self) -> Vec<Label<FileId>> {
        match self {
            Self::ValueOutOfRange {
                file_id,
                value_byte_indices,
                range,
                ..
            } => {
                vec![
                    Label::primary(*file_id, value_byte_indices.clone()).with_message(format!(
                        "not within the range: `{}..={}`",
                        range.start(),
                        range.end()
                    )),
                ]
            }
            Self::StringTooLong {
                file_id,
                value_byte_indices,
                limit,
                ..
            } => {
                vec![
                    Label::primary(*file_id, value_byte_indices.clone())
                        .with_message(format!("exceeds the {} character limit.", limit)),
                ]
            }
        }
    }

    fn notes(&self, _files: &Self::Files) -> Vec<String> {
        match self {
            Self::ValueOutOfRange { range, .. } => {
                let valid_exprs = range.clone().map(|n| Cow::Owned(n.to_string()));
                let suggestion = Note::valid_exprs(valid_exprs).expect("Failed to format note.");
                vec![suggestion]
            }
            Self::StringTooLong { .. } => vec![],
        }
    }
}
