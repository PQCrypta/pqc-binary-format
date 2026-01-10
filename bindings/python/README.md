# PQC Binary Format - Python Bindings

Python bindings for the PQC Binary Format library, providing a standardized binary format for post-quantum cryptography encrypted data interchange.

## Installation

### From Source

```bash
# Install maturin (build tool for PyO3 projects)
pip install maturin

# Build and install in development mode
maturin develop --release

# Or build a wheel for distribution
maturin build --release
pip install target/wheels/*.whl
```

### Requirements

- Python 3.8 or higher
- Rust 1.75 or higher (for building from source)

## Quick Start

```python
from pqc_binary_format import (
    Algorithm,
    EncParameters,
    KemParameters,
    PqcMetadata,
    PqcBinaryFormat,
    FormatFlags,
)

# Create algorithm
algorithm = Algorithm("hybrid")

# Create encryption parameters
enc_params = EncParameters(
    iv=bytes([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]),  # 12-byte nonce
    tag=bytes([0] * 16),  # 16-byte auth tag
)

# Create metadata
metadata = PqcMetadata(
    enc_params=enc_params,
    kem_params=None,
    sig_params=None,
    compression_params=None,
)

# Create PQC Binary Format
pqc_format = PqcBinaryFormat(algorithm, metadata, bytes([1, 2, 3, 4, 5]))

# Serialize to bytes
serialized = pqc_format.to_bytes()
print(f"Serialized size: {len(serialized)} bytes")

# Deserialize from bytes
deserialized = PqcBinaryFormat.from_bytes(serialized)
print(f"Algorithm: {deserialized.algorithm.name}")
print(f"Data: {len(deserialized.data)} bytes")

# Validate integrity
deserialized.validate()
print("✓ Validation passed - checksum verified")
```

## API Reference

### Classes

#### `Algorithm(name: str)`
Create an algorithm instance from name.

**Supported algorithms**: `"classical"`, `"hybrid"`, `"post-quantum"`, `"ml-kem-1024"`, `"multi-algorithm"`, `"multi-kem"`, `"multi-kem-triple"`, `"quad-layer"`, `"pq3-stack"`, `"lattice-code-hybrid"`

**Methods**:
- `name()` → `str`: Get algorithm name
- `id()` → `int`: Get algorithm ID

#### `EncParameters(iv: bytes, tag: bytes)`
Encryption parameters (IV/nonce and authentication tag).

#### `KemParameters(public_key: bytes, ciphertext: bytes)`
Key encapsulation mechanism parameters.

#### `SigParameters(public_key: bytes, signature: bytes)`
Digital signature parameters.

#### `CompressionParameters(algorithm: str, level: int, original_size: int)`
Compression parameters.

#### `PqcMetadata(enc_params, kem_params=None, sig_params=None, compression_params=None)`
Metadata container for PQC binary format.

#### `FormatFlags()`
Feature flags for the format.

**Methods**:
- `with_compression()` → `FormatFlags`: Enable compression flag
- `with_streaming()` → `FormatFlags`: Enable streaming flag
- `with_additional_auth()` → `FormatFlags`: Enable additional auth flag
- `with_experimental()` → `FormatFlags`: Enable experimental features flag
- `has_compression()` → `bool`: Check if compression is enabled
- `has_streaming()` → `bool`: Check if streaming is enabled
- `has_additional_auth()` → `bool`: Check if additional auth is enabled
- `has_experimental()` → `bool`: Check if experimental features are enabled

#### `PqcBinaryFormat(algorithm, metadata, data)`
Main PQC binary format class.

**Static Methods**:
- `from_bytes(data: bytes)` → `PqcBinaryFormat`: Deserialize from bytes
- `with_flags(algorithm, flags, metadata, data)` → `PqcBinaryFormat`: Create with specific flags

**Methods**:
- `to_bytes()` → `bytes`: Serialize to bytes
- `validate()`: Validate format structure
- `algorithm()` → `Algorithm`: Get algorithm
- `data()` → `bytes`: Get encrypted data
- `flags()` → `FormatFlags`: Get format flags
- `total_size()` → `int`: Get total serialized size

## Examples

See `example.py` for comprehensive examples including:
- Basic encryption format
- Format with KEM parameters
- Format with feature flags
- Algorithm comparison

## Testing

```bash
# Run example
python example.py
```

## License

Licensed under either of:
- MIT License
- Apache License, Version 2.0

at your option.

## Links

- [Main Repository](https://github.com/PQCrypta/pqcrypta-community)
- [Documentation](https://docs.rs/pqc-binary-format)
- [PQCrypta Platform](https://pqcrypta.com)
