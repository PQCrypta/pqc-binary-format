# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.6] - 2026-01-10

### Added
- Comprehensive examples for all language bindings (Rust, Python, JavaScript, Go, C, C++)
- `pyproject.toml` for proper Python package configuration
- Go module configuration for examples (`examples/go/go.mod`)
- Testing validation for all 9 examples across 6 languages

### Changed
- Updated Python API to use properties instead of methods (`.name` instead of `.name()`)
- Updated JavaScript import paths to use `pkg/` directory from wasm-pack
- Improved build documentation with correct commands for all languages
- Updated examples/README.md with validated build instructions

### Fixed
- Python bindings now correctly expose `.name`, `.id`, `.data` as properties
- Go bindings package conflict resolved (moved `example.go` to `.bak`)
- C/C++ examples now build correctly with `--no-default-features` flag
- JavaScript WASM module compatibility notes added for Node.js v22+

### Documentation
- Root README.md updated with corrected Python examples
- examples/README.md updated with tested build commands
- Added testing status badges to all language sections
- Clarified FFI build requirements for C/C++/Go bindings

### Tested
- ✅ Rust: 3 examples (basic_usage, algorithm_comparison, with_compression)
- ✅ Python: 2 examples (basic_usage, algorithm_comparison)
- ✅ C: 1 example (basic_usage)
- ✅ C++: 1 example (basic_usage)
- ✅ Go: 1 example (basic_usage)
- ⚠️ JavaScript: 1 example (WASM builds successfully, requires browser for Node.js v22+)

## [1.0.5] - 2026-01-10

### Changed
- Prepared for crates.io publication
- Version bump across all packages

## [1.0.4] - 2026-01-10

### Fixed
- Fixed type mismatches in Python/WASM bindings (compression level: i32 → u8)
- Removed unused imports in python.rs and ffi.rs
- Fixed cbindgen double-prefix issue (PqcPqcByteBuffer → ByteBuffer)
- Resolved all clippy warnings with strategic allow directives
- Fixed code formatting issues

### Changed
- Git author configuration set to "PQCrypta <allan@pqcrypta.com>"
- Updated all documentation version references
- Synchronized version numbers across all language bindings

## [1.0.3] - 2026-01-10

### Added
- Initial language bindings for Python, JavaScript (WASM), Go, C/C++
- FFI layer for C interoperability
- Type safety improvements

## [1.0.0] - 2026-01-10

### Added
- Initial release of PQC Binary Format
- Support for 28 cryptographic algorithms
- Standardized binary format specification
- Cross-platform compatibility
- SHA-256 integrity verification
- Core Rust implementation
