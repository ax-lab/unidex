//! Provides utilities for loading Unicode Character Database (UCD) files
//! and parsing the data.
//!
//! This is NOT meant to be used at runtime in production systems. Instead,
//! the purpose of this crate is to be used as part of the build process
//! when generating Unicode related code.
//!
//! The above has consequences particularly for error handling. For simplicity
//! sake and ease of use, most errors are handled with simple assertions and
//! panics.

mod input;
pub use input::*;

pub mod blocks;