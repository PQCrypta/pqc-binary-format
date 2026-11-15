// The WebAssembly binding in Node, as CI's `bindings` job builds it
// (`wasm-pack build --target nodejs --out-dir pkg-node -- --features wasm`):
// a format written with one of the newest identifiers reads back the same,
// and supportedAlgorithms() is the whole registry.
'use strict';
const assert = require('node:assert/strict');
const path = require('node:path');
const pqc = require(path.join(__dirname, '../../pkg-node/pqc_binary_format.js'));

const names = pqc.WasmAlgorithm.supportedAlgorithms();
for (const name of ['classical', 'ml-kem-1024', 'x25519-mlkem768', 'frodokem-976', 'slh-dsa-sha2-256f']) {
    assert.ok(names.includes(name), `supportedAlgorithms() lacks ${name}`);
}

const algorithm = new pqc.WasmAlgorithm('x25519-mlkem768');
assert.equal(algorithm.id, 0x0208);
const metadata = new pqc.WasmPqcMetadata(new pqc.WasmEncParameters(new Uint8Array(12).fill(1), new Uint8Array(16).fill(2)));
const data = new Uint8Array([1, 2, 3, 4, 5]);
const bytes = new pqc.WasmPqcBinaryFormat(algorithm, metadata, data).toBytes();
const back = pqc.WasmPqcBinaryFormat.fromBytes(bytes);
assert.equal(back.algorithm.id, 0x0208);
assert.deepEqual(Array.from(back.data), Array.from(data));
console.log(`JavaScript binding ${pqc.getVersion()}: ${names.length} algorithms, round trip of ${bytes.length} bytes`);
