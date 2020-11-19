use crate::SourceErrorFormatter;

pub use self::ansi_color_styler::AnsiColorStyler;

mod ansi_color_styler;

/// Formats a [`SourceError`] as plain text.
///
/// [`SourceError`]: `crate::SourceError`
pub type AnsiColorFormatter<W> = SourceErrorFormatter<W, AnsiColorStyler>;
