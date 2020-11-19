use crate::Severity;

/// How a message should be highlighted.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HighlightLevel {
    /// Error -- the software cannot function until this is corrected.
    Error,
    /// Warning -- the software may function, but a change should be made.
    Warning,
    /// Informative -- used for hints.
    Info,
}

impl From<Severity> for HighlightLevel {
    fn from(s: Severity) -> Self {
        match s {
            Severity::Deny => HighlightLevel::Error,
            Severity::Warning => HighlightLevel::Warning,
        }
    }
}
