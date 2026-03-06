use std::env;

fn main() {
    // Get the target OS and check if FFI feature is enabled
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let is_ffi_enabled = env::var("CARGO_FEATURE_FFI").is_ok();

    // Only embed Windows resources when:
    // 1. Building for Windows target
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

}
