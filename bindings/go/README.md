# PQC Binary Format - Go Bindings

Go bindings for the PQC Binary Format library via cgo, providing a standardized binary format for post-quantum cryptography encrypted data interchange.

## Installation

### Build Requirements

1. Build the Rust library first:
```bash
cd ../..
cargo build --release
```

2. Add to your `go.mod`:
```go
require github.com/PQCrypta/pqcrypta-community/bindings/go v1.0.4
```

## Quick Start

```go
package main

import (
    "fmt"
    "log"
    pqc "github.com/PQCrypta/pqcrypta-community/bindings/go"
)

func main() {
    // Create encryption parameters
    iv := []byte{1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12} // 12-byte nonce
    tag := make([]byte, 16)                              // 16-byte auth tag
    encryptedData := []byte{1, 2, 3, 4, 5}

    // Create PQC Binary Format
    format, err := pqc.NewPqcBinaryFormat(pqc.AlgorithmHybrid, iv, tag, encryptedData)
    if err != nil {
        log.Fatalf("Failed to create format: %v", err)
    }
    defer format.Free() // Important: always free resources

    // Get algorithm info
    fmt.Printf("Algorithm: %s (ID: %d)\n", format.GetAlgorithmName(), format.GetAlgorithmID())

    // Serialize to bytes
    serialized, err := format.ToBytes()
    if err != nil {
        log.Fatalf("Failed to serialize: %v", err)
    }
    fmt.Printf("Serialized size: %d bytes\n", len(serialized))

    // Deserialize from bytes
    deserialized, err := pqc.FromBytes(serialized)
    if err != nil {
        log.Fatalf("Failed to deserialize: %v", err)
    }
    defer deserialized.Free()

    fmt.Printf("Deserialized algorithm: %s\n", deserialized.GetAlgorithmName())
    data := deserialized.GetData()
    fmt.Printf("Data length: %d bytes\n", len(data))

    // Validate
    if err := deserialized.Validate(); err != nil {
        log.Fatalf("Validation failed: %v", err)
    }
    fmt.Println("✓ Validation passed")
}
```

## API Reference

### Constants

#### Algorithm IDs
```go
const (
    AlgorithmClassical         = 0x0050
    AlgorithmPasswordClassical = 0x0051
    AlgorithmHybrid            = 0x0100
    AlgorithmPostQuantum       = 0x0200
    AlgorithmMlKem1024         = 0x0202
    AlgorithmMultiKem          = 0x0203
    AlgorithmMultiKemTriple    = 0x0204
    AlgorithmQuadLayer         = 0x0205
    AlgorithmPq3Stack          = 0x0207
    AlgorithmLatticeCodeHybrid = 0x0208
)
```

### Types

#### `PqcBinaryFormat`
Main type representing a PQC binary format structure.

### Functions

#### `NewPqcBinaryFormat(algorithmID uint16, iv, tag, data []byte) (*PqcBinaryFormat, error)`
Create a new PQC binary format structure.

**Parameters**:
- `algorithmID` - Algorithm identifier (use constants above)
- `iv` - IV/nonce bytes
- `tag` - Authentication tag bytes
- `data` - Encrypted data bytes

**Returns**: PqcBinaryFormat pointer and error

#### `NewPqcBinaryFormatWithKEM(algorithmID uint16, iv, tag, kemPublicKey, kemCiphertext, data []byte) (*PqcBinaryFormat, error)`
Create a new PQC binary format with KEM parameters.

**Parameters**:
- `algorithmID` - Algorithm identifier
- `iv` - IV/nonce bytes
- `tag` - Authentication tag bytes
- `kemPublicKey` - KEM public key bytes
- `kemCiphertext` - Encapsulated key ciphertext
- `data` - Encrypted data bytes

**Returns**: PqcBinaryFormat pointer and error

#### `FromBytes(data []byte) (*PqcBinaryFormat, error)`
Deserialize PQC binary format from bytes.

**Parameters**:
- `data` - Bytes to deserialize

**Returns**: PqcBinaryFormat pointer and error

#### `GetVersion() string`
Get library version string.

#### `GetBinaryVersion() uint8`
Get binary format version number.

### Methods

#### `(p *PqcBinaryFormat) ToBytes() ([]byte, error)`
Serialize the PQC binary format to bytes.

**Returns**: Serialized bytes and error

#### `(p *PqcBinaryFormat) GetAlgorithmID() uint16`
Get the algorithm ID.

**Returns**: Algorithm ID

#### `(p *PqcBinaryFormat) GetAlgorithmName() string`
Get the algorithm name.

**Returns**: Algorithm name string

#### `(p *PqcBinaryFormat) GetData() []byte`
Get the encrypted data.

**Returns**: Encrypted data bytes

#### `(p *PqcBinaryFormat) Validate() error`
Validate the format structure.

**Returns**: Error if validation fails, nil otherwise

#### `(p *PqcBinaryFormat) GetTotalSize() int`
Get the total serialized size.

**Returns**: Size in bytes

#### `(p *PqcBinaryFormat) Free()`
Free the PQC binary format handle. **Important**: Always call this when done using the format to prevent memory leaks.

## Memory Management

**Important**: Always call `Free()` on PqcBinaryFormat instances when done. The recommended pattern is:

```go
format, err := pqc.NewPqcBinaryFormat(...)
if err != nil {
    return err
}
defer format.Free() // Ensures cleanup even if function returns early
```

## Examples

See `example.go` for comprehensive examples including:
- Basic encryption format
- Format with KEM parameters
- Algorithm comparison
- Cross-platform interoperability

## Building

```bash
# Build the example
go build example.go

# Run the example
./example
```

## CGO Requirements

This package uses cgo to call into the Rust library. You need:
- GCC or Clang compiler
- The Rust library built (`libpqc_binary_format.so` or `.dylib` or `.dll`)

Set the library path if needed:
```bash
export CGO_LDFLAGS="-L/path/to/rust/target/release"
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
