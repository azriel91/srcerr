/// Suggestions to provide to the user.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Suggestion<'source> {
    /// Suggestions to provide as valid exprs.
    ValidExprs(Vec<&'source str>),
    /// Simple message to give to the user.
    Hint(&'source str),
}
