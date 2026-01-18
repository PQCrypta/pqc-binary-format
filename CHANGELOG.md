# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.14] - 2026-01-17

### Changed
- **Universal Algorithm Count**: Updated from 36/42 to **47 algorithm identifiers** across all documentation
  - Format is designed for universal PQC ecosystem use, not just PQ Crypta engines
  - Includes all NIST FIPS standalone variants (ML-KEM-512/768, ML-DSA-44/65/87, SLH-DSA-*)
- All package versions synchronized to 1.0.14
- Updated README.md, Go bindings, and all documentation to reflect 47 algorithms

### Documentation
- Comprehensive algorithm breakdown across 9 categories:
  - Core Algorithms (6)
  - Multi-KEM & Stacks (5)
  - Max-Secure Series (7)
  - HQC Code-Based (3)
  - ML-KEM FIPS 203 (2)
  - FN-DSA Signatures (8)
  - Experimental (7)
  - ML-DSA FIPS 204 (3)
  - SLH-DSA FIPS 205 (6)

## [1.0.13] - 2026-01-17

### Added
- **NIST Standard Algorithm IDs** - Complete FIPS algorithm identifier registry
  - **ML-KEM Variants (FIPS 203)** - Key Encapsulation Mechanisms
    - ML-KEM-512 (0x0700) - NIST Level 1, 128-bit security
    - ML-KEM-768 (0x0701) - NIST Level 3, 192-bit security
  - **ML-DSA Variants (FIPS 204)** - Digital Signature Algorithm
    - ML-DSA-44 (0x0800) - NIST Level 2, 128-bit security
    - ML-DSA-65 (0x0801) - NIST Level 3, 192-bit security
    - ML-DSA-87 (0x0802) - NIST Level 5, 256-bit security
  - **SLH-DSA Variants (FIPS 205)** - Stateless Hash-Based Digital Signatures
    - SLH-DSA-SHA2-128s (0x0900) - NIST Level 1, small signatures
    - SLH-DSA-SHA2-128f (0x0901) - NIST Level 1, fast signatures
    - SLH-DSA-SHA2-192s (0x0902) - NIST Level 3, small signatures
    - SLH-DSA-SHA2-192f (0x0903) - NIST Level 3, fast signatures
    - SLH-DSA-SHA2-256s (0x0904) - NIST Level 5, small signatures
    - SLH-DSA-SHA2-256f (0x0905) - NIST Level 5, fast signatures
- New algorithm ID ranges reserved: 0x0700-0x07FF (ML-KEM), 0x0800-0x08FF (ML-DSA), 0x0900-0x09FF (SLH-DSA)
- C/C++ FFI constants for all NIST algorithms (PQC_ALGORITHM_ML_KEM_512, etc.)
- **docs.rs Build Configuration**: Added `[package.metadata.docs.rs]` section to Cargo.toml
  - Configured `all-features = true` for comprehensive documentation
  - Added rustdoc-args for docs.rs environment detection
  - Specified target platform for consistent builds

### Changed
- Algorithm count increased from 31 to 47 algorithms (universal PQC support)
- All package versions at 1.0.14 for consistency
- Improved documentation build reliability across all registries

### Impact
- **NIST FIPS Compliance**: Full implementation of NIST PQC standard algorithm identifiers
- **Interoperability**: Dedicated IDs enable standardized cross-implementation communication
- **Future-Proof**: Reserved ID ranges for future NIST algorithm variants

## [1.0.11] - 2026-01-12

### Fixed
- **Documentation Sync**: Updated all README files to show correct version (v1.0.11)
- **Algorithm Counts**: Corrected all references from "28 algorithms" to "31 algorithms"
- **pkg/README.md**: Updated version header and algorithm counts
- **docs/algorithms.md**: Added complete HQC algorithm specifications and updated totals
- **npm Workflow**: Fixed wasm-pack build command syntax for automated publishing

### Changed
- All package versions incremented to 1.0.11 for consistency
- README documentation now accurately reflects 31 total algorithms
- Algorithm documentation expanded with HQC code-based series details

### Documentation
- Added comprehensive HQC-128/192/256 specifications to docs/algorithms.md
- Updated algorithm category index to include HQC series
- Corrected package registry status tables

## [1.0.10] - 2026-01-12

### Added
- **HQC Code-Based Cryptography** - NIST 2025 Backup KEM standard
  - HQC-128 (0x0600) - NIST Level 1, 128-bit security
  - HQC-192 (0x0601) - NIST Level 3, 192-bit security
  - HQC-256 (0x0602) - NIST Level 5, 256-bit security
- Algorithm family 0x0600-0x06FF reserved for code-based algorithms
- HQC constants in C/C++ FFI (`PQC_ALGORITHM_HQC_128/192/256`)
- HQC constants in Go bindings (`AlgorithmHqc128/192/256`)

### Changed
- Updated algorithm count from 28+ to 31+ algorithms
- Updated Rust crate documentation
- Updated Go package documentation
- Updated JavaScript package documentation
- Updated Python package documentation

### Impact
- **Cryptographic Diversity**: Adds code-based cryptography alongside lattice-based algorithms
- **NIST Compliance**: Implements NIST 2025 backup KEM standard
- **Quantum Resistance**: Provides additional security layer with different mathematical foundation

## [1.0.9] - 2026-01-11

### Published
- ✅ **crates.io**: https://crates.io/crates/pqc-binary-format v1.0.9
- ✅ **PyPI**: https://pypi.org/project/pqc-binary-format/ v1.0.9
- ✅ **npm**: https://www.npmjs.com/package/pqc-binary-format v1.0.9
- ✅ **pkg.go.dev**: https://pkg.go.dev/github.com/PQCrypta/pqcrypta-community/bindings/go@v1.0.9

### Added
- **Pure Go implementation** - Complete rewrite of Go bindings eliminating CGO dependencies
- Comprehensive Go test suite with 12 tests and benchmarks
- Go examples directory with detailed usage examples
- MIT LICENSE file in Go bindings directory for pkg.go.dev compliance
- Full pkg.go.dev documentation now visible and indexed

### Changed
- **Breaking change in Go**: Go bindings now pure Go implementation (no CGO)
  - API changed from CGO-based to pure Go functions
  - Old: `NewPqcBinaryFormat()` → New: `pqc.New()`
  - Removed dependency on Rust library compilation
  - Fully portable across all Go-supported platforms
- Updated all language binding versions to 1.0.9 for consistency
- Updated all documentation references to version 1.0.9

### Fixed
- Go bindings now properly indexed on pkg.go.dev
- Go package documentation now displays correctly (license issue resolved)
- Cross-platform compatibility improved with pure Go implementation

### Performance
- Go bindings performance: Serialize 1KB ~50μs, Parse 1KB ~40μs
- Constant-time checksum verification
- Zero-copy operations where possible

## [1.0.7] - 2026-01-10

### Published
- ✅ **crates.io**: https://crates.io/crates/pqc-binary-format
- ✅ **PyPI**: https://pypi.org/project/pqc-binary-format/
- ⏳ **npm**: https://www.npmjs.com/package/pqc-binary-format (publishing)
- ✅ **pkg.go.dev**: https://pkg.go.dev/github.com/PQCrypta/pqcrypta-community/bindings/go

### Added
- New "Language Bindings" section in README with comprehensive table of all bindings
- Cross-language compatibility examples and workflow documentation
- Installation quick reference for all supported languages
- Package distribution status tracking table
- "Current Status" section in Contributing guidelines
- GitHub Actions workflows for automated publishing to npm and PyPI
- Comprehensive PUBLISHING.md guide for all package registries

### Changed
- Updated "Areas for Contribution" to reflect completed language bindings
- Clarified that Python, JavaScript, Go, C, C++ bindings are production-ready and tested
- Improved contributing guidelines with clearer focus areas
- Version bumped to 1.0.7 across all packages
- Press release updated with all package registry links

### Documentation
- Enhanced README with language binding details and cross-platform examples
- Updated all version references to 1.0.7
- Improved clarity on which bindings are complete vs. contribution opportunities
- Updated press release with multi-platform availability

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
