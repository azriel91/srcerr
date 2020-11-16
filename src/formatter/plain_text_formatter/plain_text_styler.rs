use std::io;

use crate::Styler;

/// Provides no styling to the formatted text.
#[derive(Debug)]
pub struct PlainTextStyler;

impl<W> Styler<W> for PlainTextStyler
where
    W: io::Write,
{
    fn margin_begin(_buffer: &mut W) -> Result<(), io::Error> {
        Ok(())
    }

    fn margin_end(_buffer: &mut W) -> Result<(), io::Error> {
        Ok(())
    }

    fn error_code_begin(_buffer: &mut W) -> Result<(), io::Error> {
        Ok(())
    }

    fn error_code_end(_buffer: &mut W) -> Result<(), io::Error> {
        Ok(())
    }

    fn error_tag_begin(_buffer: &mut W) -> Result<(), io::Error> {
        Ok(())
    }

    fn error_tag_end(_buffer: &mut W) -> Result<(), io::Error> {
        Ok(())
    }

    fn error_description_begin(_buffer: &mut W) -> Result<(), io::Error> {
        Ok(())
    }

    fn error_description_end(_buffer: &mut W) -> Result<(), io::Error> {
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

    fn number_begin(_buffer: &mut W) -> Result<(), io::Error> {
        Ok(())
    }

    fn number_end(_buffer: &mut W) -> Result<(), io::Error> {
        Ok(())
    }

    fn path_begin(_buffer: &mut W) -> Result<(), io::Error> {
        Ok(())
    }

    fn path_end(_buffer: &mut W) -> Result<(), io::Error> {
        Ok(())
    }

    fn warning_tag_begin(_buffer: &mut W) -> Result<(), io::Error> {
        Ok(())
    }

    fn warning_tag_end(_buffer: &mut W) -> Result<(), io::Error> {
        Ok(())
    }
}
