/// Error codes of an application.
///
/// This is intended to be implemented by a flat enum in an application that
/// defines all of its errors.
pub trait ErrorCode: Copy + Eq {
    /// Returns the largest possible error code value.
    ///
    /// This is used to determine the number of leading `0`s when formatting the
    /// error message.
    const ERROR_CODE_MAX: usize;

    /// Returns the `&str` to prefix the error code -- the `"E"` in `"E001"`.
    ///
    /// Defaults to `"E"`.
    const PREFIX: &'static str = "E";

    /// Returns the error code.
    fn code(self) -> usize;

    /// Returns a short description of the error.
    fn description(self) -> &'static str;
}
