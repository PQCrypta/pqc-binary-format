//! Metadata structures for algorithm-specific parameters.

use crate::{error::CryptoError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Algorithm-specific metadata container
///
/// Contains all parameters needed to decrypt and verify the encrypted data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PqcMetadata {
    /// Key encapsulation parameters (optional)
    pub kem_params: Option<KemParameters>,
    /// Signature parameters (optional)
    pub sig_params: Option<SigParameters>,
    /// Encryption parameters (required)
    pub enc_params: EncParameters,
    /// Compression parameters (optional)
    pub compression_params: Option<CompressionParameters>,
    /// Additional custom parameters
    pub custom: HashMap<String, Vec<u8>>,
}

/// Key Encapsulation Mechanism (KEM) parameters
///
/// Used by algorithms that employ key encapsulation (ML-KEM, etc.)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KemParameters {
    /// Public key for KEM
    pub public_key: Vec<u8>,
    /// Encapsulated ciphertext
    pub ciphertext: Vec<u8>,
    /// Algorithm-specific parameters
    pub params: HashMap<String, Vec<u8>>,
}

/// Digital signature parameters
///
/// Used by algorithms that include signatures (ML-DSA, Ed25519, etc.)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SigParameters {
    /// Public key for signature verification
    pub public_key: Vec<u8>,
    /// Digital signature
    pub signature: Vec<u8>,
    /// Algorithm-specific parameters
    pub params: HashMap<String, Vec<u8>>,
}

/// Symmetric encryption parameters
///
/// Required for all algorithms as they all use symmetric encryption
/// for the actual data encryption.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EncParameters {
    /// Initialization vector / nonce
    pub iv: Vec<u8>,
    /// Authentication tag (for AEAD ciphers like AES-GCM)
    pub tag: Vec<u8>,
    /// Algorithm-specific parameters
    pub params: HashMap<String, Vec<u8>>,
}

/// Compression algorithm parameters
///
/// Indicates if compression was applied before encryption.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompressionParameters {
    /// Compression algorithm name (e.g., "zstd", "brotli", "lz4")
    pub algorithm: String,
    /// Compression level (0-9)
    pub level: u8,
    /// Original size before compression
    pub original_size: u64,
    /// Algorithm-specific parameters
    pub params: HashMap<String, Vec<u8>>,
}

impl PqcMetadata {
    /// Create new empty metadata with default encryption parameters
    ///
    /// # Example
    ///
    /// ```
    /// use pqc_binary_format::PqcMetadata;
    ///
    /// let mut metadata = PqcMetadata::new();
    /// metadata.enc_params.iv = vec![0; 12];
    /// metadata.enc_params.tag = vec![0; 16];
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            kem_params: None,
            sig_params: None,
            enc_params: EncParameters {
                iv: Vec::new(),
                tag: Vec::new(),
                params: HashMap::new(),
            },
            compression_params: None,
            custom: HashMap::new(),
        }
    }

    /// Validate metadata structure
    ///
    /// Ensures all required fields are present and valid.
    pub fn validate(&self) -> Result<()> {
        // Validate encryption parameters (required)
        if self.enc_params.iv.is_empty() {
            return Err(CryptoError::MetadataError(
                "Encryption IV cannot be empty".to_string(),
            ));
        }

        // Additional validation can be added here for specific parameters
        Ok(())
    }

    /// Add custom parameter
    ///
    /// # Example
    ///
    /// ```
    /// use pqc_binary_format::PqcMetadata;
    ///
    /// let mut metadata = PqcMetadata::new();
    /// metadata.add_custom("my_param".to_string(), vec![1, 2, 3]);
    /// ```
    pub fn add_custom(&mut self, key: String, value: Vec<u8>) {
        self.custom.insert(key, value);
    }

    /// Get custom parameter
    ///
    /// # Example
    ///
    /// ```
    /// use pqc_binary_format::PqcMetadata;
    ///
    /// let mut metadata = PqcMetadata::new();
    /// metadata.add_custom("my_param".to_string(), vec![1, 2, 3]);
    /// assert_eq!(metadata.get_custom("my_param"), Some(&[1, 2, 3][..]));
    /// ```
    #[must_use]
    pub fn get_custom(&self, key: &str) -> Option<&[u8]> {
        self.custom.get(key).map(Vec::as_slice)
    }
}

impl Default for PqcMetadata {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata_validation() {
        let mut metadata = PqcMetadata::new();

        // Should fail with empty IV
        assert!(metadata.validate().is_err());

        // Should succeed with valid IV
        metadata.enc_params.iv = vec![1; 12];
        assert!(metadata.validate().is_ok());
    }

    #[test]
    fn test_custom_parameters() {
        let mut metadata = PqcMetadata::new();
        metadata.add_custom("test".to_string(), vec![1, 2, 3]);

        assert_eq!(metadata.get_custom("test"), Some(&[1, 2, 3][..]));
        assert_eq!(metadata.get_custom("missing"), None);
    }
}
