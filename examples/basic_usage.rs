//! Basic usage example of PQC Binary Format

use pqc_binary_format::{Algorithm, EncParameters, PqcBinaryFormat, PqcMetadata};
use std::collections::HashMap;

fn main() {
    println!("=== PQC Binary Format v1.0 - Basic Usage ===\n");

    // Step 1: Create metadata with encryption parameters
    let metadata = PqcMetadata {
        enc_params: EncParameters {
            iv: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12], // 12-byte nonce
            tag: vec![0; 16],                                // 16-byte authentication tag
            params: HashMap::new(),
        },
        ..Default::default()
    };

    // Step 2: Create encrypted data (in real usage, this would be actual encrypted bytes)
    let encrypted_data = b"This is encrypted data".to_vec();

    // Step 3: Create format structure
    let format = PqcBinaryFormat::new(Algorithm::Hybrid, metadata, encrypted_data);

    println!("Created format:");
    println!("  Algorithm: {}", format.algorithm().name());
    println!("  Algorithm ID: {:#06x}", format.algorithm().as_id());
    println!("  Data length: {} bytes", format.data().len());
    println!();

    // Step 4: Serialize to bytes
    let bytes = format.to_bytes().unwrap();
    println!("Serialized to {} bytes", bytes.len());
    println!();

    // Step 5: Deserialize from bytes (includes automatic checksum verification)
    let recovered = PqcBinaryFormat::from_bytes(&bytes).unwrap();
    println!("Deserialized successfully!");
    println!("  Checksum: ✓ Valid");
    println!("  Algorithm: {}", recovered.algorithm().name());
    println!("  Data matches: {}", recovered.data() == format.data());
    println!();

    // Step 6: Demonstrate validation
    assert_eq!(format, recovered);
    println!("✓ Full roundtrip successful!");
}
