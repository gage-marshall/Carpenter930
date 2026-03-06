fn main() {
    // Only embed Windows resources when building for Windows with FFI feature
    #[cfg(all(target_os = "windows", feature = "ffi"))]
    {
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

    // For non-Windows or non-FFI builds, this script does nothing
    #[cfg(not(all(target_os = "windows", feature = "ffi")))]
    {
        // No-op: Windows resources are only relevant for Windows DLL builds
    }
}
