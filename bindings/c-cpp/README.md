# PQC Binary Format - C/C++ Bindings

C and C++ bindings for the PQC Binary Format library via FFI, providing a standardized binary format for post-quantum cryptography encrypted data interchange.

## Installation

### Build Requirements

1. Build the Rust library:
```bash
cd ../..
cargo build --release
```

2. Generate the C header:
```bash
cbindgen --config cbindgen.toml --output include/pqc_binary_format.h
```

3. The library will be at `target/release/libpqc_binary_format.so` (Linux), `.dylib` (macOS), or `.dll` (Windows)

## Quick Start

### C++ Example

```cpp
#include "pqc_binary_format.h"
#include <iostream>
#include <vector>

int main() {
    // Create encryption parameters
    std::vector<uint8_t> iv = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12};
    std::vector<uint8_t> tag(16, 0);
    std::vector<uint8_t> data = {1, 2, 3, 4, 5};

    // Create PQC Binary Format
    PqcFormatHandle* format = pqc_format_new(
        PQC_ALGORITHM_HYBRID,
        iv.data(), iv.size(),
        tag.data(), tag.size(),
        data.data(), data.size()
    );

    if (!format) {
        std::cerr << "Failed to create format" << std::endl;
        return 1;
    }

    // Get algorithm info
    char* alg_name = pqc_format_get_algorithm_name(format);
    std::cout << "Algorithm: " << alg_name << std::endl;
    pqc_free_string(alg_name);

    // Serialize to bytes
    PqcByteBuffer serialized = pqc_format_to_bytes(format);
    if (!serialized.data) {
        std::cerr << "Serialization failed" << std::endl;
        pqc_format_free(format);
        return 1;
    }
    std::cout << "Serialized size: " << serialized.len << " bytes" << std::endl;

    // Deserialize from bytes
    PqcFormatHandle* deserialized = pqc_format_from_bytes(serialized.data, serialized.len);
    if (!deserialized) {
        std::cerr << "Deserialization failed" << std::endl;
        pqc_free_buffer(serialized);
        pqc_format_free(format);
        return 1;
    }

    // Validate
    if (pqc_format_validate(deserialized) == 0) {
        std::cout << "✓ Validation passed" << std::endl;
    }

    // Clean up
    pqc_free_buffer(serialized);
    pqc_format_free(deserialized);
    pqc_format_free(format);

    return 0;
}
```

### C Example

```c
#include "pqc_binary_format.h"
#include <stdio.h>
#include <stdint.h>

int main(void) {
    uint8_t iv[] = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12};
    uint8_t tag[16] = {0};
    uint8_t data[] = {1, 2, 3, 4, 5};

    PqcFormatHandle* format = pqc_format_new(
        PQC_ALGORITHM_HYBRID,
        iv, sizeof(iv),
        tag, sizeof(tag),
        data, sizeof(data)
    );

    if (!format) {
        fprintf(stderr, "Failed to create format\n");
        return 1;
    }

    char* version = pqc_get_version();
    printf("Library version: %s\n", version);
    pqc_free_string(version);

    PqcByteBuffer serialized = pqc_format_to_bytes(format);
    printf("Serialized size: %zu bytes\n", serialized.len);

    pqc_free_buffer(serialized);
    pqc_format_free(format);

    return 0;
}
```

## API Reference

### Types

#### `PqcFormatHandle`
Opaque handle to a PQC binary format structure.

#### `PqcByteBuffer`
Byte buffer structure:
```c
typedef struct {
    unsigned char* data;
    size_t len;
    size_t capacity;
} PqcByteBuffer;
```

### Constants

#### Algorithm IDs
```c
extern const uint16_t PQC_ALGORITHM_CLASSICAL;          // 0x0050
extern const uint16_t PQC_ALGORITHM_PASSWORD_CLASSICAL; // 0x0051
extern const uint16_t PQC_ALGORITHM_HYBRID;             // 0x0100
extern const uint16_t PQC_ALGORITHM_POST_QUANTUM;       // 0x0200
extern const uint16_t PQC_ALGORITHM_ML_KEM_1024;        // 0x0202
extern const uint16_t PQC_ALGORITHM_MULTI_KEM;          // 0x0203
extern const uint16_t PQC_ALGORITHM_MULTI_KEM_TRIPLE;   // 0x0204
extern const uint16_t PQC_ALGORITHM_QUAD_LAYER;         // 0x0205
extern const uint16_t PQC_ALGORITHM_PQ3_STACK;          // 0x0207
extern const uint16_t PQC_ALGORITHM_LATTICE_CODE_HYBRID;// 0x0208
```

### Functions

#### Creating Formats

##### `pqc_format_new`
```c
PqcFormatHandle* pqc_format_new(
    uint16_t algorithm_id,
    const unsigned char* iv,
    size_t iv_len,
    const unsigned char* tag,
    size_t tag_len,
    const unsigned char* data,
    size_t data_len
);
```
Create a new PQC binary format structure.

**Returns**: Handle or NULL on error

##### `pqc_format_new_with_kem`
```c
PqcFormatHandle* pqc_format_new_with_kem(
    uint16_t algorithm_id,
    const unsigned char* iv,
    size_t iv_len,
    const unsigned char* tag,
    size_t tag_len,
    const unsigned char* kem_public_key,
    size_t kem_public_key_len,
    const unsigned char* kem_ciphertext,
    size_t kem_ciphertext_len,
    const unsigned char* data,
    size_t data_len
);
```
Create a PQC binary format with KEM parameters.

**Returns**: Handle or NULL on error

#### Serialization

##### `pqc_format_to_bytes`
```c
PqcByteBuffer pqc_format_to_bytes(const PqcFormatHandle* handle);
```
Serialize PQC binary format to bytes.

**Returns**: ByteBuffer (check `.data != NULL` for success)

##### `pqc_format_from_bytes`
```c
PqcFormatHandle* pqc_format_from_bytes(
    const unsigned char* data,
    size_t len
);
```
Deserialize PQC binary format from bytes.

**Returns**: Handle or NULL on error

#### Accessors

##### `pqc_format_get_algorithm_id`
```c
uint16_t pqc_format_get_algorithm_id(const PqcFormatHandle* handle);
```
Get algorithm ID.

##### `pqc_format_get_algorithm_name`
```c
char* pqc_format_get_algorithm_name(const PqcFormatHandle* handle);
```
Get algorithm name. **Must free with `pqc_free_string`**.

##### `pqc_format_get_data`
```c
PqcByteBuffer pqc_format_get_data(const PqcFormatHandle* handle);
```
Get encrypted data. **Must free with `pqc_free_buffer`**.

##### `pqc_format_get_total_size`
```c
size_t pqc_format_get_total_size(const PqcFormatHandle* handle);
```
Get total serialized size.

#### Validation

##### `pqc_format_validate`
```c
int pqc_format_validate(const PqcFormatHandle* handle);
```
Validate format structure.

**Returns**: 0 on success, -1 on error

#### Version Info

##### `pqc_get_version`
```c
char* pqc_get_version(void);
```
Get library version. **Must free with `pqc_free_string`**.

##### `pqc_get_binary_version`
```c
uint8_t pqc_get_binary_version(void);
```
Get binary format version.

#### Memory Management

##### `pqc_format_free`
```c
void pqc_format_free(PqcFormatHandle* handle);
```
Free a PQC binary format handle. **Must be called for every created handle**.

##### `pqc_free_buffer`
```c
void pqc_free_buffer(PqcByteBuffer buffer);
```
Free a byte buffer. **Must be called for every buffer returned by the library**.

##### `pqc_free_string`
```c
void pqc_free_string(char* s);
```
Free a string. **Must be called for every string returned by the library**.

## Memory Management

**Critical**: Always free resources allocated by the library:

```c
// Free handles
PqcFormatHandle* format = pqc_format_new(...);
// ... use format ...
pqc_format_free(format);  // REQUIRED

// Free buffers
PqcByteBuffer buffer = pqc_format_to_bytes(format);
// ... use buffer ...
pqc_free_buffer(buffer);  // REQUIRED

// Free strings
char* name = pqc_format_get_algorithm_name(format);
// ... use name ...
pqc_free_string(name);    // REQUIRED
```

## Building

### Using Make (Linux/macOS)

```bash
cd bindings/c-cpp
make
make run
```

### Manual Compilation

#### Linux/macOS
```bash
g++ -std=c++17 -I../../include example.cpp \
    -L../../target/release -lpqc_binary_format \
    -ldl -lpthread -lm -o example

export LD_LIBRARY_PATH=../../target/release:$LD_LIBRARY_PATH
./example
```

#### Windows (MSVC)
```cmd
cl /EHsc /I..\..\include example.cpp /link ..\..\target\release\pqc_binary_format.lib
```

## Examples

See `example.cpp` for comprehensive examples including:
- Basic encryption format
- Format with KEM parameters
- Algorithm comparison
- Cross-platform interoperability

## Thread Safety

The library is thread-safe for independent PqcFormatHandle instances. Do not share handles between threads without external synchronization.

## License

Licensed under either of:
- MIT License
- Apache License, Version 2.0

at your option.

## Links

- [Main Repository](https://github.com/PQCrypta/pqcrypta-community)
- [Documentation](https://docs.rs/pqc-binary-format)
- [PQCrypta Platform](https://pqcrypta.com)
