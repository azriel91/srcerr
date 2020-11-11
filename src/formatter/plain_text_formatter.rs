use std::{
    borrow::Cow,
    io::{self, Write},
};

use crate::{ErrorCode, Severity, SourceError, SourceHighlighted, SourceRefHint, Suggestion};

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
        let expr_context = &source_error.invalid_source.expr_context;
        let line_number_digits = Self::digits(expr_context.line_number);

        Self::fmt_error_code(buffer, source_error)?;
        Self::fmt_path(buffer, source_error)?;
        Self::fmt_error_expr(buffer, source_error, line_number_digits)?;
        Self::fmt_suggestions(buffer, source_error, line_number_digits)?;

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
            let (line_number, col_number) = if let Some(expr) = &invalid_source.expr {
                (expr.line_number, expr.col_number)
            } else {
                let expr_context = &invalid_source.expr_context;
                (expr_context.line_number, expr_context.col_number)
            };

            writeln!(
                buffer,
                " --> {path}:{line}:{col}",
                path = path.display(),
                line = line_number,
                col = col_number,
            )?;
        }

        Ok(())
    }

    fn fmt_error_expr<'f, 'path, 'source, W, E>(
        buffer: &mut W,
        source_error: &'f SourceError<'path, 'source, E>,
        line_number_digits: usize,
    ) -> Result<(), io::Error>
    where
        W: Write,
        E: ErrorCode,
    {
        Self::fmt_source_highlighted(buffer, &source_error.invalid_source, line_number_digits)
    }

    fn fmt_suggestions<'f, 'path, 'source, W, E>(
        buffer: &mut W,
        source_error: &'f SourceError<'path, 'source, E>,
        line_number_digits: usize,
    ) -> Result<(), io::Error>
    where
        W: Write,
        E: ErrorCode,
    {
        source_error
            .suggestions
            .iter()
            .try_for_each(|suggestion| match suggestion {
                Suggestion::ValidExprs(valid_exprs) => {
                    Self::fmt_suggestion_valid_expr(buffer, valid_exprs, line_number_digits)
                }
                Suggestion::SourceRefHint(_source_ref_hint) => {
                    Self::fmt_suggestion_source_ref_hint(
                        buffer,
                        _source_ref_hint,
                        line_number_digits,
                    )
                }
                Suggestion::Hint(_hint) => Ok(()),
            })?;

        Ok(())
    }

    fn fmt_suggestion_valid_expr<'f, 'path, 'source, W>(
        buffer: &mut W,
        valid_exprs: &[Cow<'source, str>],
        line_number_digits: usize,
    ) -> Result<(), io::Error>
    where
        W: Write,
    {
        write!(
            buffer,
            " {space:^width$} = note: expected one of: ",
            space = " ",
            width = line_number_digits
        )?;

        let mut valid_exprs = valid_exprs.iter();
        if let Some(first_valid_expr) = valid_exprs.next() {
            write!(buffer, "`{}`", first_valid_expr)?;
        }
        valid_exprs.try_for_each(|valid_expr| write!(buffer, ", `{}`", valid_expr))?;
        writeln!(buffer)?;

        Ok(())
    }

    /// Formats a suggestion that references a source file.
    ///
    /// Example output:
    ///
    /// ```rust,ignore
    /// help: `chosen` value must come from one of `available` values:
    ///  --> src/dynamic_value.yaml:1:1
    ///   |
    /// 1 | available:
    /// 2 |  - abc
    /// 3 |  - def
    ///   |
    ///   = hint: first defined here
    /// ```
    fn fmt_suggestion_source_ref_hint<'f, 'path, 'source, W>(
        buffer: &mut W,
        source_ref_hint: &SourceRefHint<'path, 'source>,
        line_number_digits: usize,
    ) -> Result<(), io::Error>
    where
        W: Write,
    {
        writeln!(buffer)?;
        writeln!(
            buffer,
            "help: {description}",
            description = source_ref_hint.description
        )?;
        Self::fmt_source_highlighted(buffer, &source_ref_hint.source_ref, line_number_digits)?;

        writeln!(buffer)?;

        Ok(())
    }

    fn fmt_source_highlighted<'f, 'path, 'source, W>(
        buffer: &mut W,
        source_highlighted: &'f SourceHighlighted<'path, 'source>,
        line_number_digits: usize,
    ) -> Result<(), io::Error>
    where
        W: Write,
    {
        let expr_context = &source_highlighted.expr_context;
        let expr = &source_highlighted.expr;

        // Leading empty line.
        writeln!(
            buffer,
            " {space:^width$} |",
            space = " ",
            width = line_number_digits
        )?;

        // Expression in context.
        writeln!(
            buffer,
            " {line_number:^width$} | {expr_context}",
            line_number = expr_context.line_number,
            width = line_number_digits,
            expr_context = expr_context.value,
        )?;

        if let Some(expr) = expr {
            let expr_char_count = expr.value.chars().count();
            let marker = "^".repeat(expr_char_count);

            // Highlight the expression.
            writeln!(
                buffer,
                " {space:^width$} | {marker:>pad$}",
                space = " ",
                width = line_number_digits,
                marker = marker,
                pad = expr.col_number - expr_context.col_number + expr_char_count,
            )?;
        } else {
            writeln!(
                buffer,
                " {space:^width$} |",
                space = " ",
                width = line_number_digits,
            )?;
        }

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

#[cfg(test)]
mod tests {
    use std::{borrow::Cow, io, ops::RangeInclusive, path::Path};

    use crate::{ErrorCode, Expr, Severity, SourceError, SourceHighlighted, Span, Suggestion};

    use super::PlainTextFormatter;

    #[test]
    fn formats_single_line_expr() {
        let path = Path::new("plain_text_formatter/formats_single_line_expr.toml");
        let content = "\
            [simple]\n\
            i32_value = -1\n\
            ";
        let value_out_of_range = value_out_of_range(&path, content);

        let formatted_err = PlainTextFormatter::fmt(&value_out_of_range);

        assert_eq!(
            r#"error[E1]: `-1` is out of the range: `1..3`.
 --> plain_text_formatter/formats_single_line_expr.toml:2:13
   |
 2 | i32_value = -1
   |             ^^
   = note: expected one of: `1`, `2`, `3`
"#,
            formatted_err
        );
    }

    #[test]
    fn zero_pads_error_code_log_10_exact() {
        let path = Path::new("plain_text_formatter/zero_pads_error_code_log_10_exact.toml");
        let content = "\
            [simple]\n\
            i32_value = -1\n\
            ";
        let value_out_of_range = error_code_log_10_exact(&path, content);

        let formatted_err = PlainTextFormatter::fmt(&value_out_of_range);

        assert_eq!(
            r#"error[E0091]: `-1` is out of range.
 --> plain_text_formatter/zero_pads_error_code_log_10_exact.toml:2:13
   |
 2 | i32_value = -1
   |             ^^
"#,
            formatted_err
        );
    }

    #[test]
    fn zero_pads_error_code_log_10_inexact() {
        let path = Path::new("plain_text_formatter/zero_pads_error_code_log_10_inexact.toml");
        let content = "\
            [simple]\n\
            i32_value = -1\n\
            ";
        let value_out_of_range = error_code_log_10_inexact(&path, content);

        let formatted_err = PlainTextFormatter::fmt(&value_out_of_range);

        assert_eq!(
            r#"error[E0091]: `-1` is out of range.
 --> plain_text_formatter/zero_pads_error_code_log_10_inexact.toml:2:13
   |
 2 | i32_value = -1
   |             ^^
"#,
            formatted_err
        );
    }

    fn value_out_of_range<'path, 'source>(
        path: &'path Path,
        content: &'source str,
    ) -> SourceError<'path, 'source, ValueOutOfRange> {
        let range = 1..=3;
        let error_code = ValueOutOfRange {
            value: -1,
            range: range.clone(),
        };
        let valid_exprs = range
            .map(|n| n.to_string())
            .map(Cow::Owned)
            .collect::<Vec<_>>();
        let suggestion_0 = Suggestion::ValidExprs(valid_exprs);
        let suggestions = vec![suggestion_0];

        source_error(path, content, error_code, suggestions)
    }

    fn error_code_log_10_exact<'path, 'source>(
        path: &'path Path,
        content: &'source str,
    ) -> SourceError<'path, 'source, ErrorCodeLog10Exact> {
        let error_code = ErrorCodeLog10Exact { value: -1 };
        source_error(path, content, error_code, vec![])
    }

    fn error_code_log_10_inexact<'path, 'source>(
        path: &'path Path,
        content: &'source str,
    ) -> SourceError<'path, 'source, ErrorCodeLog10Inexact> {
        let error_code = ErrorCodeLog10Inexact { value: -1 };
        source_error(path, content, error_code, vec![])
    }

    fn source_error<'path, 'source, E>(
        path: &'path Path,
        content: &'source str,
        error_code: E,
        suggestions: Vec<Suggestion<'path, 'source>>,
    ) -> SourceError<'path, 'source, E>
    where
        E: ErrorCode,
    {
        let expr = Expr {
            span: Span { start: 21, end: 23 },
            line_number: 2,
            col_number: 13,
            value: Cow::Borrowed(&content[21..23]),
        };
        let expr_context = Expr {
            span: Span { start: 9, end: 23 },
            line_number: 2,
            col_number: 1,
            value: Cow::Borrowed(&content[9..23]),
        };
        let invalid_source = SourceHighlighted {
            path: Some(Cow::Borrowed(path)),
            expr_context,
            expr: Some(expr),
        };
        let severity = Severity::Deny;

        SourceError {
            error_code,
            invalid_source,
            suggestions,
            severity,
        }
    }

    #[derive(Debug)]
    pub struct ValueOutOfRange {
        value: i32,
        range: RangeInclusive<u32>,
    }

    impl<'source> ErrorCode for ValueOutOfRange {
        const ERROR_CODE_MAX: usize = 2;
        const PREFIX: &'static str = "E";

        fn code(&self) -> usize {
            1
        }

        fn fmt_description<W>(&self, buffer: &mut W) -> Result<(), io::Error>
        where
            W: io::Write,
        {
            write!(
                buffer,
                "`{}` is out of the range: `{}..{}`.",
                self.value,
                self.range.start(),
                self.range.end()
            )
        }
    }

    #[derive(Debug)]
    pub struct ErrorCodeLog10Exact {
        value: i32,
    }

    impl<'source> ErrorCode for ErrorCodeLog10Exact {
        const ERROR_CODE_MAX: usize = 1000;
        const PREFIX: &'static str = "E";

        fn code(&self) -> usize {
            91
        }

        fn fmt_description<W>(&self, buffer: &mut W) -> Result<(), io::Error>
        where
            W: io::Write,
        {
            write!(buffer, "`{}` is out of range.", self.value)
        }
    }

    #[derive(Debug)]
    pub struct ErrorCodeLog10Inexact {
        value: i32,
    }

    impl<'source> ErrorCode for ErrorCodeLog10Inexact {
        const ERROR_CODE_MAX: usize = 9999;
        const PREFIX: &'static str = "E";

        fn code(&self) -> usize {
            91
        }

        fn fmt_description<W>(&self, buffer: &mut W) -> Result<(), io::Error>
        where
            W: io::Write,
        {
            write!(buffer, "`{}` is out of range.", self.value)
        }
    }
}
