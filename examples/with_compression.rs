//! Example showing compression metadata usage

use pqc_binary_format::{
    Algorithm, CompressionParameters, EncParameters, FormatFlags, PqcBinaryFormat, PqcMetadata,
};
use std::collections::HashMap;

fn main() {
    println!("=== PQC Binary Format - With Compression ===\n");

    // Create metadata with compression parameters
    let mut metadata = PqcMetadata {
        enc_params: EncParameters {
            iv: vec![1; 12],
            tag: vec![1; 16],
            params: HashMap::new(),
        },
        compression_params: Some(CompressionParameters {
            algorithm: "zstd".to_string(),
            level: 3,
            original_size: 1024,
            params: HashMap::new(),
        }),
        ..Default::default()
    };

    // Add custom parameters
    metadata.add_custom("app_version".to_string(), b"1.0.0".to_vec());
    metadata.add_custom("timestamp".to_string(), b"2026-01-09".to_vec());

    // Create format with compression flag enabled
    let flags = FormatFlags::new().with_compression();

    let format = PqcBinaryFormat::with_flags(
        Algorithm::PostQuantum,
        flags,
        metadata,
        vec![1, 2, 3, 4, 5],
    );

    println!("Created format with:");
    println!("  Algorithm: {}", format.algorithm().name());
    println!("  Compression: {}", format.flags().has_compression());
    println!("  Streaming: {}", format.flags().has_streaming());
    println!();

    if let Some(ref comp) = format.metadata().compression_params {
        println!("Compression details:");
        println!("  Algorithm: {}", comp.algorithm);
        println!("  Level: {}", comp.level);
        println!("  Original size: {} bytes", comp.original_size);
        println!();
    }

    // Serialize and deserialize
    let bytes = format.to_bytes().unwrap();
    let recovered = PqcBinaryFormat::from_bytes(&bytes).unwrap();

    // Verify custom parameters
    if let Some(version) = recovered.metadata().get_custom("app_version") {
        println!("Custom parameter 'app_version': {}", String::from_utf8_lossy(version));
    }

    println!("\n✓ Compression metadata preserved!");
}
