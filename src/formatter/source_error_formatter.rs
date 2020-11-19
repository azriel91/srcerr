use std::{
    borrow::Cow,
    io::{self, Write},
    marker::PhantomData,
};

use crate::{
    ErrorCode, ExprHighlighted, HighlightLevel, Severity, SourceError, SourceHighlighted,
    SourceRefHint, Styler, Suggestion,
};

type StyleMarkerFns<W> = (
    fn(&mut W) -> Result<(), io::Error>,
    fn(&mut W) -> Result<(), io::Error>,
);

const DOTS_PREFIX: &str = ".. ";
const DOTS_SUFFIX: &str = " ..";

/// Formats a [`SourceError`], delegating styling to the parameterized type.
///
/// # Type Parameters
///
/// * `S`: [`Styler`] implementation
#[derive(Debug)]
pub struct SourceErrorFormatter<W, S>(pub PhantomData<(W, S)>);

impl<S> SourceErrorFormatter<Vec<u8>, S>
where
    S: Styler<Vec<u8>>,
{
    /// Formats the source error as a string.
    pub fn fmt<E>(source_error: &SourceError<'_, '_, E>) -> String
    where
        E: ErrorCode,
    {
        let mut buffer = Vec::new();
        SourceErrorFormatter::<Vec<u8>, S>::fmt_buffer(&mut buffer, source_error)
            .expect("Failed to format source error.");
        String::from_utf8(buffer).expect("Source error is not valid UTF8.")
    }
}

impl<W, S> SourceErrorFormatter<W, S>
where
    W: Write,
    S: Styler<W>,
{
    /// Formats the source error, delegating to the styler for styles.
    pub fn fmt_buffer<'path, 'source, E>(
        buffer: &mut W,
        source_error: &SourceError<'path, 'source, E>,
    ) -> Result<(), io::Error>
    where
        E: ErrorCode,
    {
        let expr_context = &source_error.invalid_source.expr_context;
        let line_number_digits = Self::digits(expr_context.inner.line_number);

        Self::fmt_error_code(buffer, source_error)?;
        Self::fmt_path(buffer, &source_error.invalid_source)?;
        Self::fmt_error_expr(buffer, source_error, line_number_digits)?;
        Self::fmt_suggestions(buffer, source_error, line_number_digits)?;

        Ok(())
    }

    fn fmt_error_code<'path, 'source, E>(
        buffer: &mut W,
        source_error: &SourceError<'path, 'source, E>,
    ) -> Result<(), io::Error>
    where
        E: ErrorCode,
    {
        let digits = Self::digits(E::ERROR_CODE_MAX);
        let error_code = &source_error.error_code;

        match source_error.severity {
            Severity::Deny => {
                S::error_tag_begin(buffer)?;
                write!(buffer, "error[")?;
                S::error_tag_end(buffer)?;

                S::error_code_begin(buffer)?;
                write!(
                    buffer,
                    "{prefix}{code:0>width$}",
                    prefix = E::PREFIX,
                    code = error_code.code(),
                    width = digits
                )?;
                S::error_code_end(buffer)?;

                S::error_tag_begin(buffer)?;
                write!(buffer, "]")?;
                S::error_tag_end(buffer)?;
            }
            Severity::Warning => {
                S::warning_tag_begin(buffer)?;
                write!(buffer, "warning[")?;
                S::warning_tag_end(buffer)?;

                S::error_code_begin(buffer)?;
                write!(
                    buffer,
                    "{prefix}{code:0>width$}",
                    prefix = E::PREFIX,
                    code = error_code.code(),
                    width = digits
                )?;
                S::error_code_end(buffer)?;

                S::warning_tag_begin(buffer)?;
                write!(buffer, "]")?;
                S::warning_tag_end(buffer)?;
            }
        }

        S::error_description_begin(buffer)?;
        write!(buffer, ": ")?;
        error_code.fmt_description(buffer)?;
        S::error_description_end(buffer)?;

        write!(buffer, "{}", S::NEWLINE)?;

        Ok(())
    }

    fn fmt_path<'path, 'source>(
        buffer: &mut W,
        source_highlighted: &SourceHighlighted<'path, 'source>,
    ) -> Result<(), io::Error> {
        if let Some(path) = source_highlighted.path.as_ref() {
            let (line_number, col_number) = if let Some(expr) = &source_highlighted.expr {
                (expr.inner.line_number, expr.inner.col_number)
            } else {
                let expr_context = &source_highlighted.expr_context;
                (
                    expr_context.inner.line_number,
                    expr_context.inner.col_number,
                )
            };

            // Unstyled:
            //
            // writeln!(
            //     buffer,
            //     "  --> {path}:{line}:{col}",
            //     path = path.display(),
            //     line = line_number,
            //     col = col_number,
            // )?;

            write!(buffer, "  ")?;
            S::margin_begin(buffer)?;
            write!(buffer, "-->")?;
            S::margin_end(buffer)?;
            write!(buffer, " ")?;

            S::path_begin(buffer)?;
            write!(buffer, "{path}", path = path.display())?;
            S::path_end(buffer)?;

            write!(buffer, ":")?;

            S::number_begin(buffer)?;
            write!(buffer, "{line}", line = line_number)?;
            S::number_end(buffer)?;

            write!(buffer, ":")?;

            S::number_begin(buffer)?;
            write!(buffer, "{col}", col = col_number)?;
            S::number_end(buffer)?;

            write!(buffer, "{}", S::NEWLINE)?;
        }

        Ok(())
    }

    fn fmt_error_expr<'path, 'source, E>(
        buffer: &mut W,
        source_error: &SourceError<'path, 'source, E>,
        line_number_digits: usize,
    ) -> Result<(), io::Error>
    where
        E: ErrorCode,
    {
        Self::fmt_source_highlighted(
            buffer,
            &source_error.invalid_source,
            HighlightLevel::from(source_error.severity),
            line_number_digits,
            "^",
        )
    }

    fn fmt_suggestions<'path, 'source, E>(
        buffer: &mut W,
        source_error: &SourceError<'path, 'source, E>,
        line_number_digits: usize,
    ) -> Result<(), io::Error>
    where
        E: ErrorCode,
    {
        source_error
            .suggestions
            .iter()
            .try_for_each(|suggestion| match suggestion {
                Suggestion::ValidExprs(valid_exprs) => {
                    Self::fmt_suggestion_valid_expr(buffer, valid_exprs, line_number_digits)
                }
                Suggestion::SourceRefHint(source_ref_hint) => Self::fmt_suggestion_source_ref_hint(
                    buffer,
                    HighlightLevel::Info,
                    source_ref_hint,
                    line_number_digits,
                ),
                Suggestion::Hint(hint) => {
                    Self::fmt_suggestion_hint(buffer, hint, line_number_digits)
                }
            })?;

        Ok(())
    }

    fn fmt_suggestion_valid_expr<'source>(
        buffer: &mut W,
        valid_exprs: &[Cow<'source, str>],
        line_number_digits: usize,
    ) -> Result<(), io::Error> {
        S::hint_info_begin(buffer)?;

        write!(
            buffer,
            " {space:>width$} = note: expected one of: ",
            space = " ",
            width = line_number_digits
        )?;

        let mut valid_exprs = valid_exprs.iter();
        if let Some(first_valid_expr) = valid_exprs.next() {
            write!(buffer, "`{}`", first_valid_expr)?;
        }
        valid_exprs.try_for_each(|valid_expr| write!(buffer, ", `{}`", valid_expr))?;

        S::hint_info_end(buffer)?;

        write!(buffer, "{}", S::NEWLINE)?;

        Ok(())
    }

    /// Formats a suggestion that references a source file.
    ///
    /// Example output:
    ///
    /// ```rust,ignore
    /// help: `chosen` value must come from one of `available` values:
    ///   --> src/dynamic_value.yaml:1:1
    ///    |
    ///  1 | available:
    ///  2 |  - abc
    ///  3 |  - def
    ///    |
    /// ```
    fn fmt_suggestion_source_ref_hint<'path, 'source>(
        buffer: &mut W,
        highlight_level: HighlightLevel,
        source_ref_hint: &SourceRefHint<'path, 'source>,
        line_number_digits: usize,
    ) -> Result<(), io::Error> {
        write!(buffer, "{}", S::NEWLINE)?;

        S::hint_info_begin(buffer)?;
        write!(
            buffer,
            "help: {description}:",
            description = source_ref_hint.description
        )?;
        S::hint_info_end(buffer)?;
        write!(buffer, "{}", S::NEWLINE)?;

        Self::fmt_path(buffer, &source_ref_hint.source_ref)?;
        Self::fmt_source_highlighted(
            buffer,
            &source_ref_hint.source_ref,
            highlight_level,
            line_number_digits,
            "-",
        )?;

        Ok(())
    }

    /// Formats a string hint.
    ///
    /// Example output:
    ///
    /// ```rust,ignore
    ///   = hint: first defined here
    /// ```
    fn fmt_suggestion_hint(
        buffer: &mut W,
        hint: &str,
        line_number_digits: usize,
    ) -> Result<(), io::Error> {
        write!(
            buffer,
            " {space:>width$} ",
            space = " ",
            width = line_number_digits,
        )?;

        S::hint_info_begin(buffer)?;
        write!(buffer, "= hint: {hint}", hint = hint)?;
        S::hint_info_end(buffer)?;

        write!(buffer, "{}", S::NEWLINE)?;

        Ok(())
    }

    fn fmt_source_highlighted<'path, 'source>(
        buffer: &mut W,
        source_highlighted: &SourceHighlighted<'path, 'source>,
        highlight_level: HighlightLevel,
        line_number_digits: usize,
        marker: &str,
    ) -> Result<(), io::Error> {
        let expr_context = &source_highlighted.expr_context;
        let expr = &source_highlighted.expr;

        // Leading empty line.
        write!(
            buffer,
            " {space:>width$} ",
            space = " ",
            width = line_number_digits
        )?;
        S::margin_begin(buffer)?;
        write!(buffer, "{}", S::MARGIN_LINE)?;
        S::margin_end(buffer)?;

        write!(buffer, "{}", S::NEWLINE)?;

        // Expression context.
        let is_partial_line = expr_context.inner.col_number > 1;
        let mut expr_context_lines = expr_context.inner.value.lines();

        // We need to render the marker when the expression line number is within the
        // context.
        if let Some(expr) = expr {
            // Note: It is up to the user to ensure that the `expr` is within
            // `expr_context`. If it is not, it wouldn't be printed.
            let first_line_number = expr_context.inner.line_number;
            let last_line_number =
                expr_context_lines.try_fold(first_line_number, |current_line_number, line| {
                    let surround_with_dots =
                        current_line_number == first_line_number && is_partial_line;

                    Self::fmt_expr_context(
                        buffer,
                        line_number_digits,
                        line,
                        current_line_number,
                        surround_with_dots,
                    )?;

                    if current_line_number == expr.inner.line_number {
                        let column_offset = if is_partial_line {
                            expr_context.inner.col_number - DOTS_PREFIX.len()
                        } else {
                            expr_context.inner.col_number
                        };

                        Self::fmt_expr_highlighted(
                            buffer,
                            highlight_level,
                            expr,
                            line_number_digits,
                            column_offset,
                            marker,
                        )?;

                        // When we have written the expression highlight markers, we want to write
                        // column number hint when it is a partial line context.
                        if is_partial_line {
                            Self::fmt_expr_column_hint(
                                buffer,
                                highlight_level,
                                line_number_digits,
                                column_offset,
                                expr.inner.col_number,
                            )?;
                        }
                    }

                    Result::<usize, io::Error>::Ok(current_line_number + 1)
                })?;

            // Write the last empty line in any of the following cases:
            //
            // * The expression marker is not on the last line.
            // * We are rendering a partial context, and have written the column number hint
            //   previously.
            if last_line_number != expr.inner.line_number + 1 || is_partial_line {
                write!(
                    buffer,
                    " {space:>width$} ",
                    space = " ",
                    width = line_number_digits,
                )?;
                S::margin_begin(buffer)?;
                write!(buffer, "{}", S::MARGIN_LINE)?;
                S::margin_end(buffer)?;

                write!(buffer, "{}", S::NEWLINE)?;
            }
        } else {
            let first_line_number = expr_context.inner.line_number;
            expr_context_lines.try_fold(first_line_number, |current_line_number, line| {
                let surround_with_dots =
                    current_line_number == first_line_number && is_partial_line;

                Self::fmt_expr_context(
                    buffer,
                    line_number_digits,
                    line,
                    current_line_number,
                    surround_with_dots,
                )?;

                Result::<usize, io::Error>::Ok(current_line_number + 1)
            })?;

            write!(
                buffer,
                " {space:>width$} ",
                space = " ",
                width = line_number_digits,
            )?;
            S::margin_begin(buffer)?;
            write!(buffer, "{}", S::MARGIN_LINE)?;
            S::margin_end(buffer)?;

            write!(buffer, "{}", S::NEWLINE)?;
        }

        Ok(())
    }

    fn fmt_expr_context(
        buffer: &mut W,
        line_number_digits: usize,
        line: &str,
        current_line_number: usize,
        surround_with_dots: bool,
    ) -> Result<(), io::Error> {
        // Line numbers and margin
        write!(
            buffer,
            " {line_number:^width$} ",
            line_number = current_line_number,
            width = line_number_digits,
        )?;
        S::margin_begin(buffer)?;
        write!(buffer, "{}", S::MARGIN_LINE)?;
        S::margin_end(buffer)?;

        write!(buffer, " ")?;

        // Dots to indicate only part of the line is rendered.
        if surround_with_dots {
            write!(buffer, "{}", DOTS_PREFIX)?;
        }

        // The expression value.
        write!(buffer, "{expr_context}", expr_context = line)?;

        // Dots to indicate only part of the line is rendered.
        //
        // Notably at this point, we are not properly handling multi-line contexts with
        // partial line rendering. That would require either the user telling us that
        // the line is partial, or knowledge of the whole line that the expression
        // context comes from.
        if surround_with_dots {
            write!(buffer, "{}", DOTS_SUFFIX)?;
        }

        write!(buffer, "{}", S::NEWLINE)?;

        Ok(())
    }

    fn fmt_expr_highlighted<'source>(
        buffer: &mut W,
        highlight_level: HighlightLevel,
        expr: &ExprHighlighted<'source>,
        line_number_digits: usize,
        context_col_offset: usize,
        marker: &str,
    ) -> Result<(), io::Error> {
        let expr_char_count = expr.inner.value.chars().count();
        let marker = marker.repeat(expr_char_count);

        // Highlight the expression.
        write!(
            buffer,
            " {space:>width$} ",
            space = " ",
            width = line_number_digits,
        )?;

        S::margin_begin(buffer)?;
        write!(buffer, "{}", S::MARGIN_LINE)?;
        S::margin_end(buffer)?;
        write!(buffer, " ")?;

        // TODO: Don't include the padding in the formatting.
        let (style_marker_begin, style_marker_end) = Self::style_marker_fns(highlight_level);
        style_marker_begin(buffer)?;
        write!(
            buffer,
            "{marker:>pad$}",
            marker = marker,
            pad = expr.inner.col_number - context_col_offset + expr_char_count,
        )?;
        style_marker_end(buffer)?;

        S::hint_error_begin(buffer)?;
        if let Some(hint) = expr.hint.as_ref() {
            write!(buffer, " hint: {hint}", hint = hint)?;
        }
        S::hint_error_end(buffer)?;

        write!(buffer, "{}", S::NEWLINE)?;

        Ok(())
    }

    fn fmt_expr_column_hint(
        buffer: &mut W,
        highlight_level: HighlightLevel,
        line_number_digits: usize,
        context_col_offset: usize,
        expr_col_number: usize,
    ) -> Result<(), io::Error> {
        // Arrow body
        write!(
            buffer,
            " {space:>width$} ",
            space = " ",
            width = line_number_digits,
        )?;
        S::margin_begin(buffer)?;
        write!(buffer, "{}", S::MARGIN_LINE)?;
        S::margin_end(buffer)?;
        write!(
            buffer,
            " {space:>pad$}",
            space = " ",
            pad = expr_col_number - context_col_offset,
        )?;
        let (style_marker_begin, style_marker_end) = Self::style_marker_fns(highlight_level);
        style_marker_begin(buffer)?;
        write!(
            buffer,
            "{arrow_body}",
            arrow_body = S::HIGHLIGHT_MARKER_VERTICAL
        )?;
        style_marker_end(buffer)?;
        write!(buffer, "{}", S::NEWLINE)?;

        // Column number
        write!(
            buffer,
            " {space:>width$} | {space:>pad$}{col_number}",
            space = " ",
            width = line_number_digits,
            pad = expr_col_number - context_col_offset,
            col_number = expr_col_number,
        )?;
        write!(buffer, "{}", S::NEWLINE)?;

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

    fn style_marker_fns(highlight_level: HighlightLevel) -> StyleMarkerFns<W> {
        match highlight_level {
            HighlightLevel::Error => (S::error_marker_begin, S::error_marker_end),
            HighlightLevel::Warning => (S::warning_marker_begin, S::warning_marker_end),
            HighlightLevel::Info => (S::hint_marker_begin, S::hint_marker_end),
        }
    }
}
