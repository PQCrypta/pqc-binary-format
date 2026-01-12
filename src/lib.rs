//! # PQC Binary Format v1.0
//!
//! A standardized binary format specification for post-quantum cryptography encrypted data interchange.
//!
//! ## Overview
//!
//! This crate provides a universal, self-describing binary format for encrypted data that works
//! across different post-quantum cryptographic algorithms. It solves the "Babel Tower problem"
//! where different PQC implementations cannot interoperate due to incompatible data formats.
//!
//! ## Features
//!
//! - **Algorithm-agnostic**: Works with 31+ cryptographic algorithms
//! - **Self-describing metadata**: Algorithm parameters, compression settings, and custom fields
//! - **Integrity verification**: SHA-256 checksum of entire structure
//! - **Feature flags**: Compression, streaming, authentication, experimental features
//! - **Extensible**: Custom parameters for algorithm-specific needs
//! - **Cross-platform**: Compatible across languages and platforms
//!
//! ## Binary Layout
//!
//! ```text
//! +-------------------+
//! | Magic (4 bytes)   | "PQC\x01"
//! +-------------------+
//! | Version (1 byte)  | 0x01
//! +-------------------+
//! | Algorithm (2 bytes)| Algorithm identifier
//! +-------------------+
//! | Flags (1 byte)    | Feature flags
//! +-------------------+
//! | Metadata Len (4)  | Length of metadata section
//! +-------------------+
//! | Data Len (8)      | Length of encrypted data
//! +-------------------+
//! | Metadata (var)    | Algorithm-specific metadata
//! +-------------------+
//! | Data (var)        | Encrypted payload
//! +-------------------+
//! | Checksum (32)     | SHA-256 of entire structure
//! +-------------------+
//! ```
//!
//! ## Quick Example
//!
//! ```rust
//! use pqc_binary_format::{PqcBinaryFormat, Algorithm, PqcMetadata, EncParameters};
//! use std::collections::HashMap;
//!
//! // Create metadata with encryption parameters
//! let metadata = PqcMetadata {
//!     enc_params: EncParameters {
//!         iv: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
//!         tag: vec![1; 16],
//!         params: HashMap::new(),
//!     },
//!     ..Default::default()
//! };
//!
//! // Create encrypted data container
//! let encrypted_data = vec![1, 2, 3, 4, 5];
//! let format = PqcBinaryFormat::new(Algorithm::Hybrid, metadata, encrypted_data);
//!
//! // Serialize to bytes
//! let bytes = format.to_bytes().unwrap();
//!
//! // Deserialize from bytes (includes checksum verification)
//! let deserialized = PqcBinaryFormat::from_bytes(&bytes).unwrap();
//! assert_eq!(format, deserialized);
//! ```
//!
//! ## Supported Algorithms
//!
//! - **Classical**: X25519 + Ed25519 + AES-256-GCM
//! - **Hybrid**: ML-KEM-1024 + X25519 + ML-DSA-87 + Ed25519
//! - **Post-Quantum**: ML-KEM-1024 + ML-DSA-87
//! - **ML-KEM-1024**: Pure ML-KEM with AES-256-GCM
//! - **Multi-KEM**: Multiple key encapsulation layers
//! - **Quad-Layer**: Four independent cryptographic layers
//! - And 22 more algorithm identifiers...
//!
//! ## Use Cases
//!
//! - **Cross-platform encryption**: Encrypt in Rust, decrypt in Python/JavaScript/Go
//! - **Algorithm migration**: Seamlessly switch between algorithms
//! - **Long-term archival**: Self-describing format ensures future compatibility
//! - **Compliance**: Embedded metadata for audit trails
//! - **Research**: Standardized format for benchmarking PQC algorithms

#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::uninlined_format_args)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::return_self_not_must_use)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::unused_self)]
#![allow(clippy::struct_field_names)]
#![allow(clippy::ptr_as_ptr)]
#![allow(clippy::manual_let_else)]
#![allow(clippy::new_without_default)]

pub mod algorithm;
pub mod error;
pub mod format;
pub mod metadata;

// Language bindings (conditional compilation)
#[cfg(feature = "python")]
pub mod python;

#[cfg(feature = "wasm")]
pub mod wasm;

// FFI bindings always available for C/C++/Go
pub mod ffi;

// Re-exports for convenience
pub use algorithm::Algorithm;
pub use error::{CryptoError, Result};
pub use format::{FormatFlags, PqcBinaryFormat};
pub use metadata::{
    CompressionParameters, EncParameters, KemParameters, PqcMetadata, SigParameters,
};

/// Current version of the PQC Binary Format specification
pub const PQC_BINARY_VERSION: u8 = 0x01;

/// Magic bytes identifying PQC Binary Format v1.0
pub const PQC_MAGIC: [u8; 4] = *b"PQC\x01";

/// Version string for this crate
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
