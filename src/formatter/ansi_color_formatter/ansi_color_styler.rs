use std::io;

use ansi_term::{Color, Style};

use crate::Styler;

/// Provides ANSI color styling to the formatted text.
#[derive(Debug)]
pub struct AnsiColorStyler;

const BLUE_BOLD: Style = Style {
    foreground: Some(Color::Fixed(12)),
    background: None,
    is_bold: true,
    is_dimmed: false,
    is_italic: false,
    is_underline: false,
    is_blink: false,
    is_reverse: false,
    is_hidden: false,
    is_strikethrough: false,
};

const RED_BOLD: Style = Style {
    foreground: Some(Color::Fixed(9)),
    background: None,
    is_bold: true,
    is_dimmed: false,
    is_italic: false,
    is_underline: false,
    is_blink: false,
    is_reverse: false,
    is_hidden: false,
    is_strikethrough: false,
};

const YELLOW_BOLD: Style = Style {
    foreground: Some(Color::Yellow),
    background: None,
    is_bold: true,
    is_dimmed: false,
    is_italic: false,
    is_underline: false,
    is_blink: false,
    is_reverse: false,
    is_hidden: false,
    is_strikethrough: false,
};

const BOLD: Style = Style {
    foreground: None,
    background: None,
    is_bold: true,
    is_dimmed: false,
    is_italic: false,
    is_underline: false,
    is_blink: false,
    is_reverse: false,
    is_hidden: false,
    is_strikethrough: false,
};

impl<W> Styler<W> for AnsiColorStyler
where
    W: io::Write,
{
    fn margin_begin(buffer: &mut W) -> Result<(), io::Error> {
        write!(buffer, "{}", BLUE_BOLD.prefix())
    }

    fn margin_end(buffer: &mut W) -> Result<(), io::Error> {
        write!(buffer, "{}", BLUE_BOLD.suffix())
    }

    fn error_code_begin(buffer: &mut W) -> Result<(), io::Error> {
        write!(buffer, "{}", BOLD.prefix())
    }

    fn error_code_end(buffer: &mut W) -> Result<(), io::Error> {
        write!(buffer, "{}", BOLD.suffix())
    }

    fn error_tag_begin(buffer: &mut W) -> Result<(), io::Error> {
        write!(buffer, "{}", RED_BOLD.prefix())
    }

    fn error_tag_end(buffer: &mut W) -> Result<(), io::Error> {
        write!(buffer, "{}", RED_BOLD.suffix())
    }

    fn error_description_begin(buffer: &mut W) -> Result<(), io::Error> {
        write!(buffer, "{}", BOLD.prefix())
    }

    fn error_description_end(buffer: &mut W) -> Result<(), io::Error> {
        write!(buffer, "{}", BOLD.suffix())
    }

    fn error_marker_begin(buffer: &mut W) -> Result<(), io::Error> {
        write!(buffer, "{}", RED_BOLD.prefix())
    }

    fn error_marker_end(buffer: &mut W) -> Result<(), io::Error> {
        write!(buffer, "{}", RED_BOLD.suffix())
    }

    fn hint_marker_begin(buffer: &mut W) -> Result<(), io::Error> {
        write!(buffer, "{}", BLUE_BOLD.prefix())
    }

    fn hint_marker_end(buffer: &mut W) -> Result<(), io::Error> {
        write!(buffer, "{}", BLUE_BOLD.suffix())
    }

    fn hint_error_begin(buffer: &mut W) -> Result<(), io::Error> {
        write!(buffer, "{}", BLUE_BOLD.prefix())
    }

    fn hint_error_end(buffer: &mut W) -> Result<(), io::Error> {
        write!(buffer, "{}", BLUE_BOLD.suffix())
    }

    fn hint_info_begin(buffer: &mut W) -> Result<(), io::Error> {
        write!(buffer, "{}", BLUE_BOLD.prefix())
    }

    fn hint_info_end(buffer: &mut W) -> Result<(), io::Error> {
        write!(buffer, "{}", BLUE_BOLD.suffix())
    }

    fn number_begin(buffer: &mut W) -> Result<(), io::Error> {
        write!(buffer, "{}", BOLD.prefix())
    }

    fn number_end(buffer: &mut W) -> Result<(), io::Error> {
        write!(buffer, "{}", BOLD.suffix())
    }

    fn path_begin(buffer: &mut W) -> Result<(), io::Error> {
        write!(buffer, "{}", BOLD.prefix())
    }

    fn path_end(buffer: &mut W) -> Result<(), io::Error> {
        write!(buffer, "{}", BOLD.suffix())
    }

    fn warning_marker_begin(buffer: &mut W) -> Result<(), io::Error> {
        write!(buffer, "{}", YELLOW_BOLD.prefix())
    }

    fn warning_marker_end(buffer: &mut W) -> Result<(), io::Error> {
        write!(buffer, "{}", YELLOW_BOLD.suffix())
    }

    fn warning_tag_begin(buffer: &mut W) -> Result<(), io::Error> {
        write!(buffer, "{}", YELLOW_BOLD.prefix())
    }

    fn warning_tag_end(buffer: &mut W) -> Result<(), io::Error> {
        write!(buffer, "{}", YELLOW_BOLD.suffix())
    }
}
