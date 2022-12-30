use std::{
    borrow::Cow,
    fmt::{self, Write},
};

/// Formatting helpers for writing notes with consistent sentences.
#[derive(Debug)]
pub struct Note;

impl Note {
    /// Returns a note suggesting valid expressions.
    ///
    /// See [`Self::fmt_valid_exprs`] for a non-allocating version of this.
    ///
    /// # Parameters
    ///
    /// * `valid_exprs`: Valid expressions the user can choose from.
    pub fn valid_exprs<'s>(
        valid_exprs: impl Iterator<Item = Cow<'s, str>>,
    ) -> Result<String, fmt::Error> {
        let mut buffer = String::with_capacity(512);
        Self::fmt_valid_exprs(&mut buffer, valid_exprs)?;

        Ok(buffer)
    }

    /// Writes a note suggesting valid expressions into the buffer.
    ///
    /// See [`Self::valid_exprs`] for a version that allocates a `String`.
    ///
    /// # Parameters
    ///
    /// * `buffer`: The buffer to write into.
    /// * `valid_exprs`: Valid expressions the user can choose from.
    pub fn fmt_valid_exprs<'s>(
        buffer: &mut String,
        mut valid_exprs: impl Iterator<Item = Cow<'s, str>>,
    ) -> Result<(), fmt::Error> {
        write!(buffer, "expected value to be one of: ")?;

        if let Some(first_valid_expr) = valid_exprs.next() {
            write!(buffer, "`{first_valid_expr}`")?;
        }
        valid_exprs.try_for_each(|valid_expr| write!(buffer, ", `{valid_expr}`"))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::{borrow::Cow, fmt};

    use super::Note;

    #[test]
    fn valid_exprs_does_not_have_comma_when_one_suggestion() -> Result<(), fmt::Error> {
        let valid_exprs = std::iter::once(Cow::Borrowed("abc"));

        let note = Note::valid_exprs(valid_exprs)?;

        assert_eq!("expected value to be one of: `abc`", note);
        Ok(())
    }

    #[test]
    fn valid_exprs_returns_list_of_suggested_values() -> Result<(), fmt::Error> {
        let valid_exprs = vec![Cow::Borrowed("abc"), Cow::Borrowed("def")];

        let note = Note::valid_exprs(valid_exprs.into_iter())?;

        assert_eq!("expected value to be one of: `abc`, `def`", note);
        Ok(())
    }
}
