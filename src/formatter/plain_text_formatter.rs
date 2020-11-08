use std::io::{self, Write};

use crate::{ErrorCode, Severity, SourceError};

/// Formats a [`SourceError`] as plain text.
#[derive(Debug)]
pub struct PlainTextFormatter;

impl PlainTextFormatter {
    /// Formats the source error as plain text.
    pub fn fmt<'f, 'path, 'source, E>(source_error: &'f SourceError<'path, 'source, E>) -> String
    where
        E: ErrorCode,
    {
        let mut buffer = Vec::new();
        Self::fmt_buffer(&mut buffer, source_error).expect("Failed to format source error.");
        String::from_utf8(buffer).expect("Source error is not valid UTF8.")
    }

    /// Formats the source error as plain text.
    pub fn fmt_buffer<'f, 'path, 'source, W, E>(
        buffer: &mut W,
        source_error: &'f SourceError<'path, 'source, E>,
    ) -> Result<(), io::Error>
    where
        W: Write,
        E: ErrorCode,
    {
        Self::fmt_error_code(buffer, source_error)?;
        Self::fmt_path(buffer, source_error)?;
        Self::fmt_error_expr(buffer, source_error)?;

        // TODO: suggestions

        Ok(())
    }

    fn fmt_error_code<'f, 'path, 'source, W, E>(
        buffer: &mut W,
        source_error: &'f SourceError<'path, 'source, E>,
    ) -> Result<(), io::Error>
    where
        W: Write,
        E: ErrorCode,
    {
        match source_error.severity {
            Severity::Deny => write!(buffer, "error")?,
            Severity::Warn => write!(buffer, "warn")?,
        }

        let digits = Self::digits(E::ERROR_CODE_MAX);
        let error_code = &source_error.error_code;
        write!(
            buffer,
            "[{prefix}{code:0>width$}]: ",
            prefix = E::PREFIX,
            code = error_code.code(),
            width = digits
        )?;
        error_code.fmt_description(buffer)?;
        writeln!(buffer)?;

        Ok(())
    }

    fn fmt_path<'f, 'path, 'source, W, E>(
        buffer: &mut W,
        source_error: &'f SourceError<'path, 'source, E>,
    ) -> Result<(), io::Error>
    where
        W: Write,
        E: ErrorCode,
    {
        let invalid_source = &source_error.invalid_source;
        if let Some(path) = invalid_source.path.as_ref() {
            let expr = &invalid_source.expr_context.expr;

            writeln!(
                buffer,
                " --> {path}:{line}:{col}",
                path = path.display(),
                line = expr.line_number,
                col = expr.col_number,
            )?;
        }

        Ok(())
    }

    fn fmt_error_expr<'f, 'path, 'source, W, E>(
        buffer: &mut W,
        source_error: &'f SourceError<'path, 'source, E>,
    ) -> Result<(), io::Error>
    where
        W: Write,
        E: ErrorCode,
    {
        let expr_context = &source_error.invalid_source.expr_context;
        let expr = &expr_context.expr;
        let digits = Self::digits(expr_context.line_number);

        // Leading empty line.
        writeln!(buffer, " {space:^width$} |", space = " ", width = digits)?;

        // Expression in context.
        writeln!(
            buffer,
            " {line_number:^width$} | {expr_context}",
            line_number = expr_context.line_number,
            width = digits,
            expr_context = expr_context.value,
        )?;

        let expr_char_count = expr.value.chars().count();
        let marker = "^".repeat(expr_char_count);

        // Highlight the expression.
        writeln!(
            buffer,
            " {space:^width$} | {marker:>pad$}",
            space = " ",
            width = digits,
            marker = marker,
            pad = expr.col_number - expr_context.col_number + expr_char_count,
        )?;

        Ok(())
    }

    /// Return the number of digits that the given max value fits into.
    fn digits(value_max: usize) -> usize {
        // `Integer::log10` pending: <https://github.com/rust-lang/rust/issues/70887>
        // `FloatToInt` pending: <https://github.com/rust-lang/rust/issues/67057>
        //
        // +1 is because error codes should generally start from 1, not 0.
        (value_max as f32).log10().floor() as usize + 1
    }
}
