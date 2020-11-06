use std::io;

/// Error code contract of an application.
///
/// This is intended to be implemented by an enum in an application that defines
/// all of its errors.
pub trait ErrorCode {
    /// Returns the error code.
    fn code(&self) -> u32;

    /// Returns a short description of the error.
    fn description<W>(&self, buffer: &mut W) -> Result<(), io::Error>
    where
        W: io::Write;

    /// Returns the `&str` to prefix the error code -- the `"E"` in `"E001"`.
    ///
    /// Defaults to `"E"`.
    fn prefix() -> &'static str {
        "E"
    }

    /// Returns the total number of errors in the list of errors.
    ///
    /// This is used to determine the number of leading `0`s when formatting the
    /// error message.
    fn error_count() -> u32;
}
