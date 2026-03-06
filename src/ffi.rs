//! C FFI wrapper for Carpenter 930 library
//!
//! This module contains unsafe code for C interoperability.
//! It is only compiled when the `ffi` feature is enabled.

use crate::core::{generate_mem_file, Carpenter930Params};
use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;
use std::slice;

/// FFI-safe result structure
#[repr(C)]
pub struct GenerateResult {
    pub success: bool,
    pub data: *mut u8,
    pub len: usize,
    pub error_message: *const c_char,
}

/// Generate a .MEM file from parameters
///
/// # Safety
/// This function is unsafe because it:
/// - Dereferences raw pointers
/// - Allocates memory that must be freed by caller using `carpenter930_free_buffer`
#[no_mangle]
pub unsafe extern "C" fn carpenter930_generate(params: *const Carpenter930Params) -> GenerateResult {
    if params.is_null() {
        return GenerateResult {
            success: false,
            data: ptr::null_mut(),
            len: 0,
            error_message: b"Null params pointer\0".as_ptr() as *const c_char,
        };
    }

    let params_ref = &*params;
    let mem_data = generate_mem_file(params_ref);

    // Allocate buffer for return
    let boxed_slice = mem_data.to_vec().into_boxed_slice();
    let len = boxed_slice.len();
    let data_ptr = Box::into_raw(boxed_slice) as *mut u8;

    GenerateResult {
        success: true,
        data: data_ptr,
        len,
        error_message: ptr::null(),
    }
}

/// Generate a .MEM file and write to a file path
///
/// # Safety
/// This function is unsafe because it dereferences raw pointers
#[no_mangle]
pub unsafe extern "C" fn carpenter930_generate_to_file(
    params: *const Carpenter930Params,
    file_path: *const c_char,
) -> bool {
    if params.is_null() || file_path.is_null() {
        return false;
    }

    let params_ref = &*params;
    let c_str = CStr::from_ptr(file_path);
    let path_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return false,
    };

    let mem_data = generate_mem_file(params_ref);

    match std::fs::write(path_str, &mem_data) {
        Ok(_) => true,
        Err(_) => false,
    }
}

/// Free memory allocated by carpenter930_generate
///
/// # Safety
/// This function is unsafe because it:
/// - Dereferences raw pointers
/// - Assumes the pointer was allocated by carpenter930_generate
#[no_mangle]
pub unsafe extern "C" fn carpenter930_free_buffer(data: *mut u8, len: usize) {
    if !data.is_null() && len > 0 {
        let _ = Box::from_raw(slice::from_raw_parts_mut(data, len));
    }
}

/// Create a new Carpenter930Params with default values
#[no_mangle]
pub extern "C" fn carpenter930_params_new() -> *mut Carpenter930Params {
    Box::into_raw(Box::new(Carpenter930Params::default()))
}

/// Free a Carpenter930Params structure
///
/// # Safety
/// This function is unsafe because it dereferences raw pointers
#[no_mangle]
pub unsafe extern "C" fn carpenter930_params_free(params: *mut Carpenter930Params) {
    if !params.is_null() {
        let _ = Box::from_raw(params);
    }
}

/// Set the program name on a Carpenter930Params
///
/// # Safety
/// This function is unsafe because it dereferences raw pointers
#[no_mangle]
pub unsafe extern "C" fn carpenter930_params_set_name(
    params: *mut Carpenter930Params,
    name: *const c_char,
) -> bool {
    if params.is_null() || name.is_null() {
        return false;
    }

    let c_str = CStr::from_ptr(name);
    let name_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return false,
    };

    (*params).set_program_name(name_str);
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ffi_generate() {
        unsafe {
            let params = carpenter930_params_new();
            (*params).wire_length = 100;
            (*params).set_program_name("TEST");

            let result = carpenter930_generate(params);
            assert!(result.success);
            assert_eq!(result.len, 512);
            assert!(!result.data.is_null());

            carpenter930_free_buffer(result.data, result.len);
            carpenter930_params_free(params);
        }
    }
}
