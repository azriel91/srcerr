//! Formatters for source errors.

pub use self::{
    plain_text_formatter::PlainTextFormatter, source_error_formatter::SourceErrorFormatter,
    styler::Styler,
};

mod plain_text_formatter;
mod source_error_formatter;
mod styler;

#[cfg(feature = "ansi_color")]
pub use self::ansi_color_formatter::AnsiColorFormatter;

#[cfg(feature = "ansi_color")]
mod ansi_color_formatter;
