# PQC Binary Format - Examples

This directory contains working examples for all supported language bindings.

## Directory Structure

```
examples/
├── algorithm_comparison.rs    # Rust: Compare different algorithms
├── basic_usage.rs             # Rust: Basic format usage
├── with_compression.rs        # Rust: Using compression features
├── python/                    # Python examples
│   ├── basic_usage.py
│   └── algorithm_comparison.py
├── javascript/                # JavaScript/WASM examples
│   └── basic_usage.js
├── go/                        # Go examples
│   └── basic_usage.go
├── c/                         # C examples
│   └── basic_usage.c
└── cpp/                       # C++ examples
    └── basic_usage.cpp
```

## Quick Start by Language

### Rust Examples

```bash
# Run basic usage
cargo run --example basic_usage

# Run algorithm comparison
cargo run --example algorithm_comparison

# Run with compression
cargo run --example with_compression
```

### Python Examples

```bash
# First, build and install Python bindings
cd bindings/python
pip install maturin
maturin develop --release

# Then run examples
cd ../../examples/python
python3 basic_usage.py
python3 algorithm_comparison.py
```

### JavaScript Examples

```bash
# First, build WASM bindings
cd bindings/javascript
npm install
npm run build

# Then run examples
cd ../../examples/javascript
node basic_usage.js
```

### Go Examples

```bash
# First, build the Rust library
cargo build --release

# Then run examples
cd examples/go
go run basic_usage.go
```

### C Examples

```bash
# First, build the Rust library
cargo build --release

# Compile the example
cd examples/c
gcc -o basic_usage basic_usage.c \
    -I../../include \
    -L../../target/release \
    -lpqc_binary_format \
    -ldl -lpthread -lm

# Run with library path
LD_LIBRARY_PATH=../../target/release ./basic_usage
```

### C++ Examples

```bash
# First, build the Rust library
cargo build --release

# Compile the example
cd examples/cpp
g++ -std=c++17 -o basic_usage basic_usage.cpp \
    -I../../include \
    -L../../target/release \
    -lpqc_binary_format \
    -ldl -lpthread -lm

# Run with library path
LD_LIBRARY_PATH=../../target/release ./basic_usage
```

## Example Descriptions

### basic_usage

Demonstrates the core workflow:
1. Create encryption parameters (IV, authentication tag)
2. Create metadata container
3. Select cryptographic algorithm
4. Create PQC Binary Format structure
5. Serialize to bytes
6. Deserialize from bytes
7. Validate integrity (checksum verification)
8. Verify roundtrip (data matches)

### algorithm_comparison

Shows how different algorithms work with the same format:
- Classical (X25519 + Ed25519)
- Hybrid (ML-KEM-1024 + X25519 + ML-DSA-87)
- Post-Quantum (ML-KEM-1024 + ML-DSA-87)
- ML-KEM-1024 (Pure)
- Multi-Algorithm (Runtime selection)

### with_compression

Demonstrates using compression features:
- Enable compression flag
- Store compression metadata
- Format flags usage

## Cross-Language Interoperability

All examples produce compatible binary formats. You can:

1. **Encrypt in one language, decrypt in another:**
   ```bash
   # Create with Python
   python3 examples/python/basic_usage.py > encrypted.bin

   # Read with C++
   ./examples/cpp/basic_usage < encrypted.bin
   ```

2. **Mix and match:**
   - Rust → JavaScript
   - Go → Python
   - C++ → Rust
   - Any combination works!

## Common Issues

### Python: ModuleNotFoundError

```bash
# Install Python bindings first
cd bindings/python
pip install maturin
maturin develop --release
```

### JavaScript: Cannot find module

```bash
# Build WASM bindings first
cd bindings/javascript
npm run build
```

### C/C++: Library not found

```bash
# Always set LD_LIBRARY_PATH
export LD_LIBRARY_PATH=../../target/release
./basic_usage
```

### Go: Cannot find package

```bash
# Ensure library is built
cargo build --release

# Run from examples/go directory
cd examples/go
go run basic_usage.go
```

## Additional Resources

- [Main README](../../README.md) - Project overview
- [Python Bindings](../../bindings/python/README.md) - Python API reference
- [JavaScript Bindings](../../bindings/javascript/README.md) - JavaScript/WASM API
- [Go Bindings](../../bindings/go/README.md) - Go API reference
- [C/C++ Bindings](../../bindings/c-cpp/README.md) - C/C++ API reference

## Contributing

Found a bug in an example? Want to add a new example? See [CONTRIBUTING.md](../../CONTRIBUTING.md)!
