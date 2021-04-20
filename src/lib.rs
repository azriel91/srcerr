#![deny(missing_docs, missing_debug_implementations)]

//! Types to track error codes and details.
//!
//! This library provies a [`SourceError`] struct that holds:
//!
//! * [`ErrorCode`]: Enum whose variants indicate error code, simple
//!   description.
//! * [`ErrorDetail`]: Enum with matching variants to `ErrorCode`, but each
//!   variant contains information specific to an instance of the error.
//! * [`Severity`]: The severity to report the error.
//!
//! [`Severity`]: crate::codespan_reporting::Severity
//!
//! # Examples
//!
//! Sample usage can be seen in the repository [examples].
//!
//! [examples]: https://github.com/azriel91/srcerr/tree/main/examples

pub use crate::model::{ErrorCode, ErrorDetail, SourceError};

// Re-export `codespan` so consumers don't have to depend on the crate directly.
#[cfg(feature = "codespan")]
pub use codespan;

// Re-export `codespan_reporting` so consumers don't have to depend on the crate
// directly.
pub use codespan_reporting;

pub mod fmt;
pub mod model;
