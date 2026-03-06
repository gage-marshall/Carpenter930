#![forbid(unsafe_code)]

//! Safe core library for Carpenter 930 .MEM file generation
//!
//! This module contains zero unsafe code and can be used directly
//! from Rust without any FFI concerns.

// Re-export from crate root modules (which are declared in lib.rs)
pub use crate::generator::generate_mem_file;
pub use crate::types::Carpenter930Params;
