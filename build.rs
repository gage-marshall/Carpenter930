use std::env;

fn main() {
    // Get the target OS and check if FFI feature is enabled
    // IMPORTANT: In build.rs, we must use environment variables, not #[cfg] attributes!
    // - CARGO_CFG_TARGET_OS gives us the TARGET platform (not the host)
    // - CARGO_FEATURE_FFI is set when --features ffi is used
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let is_ffi_enabled = env::var("CARGO_FEATURE_FFI").is_ok();

    // Only embed Windows resources when:
    // 1. Building for Windows target (not host!)
    // 2. FFI feature is enabled
    if target_os == "windows" && is_ffi_enabled {
        let mut res = winres::WindowsResource::new();

        // Set DLL-specific metadata
        res.set("InternalName", "carpenter930")
           .set("OriginalFilename", "carpenter930.dll")
           .set("ProductName", "Carpenter 930 Library");

        // Compile the resource file
        // This will embed version info from Cargo.toml [package] and [package.metadata.winres]
        if let Err(e) = res.compile() {
            eprintln!("Failed to compile Windows resources: {}", e);
            std::process::exit(1);
        }
    }

    // For non-Windows targets or non-FFI builds, this script does nothing
}
