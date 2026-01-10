/**
 * Example usage of PQC Binary Format JavaScript/WebAssembly bindings
 */

import init, {
    WasmAlgorithm,
    WasmEncParameters,
    WasmKemParameters,
    WasmPqcMetadata,
    WasmPqcBinaryFormat,
    WasmFormatFlags,
    getVersion,
    getBinaryVersion
} from './pqc_binary_format.js';

async function main() {
    // Initialize the WASM module
    await init();

    console.log('PQC Binary Format JavaScript/WASM Example');
    console.log('='.repeat(50));
    console.log(`Library version: ${getVersion()}`);
    console.log(`Binary format version: ${getBinaryVersion()}`);

    // Example 1: Basic encryption format
    console.log('\n1. Basic Encryption Format');
    console.log('-'.repeat(50));

    // Create algorithm
    const algorithm = new WasmAlgorithm('hybrid');
    console.log(`Algorithm: ${algorithm.name} (ID: ${algorithm.id})`);

    // Create encryption parameters
    const encParams = new WasmEncParameters(
        new Uint8Array([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]), // 12-byte nonce
        new Uint8Array(16) // 16-byte auth tag
    );

    // Create metadata
    const metadata = new WasmPqcMetadata(encParams);

    // Create encrypted data (simulated)
    const encryptedData = new Uint8Array([1, 2, 3, 4, 5]);

    // Create PQC Binary Format
    const pqcFormat = new WasmPqcBinaryFormat(algorithm, metadata, encryptedData);

    // Serialize to bytes
    const serialized = pqcFormat.toBytes();
    console.log(`Serialized size: ${serialized.length} bytes`);
    console.log(`Total format size: ${pqcFormat.totalSize()} bytes`);

    // Deserialize from bytes
    const deserialized = WasmPqcBinaryFormat.fromBytes(serialized);
    console.log(`Deserialized algorithm: ${deserialized.algorithm.name}`);
    console.log(`Data length: ${deserialized.data.length} bytes`);

    // Validate
    deserialized.validate();
    console.log('✓ Validation passed');

    // Example 2: Format with KEM parameters
    console.log('\n2. Format with KEM Parameters');
    console.log('-'.repeat(50));

    // Create KEM parameters
    const kemParams = new WasmKemParameters(
        new Uint8Array(1568), // ML-KEM-1024 public key
        new Uint8Array(1568)  // Encapsulated key
    );

    // Create metadata with KEM
    const metadataWithKem = new WasmPqcMetadata(encParams);
    metadataWithKem.setKemParams(kemParams);

    // Create format
    const pqcWithKem = new WasmPqcBinaryFormat(algorithm, metadataWithKem, encryptedData);
    const serializedWithKem = pqcWithKem.toBytes();
    console.log(`Serialized size with KEM: ${serializedWithKem.length} bytes`);

    // Example 3: Format with flags
    console.log('\n3. Format with Feature Flags');
    console.log('-'.repeat(50));

    // Create flags
    const flags = new WasmFormatFlags()
        .withCompression()
        .withStreaming();

    console.log(`Compression enabled: ${flags.hasCompression()}`);
    console.log(`Streaming enabled: ${flags.hasStreaming()}`);
    console.log(`Additional auth: ${flags.hasAdditionalAuth()}`);

    // Create format with flags
    const pqcWithFlags = WasmPqcBinaryFormat.withFlags(
        algorithm,
        flags,
        metadata,
        encryptedData
    );
    console.log(`Format with flags size: ${pqcWithFlags.totalSize()} bytes`);

    // Example 4: Algorithm comparison
    console.log('\n4. Algorithm Comparison');
    console.log('-'.repeat(50));

    const algorithms = ['classical', 'hybrid', 'post-quantum', 'ml-kem-1024'];

    for (const algName of algorithms) {
        const alg = new WasmAlgorithm(algName);
        const fmt = new WasmPqcBinaryFormat(alg, metadata, encryptedData);
        const size = fmt.totalSize();
        console.log(`${alg.name.padEnd(20)} - ${size} bytes`);
    }

    // Example 5: Cross-platform interoperability
    console.log('\n5. Cross-Platform Interoperability');
    console.log('-'.repeat(50));
    console.log('This binary format can be:');
    console.log('  • Created in JavaScript and read in Python');
    console.log('  • Created in Rust and read in Go');
    console.log('  • Created in C++ and read in JavaScript');
    console.log('All using the same standardized binary format!');

    console.log('\n✓ All examples completed successfully!');
}

main().catch(err => {
    console.error('Error:', err);
    process.exit(1);
});
