//! Carpenter 930 Wire Cutter Memory File Generator
//!
//! A library for generating `.MEM` memory files for the Carpenter 930 wire cutter.
//!
//! ## Pure Rust Usage (No unsafe code)
//!
//! ```rust
//! use carpenter930::{Carpenter930Params, generate_mem_file};
//! # use std::error::Error;
//!
//! # fn main() -> Result<(), Box<dyn Error>> {
//! let mut params = Carpenter930Params::new("MYPROGRAM");
//! params.wire_length = 500;
//! params.wire_type = 0; // stranded
//!
//! let mem_data = generate_mem_file(&params);
//! std::fs::write("output.mem", &mem_data)?;
//! # Ok(())
//! # }
//! ```
//!
//! ## C FFI Usage
//!
//! Enable the `ffi` feature to access C-compatible functions:
//!
//! ```toml
//! [dependencies]
//! carpenter930 = { version = "0.1", features = ["ffi"] }
//! ```

mod generator;
mod types;
mod core;

// Re-export safe core API (which provides #![forbid(unsafe_code)] guarantee)
pub use crate::core::{generate_mem_file, Carpenter930Params};

// Conditionally compile FFI module
#[cfg(feature = "ffi")]
mod ffi;

#[cfg(feature = "ffi")]
pub use ffi::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_basic() {
        let params = Carpenter930Params::default();
        let mem = generate_mem_file(&params);
        assert_eq!(mem.len(), 512);
    }

    #[test]
    fn test_program_name() {
        let mut params = Carpenter930Params::new("TEST");
        params.wire_length = 100;

        let mem = generate_mem_file(&params);
        assert_eq!(mem.len(), 512);

        // Check program name at offset 240
        assert_eq!(&mem[240..244], b"TEST");
    }
}
