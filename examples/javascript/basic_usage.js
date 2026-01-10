/**
 * Basic usage example for PQC Binary Format JavaScript/WASM bindings
 * Demonstrates encryption format creation, serialization, and deserialization
 *
 * Usage:
 *   First build the WASM bindings:
 *     cd ../../bindings/javascript && npm run build
 *   Then run this example:
 *     node basic_usage.js
 */

import init, {
    WasmAlgorithm,
    WasmEncParameters,
    WasmPqcMetadata,
    WasmPqcBinaryFormat,
    getVersion,
    getBinaryVersion
} from '../../pkg/pqc_binary_format.js';

async function main() {
    console.log('='.repeat(60));
    console.log('PQC Binary Format - JavaScript Basic Usage Example');
    console.log('='.repeat(60));
    console.log();

    // Initialize WASM
    console.log('Initializing WASM module...');
    await init();
    console.log(`  ✓ Library version: ${getVersion()}`);
    console.log(`  ✓ Binary format version: ${getBinaryVersion()}`);
    console.log();

    // Step 1: Create encryption parameters
    console.log('Step 1: Creating encryption parameters...');
    const iv = new Uint8Array([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
    const tag = new Uint8Array(16).fill(0);
    const encParams = new WasmEncParameters(iv, tag);
    console.log(`  ✓ IV: ${iv.length} bytes`);
    console.log(`  ✓ Tag: ${tag.length} bytes`);
    console.log();

    // Step 2: Create metadata container
    console.log('Step 2: Creating metadata container...');
    const metadata = new WasmPqcMetadata(encParams);
    console.log('  ✓ Metadata created with encryption parameters');
    console.log();

    // Step 3: Create algorithm
    console.log('Step 3: Selecting cryptographic algorithm...');
    const algorithm = new WasmAlgorithm('hybrid');
    console.log(`  ✓ Algorithm: ${algorithm.name}`);
    console.log(`  ✓ Algorithm ID: 0x${algorithm.id.toString(16).padStart(4, '0')}`);
    console.log();

    // Step 4: Create PQC Binary Format
    console.log('Step 4: Creating PQC Binary Format...');
    const encryptedData = new Uint8Array([1, 2, 3, 4, 5]);
    const pqcFormat = new WasmPqcBinaryFormat(algorithm, metadata, encryptedData);
    console.log(`  ✓ Format created with ${encryptedData.length} bytes of data`);
    console.log();

    // Step 5: Serialize to bytes
    console.log('Step 5: Serializing to binary format...');
    const serialized = pqcFormat.toBytes();
    console.log(`  ✓ Serialized size: ${serialized.length} bytes`);
    console.log(`  ✓ Total format size: ${pqcFormat.totalSize()} bytes`);
    console.log();

    // Step 6: Deserialize from bytes
    console.log('Step 6: Deserializing from binary format...');
    const deserialized = WasmPqcBinaryFormat.fromBytes(serialized);
    console.log(`  ✓ Deserialized algorithm: ${deserialized.algorithm.name}`);
    console.log(`  ✓ Data length: ${deserialized.data.length} bytes`);
    console.log();

    // Step 7: Validate integrity
    console.log('Step 7: Validating format integrity...');
    try {
        deserialized.validate();
        console.log('  ✓ Validation passed - checksum verified');
    } catch (e) {
        console.log(`  ✗ Validation failed: ${e.message}`);
        return 1;
    }
    console.log();

    // Step 8: Verify roundtrip
    console.log('Step 8: Verifying roundtrip integrity...');
    const originalData = pqcFormat.data;
    const recoveredData = deserialized.data;

    const arraysEqual = originalData.length === recoveredData.length &&
        originalData.every((val, idx) => val === recoveredData[idx]);

    if (arraysEqual) {
        console.log('  ✓ Roundtrip successful - data matches!');
    } else {
        console.log('  ✗ Roundtrip failed - data mismatch!');
        return 1;
    }
    console.log();

    console.log('='.repeat(60));
    console.log('✅ All steps completed successfully!');
    console.log('='.repeat(60));
    console.log();
    console.log('Summary:');
    console.log(`  • Algorithm: ${algorithm.name} (0x${algorithm.id.toString(16).padStart(4, '0')})`);
    console.log(`  • Data size: ${encryptedData.length} bytes`);
    console.log(`  • Serialized size: ${serialized.length} bytes`);
    console.log(`  • Overhead: ${serialized.length - encryptedData.length} bytes`);
    console.log();

    return 0;
}

main().then(code => process.exit(code)).catch(err => {
    console.error('Error:', err);
    process.exit(1);
});
