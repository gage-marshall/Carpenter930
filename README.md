# Carpenter 930 Wire Cutter Memory File Generator

A Rust library for generating `.MEM` memory files for the Carpenter 930 wire cutter.

**Core library is 100% safe Rust with `#![forbid(unsafe_code)]`**

The library can be used as:
- **Pure Rust library** (zero unsafe code, default)
- **C FFI library** (optional `ffi` feature, isolated unsafe code)

## File Format

- **Total size**: 512 bytes (128 × 4-byte records)
- **Layout**: Little-endian 32-bit unsigned integers
- **Records 1-34**: Machine parameters (wire dimensions, speeds, settings)
- **Records 35-40**: Fixed constants
- **Records 41-49**: Zeros
- **Records 50-60**: Padding (0xFFFFFFFF)
- **Records 61-63**: Program name (10 ASCII chars, space-padded) + 0x0000
- **Record 64**: Fixed value 0x0127C8A8
- **Records 65-128**: Padding (0xFFFFFFFF)

## Pre-built Binaries

**Download pre-built shared libraries for your platform from [GitHub Releases](../../releases)!**

Supports:
- **Windows** (x64): MSVC and GNU toolchains
- **Linux** (x64/ARM64): glibc and musl
- **macOS** (Intel/Apple Silicon): Universal binaries

See [TARGETS.md](TARGETS.md) for complete platform support details.

## Building from Source

### Pure Rust (No unsafe code)

```bash
cargo build --release
```

Outputs `target/release/libcarpenter930.rlib` - a pure Rust static library with **zero unsafe code**.

### C FFI Shared Library

```bash
cargo build --release --features ffi
```

Outputs platform-specific shared libraries:
- **Windows**: `carpenter930.dll` (with embedded version info)
- **Linux**: `libcarpenter930.so`
- **macOS**: `libcarpenter930.dylib`

### Cross-Compilation

For cross-compiling to other platforms:

```bash
# Install cross tool
cargo install cross

# Build for specific target
cross build --release --features ffi --target x86_64-unknown-linux-musl
```

See [TARGETS.md](TARGETS.md) for all supported targets and detailed build instructions.

## Usage

### Pure Rust API (100% Safe)

The core library enforces `#![forbid(unsafe_code)]`:

```rust
use carpenter930::{Carpenter930Params, generate_mem_file};

let mut params = Carpenter930Params::new("MYPROGRAM");
params.wire_length = 500;
params.wire_type = 0; // stranded
// ... set other parameters ...

let mem_data = generate_mem_file(&params);
std::fs::write("output.mem", &mem_data)?;
```

### C FFI (Requires `ffi` feature)

Enable the `ffi` feature in your `Cargo.toml`:

```toml
[dependencies]
carpenter930 = { version = "0.1", features = ["ffi"] }
```

Then use from C:

```c
#include <stdio.h>

// Function declarations
void* carpenter930_params_new(void);
void carpenter930_params_free(void* params);
bool carpenter930_params_set_name(void* params, const char* name);
bool carpenter930_generate_to_file(void* params, const char* path);

int main() {
    void* params = carpenter930_params_new();

    carpenter930_params_set_name(params, "TEST");
    // Set other fields via pointer manipulation or additional setter functions

    carpenter930_generate_to_file(params, "output.mem");
    carpenter930_params_free(params);

    return 0;
}
```

## Parameters

See `src/types.rs` for the complete `Carpenter930Params` structure with all 34 configurable parameters.

Key parameters include:
- `wire_length`: Wire length to cut
- `wire_type`: 0=stranded, 1=solid, 2=flat
- `blade_speed`, `feed_speed`, `pull_speed`: Speed settings (1-5)
- `full_eject`: Boolean (0 or 0x8000)
- `inch_mm`: Units (0=inch, 1=mm)
- `program_name`: 10-character identifier

## Testing

```bash
cargo test
cargo run --example generate_test
```

## License

MIT
