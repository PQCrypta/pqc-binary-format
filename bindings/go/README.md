# PQC Binary Format - Go Bindings

Pure Go implementation of the PQC Binary Format v1.0 specification, providing a standardized binary format for post-quantum cryptography encrypted data interchange.

## Features

✅ **Pure Go** - No CGO, no external dependencies
✅ **Fully Portable** - Works on any platform Go supports
✅ **31+ Algorithms** - Support for all PQC Binary Format algorithms
✅ **Constant-time operations** - Side-channel resistant parsing
✅ **Zero-copy where possible** - Optimized for performance
✅ **100% Test Coverage** - Comprehensive test suite
✅ **Cross-platform** - Interoperable with Rust, Python, JavaScript, and C/C++ implementations

## Installation

```bash
go get github.com/PQCrypta/pqcrypta-community/bindings/go@v1.0.12
```

No build dependencies required - pure Go!

## Quick Start

```go
package main

import (
    "fmt"
    "log"
    pqc "github.com/PQCrypta/pqcrypta-community/bindings/go"
)

func main() {
    // Create metadata and encrypted data
    metadata := []byte(`{"algorithm":"Hybrid","encryption":"AES-256-GCM"}`)
    encryptedData := []byte("your encrypted data here")

    // Create new PQC Binary Format
    format := pqc.New(pqc.AlgorithmHybrid, metadata, encryptedData)
    fmt.Printf("Created: %s\n", format)
    fmt.Printf("Quantum Resistant: %v\n", format.IsQuantumResistant())

    // Serialize to binary
    binary, err := format.Serialize()
    if err != nil {
        log.Fatal(err)
    }
    fmt.Printf("Serialized: %d bytes\n", len(binary))

    // Parse binary data
    parsed, err := pqc.Parse(binary)
    if err != nil {
        log.Fatal(err)
    }
    fmt.Printf("Algorithm: %s\n", parsed.AlgorithmName())
    fmt.Printf("Checksum valid: %v\n", parsed.VerifyChecksum())
}
```

## API Reference

### Creating Formats

#### `New(algorithmID uint16, metadata, data []byte) *PqcBinaryFormat`

Create a new PQC Binary Format structure.

**Parameters**:
- `algorithmID` - Algorithm identifier (use constants below)
- `metadata` - Algorithm-specific metadata (JSON recommended)
- `data` - Encrypted payload data

**Returns**: `*PqcBinaryFormat`

**Example**:
```go
format := pqc.New(pqc.AlgorithmHybrid, metadata, encryptedData)
```

### Parsing Formats

#### `Parse(data []byte) (*PqcBinaryFormat, error)`

Parse PQC Binary Format from bytes with checksum verification.

**Parameters**:
- `data` - Binary data to parse

**Returns**: `*PqcBinaryFormat, error`

**Example**:
```go
format, err := pqc.Parse(binaryData)
if err != nil {
    log.Fatal(err)
}
```

### Methods

#### `Serialize() ([]byte, error)`

Serialize the format to bytes with SHA-256 checksum.

**Returns**: `[]byte, error`

#### `VerifyChecksum() bool`

Verify the SHA-256 checksum using constant-time comparison.

**Returns**: `bool`

#### `AlgorithmName() string`

Get human-readable algorithm name.

**Returns**: `string`

#### `IsQuantumResistant() bool`

Check if the algorithm is quantum-resistant.

**Returns**: `bool` - `false` for classical algorithms, `true` for all others

#### `Size() int`

Get total serialized size in bytes.

**Returns**: `int`

#### `String() string`

Get string representation of the format.

**Returns**: `string`

### Algorithm Constants

#### Core Algorithms
```go
AlgorithmClassical         uint16 = 0x0050  // X25519 + Ed25519
AlgorithmPasswordClassical uint16 = 0x0051  // Password-based classical
AlgorithmHybrid            uint16 = 0x0100  // Classical + Post-Quantum
AlgorithmPostQuantum       uint16 = 0x0200  // Pure post-quantum
AlgorithmMlKem1024         uint16 = 0x0202  // ML-KEM-1024 only
AlgorithmMultiAlgorithm    uint16 = 0x0201  // Runtime selection
```

#### Multi-KEM Series
```go
AlgorithmMultiKem          uint16 = 0x0203  // Multiple KEMs
AlgorithmMultiKemTriple    uint16 = 0x0204  // Triple-layer KEMs
AlgorithmQuadLayer         uint16 = 0x0205  // Quad-layer redundant
```

#### Advanced Stacks
```go
AlgorithmPq3Stack          uint16 = 0x0207  // PQ3-Stack forward secrecy
AlgorithmLatticeCodeHybrid uint16 = 0x0208  // Lattice-code hybrid
```

#### Max Secure Series
```go
AlgorithmMaxSecureLightweight      uint16 = 0x0300
AlgorithmMaxSecurePurePQ           uint16 = 0x0301
AlgorithmMaxSecureHybridTransition uint16 = 0x0302
AlgorithmMaxSecureStateless        uint16 = 0x0303
AlgorithmMaxSecureCryptoAgile      uint16 = 0x0304
AlgorithmMaxSecurePQCZK            uint16 = 0x0305
AlgorithmMaxSecureHybrid           uint16 = 0x0306
```

#### FN-DSA Signature Series
```go
AlgorithmFnDsa512Compact      uint16 = 0x0400
AlgorithmFnDsa1024Security    uint16 = 0x0401
AlgorithmFnDsaFPHardened      uint16 = 0x0402
AlgorithmFnDsaDualSignature   uint16 = 0x0403
AlgorithmFnDsaTransitionStack uint16 = 0x0404
AlgorithmFnDsaZKStack         uint16 = 0x0405
```

#### Experimental Series
```go
AlgorithmQuantumLatticeFusion      uint16 = 0x0500
AlgorithmPostZKHomomorphic         uint16 = 0x0501
AlgorithmQuantumResistantConsensus uint16 = 0x0502
AlgorithmEntropyOrchestrated       uint16 = 0x0503
AlgorithmAISynthesizedCryptoAgile  uint16 = 0x0504
```

### Error Handling

```go
var (
    ErrInvalidMagic    = errors.New("invalid magic bytes")
    ErrInvalidVersion  = errors.New("invalid version")
    ErrInvalidChecksum = errors.New("invalid checksum")
    ErrInvalidLength   = errors.New("invalid data length")
    ErrBufferTooSmall  = errors.New("buffer too small")
)
```

## Binary Format Structure

```
Offset | Size | Field
-------|------|------------------
0      | 4    | Magic ("PQC\x01")
4      | 1    | Version (0x01)
5      | 2    | Algorithm ID (big-endian)
7      | 4    | Metadata Length (big-endian)
11     | 8    | Data Length (big-endian)
19     | N    | Metadata
19+N   | M    | Encrypted Data
19+N+M | 32   | SHA-256 Checksum
```

Total size: `51 + len(metadata) + len(data)` bytes

## Examples

### Example 1: Basic Usage

```go
metadata := []byte(`{"encryption":"AES-256-GCM"}`)
data := []byte("encrypted payload")

format := pqc.New(pqc.AlgorithmHybrid, metadata, data)
binary, _ := format.Serialize()

parsed, _ := pqc.Parse(binary)
fmt.Println(parsed.AlgorithmName()) // "Hybrid"
```

### Example 2: Algorithm Iteration

```go
algorithms := []uint16{
    pqc.AlgorithmClassical,
    pqc.AlgorithmHybrid,
    pqc.AlgorithmPostQuantum,
    pqc.AlgorithmMlKem1024,
}

for _, algID := range algorithms {
    format := pqc.New(algID, nil, nil)
    fmt.Printf("%s - Quantum Resistant: %v\n",
        format.AlgorithmName(),
        format.IsQuantumResistant())
}
```

### Example 3: Checksum Verification

```go
format := pqc.New(pqc.AlgorithmPostQuantum, metadata, data)
binary, _ := format.Serialize()

// Corrupt data
binary[len(binary)-1] ^= 0xFF

parsed, err := pqc.Parse(binary)
if err == pqc.ErrInvalidChecksum {
    fmt.Println("Checksum validation caught corruption!")
}
```

### Example 4: Cross-Platform Interoperability

```go
// Create in Go
format := pqc.New(pqc.AlgorithmHybrid, metadata, data)
binary, _ := format.Serialize()

// Save to file - can be read by Rust, Python, JavaScript implementations
os.WriteFile("encrypted.pqc", binary, 0644)

// Later, parse from file
data, _ := os.ReadFile("encrypted.pqc")
parsed, _ := pqc.Parse(data)
```

## Testing

```bash
# Run tests
go test -v

# Run tests with coverage
go test -v -cover

# Run benchmarks
go test -bench=. -benchmem
```

## Performance

Typical benchmarks on modern hardware:

- **Serialize (1 KB)**: ~50 μs
- **Parse (1 KB)**: ~40 μs
- **Checksum verification**: ~30 μs
- **Overhead**: ~51 bytes + metadata length

## Cross-Platform Compatibility

This pure Go implementation is 100% compatible with:

- ✅ **Rust** - [crates.io/crates/pqc-binary-format](https://crates.io/crates/pqc-binary-format)
- ✅ **Python** - [pypi.org/project/pqc-binary-format](https://pypi.org/project/pqc-binary-format)
- ✅ **JavaScript** - [npmjs.com/package/pqc-binary-format](https://www.npmjs.com/package/pqc-binary-format)
- ✅ **C/C++** - FFI bindings available

All implementations use the same standardized binary format.

## Running the Example

```bash
# Build and run the example
go run example.go

# Or build first
go build example.go
./example
```

## License

Licensed under either of:

- MIT License
- Apache License, Version 2.0

at your option.

## Links

- **GitHub Repository**: https://github.com/PQCrypta/pqcrypta-community
- **Documentation**: https://pkg.go.dev/github.com/PQCrypta/pqcrypta-community/bindings/go
- **White Paper**: https://pqcrypta.com/pqcbv1/
- **Rust Crate**: https://crates.io/crates/pqc-binary-format
- **Python Package**: https://pypi.org/project/pqc-binary-format/
- **JavaScript Package**: https://www.npmjs.com/package/pqc-binary-format

## Contributing

Contributions are welcome! Please see the main repository for contribution guidelines.
