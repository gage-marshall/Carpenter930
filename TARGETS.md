# Supported Build Targets

The Carpenter 930 library supports building for multiple platforms and architectures. Pre-built binaries are available for all platforms listed below via [GitHub Releases](../../releases).

## Platform Support

### Windows (x86_64)

| Target Triple | Compiler | Output | Notes |
|---------------|----------|--------|-------|
| `x86_64-pc-windows-msvc` | MSVC | `carpenter930.dll` | **Recommended** - Native Windows compiler |
| `x86_64-pc-windows-gnu` | MinGW-w64 | `carpenter930.dll` | Alternative GNU toolchain |

**Windows Features:**
- Full version information embedded in DLL
- Company name, product name, copyright metadata
- Compatible with Visual Studio and MinGW toolchains

### Linux (x86_64 and ARM64)

| Target Triple | ABI | Output | Notes |
|---------------|-----|--------|-------|
| `x86_64-unknown-linux-gnu` | glibc | `libcarpenter930.so` | **Recommended** - Standard Linux |
| `x86_64-unknown-linux-musl` | musl | `libcarpenter930.so` | Static linking, no glibc dependency |
| `aarch64-unknown-linux-gnu` | glibc | `libcarpenter930.so` | ARM64 (Raspberry Pi 4+, AWS Graviton) |

**Linux Features:**
- Position-independent code (PIC)
- musl builds are fully static
- Compatible with all major distributions

### macOS (Intel and Apple Silicon)

| Target Triple | Architecture | Output | Notes |
|---------------|--------------|--------|-------|
| `x86_64-apple-darwin` | Intel x86_64 | `libcarpenter930.dylib` | Intel Macs |
| `aarch64-apple-darwin` | Apple Silicon (ARM64) | `libcarpenter930.dylib` | M1/M2/M3/M4 Macs |

**macOS Features:**
- Native Apple Silicon support
- Compatible with macOS 10.13+ (Intel) and 11.0+ (ARM)
- Code-signed binaries (when built on macOS)

## Building Locally

### Prerequisites

1. Install [Rust](https://rustup.rs/):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. For cross-compilation, install the target:
   ```bash
   rustup target add <target-triple>
   ```

### Build Commands

**Native build:**
```bash
cargo build --release --features ffi
```

**Cross-compilation (Linux to other Linux):**
```bash
cargo install cross
cross build --release --features ffi --target <target-triple>
```

**Cross-compilation (using zig as linker):**
```bash
cargo install cargo-zigbuild
cargo zigbuild --release --features ffi --target <target-triple>
```

## Output Artifacts

After building, the shared library will be located at:
```
target/<target-triple>/release/<library-name>
```

Examples:
- Windows: `target/x86_64-pc-windows-msvc/release/carpenter930.dll`
- Linux: `target/x86_64-unknown-linux-gnu/release/libcarpenter930.so`
- macOS: `target/x86_64-apple-darwin/release/libcarpenter930.dylib`

## C/C++ Integration

### Header File

A C header file is recommended for FFI usage. Example `carpenter930.h`:

```c
#ifndef CARPENTER930_H
#define CARPENTER930_H

#include <stdint.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

// Function declarations
void* carpenter930_params_new(void);
void carpenter930_params_free(void* params);
bool carpenter930_params_set_name(void* params, const char* name);
bool carpenter930_generate_to_file(void* params, const char* path);

#ifdef __cplusplus
}
#endif

#endif // CARPENTER930_H
```

### Linking

**GCC/Clang (Linux/macOS):**
```bash
gcc -o myapp myapp.c -L. -lcarpenter930
```

**MSVC (Windows):**
```cmd
cl myapp.c carpenter930.lib
```

**MinGW (Windows):**
```bash
gcc -o myapp.exe myapp.c -L. -lcarpenter930
```

## Release Artifacts

GitHub releases include pre-built binaries for all supported targets:

- `carpenter930-v1.0.0-x86_64-pc-windows-msvc.zip`
- `carpenter930-v1.0.0-x86_64-pc-windows-gnu.zip`
- `carpenter930-v1.0.0-x86_64-unknown-linux-gnu.tar.gz`
- `carpenter930-v1.0.0-x86_64-unknown-linux-musl.tar.gz`
- `carpenter930-v1.0.0-aarch64-unknown-linux-gnu.tar.gz`
- `carpenter930-v1.0.0-x86_64-apple-darwin.tar.gz`
- `carpenter930-v1.0.0-aarch64-apple-darwin.tar.gz`

Download the appropriate archive for your platform and extract the shared library.

## Troubleshooting

### Linux: "version `GLIBC_X.XX' not found"

The `x86_64-unknown-linux-gnu` build requires a recent glibc version. Use the `musl` target for maximum compatibility:
```bash
cross build --release --features ffi --target x86_64-unknown-linux-musl
```

### macOS: "dylib cannot be opened because the developer cannot be verified"

Right-click the dylib → "Open" → confirm to trust it, or use:
```bash
xattr -d com.apple.quarantine libcarpenter930.dylib
```

### Windows: Missing VCRUNTIME140.dll

The MSVC build requires the Visual C++ Redistributable. Download from [Microsoft](https://learn.microsoft.com/en-us/cpp/windows/latest-supported-vc-redist). Alternatively, use the GNU build which is self-contained.

## Additional Resources

- [Rust Cross-Compilation Guide](https://rust-lang.github.io/rustup/cross-compilation.html)
- [cross tool documentation](https://github.com/cross-rs/cross)
- [cargo-zigbuild](https://github.com/rust-cross/cargo-zigbuild)
