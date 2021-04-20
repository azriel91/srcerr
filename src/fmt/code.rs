use std::{
    fmt::{self, Write},
    marker::PhantomData,
};

use crate::ErrorCode;

/// Formatting helpers for writing notes with consistent sentences.
#[derive(Debug)]
pub struct Code<E>(PhantomData<E>);

impl<E> Code<E>
where
    E: ErrorCode,
{
    /// Returns a `String` representing the error code.
    ///
    /// See [`Self::fmt_string`] for a non-allocating version of this.
    ///
    /// # Parameters
    ///
    /// * `error_code`: The error code.
    pub fn string(error_code: E) -> String {
        let mut buffer = String::new();
        Self::fmt_string(&mut buffer, error_code).expect("Failed to format error code.");
        buffer
    }

    /// Writes the error code into the buffer.
    ///
    /// See [`Self::string`] for a version that allocates a `String`.
    ///
    /// # Parameters
    ///
    /// * `buffer`: The buffer to write into.
    /// * `error_code`: The error code.
    pub fn fmt_string(buffer: &mut String, error_code: E) -> Result<(), fmt::Error> {
        let digits = Self::digits(E::ERROR_CODE_MAX);

        write!(
            buffer,
            "{prefix}{code:0>width$}",
            prefix = E::PREFIX,
            code = error_code.code(),
            width = digits
        )
    }

    /// Returns the number of digits that the given max value fits into.
    pub fn digits(value_max: usize) -> usize {
        // `Integer::log10` pending: <https://github.com/rust-lang/rust/pull/80918>
        // `FloatToInt` pending: <https://github.com/rust-lang/rust/issues/67057>
        //
        // +1 is because error codes should generally start from 1, not 0.
        (value_max as f32).log10().floor() as usize + 1
    }
}

#[cfg(test)]
mod tests {
    use super::Code;
    use crate::ErrorCode;

    #[test]
    fn zero_pads_error_code_log_10_exact() {
        let code_string = Code::string(Error11Max100);

        assert_eq!("E011", code_string);
    }

    #[test]
    fn zero_pads_error_code_log_10_inexact() {
        let code_string = Code::string(Error11Max99);

        assert_eq!("E11", code_string);
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct Error11Max99;
    impl ErrorCode for Error11Max99 {
        const ERROR_CODE_MAX: usize = 99;

        fn code(self) -> usize {
            11
        }

        fn description(self) -> &'static str {
            ""
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct Error11Max100;
    impl ErrorCode for Error11Max100 {
        const ERROR_CODE_MAX: usize = 100;

        fn code(self) -> usize {
            11
        }

        fn description(self) -> &'static str {
            ""
        }
    }
}
