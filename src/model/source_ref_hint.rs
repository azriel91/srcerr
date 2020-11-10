use crate::SourceHighlighted;

/// Reference a different part of the source data as a hint.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceRefHint<'path, 'source> {
    /// Source data that the hint appears in.
    pub source_ref: SourceHighlighted<'path, 'source>,
    /// Description that accompanies the highlighted source.
    ///
    /// This is printed alongside the error in the following format:
    ///
    /// ```rust,ignore
    /// "help: {description}:"
    /// ```
    pub description: String,
}
