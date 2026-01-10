//! Example comparing different algorithms

use pqc_binary_format::{Algorithm, EncParameters, PqcBinaryFormat, PqcMetadata};
use std::collections::HashMap;

fn main() {
    println!("=== PQC Binary Format - Algorithm Comparison ===\n");

    let algorithms = vec![
        Algorithm::Classical,
        Algorithm::Hybrid,
        Algorithm::PostQuantum,
        Algorithm::MlKem1024,
        Algorithm::QuadLayer,
    ];

    let test_data = b"Hello, Post-Quantum World!".to_vec();

    println!(
        "{:<30} {:<10} {:<15} Size (bytes)",
        "Algorithm", "ID", "Experimental"
    );
    println!("{}", "-".repeat(70));

    for algo in algorithms {
        let metadata = PqcMetadata {
            enc_params: EncParameters {
                iv: vec![1; 12],
                tag: vec![1; 16],
                params: HashMap::new(),
            },
            ..Default::default()
        };

        let format = PqcBinaryFormat::new(algo, metadata, test_data.clone());
        let bytes = format.to_bytes().unwrap();

        println!(
            "{:<30} {:#06x}    {:<15} {}",
            algo.name(),
            algo.as_id(),
            if algo.is_experimental() { "Yes" } else { "No" },
            bytes.len()
        );
    }

    println!("\n✓ All algorithms use the same binary format!");
}
