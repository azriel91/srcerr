use crate::SourceErrorFormatter;

pub use self::ansi_color_styler::AnsiColorStyler;

mod ansi_color_styler;

/// Formats a [`SourceError`] with ANSI colors.
///
/// [`SourceError`]: `crate::SourceError`
pub type AnsiColorFormatter<W> = SourceErrorFormatter<W, AnsiColorStyler>;

#[cfg(test)]
mod tests {
    use std::{borrow::Cow, io, ops::RangeInclusive, path::Path};

    use pretty_assertions::assert_eq;

    use crate::{
        ErrorCode, Expr, ExprHighlighted, Severity, SourceError, SourceHighlighted, SourceRefHint,
        Suggestion,
    };

    use super::AnsiColorFormatter;

    #[test]
    fn formats_single_line_expr() {
        let path = Path::new("ansi_color_formatter/formats_single_line_expr.toml");
        let content = "\
            [simple]\n\
            i32_value = -1\n\
            ";
        let value_out_of_range = value_out_of_range(&path, content);

        let formatted_err = AnsiColorFormatter::fmt(&value_out_of_range);

        assert_eq!(
            "\
\u{1b}[1;38;5;9merror[\u{1b}[0m\u{1b}[1mE1\u{1b}[0m\u{1b}[1;38;5;9m]\u{1b}[0m\u{1b}[1m: `-1` is out of the range: `1..3`.\u{1b}[0m
  \u{1b}[1;38;5;12m-->\u{1b}[0m \u{1b}[1mansi_color_formatter/formats_single_line_expr.toml\u{1b}[0m:\u{1b}[1m2\u{1b}[0m:\u{1b}[1m13\u{1b}[0m
   \u{1b}[1;38;5;12m|\u{1b}[0m
\u{1b}[1;38;5;12m 2 |\u{1b}[0m i32_value = -1
   \u{1b}[1;38;5;12m|\u{1b}[0m \u{1b}[1;38;5;9m            ^^\u{1b}[0m\u{1b}[1;38;5;12m\u{1b}[0m
\u{1b}[1;38;5;12m   = note: expected one of: `1`, `2`, `3`\u{1b}[0m
",
            formatted_err
        );
    }

    #[test]
    fn zero_pads_error_code_log_10_exact() {
        let path = Path::new("ansi_color_formatter/zero_pads_error_code_log_10_exact.toml");
        let content = "\
            [simple]\n\
            i32_value = -1\n\
            ";
        let value_out_of_range = error_code_log_10_exact(&path, content);

        let formatted_err = AnsiColorFormatter::fmt(&value_out_of_range);

        assert_eq!(
            "\
\u{1b}[1;38;5;9merror[\u{1b}[0m\u{1b}[1mE0091\u{1b}[0m\u{1b}[1;38;5;9m]\u{1b}[0m\u{1b}[1m: `-1` is out of range.\u{1b}[0m
  \u{1b}[1;38;5;12m-->\u{1b}[0m \u{1b}[1mansi_color_formatter/zero_pads_error_code_log_10_exact.toml\u{1b}[0m:\u{1b}[1m2\u{1b}[0m:\u{1b}[1m13\u{1b}[0m
   \u{1b}[1;38;5;12m|\u{1b}[0m
\u{1b}[1;38;5;12m 2 |\u{1b}[0m i32_value = -1
   \u{1b}[1;38;5;12m|\u{1b}[0m \u{1b}[1;38;5;9m            ^^\u{1b}[0m\u{1b}[1;38;5;12m\u{1b}[0m
",
            formatted_err
        );
    }

    #[test]
    fn zero_pads_error_code_log_10_inexact() {
        let path = Path::new("ansi_color_formatter/zero_pads_error_code_log_10_inexact.toml");
        let content = "\
            [simple]\n\
            i32_value = -1\n\
            ";
        let value_out_of_range = error_code_log_10_inexact(&path, content);

        let formatted_err = AnsiColorFormatter::fmt(&value_out_of_range);

        assert_eq!(
            "\
\u{1b}[1;38;5;9merror[\u{1b}[0m\u{1b}[1mE0091\u{1b}[0m\u{1b}[1;38;5;9m]\u{1b}[0m\u{1b}[1m: `-1` is out of range.\u{1b}[0m
  \u{1b}[1;38;5;12m-->\u{1b}[0m \u{1b}[1mansi_color_formatter/zero_pads_error_code_log_10_inexact.toml\u{1b}[0m:\u{1b}[1m2\u{1b}[0m:\u{1b}[1m13\u{1b}[0m
   \u{1b}[1;38;5;12m|\u{1b}[0m
\u{1b}[1;38;5;12m 2 |\u{1b}[0m i32_value = -1
   \u{1b}[1;38;5;12m|\u{1b}[0m \u{1b}[1;38;5;9m            ^^\u{1b}[0m\u{1b}[1;38;5;12m\u{1b}[0m
",
            formatted_err
        );
    }

    #[test]
    fn formats_multi_line_expr_context_before() {
        let path = Path::new("ansi_color_formatter/formats_multi_line_expr_context_before.toml");
        let content = "\
            [simple]\n\
            i32_value = -1\n\
            ";
        let multi_line_expr_context_before_error =
            multi_line_expr_context_before_error(&path, content);

        let formatted_err = AnsiColorFormatter::fmt(&multi_line_expr_context_before_error);

        assert_eq!(
            "\
\u{1b}[1;38;5;9merror[\u{1b}[0m\u{1b}[1mE1\u{1b}[0m\u{1b}[1;38;5;9m]\u{1b}[0m\u{1b}[1m: `-1` is out of the range: `1..3`.\u{1b}[0m
  \u{1b}[1;38;5;12m-->\u{1b}[0m \u{1b}[1mansi_color_formatter/formats_multi_line_expr_context_before.toml\u{1b}[0m:\u{1b}[1m2\u{1b}[0m:\u{1b}[1m13\u{1b}[0m
   \u{1b}[1;38;5;12m|\u{1b}[0m
\u{1b}[1;38;5;12m 1 |\u{1b}[0m [simple]
\u{1b}[1;38;5;12m 2 |\u{1b}[0m i32_value = -1
   \u{1b}[1;38;5;12m|\u{1b}[0m \u{1b}[1;38;5;9m            ^^\u{1b}[0m\u{1b}[1;38;5;12m\u{1b}[0m
",
            formatted_err
        );
    }

    #[test]
    fn formats_multi_line_expr_context_both() {
        let path = Path::new("ansi_color_formatter/formats_multi_line_expr_context_both.yaml");
        let content = r#"---
available:
- abc
- def

chosen: "ghi"
"#;
        let multi_line_expr_context_both_error = multi_line_expr_context_both_error(&path, content);

        let formatted_err = AnsiColorFormatter::fmt(&multi_line_expr_context_both_error);

        assert_eq!(
            "\
\u{1b}[1;38;5;9merror[\u{1b}[0m\u{1b}[1mE100\u{1b}[0m\u{1b}[1;38;5;9m]\u{1b}[0m\u{1b}[1m: `chosen` value `ghi` is invalid.\u{1b}[0m
  \u{1b}[1;38;5;12m-->\u{1b}[0m \u{1b}[1mansi_color_formatter/formats_multi_line_expr_context_both.yaml\u{1b}[0m:\u{1b}[1m6\u{1b}[0m:\u{1b}[1m9\u{1b}[0m
   \u{1b}[1;38;5;12m|\u{1b}[0m
\u{1b}[1;38;5;12m 6 |\u{1b}[0m chosen: \"ghi\"
   \u{1b}[1;38;5;12m|\u{1b}[0m \u{1b}[1;38;5;9m        ^^^^^\u{1b}[0m\u{1b}[1;38;5;12m\u{1b}[0m
\u{1b}[1;38;5;12m   = note: expected one of: `abc`, `def`\u{1b}[0m

\u{1b}[1;38;5;12mhelp: `chosen` value must come from one of `available` values:\u{1b}[0m
  \u{1b}[1;38;5;12m-->\u{1b}[0m \u{1b}[1mansi_color_formatter/formats_multi_line_expr_context_both.yaml\u{1b}[0m:\u{1b}[1m2\u{1b}[0m:\u{1b}[1m1\u{1b}[0m
   \u{1b}[1;38;5;12m|\u{1b}[0m
\u{1b}[1;38;5;12m 2 |\u{1b}[0m available:
\u{1b}[1;38;5;12m 3 |\u{1b}[0m - abc
\u{1b}[1;38;5;12m 4 |\u{1b}[0m - def
   \u{1b}[1;38;5;12m|\u{1b}[0m
   \u{1b}[1;38;5;12m= hint: first defined here\u{1b}[0m
",
            formatted_err
        );
    }

    #[test]
    fn formats_multi_line_expr_context_expr() {
        let path = Path::new("ansi_color_formatter/formats_multi_line_expr_context_expr.yaml");
        let content = r#"---
available:
- abc
- def

chosen: "ghi"
"#;
        let multi_line_expr_context_expr_error = multi_line_expr_context_expr_error(&path, content);

        let formatted_err = AnsiColorFormatter::fmt(&multi_line_expr_context_expr_error);

        assert_eq!(
            "\
\u{1b}[1;38;5;9merror[\u{1b}[0m\u{1b}[1mE100\u{1b}[0m\u{1b}[1;38;5;9m]\u{1b}[0m\u{1b}[1m: `chosen` value `ghi` is invalid.\u{1b}[0m
  \u{1b}[1;38;5;12m-->\u{1b}[0m \u{1b}[1mansi_color_formatter/formats_multi_line_expr_context_expr.yaml\u{1b}[0m:\u{1b}[1m6\u{1b}[0m:\u{1b}[1m9\u{1b}[0m
   \u{1b}[1;38;5;12m|\u{1b}[0m
\u{1b}[1;38;5;12m 6 |\u{1b}[0m chosen: \"ghi\"
   \u{1b}[1;38;5;12m|\u{1b}[0m \u{1b}[1;38;5;9m        ^^^^^\u{1b}[0m\u{1b}[1;38;5;12m\u{1b}[0m
\u{1b}[1;38;5;12m   = note: expected one of: `abc`, `def`\u{1b}[0m

\u{1b}[1;38;5;12mhelp: `chosen` value must come from one of `available` values:\u{1b}[0m
  \u{1b}[1;38;5;12m-->\u{1b}[0m \u{1b}[1mansi_color_formatter/formats_multi_line_expr_context_expr.yaml\u{1b}[0m:\u{1b}[1m2\u{1b}[0m:\u{1b}[1m1\u{1b}[0m
   \u{1b}[1;38;5;12m|\u{1b}[0m
\u{1b}[1;38;5;12m 2 |\u{1b}[0m available:
   \u{1b}[1;38;5;12m|\u{1b}[0m \u{1b}[1;38;5;12m----------\u{1b}[0m\u{1b}[1;38;5;12m hint: first defined here\u{1b}[0m
\u{1b}[1;38;5;12m 3 |\u{1b}[0m - abc
\u{1b}[1;38;5;12m 4 |\u{1b}[0m - def
   \u{1b}[1;38;5;12m|\u{1b}[0m
",
            formatted_err
        );
    }

    #[test]
    fn formats_long_line_expr_context_expr() {
        let path = Path::new("ansi_color_formatter/formats_long_line_expr_context_expr.json");
        let content = r#"{"a":0,"b":1,"c":2,"d":3,"e":4,"f":5,"g":6,"h":7,"i":8,"j":9,"k":10,"l":11,"m":12,"n":13,"o":14,"p":150,"q":16,"r":17,"s":18,"t":19,"u":20,"v":21,"w":22,"x":23,"y":24,"z":25}"#;
        let long_line_expr_context_expr = long_line_expr_context_expr(&path, content);

        let formatted_err = AnsiColorFormatter::fmt(&long_line_expr_context_expr);

        assert_eq!(
            "\
\u{1b}[1;38;5;9merror[\u{1b}[0m\u{1b}[1mE09\u{1b}[0m\u{1b}[1;38;5;9m]\u{1b}[0m\u{1b}[1m: Value `150` is invalid.\u{1b}[0m
  \u{1b}[1;38;5;12m-->\u{1b}[0m \u{1b}[1mansi_color_formatter/formats_long_line_expr_context_expr.json\u{1b}[0m:\u{1b}[1m1\u{1b}[0m:\u{1b}[1m101\u{1b}[0m
   \u{1b}[1;38;5;12m|\u{1b}[0m
\u{1b}[1;38;5;12m 1 |\u{1b}[0m .. \"p\":150, ..
   \u{1b}[1;38;5;12m|\u{1b}[0m \u{1b}[1;38;5;9m       ^^^\u{1b}[0m\u{1b}[1;38;5;12m\u{1b}[0m
   \u{1b}[1;38;5;12m|\u{1b}[0m        \u{1b}[1;38;5;9m|\u{1b}[0m
   |        101
   \u{1b}[1;38;5;12m|\u{1b}[0m
   \u{1b}[1;38;5;12m= hint: expected value to be less than 26\u{1b}[0m
",
            formatted_err
        );
    }

    #[test]
    fn path_arrow_moves_with_line_number_margin() {
        let path = Path::new("ansi_color_formatter/path_arrow_moves_with_line_number_margin.toml");
        let content = "\
            [simple]\n\
            i32_value = -1\n\
            ";
        let value_out_of_range_high_line = value_out_of_range_high_line(&path, content);

        let formatted_err = AnsiColorFormatter::fmt(&value_out_of_range_high_line);

        assert_eq!(
            "\
\u{1b}[1;38;5;9merror[\u{1b}[0m\u{1b}[1mE1\u{1b}[0m\u{1b}[1;38;5;9m]\u{1b}[0m\u{1b}[1m: `-1` is out of the range: `1..3`.\u{1b}[0m
    \u{1b}[1;38;5;12m-->\u{1b}[0m \u{1b}[1mansi_color_formatter/path_arrow_moves_with_line_number_margin.toml\u{1b}[0m:\u{1b}[1m201\u{1b}[0m:\u{1b}[1m13\u{1b}[0m
     \u{1b}[1;38;5;12m|\u{1b}[0m
\u{1b}[1;38;5;12m 201 |\u{1b}[0m i32_value = -1
     \u{1b}[1;38;5;12m|\u{1b}[0m \u{1b}[1;38;5;9m            ^^\u{1b}[0m\u{1b}[1;38;5;12m\u{1b}[0m
\u{1b}[1;38;5;12m     = note: expected one of: `1`, `2`, `3`\u{1b}[0m
",
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

        source_error(
            path,
            content,
            error_code,
            expr_toml_single,
            expr_context_single,
            suggestions,
        )
    }

    fn value_out_of_range_high_line<'path, 'source>(
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

        source_error(
            path,
            content,
            error_code,
            expr_toml_single_high_line,
            expr_context_single_high_line,
            suggestions,
        )
    }

    fn error_code_log_10_exact<'path, 'source>(
        path: &'path Path,
        content: &'source str,
    ) -> SourceError<'path, 'source, ErrorCodeLog10Exact> {
        let error_code = ErrorCodeLog10Exact { value: -1 };
        source_error(
            path,
            content,
            error_code,
            expr_toml_single,
            expr_context_single,
            vec![],
        )
    }

    fn error_code_log_10_inexact<'path, 'source>(
        path: &'path Path,
        content: &'source str,
    ) -> SourceError<'path, 'source, ErrorCodeLog10Inexact> {
        let error_code = ErrorCodeLog10Inexact { value: -1 };
        source_error(
            path,
            content,
            error_code,
            expr_toml_single,
            expr_context_single,
            vec![],
        )
    }

    fn multi_line_expr_context_before_error<'path, 'source>(
        path: &'path Path,
        content: &'source str,
    ) -> SourceError<'path, 'source, ValueOutOfRange> {
        let error_code = ValueOutOfRange {
            value: -1,
            range: 1..=3,
        };
        source_error(
            path,
            content,
            error_code,
            expr_toml_single,
            expr_context_before,
            vec![],
        )
    }

    fn multi_line_expr_context_both_error<'path, 'source>(
        path: &'path Path,
        content: &'source str,
    ) -> SourceError<'path, 'source, ChosenInvalid<'source>> {
        let suggestion_0 = Suggestion::ValidExprs(vec![
            Cow::Borrowed(&content[17..20]),
            Cow::Borrowed(&content[23..26]),
        ]);
        let suggestion_1 = Suggestion::SourceRefHint(Box::new(SourceRefHint {
            source_ref: SourceHighlighted {
                path: Some(Cow::Borrowed(path)),
                expr_context: expr_context_hint_yaml_both(content),
                expr: None,
            },
            description: String::from("`chosen` value must come from one of `available` values"),
        }));
        let suggestion_2 = Suggestion::Hint("first defined here");
        let suggestions = vec![suggestion_0, suggestion_1, suggestion_2];

        let error_code = ChosenInvalid {
            value: &content[37..40],
        };
        source_error(
            path,
            content,
            error_code,
            expr_yaml_single,
            expr_context_yaml_single,
            suggestions,
        )
    }

    fn multi_line_expr_context_expr_error<'path, 'source>(
        path: &'path Path,
        content: &'source str,
    ) -> SourceError<'path, 'source, ChosenInvalid<'source>> {
        let suggestion_0 = Suggestion::ValidExprs(vec![
            Cow::Borrowed(&content[17..20]),
            Cow::Borrowed(&content[23..26]),
        ]);
        let suggestion_1 = Suggestion::SourceRefHint(Box::new(SourceRefHint {
            source_ref: SourceHighlighted {
                path: Some(Cow::Borrowed(path)),
                expr_context: expr_context_hint_yaml_both(content),
                expr: Some(expr_context_hint_yaml_single(
                    content,
                    Some(Cow::Borrowed("first defined here")),
                )),
            },
            description: String::from("`chosen` value must come from one of `available` values"),
        }));
        let suggestions = vec![suggestion_0, suggestion_1];

        let error_code = ChosenInvalid {
            value: &content[37..40],
        };
        source_error(
            path,
            content,
            error_code,
            expr_yaml_single,
            expr_context_yaml_single,
            suggestions,
        )
    }

    fn long_line_expr_context_expr<'path, 'source>(
        path: &'path Path,
        content: &'source str,
    ) -> SourceError<'path, 'source, ValueInvalid<'source>> {
        let suggestion_0 = Suggestion::Hint("expected value to be less than 26");
        let suggestions = vec![suggestion_0];

        let error_code = ValueInvalid {
            value: &content[100..103],
        };
        source_error(
            path,
            content,
            error_code,
            expr_json_single,
            expr_context_json_single,
            suggestions,
        )
    }

    fn source_error<'path, 'source, E>(
        path: &'path Path,
        content: &'source str,
        error_code: E,
        expr: fn(&'source str) -> ExprHighlighted<'source>,
        expr_context: fn(&'source str) -> ExprHighlighted<'source>,
        suggestions: Vec<Suggestion<'path, 'source>>,
    ) -> SourceError<'path, 'source, E>
    where
        E: ErrorCode,
    {
        let expr = expr(content);
        let expr_context = expr_context(content);
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

    fn expr_toml_single<'source>(content: &'source str) -> ExprHighlighted<'source> {
        let inner = Expr {
            line_number: 2,
            col_number: 13,
            value: Cow::Borrowed(&content[21..23]),
        };
        ExprHighlighted { inner, hint: None }
    }

    fn expr_toml_single_high_line<'source>(content: &'source str) -> ExprHighlighted<'source> {
        let inner = Expr {
            line_number: 201,
            col_number: 13,
            value: Cow::Borrowed(&content[21..23]),
        };
        ExprHighlighted { inner, hint: None }
    }

    fn expr_yaml_single<'source>(content: &'source str) -> ExprHighlighted<'source> {
        let inner = Expr {
            line_number: 6,
            col_number: 9,
            value: Cow::Borrowed(&content[36..41]),
        };
        ExprHighlighted { inner, hint: None }
    }

    fn expr_context_before<'source>(content: &'source str) -> ExprHighlighted<'source> {
        let inner = Expr {
            line_number: 1,
            col_number: 1,
            value: Cow::Borrowed(&content[0..23]),
        };
        ExprHighlighted { inner, hint: None }
    }

    fn expr_context_single<'source>(content: &'source str) -> ExprHighlighted<'source> {
        let inner = Expr {
            line_number: 2,
            col_number: 1,
            value: Cow::Borrowed(&content[9..23]),
        };
        ExprHighlighted { inner, hint: None }
    }

    fn expr_context_single_high_line<'source>(content: &'source str) -> ExprHighlighted<'source> {
        let inner = Expr {
            line_number: 201,
            col_number: 1,
            value: Cow::Borrowed(&content[9..23]),
        };
        ExprHighlighted { inner, hint: None }
    }

    fn expr_context_yaml_single<'source>(content: &'source str) -> ExprHighlighted<'source> {
        let inner = Expr {
            line_number: 6,
            col_number: 1,
            value: Cow::Borrowed(&content[28..41]),
        };
        ExprHighlighted { inner, hint: None }
    }

    fn expr_context_hint_yaml_single<'source>(
        content: &'source str,
        hint: Option<Cow<'source, str>>,
    ) -> ExprHighlighted<'source> {
        let inner = Expr {
            line_number: 2,
            col_number: 1,
            value: Cow::Borrowed(&content[4..14]),
        };
        ExprHighlighted { inner, hint }
    }

    fn expr_context_hint_yaml_both<'source>(content: &'source str) -> ExprHighlighted<'source> {
        let inner = Expr {
            line_number: 2,
            col_number: 1,
            value: Cow::Borrowed(&content[4..26]),
        };
        ExprHighlighted { inner, hint: None }
    }

    fn expr_json_single<'source>(content: &'source str) -> ExprHighlighted<'source> {
        let inner = Expr {
            line_number: 1,
            col_number: 101,
            value: Cow::Borrowed(&content[100..103]),
        };
        ExprHighlighted { inner, hint: None }
    }

    fn expr_context_json_single<'source>(content: &'source str) -> ExprHighlighted<'source> {
        let inner = Expr {
            line_number: 1,
            col_number: 97,
            value: Cow::Borrowed(&content[96..104]),
        };
        ExprHighlighted { inner, hint: None }
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

    #[derive(Debug)]
    pub struct ChosenInvalid<'source> {
        value: &'source str,
    }

    impl<'source> ErrorCode for ChosenInvalid<'source> {
        const ERROR_CODE_MAX: usize = 300;
        const PREFIX: &'static str = "E";

        fn code(&self) -> usize {
            100
        }

        fn fmt_description<W>(&self, buffer: &mut W) -> Result<(), io::Error>
        where
            W: io::Write,
        {
            write!(buffer, "`chosen` value `{}` is invalid.", self.value)
        }
    }

    #[derive(Debug)]
    pub struct ValueInvalid<'source> {
        value: &'source str,
    }

    impl<'source> ErrorCode for ValueInvalid<'source> {
        const ERROR_CODE_MAX: usize = 99;
        const PREFIX: &'static str = "E";

        fn code(&self) -> usize {
            9
        }

        fn fmt_description<W>(&self, buffer: &mut W) -> Result<(), io::Error>
        where
            W: io::Write,
        {
            write!(buffer, "Value `{}` is invalid.", self.value)
        }
    }
}
