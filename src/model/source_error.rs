use std::marker::PhantomData;

use codespan_reporting::{
    diagnostic::{Diagnostic, Severity},
    files::Files,
};

use crate::{
    fmt::Code,
    model::{ErrorCode, ErrorDetail},
};

/// Information about an error from source data.
///
/// # Type Parameters
///
/// * `E`: [`ErrorCode`][crate::ErrorCode] type.
/// * `F`: [`Files`] referenced by this error.
///
/// [`Files`][codespan_reporting::files::Files]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceError<'files, Ec, Ed, Fs> {
    /// Code within the [`ErrorCode`] this error corresponds to.
    pub code: Ec,
    /// Suggestions or hints to provide to the user.
    pub detail: Ed,
    /// Severity level for diagnostic messages.
    pub severity: Severity,
    /// Marker.
    pub marker: PhantomData<&'files Fs>,
}

impl<'files, Ec, Ed, Fs> SourceError<'files, Ec, Ed, Fs>
where
    Ec: ErrorCode,
    Ed: ErrorDetail<'files, Files = Fs>,
    Fs: Files<'files>,
{
    /// Returns a new `SourceError`.
    pub fn new(code: Ec, detail: Ed, severity: Severity) -> Self {
        Self {
            code,
            detail,
            severity,
            marker: PhantomData,
        }
    }

    /// Returns a `Diagnostic` built from this error.
    pub fn as_diagnostic(&self, files: &Fs) -> Diagnostic<Fs::FileId> {
        let SourceError {
            code,
            detail,
            severity,
            marker: _,
        } = self;
        let code_str = Code::string(*code);
        Diagnostic::new(*severity)
            .with_code(code_str)
            .with_message(code.description())
            .with_labels(detail.labels())
            .with_notes(detail.notes(files))
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Range;

    use codespan_reporting::{
        diagnostic::{Diagnostic, Label, Severity},
        files::SimpleFiles,
    };

    use super::SourceError;
    use crate::{ErrorCode, ErrorDetail};

    #[test]
    fn as_diagnostic_passes_through_all_members() {
        let mut files = SimpleFiles::new();
        let (source_error, file_id) = {
            let file_id = files.add("path/to/file", "---\ncon: tent\n");
            let error_code = TestErrorCode;
            let error_detail = TestErrorDetail {
                file_id,
                value: String::from("tent"),
                value_byte_indices: 9..13,
            };
            let source_error = SourceError::new(error_code, error_detail, Severity::Error);

            (source_error, file_id)
        };

        let diagnostic = source_error.as_diagnostic(&files);

        assert_eq!(
            Diagnostic {
                severity: Severity::Error,
                code: Some(String::from("E01")),
                message: String::from("`chosen` value is invalid."),
                labels: vec![Label::primary(file_id, 9..13).with_message("label_message")],
                notes: vec![String::from("note_message")]
            },
            diagnostic
        );
    }

    /// Error codes for `long_expr_context` example.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct TestErrorCode;

    impl ErrorCode for TestErrorCode {
        const ERROR_CODE_MAX: usize = 10;
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
    pub struct TestErrorDetail {
        /// ID of the file containing the invalid value.
        pub file_id: usize,
        /// The value that is too long.
        pub value: String,
        /// Byte begin and end indices where the value is defined.
        pub value_byte_indices: Range<usize>,
    }

    impl<'files> ErrorDetail<'files> for TestErrorDetail {
        type Files = SimpleFiles<&'files str, &'static str>;

        fn labels(&self) -> Vec<Label<usize>> {
            let Self {
                file_id,
                value: _,
                value_byte_indices,
            } = self;

            vec![Label::primary(*file_id, value_byte_indices.clone()).with_message("label_message")]
        }

        fn notes(&self, _files: &Self::Files) -> Vec<String> {
            vec![String::from("note_message")]
        }
    }
}
