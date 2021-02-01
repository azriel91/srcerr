//! Data types representing the source error.

pub use self::{error_code::ErrorCode, error_detail::ErrorDetail, source_error::SourceError};

mod error_code;
mod error_detail;
mod source_error;
