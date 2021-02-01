#![deny(missing_docs, missing_debug_implementations)]

//! User friendly errors from source data.

pub use crate::model::{ErrorCode, ErrorDetail, SourceError};

// Re-export `codespan_reporting` so consumers don't have to depend on the crate
// directly.
pub use codespan_reporting;

pub mod fmt;
pub mod model;
