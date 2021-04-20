# 🪄 Srcerr

[![Crates.io](https://img.shields.io/crates/v/srcerr.svg)](https://crates.io/crates/srcerr)
![CI](https://github.com/azriel91/srcerr/workflows/CI/badge.svg)
[![Coverage Status](https://codecov.io/gh/azriel91/srcerr/branch/main/graph/badge.svg)](https://codecov.io/gh/azriel91/srcerr)

Types to track error codes and details.

This library provies a [`SourceError`] struct that holds:

* [`ErrorCode`]: Enum whose variants indicate error code, simple description.
* [`ErrorDetail`]: Enum with matching variants to `ErrorCode`, but each variant contains information specific to an instance of the error.
* [`Severity`]: The severity to report the error.

[`ErrorCode`]: https://docs.rs/srcerr/latest/srcerr/trait.ErrorCode.html
[`ErrorDetail`]: https://docs.rs/srcerr/latest/srcerr/trait.ErrorDetail.html
[`Severity`]: https://docs.rs/codespan-reporting/0.11.1/codespan_reporting/diagnostic/enum.Severity.html
[`SourceError`]: https://docs.rs/srcerr/latest/srcerr/struct.SourceError.html

## Usage

<details>
<summary>1. Implement ErrorCode</summary>

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SimpleErrorCode {
    /// Error when a value is out of range.
    ValueOutOfRange,
    /// Error when a string is too long.
    StringTooLong,
}

impl ErrorCode for SimpleErrorCode {
    const ERROR_CODE_MAX: usize = 2;
    const PREFIX: &'static str = "E";

    fn code(self) -> usize {
        match self {
            Self::ValueOutOfRange => 1,
            Self::StringTooLong => 2,
        }
    }

    fn description(self) -> &'static str {
        match self {
            Self::ValueOutOfRange => "Value out of range.",
            Self::StringTooLong => "String provided is too long.",
        }
    }
}
```

</details>

<details>
<summary>2. Implement ErrorDetail</summary>

```rust
#[derive(Debug)]
pub enum SimpleErrorDetail {
    /// Error when a value is out of range.
    ValueOutOfRange {
        /// ID of the file containing the invalid value.
        file_id: usize,
        /// The value.
        value: i32,
        /// Byte begin and end indices where the value is defined.
        value_byte_indices: Range<usize>,
        /// Range that the value must be within.
        range: RangeInclusive<u32>,
    },
    /// Error when a string is too long.
    StringTooLong {
        /// ID of the file containing the invalid value.
        file_id: usize,
        /// The value that is too long.
        value: String,
        /// Byte begin and end indices where the value is defined.
        value_byte_indices: Range<usize>,
        /// Maximum length allowed for the string.
        limit: usize,
    },
}

impl<'files> ErrorDetail<'files> for SimpleErrorDetail {
    type Files = SimpleFiles<&'files str, &'files str>;

    fn labels(&self) -> Vec<Label<usize>> {
        match self {
            Self::ValueOutOfRange {
                file_id,
                value_byte_indices,
                range,
                ..
            } => {
                vec![
                    Label::primary(*file_id, value_byte_indices.clone()).with_message(format!(
                        "not within the range: `{}..={}`",
                        range.start(),
                        range.end()
                    )),
                ]
            }
            Self::StringTooLong {
                file_id,
                value_byte_indices,
                limit,
                ..
            } => {
                vec![
                    Label::primary(*file_id, value_byte_indices.clone())
                        .with_message(format!("exceeds the {} character limit.", limit)),
                ]
            }
        }
    }

    fn notes(&self, _files: &Self::Files) -> Vec<String> {
        match self {
            Self::ValueOutOfRange { range, .. } => {
                let valid_exprs = range.clone().map(|n| Cow::Owned(n.to_string()));
                let suggestion = Note::valid_exprs(valid_exprs).expect("Failed to format note.");
                vec![suggestion]
            }
            Self::StringTooLong { .. } => vec![],
        }
    }
}
```

</details>

<details>
<summary>3. Construct SourceError when there is an error.</summary>

```rust
fn value_out_of_range<'f>(
    file_id: usize,
) -> SourceError<'f, SimpleErrorCode, SimpleErrorDetail, SimpleFiles<&'f str, &'f str>> {
    let error_code = SimpleErrorCode::ValueOutOfRange;
    let error_detail = SimpleErrorDetail::ValueOutOfRange {
        file_id,
        value: -1,
        value_byte_indices: 21..23,
        range: 1..=3,
    };
    let severity = Severity::Error;

    SourceError::new(error_code, error_detail, severity)
}

fn string_too_long<'f>(
    file_id: usize,
    content: &str,
) -> SourceError<'f, SimpleErrorCode, SimpleErrorDetail, SimpleFiles<&'f str, &'f str>> {
    let error_code = SimpleErrorCode::StringTooLong;
    let error_detail = SimpleErrorDetail::StringTooLong {
        file_id,
        value: content[40..47].to_string(),
        value_byte_indices: 39..48,
        limit: 5,
    };
    let severity = Severity::Error;

    SourceError::new(error_code, error_detail, severity)
}
```

</details>

<details>
<summary>4. Output the diagnostic message.</summary>

```rust
let value_out_of_range = value_out_of_range(file_id);
let value_out_of_range = value_out_of_range.as_diagnostic(&files);
let string_too_long = string_too_long(file_id, content);
let string_too_long = string_too_long.as_diagnostic(&files);

let writer = StandardStream::stderr(ColorChoice::Always);
let config = term::Config::default();
term::emit(&mut writer.lock(), &config, &files, &value_out_of_range)?;
term::emit(&mut writer.lock(), &config, &files, &string_too_long)?;
```

</details>

Sample usage can be seen in the [examples](examples).

```
cargo run --example simple
cargo run --example source_ref_hint
cargo run --example long_expr_context
cargo run --example html > /tmp/index.html
```

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
