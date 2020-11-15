use std::io;

use crate::Styler;

/// Provides no styling to the formatted text.
#[derive(Debug)]
pub struct PlainTextStyler;

impl<W> Styler<W> for PlainTextStyler
where
    W: io::Write,
{
    fn margin_line_begin(_buffer: &mut W) -> Result<(), io::Error> {
        Ok(())
    }

    fn margin_line_end(_buffer: &mut W) -> Result<(), io::Error> {
        Ok(())
    }

    fn error_marker_begin(_buffer: &mut W) -> Result<(), io::Error> {
        Ok(())
    }

    fn error_marker_end(_buffer: &mut W) -> Result<(), io::Error> {
        Ok(())
    }

    fn hint_marker_begin(_buffer: &mut W) -> Result<(), io::Error> {
        Ok(())
    }

    fn hint_marker_end(_buffer: &mut W) -> Result<(), io::Error> {
        Ok(())
    }

    fn hint_error_begin(_buffer: &mut W) -> Result<(), io::Error> {
        Ok(())
    }

    fn hint_error_end(_buffer: &mut W) -> Result<(), io::Error> {
        Ok(())
    }

    fn hint_info_begin(_buffer: &mut W) -> Result<(), io::Error> {
        Ok(())
    }

    fn hint_info_end(_buffer: &mut W) -> Result<(), io::Error> {
        Ok(())
    }
}
