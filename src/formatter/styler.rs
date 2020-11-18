use std::io;

/// Provides styling and layout information when formatting an error.
pub trait Styler<W>
where
    W: io::Write,
{
    /// `&str` to use for newline. Defaults to `"\n"`.
    const NEWLINE: &'static str = "\n";
    /// `&str` to use for the margin line.
    const MARGIN_LINE: &'static str = "|";
    /// `&str` to use for the error marker.
    ///
    /// This is assumed to render as 1 character wide, though it may be made up
    /// of multiple characters.
    const ERROR_MARKER: &'static str = "^";
    /// `&str` to use for the vertical arrow body for an error marker.
    ///
    /// This is assumed to render as 1 character wide, though it may be made up
    /// of multiple characters.
    const ERROR_MARKER_VERTICAL: &'static str = "|";
    /// `&str` to use for the hint marker.
    ///
    /// This is assumed to render as 1 character wide, though it may be made up
    /// of multiple characters.
    const HINT_MARKER: &'static str = "-";

    /// Writes a token before writing all of the margin lines.
    fn margin_begin(buffer: &mut W) -> Result<(), io::Error>;
    /// Writes a token after writing all of the margin lines.
    fn margin_end(buffer: &mut W) -> Result<(), io::Error>;

    /// Writes a token before writing the error code -- `E001`.
    fn error_code_begin(buffer: &mut W) -> Result<(), io::Error>;
    /// Writes a token after writing the error code -- `E001`.
    fn error_code_end(buffer: &mut W) -> Result<(), io::Error>;

    /// Writes a token before writing the error tag -- `error[` and `]`.
    fn error_tag_begin(buffer: &mut W) -> Result<(), io::Error>;
    /// Writes a token after writing the error tag -- `error[` and `]`.
    fn error_tag_end(buffer: &mut W) -> Result<(), io::Error>;

    /// Writes a token before writing the error description.
    fn error_description_begin(buffer: &mut W) -> Result<(), io::Error>;
    /// Writes a token after writing the error description.
    fn error_description_end(buffer: &mut W) -> Result<(), io::Error>;

    /// Writes a token before writing all of the [error markers].
    ///
    /// [error markers]: `Styler::ERROR_MARKER`
    fn error_marker_begin(buffer: &mut W) -> Result<(), io::Error>;
    /// Writes a token after writing all of the [error markers].
    ///
    /// [error markers]: `Styler::ERROR_MARKER`
    fn error_marker_end(buffer: &mut W) -> Result<(), io::Error>;

    /// Writes a token before writing all of the hint markers.
    fn hint_marker_begin(buffer: &mut W) -> Result<(), io::Error>;
    /// Writes a token after writing all of the hint markers.
    fn hint_marker_end(buffer: &mut W) -> Result<(), io::Error>;

    /// Writes a token before writing an error hint.
    fn hint_error_begin(buffer: &mut W) -> Result<(), io::Error>;
    /// Writes a token after writing an error hint.
    fn hint_error_end(buffer: &mut W) -> Result<(), io::Error>;
    /// Writes a token before writing an info hint.
    fn hint_info_begin(buffer: &mut W) -> Result<(), io::Error>;
    /// Writes a token after writing an info hint.
    fn hint_info_end(buffer: &mut W) -> Result<(), io::Error>;

    /// Writes a token before writing a line or column number.
    fn number_begin(buffer: &mut W) -> Result<(), io::Error>;
    /// Writes a token after writing a line or column number.
    fn number_end(buffer: &mut W) -> Result<(), io::Error>;

    /// Writes a token before writing the source path.
    fn path_begin(buffer: &mut W) -> Result<(), io::Error>;
    /// Writes a token after writing the source path.
    fn path_end(buffer: &mut W) -> Result<(), io::Error>;

    /// Writes a token before writing the warning tag -- `warning[` and `]`.
    ///
    /// There are no corresponding `warning_code_*` or `warning_description_*`
    /// methods, as these are assumed to use the same styles as the `error_*`
    /// format.
    fn warning_tag_begin(buffer: &mut W) -> Result<(), io::Error>;
    /// Writes a token after writing the warning tag -- `warning[` and `]`.
    ///
    /// There are no corresponding `warning_code_*` or `warning_description_*`
    /// methods, as these are assumed to use the same styles as the `error_*`
    /// format.
    fn warning_tag_end(buffer: &mut W) -> Result<(), io::Error>;
}
