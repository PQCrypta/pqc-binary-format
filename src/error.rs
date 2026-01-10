//! Error types for PQC Binary Format operations.

use thiserror::Error;

/// Result type alias for PQC Binary Format operations
pub type Result<T> = std::result::Result<T, CryptoError>;

/// Comprehensive error type for binary format operations
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum CryptoError {
    /// Binary format parsing or validation error
    #[error("Binary format error: {0}")]
    BinaryFormatError(String),

    /// Invalid input parameters
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// Serialization failed
    #[error("Serialization failed: {0}")]
    SerializationFailed(String),

    /// Deserialization failed
    #[error("Deserialization failed: {0}")]
    DeserializationFailed(String),

    /// Invalid algorithm identifier
    #[error("Unknown algorithm: {0}")]
    UnknownAlgorithm(String),

    /// Checksum verification failed
    #[error("Checksum validation failed")]
    ChecksumMismatch,

    /// Invalid magic bytes
    #[error("Invalid magic bytes")]
    InvalidMagic,

    /// Unsupported format version
    #[error("Unsupported version: {0:#x}")]
    UnsupportedVersion(u8),

    /// Metadata validation error
    #[error("Metadata validation error: {0}")]
    MetadataError(String),
}
