/// Start (inclusive) and end (exclusive) positions of a `&str`.
///
/// This type is used instead of [`std::ops::Range`] because `Range` is `!Copy`.
///
/// See:
///
/// * <https://github.com/rust-lang/rust/issues/48649>
/// * <https://github.com/rust-lang/rust/pull/27186#issuecomment-123390413>
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Span {
    /// Exclusive start position of the `&str`.
    pub start: usize,
    /// Exclusive end position of the `&str`.
    pub end: usize,
}
