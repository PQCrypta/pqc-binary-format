# PQC Binary Format - JavaScript/TypeScript Bindings

WebAssembly bindings for the PQC Binary Format library, providing a standardized binary format for post-quantum cryptography encrypted data interchange in JavaScript/TypeScript environments.

## Installation

### Build from Source

```bash
# Install wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Build for different targets
npm run build          # Web (ES modules)
npm run build:nodejs   # Node.js
npm run build:bundler  # Webpack/Rollup/Parcel
```

### Requirements

- Node.js 16 or higher
- Rust 1.75 or higher
- wasm-pack

## Quick Start

### Web/ES Modules

```javascript
import init, {
    WasmAlgorithm,
    WasmEncParameters,
    WasmPqcMetadata,
    WasmPqcBinaryFormat,
    WasmFormatFlags,
    getVersion,
    getBinaryVersion
} from './pqc_binary_format.js';

// Initialize WASM module
await init();

// Create algorithm
const algorithm = new WasmAlgorithm('hybrid');

// Create encryption parameters
const encParams = new WasmEncParameters(
    new Uint8Array([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]), // IV
    new Uint8Array(16) // Auth tag
);

// Create metadata
const metadata = new WasmPqcMetadata(encParams);

// Create PQC Binary Format
const pqcFormat = new WasmPqcBinaryFormat(
    algorithm,
    metadata,
    new Uint8Array([1, 2, 3, 4, 5])
);

// Serialize to bytes
const serialized = pqcFormat.toBytes();
console.log(`Serialized size: ${serialized.length} bytes`);

// Deserialize from bytes
const deserialized = WasmPqcBinaryFormat.fromBytes(serialized);
console.log(`Algorithm: ${deserialized.algorithm.name}`);
console.log(`Data: ${deserialized.data.length} bytes`);

// Validate
deserialized.validate();
console.log('✓ Validation passed');
```

### Node.js

```javascript
const { WasmAlgorithm, WasmPqcBinaryFormat, ...rest } = require('./pqc_binary_format');

// Same API as web version
```

## API Reference

### Functions

#### `init(input?: RequestInfo | URL | Response | BufferSource | WebAssembly.Module): Promise<void>`
Initialize the WASM module. Must be called before using any other functions.

#### `getVersion(): string`
Get library version string.

#### `getBinaryVersion(): number`
Get binary format version number.

### Classes

#### `WasmAlgorithm`
Algorithm wrapper for WASM.

**Constructor**: `new WasmAlgorithm(name: string)`

**Supported algorithms**: `"classical"`, `"hybrid"`, `"post-quantum"`, `"ml-kem-1024"`, `"multi-algorithm"`, `"multi-kem"`, `"multi-kem-triple"`, `"quad-layer"`, `"pq3-stack"`, `"lattice-code-hybrid"`

**Properties**:
- `name: string` - Algorithm name
- `id: number` - Algorithm ID

**Static Methods**:
- `supportedAlgorithms(): string[]` - Get list of supported algorithm names

#### `WasmEncParameters`
Encryption parameters.

**Constructor**: `new WasmEncParameters(iv: Uint8Array, tag: Uint8Array)`

**Properties**:
- `iv: Uint8Array` - IV/nonce
- `tag: Uint8Array` - Authentication tag

#### `WasmKemParameters`
Key encapsulation mechanism parameters.

**Constructor**: `new WasmKemParameters(publicKey: Uint8Array, ciphertext: Uint8Array)`

**Properties**:
- `publicKey: Uint8Array` - Public key
- `ciphertext: Uint8Array` - Ciphertext/encapsulated key

#### `WasmSigParameters`
Digital signature parameters.

**Constructor**: `new WasmSigParameters(publicKey: Uint8Array, signature: Uint8Array)`

**Properties**:
- `publicKey: Uint8Array` - Public key
- `signature: Uint8Array` - Signature

#### `WasmCompressionParameters`
Compression parameters.

**Constructor**: `new WasmCompressionParameters(algorithm: string, level: number, originalSize: number)`

**Properties**:
- `algorithm: string` - Compression algorithm
- `level: number` - Compression level
- `originalSize: number` - Original size before compression

#### `WasmPqcMetadata`
Metadata container.

**Constructor**: `new WasmPqcMetadata(encParams: WasmEncParameters)`

**Methods**:
- `setKemParams(kemParams: WasmKemParameters): void` - Set KEM parameters
- `setSigParams(sigParams: WasmSigParameters): void` - Set signature parameters
- `setCompressionParams(compressionParams: WasmCompressionParameters): void` - Set compression parameters

#### `WasmFormatFlags`
Feature flags.

**Constructor**: `new WasmFormatFlags()`

**Methods**:
- `withCompression(): WasmFormatFlags` - Enable compression flag
- `withStreaming(): WasmFormatFlags` - Enable streaming flag
- `withAdditionalAuth(): WasmFormatFlags` - Enable additional auth flag
- `withExperimental(): WasmFormatFlags` - Enable experimental features flag
- `hasCompression(): boolean` - Check compression flag
- `hasStreaming(): boolean` - Check streaming flag
- `hasAdditionalAuth(): boolean` - Check additional auth flag
- `hasExperimental(): boolean` - Check experimental features flag

#### `WasmPqcBinaryFormat`
Main PQC binary format class.

**Constructor**: `new WasmPqcBinaryFormat(algorithm: WasmAlgorithm, metadata: WasmPqcMetadata, data: Uint8Array)`

**Static Methods**:
- `fromBytes(data: Uint8Array): WasmPqcBinaryFormat` - Deserialize from bytes
- `withFlags(algorithm: WasmAlgorithm, flags: WasmFormatFlags, metadata: WasmPqcMetadata, data: Uint8Array): WasmPqcBinaryFormat` - Create with flags

**Methods**:
- `toBytes(): Uint8Array` - Serialize to bytes
- `validate(): void` - Validate format structure (throws on error)
- `totalSize(): number` - Get total serialized size

**Properties**:
- `algorithm: WasmAlgorithm` - Algorithm used
- `data: Uint8Array` - Encrypted data
- `flags: WasmFormatFlags` - Format flags

## Examples

See `example.js` for comprehensive examples including:
- Basic encryption format
- Format with KEM parameters
- Format with feature flags
- Algorithm comparison
- Cross-platform interoperability

## Testing

```bash
# Run example (Node.js)
node example.js

# Or in browser
# Serve the directory and open index.html
```

## TypeScript Support

TypeScript definitions are automatically generated by wasm-bindgen. Import types as:

```typescript
import init, {
    WasmAlgorithm,
    WasmPqcBinaryFormat,
    // ... other types
} from './pqc_binary_format';
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
