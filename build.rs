use std::env;

fn main() {
    // Get the target OS and check if FFI feature is enabled
    // IMPORTANT: Use CARGO_CFG_TARGET_OS to check the TARGET, not the host
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let is_ffi_enabled = env::var("CARGO_FEATURE_FFI").is_ok();

    // Only embed Windows resources when building for Windows targets with FFI enabled
    // This works correctly for cross-compilation (e.g., Linux host → Windows target)
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

    // For non-Windows targets, this build script does nothing
}
