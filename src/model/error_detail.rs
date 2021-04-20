use codespan_reporting::{diagnostic::Label, files::Files};

/// Detail of a specific error.
///
/// While [`ErrorCode`] represents a class of error, `ErrorDetail` captures the
/// information for this specific instance of the error.
///
/// [`ErrorCode`]: crate::ErrorCode
pub trait ErrorDetail<'files> {
    /// Type of the collection of data that the error arises from.
    ///
    /// Each data file may be valid on its own, but the interaction of values
    /// may produce the error.
    type Files: Files<'files>;

    /// Returns [`Label`]s used for rendering.
    ///
    /// These are the parts of the file content to be  highlighted to the
    /// user, i.e. the parts underlined with `-` or `^` in the following
    /// snippet:
    ///
    /// ```yaml
    ///  7 | available: ["abc", "def"]
    ///    |            -------------- allowed values are defined here
    ///  8 | selected: "ghi"
    ///    |           ^^^^^ this is not an available value.
    /// ```
    fn labels(&self) -> Vec<Label<<Self::Files as Files<'files>>::FileId>>;

    /// Returns the notes to display beneath the error snippets.
    ///
    /// These appear beneath the code snippet, e.g. the text following the `=`
    /// sign in the following snippet:
    ///
    /// ```yaml
    ///  8 | selected: "ghi"
    ///    |           ^^^^^ this is not an available value.
    ///    |
    ///    = `selected` value must come from one of the `available` values.
    /// ```
    fn notes(&self, files: &Self::Files) -> Vec<String>;
}
