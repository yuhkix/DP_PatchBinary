## DP-Patcher-rs

A small Rust CLI that patches the [Dragon's Prophet](https://www.dragons-prophet.com/) executable by searching for byte patterns and replacing them in-place. The tool works on a copy of the original file (Fileversion 170712_00088) to keep your source binary intact.

### Highlights
- **Rust only**: Simple, fast, and safe implementation in Rust (`src/main.rs`).
- **Binary-safe patching**: Reads and writes raw bytes without corrupting the file.
- **Pattern formats**: Accepts spaced hex strings (e.g., `4C 8D 05 ...`) and `\xNN`-style sequences.
- **Safety first**: The original file is copied to `dp_x64_patched.exe` before modifications.

### Repository structure
```text
DP-Patcher-rs/
  ├─ dp_x64_original.exe          # Input binary (place here)
  ├─ dp_x64_patched.exe           # Output binary (created by the tool)
  ├─ Cargo.toml                   # Rust package manifest
  └─ src/
      └─ main.rs                  # Rust entrypoint
```

## Getting started

### Prerequisites
- **Windows** (project authored for x64 binaries)
- **Rust** (stable) and **cargo**

### Prepare input
- Place your original binary as `dp_x64_original.exe` in the repository root.

## Usage

### Run in debug mode
```bash
cargo run
```
- Builds and runs the tool. It creates `dp_x64_patched.exe`, scans for the configured patterns, and applies in-place replacements.

### Build a release binary
```bash
cargo build --release
```
- Outputs an optimized executable at `target/release/dp_patch_binary` (or `dp_patch_binary.exe` on Windows).

## Customizing patterns

- Edit the `disable_hwid_log` array in `src/main.rs` to add or change `find: replace` pairs.
- Ensure `find` and `replace` byte sequences are the **same length**.
- Accepted formats:
  - Spaced hex: `"4C 8D 05 82 ..."`
  - Escaped hex: `"\x4C\x8D\x05\x82"`
- Note: The tool patches the **first occurrence** of each pattern.

## How it works
- **Copy**: Duplicate `dp_x64_original.exe` to `dp_x64_patched.exe`.
- **Search**: Load the patched copy and search for the target byte sequence.
- **Patch**: When found, write the replacement bytes at the exact offset.
- **Integrity**: Replacement sequences must be the same length as the originals to avoid shifting data.

## Tips and safety notes
- **Back up**: Always keep a backup of your original binary.
- **Admin rights**: If the file is protected or in a restricted directory, run your shell with appropriate permissions.
- **Performance**: Use the release build for faster scanning on large binaries.

## Disclaimer
This project is for educational purposes only. You are responsible for ensuring you have the right to modify any binary you patch and for complying with all applicable laws and agreements.
