/// Whether this is a denied error or warning.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Severity {
    /// Error should prevent the operation from progressing.
    ///
    /// Recommended when the user needs to address an issue before retrying the operation.
    Deny,
    /// Error should not prevent the operation from continuing.
    ///
    /// Recommended for informatives such as deprecation notices and convention changes.
    Warn,
}
