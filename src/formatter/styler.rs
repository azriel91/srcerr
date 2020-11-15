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
    /// `&str` to use for the hint marker.
    ///
    /// This is assumed to render as 1 character wide, though it may be made up
    /// of multiple characters.
    const HINT_MARKER: &'static str = "-";

    /// Writes a token before writing all of the margin lines.
    fn margin_line_begin(buffer: &mut W) -> Result<(), io::Error>;
    /// Writes a token after writing all of the margin lines.
    fn margin_line_end(buffer: &mut W) -> Result<(), io::Error>;

    /// Writes a token before writing all of the error markers.
    fn error_marker_begin(buffer: &mut W) -> Result<(), io::Error>;
    /// Writes a token after writing all of the error markers.
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
}
