# PQC Binary Format v1.0.14

[![Crates.io](https://img.shields.io/crates/v/pqc-binary-format.svg)](https://crates.io/crates/pqc-binary-format)
[![Documentation](https://docs.rs/pqc-binary-format/badge.svg)](https://docs.rs/pqc-binary-format)
[![License](https://img.shields.io/crates/l/pqc-binary-format.svg)](LICENSE-MIT)
[![Build Status](https://github.com/PQCrypta/pqcrypta-community/workflows/CI/badge.svg)](https://github.com/PQCrypta/pqcrypta-community/actions)

**A standardized, self-describing binary format for post-quantum cryptography encrypted data interchange.**

## 🌟 The Problem

Post-quantum cryptography (PQC) implementations suffer from the "Babel Tower problem": different implementations cannot interoperate because there is no standardized format for encrypted data. Each library uses its own proprietary format, making cross-platform and cross-language encryption impossible.

## 💡 The Solution

PQC Binary Format provides a universal, algorithm-agnostic format that:

- ✅ Works across **47 cryptographic algorithms**
- ✅ **Self-describing metadata** enables seamless decryption
- ✅ **Integrity verification** with SHA-256 checksums
- ✅ **Cross-platform compatible** (Rust, Python, JavaScript, Go, etc.)
- ✅ **Future-proof** design allows algorithm migration
- ✅ **Zero dependencies** except serde and sha2

## 🚀 Quick Start

### Rust

Add to your `Cargo.toml`:

```toml
[dependencies]
pqc-binary-format = "1.0"
```

### Basic Usage (Rust)

```rust
use pqc_binary_format::{PqcBinaryFormat, Algorithm, PqcMetadata, EncParameters};
use std::collections::HashMap;

// Create metadata with encryption parameters
let metadata = PqcMetadata {
    enc_params: EncParameters {
        iv: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],  // 12-byte nonce
        tag: vec![0; 16],                                  // 16-byte auth tag
        params: HashMap::new(),
    },
    ..Default::default()
};

// Create encrypted data container
let encrypted_data = vec![1, 2, 3, 4, 5];  // Your encrypted bytes
let format = PqcBinaryFormat::new(Algorithm::Hybrid, metadata, encrypted_data);

// Serialize to bytes (for transmission or storage)
let bytes = format.to_bytes().unwrap();

// Deserialize from bytes (includes automatic checksum verification)
let recovered = PqcBinaryFormat::from_bytes(&bytes).unwrap();

assert_eq!(format, recovered);
println!("Algorithm: {}", recovered.algorithm().name());
```

### Python

Install the Python bindings:

```bash
cd bindings/python
pip install maturin
maturin develop --release
```

```python
from pqc_binary_format import Algorithm, EncParameters, PqcMetadata, PqcBinaryFormat

# Create algorithm and metadata
algorithm = Algorithm("hybrid")
enc_params = EncParameters(
    iv=bytes([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]),
    tag=bytes([0] * 16)
)
metadata = PqcMetadata(enc_params=enc_params, kem_params=None, sig_params=None, compression_params=None)

# Create and serialize format
pqc_format = PqcBinaryFormat(algorithm, metadata, bytes([1, 2, 3, 4, 5]))
serialized = pqc_format.to_bytes()

# Deserialize and verify
deserialized = PqcBinaryFormat.from_bytes(serialized)
deserialized.validate()  # Verify checksum integrity
print(f"Algorithm: {deserialized.algorithm.name}")
```

### JavaScript/TypeScript

Build the WebAssembly bindings:

```bash
cd bindings/javascript
npm install
npm run build
```

```javascript
import init, { WasmAlgorithm, WasmEncParameters, WasmPqcMetadata, WasmPqcBinaryFormat } from './pqc_binary_format.js';

await init();

const algorithm = new WasmAlgorithm('hybrid');
const encParams = new WasmEncParameters(
    new Uint8Array([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]),
    new Uint8Array(16)
);
const metadata = new WasmPqcMetadata(encParams);
const pqcFormat = new WasmPqcBinaryFormat(algorithm, metadata, new Uint8Array([1, 2, 3, 4, 5]));

const serialized = pqcFormat.toBytes();
const deserialized = WasmPqcBinaryFormat.fromBytes(serialized);
console.log(`Algorithm: ${deserialized.algorithm.name}`);
```

### Go

Build the Rust library first, then use the Go bindings:

```bash
cargo build --release
cd bindings/go
go build example.go
```

```go
package main

import (
    "fmt"
    "log"
    pqc "github.com/PQCrypta/pqcrypta-community/bindings/go"
)

func main() {
    // Metadata is an opaque, length-prefixed section (UTF-8 JSON by convention).
    metadata := []byte(`{"encryption_params":{"iv":"...","tag":"...","params":{}}}`)
    data := []byte{1, 2, 3, 4, 5}

    format := pqc.New(pqc.AlgorithmHybrid, metadata, data)

    serialized, err := format.Serialize()
    if err != nil {
        log.Fatal(err)
    }
    deserialized, err := pqc.Parse(serialized) // verifies the checksum
    if err != nil {
        log.Fatal(err)
    }

    fmt.Printf("Algorithm: %s (0x%04X)\n", deserialized.AlgorithmName(), deserialized.AlgorithmID)
}
```

### C/C++

Build the Rust library and generate the C header:

```bash
cargo build --release
cbindgen --config cbindgen.toml --output include/pqc_binary_format.h
cd bindings/c-cpp
make
```

```cpp
#include "pqc_binary_format.h"
#include <iostream>
#include <vector>

int main() {
    std::vector<uint8_t> iv = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12};
    std::vector<uint8_t> tag(16, 0);
    std::vector<uint8_t> data = {1, 2, 3, 4, 5};

    PqcFormatHandle* format = pqc_format_new(
        PQC_ALGORITHM_HYBRID,
        iv.data(), iv.size(),
        tag.data(), tag.size(),
        data.data(), data.size()
    );

    ByteBuffer serialized = pqc_format_to_bytes(format);
    PqcFormatHandle* deserialized = pqc_format_from_bytes(serialized.data, serialized.len);

    char* alg_name = pqc_format_get_algorithm_name(deserialized);
    std::cout << "Algorithm: " << alg_name << std::endl;

    pqc_free_string(alg_name);
    pqc_free_buffer(serialized);
    pqc_format_free(deserialized);
    pqc_format_free(format);

    return 0;
}
```

## 🌐 Language Bindings

PQC Binary Format provides **production-ready, fully tested bindings** for multiple programming languages. All bindings support the complete API and produce cross-compatible binary formats.

### Available Bindings (v1.0.14)

| Language | Status | Package | Documentation | Examples |
|----------|--------|---------|---------------|----------|
| **Rust** | ✅ Native | `pqc-binary-format` | [docs.rs](https://docs.rs/pqc-binary-format) | [3 examples](examples/) |
| **Python** | ✅ Tested | `pqc_binary_format` | [Python README](bindings/python/README.md) | [2 examples](examples/python/) |
| **JavaScript/WASM** | ✅ Tested | `pqc_binary_format` (npm) | [JS README](bindings/javascript/README.md) | [1 example](examples/javascript/) |
| **Go** | ✅ Tested | `github.com/PQCrypta/pqcrypta-community/bindings/go` | [pkg.go.dev](https://pkg.go.dev/github.com/PQCrypta/pqcrypta-community/bindings/go) | [1 example](bindings/go/examples/) |
| **C** | ✅ Tested | FFI via Rust | [C/C++ README](bindings/c-cpp/README.md) | [1 example](examples/c/) |
| **C++** | ✅ Tested | FFI via Rust | [C/C++ README](bindings/c-cpp/README.md) | [1 example](examples/cpp/) |

### Installation Quick Reference

```bash
# Rust
cargo add pqc-binary-format

# Python (via maturin)
python3 -m venv .venv && source .venv/bin/activate
pip install maturin
maturin develop --release

# JavaScript/WASM (via wasm-pack)
wasm-pack build --target web --features wasm

# Go
go get github.com/PQCrypta/pqcrypta-community/bindings/go

# C/C++ (build from source)
cargo build --release --no-default-features
# Link against target/release/libpqc_binary_format.so
```

### Cross-Language Compatibility

**All language bindings are fully interoperable!** You can:
- ✅ Encrypt data in Python, decrypt in Rust
- ✅ Serialize in Go, deserialize in JavaScript
- ✅ Create format in C++, validate in Python
- ✅ Mix any combination across platforms

Example workflow:
```bash
# Create encrypted data with Python
python3 examples/python/basic_usage.py > data.bin

# Verify with C++
LD_LIBRARY_PATH=target/release ./examples/cpp/basic_usage < data.bin

# Process with Go
cd examples/go && go run basic_usage.go < ../../data.bin
```

### Binding Features

All bindings support:
- ✅ Full algorithm suite (47 algorithms)
- ✅ Metadata serialization/deserialization
- ✅ SHA-256 integrity verification
- ✅ Feature flags (compression, streaming, etc.)
- ✅ Error handling with detailed messages
- ✅ Memory safety (Rust-backed)

### Package Distribution Status

| Platform | Status | Notes |
|----------|--------|-------|
| crates.io (Rust) | ✅ **Published** | **[v1.0.14 live!](https://crates.io/crates/pqc-binary-format)** |
| PyPI (Python) | ✅ **Published** | **[v1.0.14 live!](https://pypi.org/project/pqc-binary-format/)** |
| npm (JavaScript) | ✅ **Published** | **[v1.0.14 live!](https://www.npmjs.com/package/pqc-binary-format)** |
| pkg.go.dev (Go) | ✅ **Indexed** | **[v1.0.14 live!](https://pkg.go.dev/github.com/PQCrypta/pqcrypta-community/bindings/go)** |

## 📦 Binary Format Specification

All multi-byte integers are **little-endian**; the metadata section is a
**UTF-8 JSON** object. Fields appear in this exact order (see
`draft-riddel-pqc-binary-format-00`, Section 3):

```text
+-------------------+
| Magic (4 bytes)   | "PQC\x01" - Format identifier
+-------------------+
| Version (1 byte)  | 0x01 - Format version
+-------------------+
| Algorithm (2 bytes)| Algorithm identifier (0x0050 - 0x0905), little-endian
+-------------------+
| Flags (1 byte)    | Feature flags (reserved, 0x00 in v1)
+-------------------+
| Metadata Len (4)  | Length of metadata section, little-endian
+-------------------+
| Metadata (var)    | Algorithm-specific parameters (UTF-8 JSON)
+-------------------+
| Data Len (8)      | Length of encrypted payload, little-endian
+-------------------+
| Data (var)        | Encrypted data
+-------------------+
| Checksum (32)     | SHA-256 over all preceding bytes
+-------------------+
```

Fixed-header size (excluding the variable metadata/data sections) is **52 bytes**.
The checksum is `SHA-256(magic ‖ version ‖ algorithm_id ‖ flags ‖ metadata_len ‖
metadata ‖ data_len ‖ data)`; because the JSON metadata is emitted with sorted
keys it is deterministic across implementations.

## 🔐 Supported Algorithms

The format supports 47 cryptographic algorithm identifiers:

### Classical Algorithms (0x0050-0x00FF)
- **Classical** (0x0050): X25519 + Ed25519 + AES-256-GCM
- **Password Classical** (0x0051): Password-based encryption

### Hybrid Algorithms (0x0100-0x01FF)
- **Hybrid** (0x0100): ML-KEM-1024 + X25519 + ML-DSA-87 + Ed25519

### Post-Quantum Algorithms (0x0200-0x02FF)
- **Post-Quantum** (0x0200): ML-KEM-1024 + ML-DSA-87
- **ML-KEM-1024** (0x0202): Pure ML-KEM with AES-256-GCM
- **Multi-KEM** (0x0203): Dual-layer KEM
- **Multi-KEM Triple** (0x0204): Triple-layer KEM
- **Quad-Layer** (0x0205): Four independent layers
- **PQ3-Stack** (0x0207): Forward secrecy stack

### Max Secure Series (0x0300-0x0306)
High-security configurations for enterprise use

### FN-DSA Series (0x0400-0x0407)
Falcon-based signature algorithms

### Experimental (0x0500-0x0506)
Research and next-generation algorithms

### HQC Code-Based Series (0x0600-0x0602)
NIST 2025 Backup KEM standard - code-based cryptography
- **HQC-128** (0x0600): NIST Level 1, 128-bit security
- **HQC-192** (0x0601): NIST Level 3, 192-bit security
- **HQC-256** (0x0602): NIST Level 5, 256-bit security

### NIST ML-KEM Variants - FIPS 203 (0x0700-0x07FF)
- **ML-KEM-512** (0x0700): NIST Level 1, 128-bit security
- **ML-KEM-768** (0x0701): NIST Level 3, 192-bit security

### NIST ML-DSA Variants - FIPS 204 (0x0800-0x08FF)
- **ML-DSA-44** (0x0800): NIST Level 2, 128-bit security
- **ML-DSA-65** (0x0801): NIST Level 3, 192-bit security
- **ML-DSA-87** (0x0802): NIST Level 5, 256-bit security

### NIST SLH-DSA Variants - FIPS 205 (0x0900-0x09FF)
- **SLH-DSA-SHA2-128s** (0x0900): NIST Level 1, small signatures
- **SLH-DSA-SHA2-128f** (0x0901): NIST Level 1, fast signatures
- **SLH-DSA-SHA2-192s** (0x0902): NIST Level 3, small signatures
- **SLH-DSA-SHA2-192f** (0x0903): NIST Level 3, fast signatures
- **SLH-DSA-SHA2-256s** (0x0904): NIST Level 5, small signatures
- **SLH-DSA-SHA2-256f** (0x0905): NIST Level 5, fast signatures

[View full algorithm list](docs/algorithms.md)

## 🎯 Features

### Feature Flags

Control optional behavior with feature flags:

```rust
use pqc_binary_format::{PqcBinaryFormat, Algorithm, FormatFlags, PqcMetadata, EncParameters};
use std::collections::HashMap;

let flags = FormatFlags::new()
    .with_compression()       // Data was compressed before encryption
    .with_streaming()         // Streaming encryption mode
    .with_additional_auth();  // Additional authentication layer

let metadata = PqcMetadata {
    enc_params: EncParameters {
        iv: vec![1; 12],
        tag: vec![1; 16],
        params: HashMap::new(),
    },
    ..Default::default()
};

let format = PqcBinaryFormat::with_flags(
    Algorithm::QuadLayer,
    flags,
    metadata,
    vec![1, 2, 3],
);

assert!(format.flags().has_compression());
assert!(format.flags().has_streaming());
```

### Metadata Structure

The format includes rich metadata for decryption:

```rust
use pqc_binary_format::{PqcMetadata, KemParameters, SigParameters, EncParameters, CompressionParameters};
use std::collections::HashMap;

let metadata = PqcMetadata {
    // Key Encapsulation (optional)
    kem_params: Some(KemParameters {
        public_key: vec![/* ML-KEM public key */],
        ciphertext: vec![/* encapsulated key */],
        params: HashMap::new(),
    }),

    // Digital Signature (optional)
    sig_params: Some(SigParameters {
        public_key: vec![/* ML-DSA public key */],
        signature: vec![/* signature bytes */],
        params: HashMap::new(),
    }),

    // Symmetric Encryption (required)
    enc_params: EncParameters {
        iv: vec![1; 12],              // Nonce/IV
        tag: vec![1; 16],             // AEAD auth tag
        params: HashMap::new(),
    },

    // Compression (optional)
    compression_params: Some(CompressionParameters {
        algorithm: "zstd".to_string(),
        level: 3,
        original_size: 1024,
        params: HashMap::new(),
    }),

    // Custom parameters (extensible)
    custom: HashMap::new(),
};
```

### Custom Parameters

Add your own metadata:

```rust
use pqc_binary_format::PqcMetadata;

let mut metadata = PqcMetadata::new();
metadata.add_custom("my_param".to_string(), vec![1, 2, 3]);

// Later...
if let Some(value) = metadata.get_custom("my_param") {
    println!("Custom param: {:?}", value);
}
```

## 🔍 Integrity Verification

Every format includes a SHA-256 checksum calculated over all fields:

```rust
use pqc_binary_format::PqcBinaryFormat;

let bytes = format.to_bytes().unwrap();

// Tamper with the data
// let mut corrupted = bytes.clone();
// corrupted[50] ^= 0xFF;

// Deserialization automatically verifies checksum
match PqcBinaryFormat::from_bytes(&bytes) {
    Ok(format) => println!("✓ Checksum valid"),
    Err(e) => println!("✗ Checksum failed: {}", e),
}
```

## 📚 Examples

### Example 1: Basic Encryption Format

```rust
use pqc_binary_format::{PqcBinaryFormat, Algorithm, PqcMetadata, EncParameters};
use std::collections::HashMap;

fn main() {
    let metadata = PqcMetadata {
        enc_params: EncParameters {
            iv: vec![1; 12],
            tag: vec![1; 16],
            params: HashMap::new(),
        },
        ..Default::default()
    };

    let format = PqcBinaryFormat::new(
        Algorithm::Hybrid,
        metadata,
        vec![/* your encrypted data */],
    );

    // Save to file
    let bytes = format.to_bytes().unwrap();
    std::fs::write("encrypted.pqc", &bytes).unwrap();

    // Load from file
    let loaded_bytes = std::fs::read("encrypted.pqc").unwrap();
    let loaded = PqcBinaryFormat::from_bytes(&loaded_bytes).unwrap();

    println!("Algorithm: {}", loaded.algorithm().name());
}
```

### Example 2: Cross-Language Interoperability

**Rust (Encryption)**
```rust
let format = PqcBinaryFormat::new(Algorithm::PostQuantum, metadata, data);
let bytes = format.to_bytes().unwrap();
// Send bytes to Python/JavaScript/Go/C++
```

**Python (Decryption)**
```python
from pqc_binary_format import PqcBinaryFormat

format = PqcBinaryFormat.from_bytes(bytes)
print(f"Algorithm: {format.algorithm().name()}")
print(f"Data: {len(format.data())} bytes")
```

**JavaScript (Decryption)**
```javascript
const format = WasmPqcBinaryFormat.fromBytes(bytes);
console.log(`Algorithm: ${format.algorithm.name}`);
console.log(`Data: ${format.data.length} bytes`);
```

**Go (Decryption)**
```go
format, _ := pqc.FromBytes(bytes)
defer format.Free()
fmt.Printf("Algorithm: %s\n", format.GetAlgorithmName())
fmt.Printf("Data: %d bytes\n", len(format.GetData()))
```

### Example 3: Algorithm Migration

```rust
// Old data encrypted with Classical algorithm
let old_format = PqcBinaryFormat::from_bytes(&old_encrypted_data)?;
assert_eq!(old_format.algorithm(), Algorithm::Classical);

// Re-encrypt with Post-Quantum algorithm
let plaintext = decrypt_with_classical(&old_format)?;
let new_metadata = create_pq_metadata()?;
let new_format = PqcBinaryFormat::new(
    Algorithm::PostQuantum,
    new_metadata,
    encrypt_with_pq(&plaintext)?,
);

// Same format, different algorithm!
```

## 🎓 Use Cases

### 1. **Cross-Platform Encryption**
Encrypt in Rust, decrypt in Python, JavaScript, or Go using the same format.

### 2. **Long-Term Archival**
Self-describing format ensures data can be decrypted decades later even as algorithms evolve.

### 3. **Algorithm Agility**
Switch between algorithms without changing application code.

### 4. **Compliance & Audit**
Embedded metadata provides audit trail for regulatory compliance (GDPR, HIPAA, etc.).

### 5. **Research & Benchmarking**
Standardized format enables fair comparison of PQC algorithm performance.

## 🧪 Testing

```bash
# Run tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_binary_format_roundtrip
```

## 📊 Benchmarks

```bash
# Run benchmarks
cargo bench

# View benchmark results
open target/criterion/report/index.html
```

Performance characteristics:
- **Serialization**: ~50 MB/s for typical payloads
- **Deserialization**: ~45 MB/s (includes checksum verification)
- **Overhead**: ~100 bytes + metadata size

## 🔧 Development

### Building from Source

```bash
git clone https://github.com/PQCrypta/pqcrypta-community.git
cd pqcrypta-community
cargo build --release
```

### Running Examples

```bash
cargo run --example basic_usage
cargo run --example with_compression
cargo run --example cross_platform
```

## 🤝 Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Current Status

- **Language Bindings**: ✅ **Rust** (native), ✅ **Python** (tested v1.0.14), ✅ **JavaScript/WASM** (tested v1.0.14), ✅ **Go** (tested v1.0.14), ✅ **C/C++** (tested v1.0.14)
- **Examples**: ✅ 9 validated examples across 6 languages
- **Package Distribution**: ✅ **All platforms published!** crates.io, PyPI, npm, pkg.go.dev

### Areas for Contribution

- **Additional Language Bindings**: Java, C#, Ruby, Swift, Kotlin - help us expand!
- **Documentation**: Tutorials, integration guides, video walkthroughs
- **Testing**: Additional test cases, fuzzing, property-based testing
- **Performance**: SIMD optimizations, benchmark improvements
- **Standards**: Help draft RFC for IETF standardization submission

## 📄 License

Licensed under either of:

- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

at your option.

## 🙏 Acknowledgments

This format was developed as part of the [PQCrypta](https://pqcrypta.com) enterprise post-quantum cryptography platform. Special thanks to:

- NIST Post-Quantum Cryptography Project
- The Rust cryptography community
- Contributors to pqcrypto, ring, and other foundational crates

## 📖 References

- [NIST Post-Quantum Cryptography](https://csrc.nist.gov/projects/post-quantum-cryptography)
- [ML-KEM (Kyber) Specification](https://csrc.nist.gov/pubs/fips/203/final)
- [ML-DSA (Dilithium) Specification](https://csrc.nist.gov/pubs/fips/204/final)
- [PQCrypta Documentation](https://pqcrypta.com/docs)

## 🔗 Related Projects

- [pqcrypto](https://github.com/rustpq/pqcrypto) - Rust PQC implementations
- [Open Quantum Safe](https://openquantumsafe.org/) - PQC library collection
- [CIRCL](https://github.com/cloudflare/circl) - Cloudflare's crypto library

## 💬 Community & Support

- **GitHub Issues**: [Report bugs](https://github.com/PQCrypta/pqcrypta-community/issues)
- **Discussions**: [Ask questions](https://github.com/PQCrypta/pqcrypta-community/discussions)
- **Website**: [pqcrypta.com](https://pqcrypta.com)
- **Documentation**: [docs.rs/pqc-binary-format](https://docs.rs/pqc-binary-format)

---

**Made with ❤️ by the PQCrypta Community**

*Securing the future, one byte at a time.*
