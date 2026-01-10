//! Core binary format implementation with serialization and validation.

use crate::{
    algorithm::Algorithm, error::CryptoError, metadata::PqcMetadata, Result, PQC_BINARY_VERSION,
    PQC_MAGIC,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// PQC Binary Format v1.0 specification
///
/// This structure represents encrypted data in a standardized, self-describing format
/// compatible across all post-quantum cryptographic algorithms.
///
/// # Example
///
/// ```
/// use pqc_binary_format::{PqcBinaryFormat, Algorithm, PqcMetadata, EncParameters};
/// use std::collections::HashMap;
///
/// let metadata = PqcMetadata {
///     enc_params: EncParameters {
///         iv: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
///         tag: vec![1; 16],
///         params: HashMap::new(),
///     },
///     ..Default::default()
/// };
///
/// let format = PqcBinaryFormat::new(
///     Algorithm::Hybrid,
///     metadata,
///     vec![1, 2, 3, 4, 5],
///);
///
/// // Serialize to bytes
/// let bytes = format.to_bytes().unwrap();
///
/// // Deserialize and verify
/// let recovered = PqcBinaryFormat::from_bytes(&bytes).unwrap();
/// assert_eq!(format, recovered);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PqcBinaryFormat {
    /// Magic bytes - always "PQC\x01"
    pub magic: [u8; 4],
    /// Format version - currently 0x01
    pub version: u8,
    /// Algorithm identifier
    pub algorithm: Algorithm,
    /// Feature flags
    pub flags: u8,
    /// Algorithm-specific metadata
    pub metadata: PqcMetadata,
    /// Encrypted data payload
    pub data: Vec<u8>,
    /// SHA-256 checksum of the entire structure (excluding this field)
    pub checksum: [u8; 32],
}

/// Feature flags for PQC Binary Format
///
/// Indicates optional features used in the encrypted data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FormatFlags(u8);

impl FormatFlags {
    /// Create new flags with all features disabled
    #[must_use]
    pub const fn new() -> Self {
        Self(0)
    }

    /// Enable compression flag (bit 0)
    #[must_use]
    pub const fn with_compression(mut self) -> Self {
        self.0 |= 0x01;
        self
    }

    /// Enable streaming flag (bit 1)
    #[must_use]
    pub const fn with_streaming(mut self) -> Self {
        self.0 |= 0x02;
        self
    }

    /// Enable additional authentication flag (bit 2)
    #[must_use]
    pub const fn with_additional_auth(mut self) -> Self {
        self.0 |= 0x04;
        self
    }

    /// Enable experimental features flag (bit 3)
    #[must_use]
    pub const fn with_experimental(mut self) -> Self {
        self.0 |= 0x08;
        self
    }

    /// Check if compression is enabled
    #[must_use]
    pub const fn has_compression(self) -> bool {
        (self.0 & 0x01) != 0
    }

    /// Check if streaming is enabled
    #[must_use]
    pub const fn has_streaming(self) -> bool {
        (self.0 & 0x02) != 0
    }

    /// Check if additional authentication is enabled
    #[must_use]
    pub const fn has_additional_auth(self) -> bool {
        (self.0 & 0x04) != 0
    }

    /// Check if experimental features are enabled
    #[must_use]
    pub const fn has_experimental(self) -> bool {
        (self.0 & 0x08) != 0
    }

    /// Get raw flags value
    #[must_use]
    pub const fn as_u8(self) -> u8 {
        self.0
    }

    /// Create flags from u8 value
    #[must_use]
    pub const fn from_u8(value: u8) -> Self {
        Self(value)
    }
}

impl Default for FormatFlags {
    fn default() -> Self {
        Self::new()
    }
}

impl PqcBinaryFormat {
    /// Create a new PQC binary format structure with default flags
    ///
    /// The checksum is automatically calculated and set.
    ///
    /// # Example
    ///
    /// ```
    /// use pqc_binary_format::{PqcBinaryFormat, Algorithm, PqcMetadata, EncParameters};
    /// use std::collections::HashMap;
    ///
    /// let metadata = PqcMetadata {
    ///     enc_params: EncParameters {
    ///         iv: vec![1; 12],
    ///         tag: vec![1; 16],
    ///         params: HashMap::new(),
    ///     },
    ///     ..Default::default()
    /// };
    ///
    /// let format = PqcBinaryFormat::new(
    ///     Algorithm::PostQuantum,
    ///     metadata,
    ///     vec![1, 2, 3],
    /// );
    /// ```
    #[must_use]
    pub fn new(algorithm: Algorithm, metadata: PqcMetadata, data: Vec<u8>) -> Self {
        let mut format = Self {
            magic: PQC_MAGIC,
            version: PQC_BINARY_VERSION,
            algorithm,
            flags: FormatFlags::new().as_u8(),
            metadata,
            data,
            checksum: [0u8; 32],
        };

        // Calculate and set checksum
        format.checksum = format.calculate_checksum();
        format
    }

    /// Create with specific flags
    ///
    /// # Example
    ///
    /// ```
    /// use pqc_binary_format::{PqcBinaryFormat, Algorithm, PqcMetadata, FormatFlags, EncParameters};
    /// use std::collections::HashMap;
    ///
    /// let metadata = PqcMetadata {
    ///     enc_params: EncParameters {
    ///         iv: vec![1; 12],
    ///         tag: vec![1; 16],
    ///         params: HashMap::new(),
    ///     },
    ///     ..Default::default()
    /// };
    ///
    /// let flags = FormatFlags::new().with_compression().with_streaming();
    ///
    /// let format = PqcBinaryFormat::with_flags(
    ///     Algorithm::Hybrid,
    ///     flags,
    ///     metadata,
    ///     vec![1, 2, 3],
    /// );
    ///
    /// assert!(format.flags().has_compression());
    /// assert!(format.flags().has_streaming());
    /// ```
    #[must_use]
    pub fn with_flags(
        algorithm: Algorithm,
        flags: FormatFlags,
        metadata: PqcMetadata,
        data: Vec<u8>,
    ) -> Self {
        let mut format = Self {
            magic: PQC_MAGIC,
            version: PQC_BINARY_VERSION,
            algorithm,
            flags: flags.as_u8(),
            metadata,
            data,
            checksum: [0u8; 32],
        };

        format.checksum = format.calculate_checksum();
        format
    }

    /// Serialize to binary format
    ///
    /// # Errors
    ///
    /// Returns [`CryptoError::BinaryFormatError`] if:
    /// - Format validation fails
    /// - Binary serialization fails
    ///
    /// # Example
    ///
    /// ```
    /// use pqc_binary_format::{PqcBinaryFormat, Algorithm, PqcMetadata, EncParameters};
    /// use std::collections::HashMap;
    ///
    /// # let metadata = PqcMetadata {
    /// #     enc_params: EncParameters {
    /// #         iv: vec![1; 12],
    /// #         tag: vec![1; 16],
    /// #         params: HashMap::new(),
    /// #     },
    /// #     ..Default::default()
    /// # };
    /// # let format = PqcBinaryFormat::new(Algorithm::Hybrid, metadata, vec![1, 2, 3]);
    /// let bytes = format.to_bytes().unwrap();
    /// // Send bytes over network or save to file
    /// ```
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        // Validate format before serialization
        self.validate()?;

        bincode::serialize(self)
            .map_err(|e| CryptoError::BinaryFormatError(format!("Serialization failed: {e}")))
    }

    /// Deserialize from binary format with checksum verification
    ///
    /// # Errors
    ///
    /// Returns [`CryptoError::BinaryFormatError`] if:
    /// - Binary deserialization fails
    /// - Format validation fails after deserialization
    /// - Checksum verification fails
    ///
    /// # Example
    ///
    /// ```
    /// use pqc_binary_format::PqcBinaryFormat;
    ///
    /// # use pqc_binary_format::{Algorithm, PqcMetadata, EncParameters};
    /// # use std::collections::HashMap;
    /// # let metadata = PqcMetadata {
    /// #     enc_params: EncParameters {
    /// #         iv: vec![1; 12],
    /// #         tag: vec![1; 16],
    /// #         params: HashMap::new(),
    /// #     },
    /// #     ..Default::default()
    /// # };
    /// # let format = PqcBinaryFormat::new(Algorithm::Hybrid, metadata, vec![1, 2, 3]);
    /// # let bytes = format.to_bytes().unwrap();
    /// let recovered = PqcBinaryFormat::from_bytes(&bytes).unwrap();
    /// ```
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        let format: Self = bincode::deserialize(data)
            .map_err(|e| CryptoError::BinaryFormatError(format!("Deserialization failed: {e}")))?;

        // Validate deserialized format
        format.validate()?;

        // Verify checksum
        let stored_checksum = format.checksum;
        let mut format_copy = format;
        format_copy.checksum = [0u8; 32]; // Zero out for recalculation
        let calculated_checksum = format_copy.calculate_checksum();

        // Restore checksum
        format_copy.checksum = stored_checksum;

        if stored_checksum != calculated_checksum {
            return Err(CryptoError::ChecksumMismatch);
        }

        Ok(format_copy)
    }

    /// Validate the binary format structure
    ///
    /// # Errors
    ///
    /// Returns [`CryptoError`] if:
    /// - Magic bytes are invalid
    /// - Version is unsupported
    /// - Algorithm ID is invalid
    /// - Metadata validation fails
    pub fn validate(&self) -> Result<()> {
        // Check magic bytes
        if self.magic != PQC_MAGIC {
            return Err(CryptoError::InvalidMagic);
        }

        // Check version
        if self.version != PQC_BINARY_VERSION {
            return Err(CryptoError::UnsupportedVersion(self.version));
        }

        // Validate algorithm
        if Algorithm::from_id(self.algorithm.as_id()).is_none() {
            return Err(CryptoError::UnknownAlgorithm(format!(
                "Invalid algorithm ID: {:#x}",
                self.algorithm.as_id()
            )));
        }

        // Validate metadata
        self.metadata.validate()?;

        Ok(())
    }

    /// Update the checksum field with the calculated checksum
    ///
    /// Call this after modifying any fields to maintain integrity.
    pub fn update_checksum(&mut self) {
        self.checksum = self.calculate_checksum();
    }

    /// Calculate SHA-256 checksum with deterministic field-by-field hashing
    ///
    /// Uses a deterministic approach to ensure consistent checksums across platforms.
    fn calculate_checksum(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();

        // Hash fixed fields with guaranteed deterministic ordering
        hasher.update(self.magic);
        hasher.update([self.version]);
        hasher.update(self.algorithm.as_id().to_le_bytes());
        hasher.update([self.flags]);

        // Hash metadata deterministically
        self.hash_metadata_deterministic(&mut hasher);

        // Hash data length and content
        hasher.update((self.data.len() as u64).to_le_bytes());
        hasher.update(&self.data);

        hasher.finalize().into()
    }

    /// Hash metadata in a deterministic way, field by field
    ///
    /// Ensures `HashMap` contents are sorted before hashing for consistency.
    #[allow(clippy::cast_possible_truncation)]
    fn hash_metadata_deterministic(&self, hasher: &mut Sha256) {
        // Hash KEM parameters if present
        if let Some(ref kem_params) = self.metadata.kem_params {
            hasher.update([1u8]); // Present flag
            hasher.update((kem_params.public_key.len() as u32).to_le_bytes());
            hasher.update(&kem_params.public_key);
            hasher.update((kem_params.ciphertext.len() as u32).to_le_bytes());
            hasher.update(&kem_params.ciphertext);
            // Hash params map deterministically by sorting keys
            let mut sorted_params: Vec<_> = kem_params.params.iter().collect();
            sorted_params.sort_by(|a, b| a.0.cmp(b.0));
            hasher.update((sorted_params.len() as u32).to_le_bytes());
            for (key, value) in sorted_params {
                hasher.update((key.len() as u32).to_le_bytes());
                hasher.update(key.as_bytes());
                hasher.update((value.len() as u32).to_le_bytes());
                hasher.update(value);
            }
        } else {
            hasher.update([0u8]); // Not present flag
        }

        // Hash signature parameters if present
        if let Some(ref sig_params) = self.metadata.sig_params {
            hasher.update([1u8]); // Present flag
            hasher.update((sig_params.public_key.len() as u32).to_le_bytes());
            hasher.update(&sig_params.public_key);
            hasher.update((sig_params.signature.len() as u32).to_le_bytes());
            hasher.update(&sig_params.signature);
            // Hash params map deterministically
            let mut sorted_params: Vec<_> = sig_params.params.iter().collect();
            sorted_params.sort_by(|a, b| a.0.cmp(b.0));
            hasher.update((sorted_params.len() as u32).to_le_bytes());
            for (key, value) in sorted_params {
                hasher.update((key.len() as u32).to_le_bytes());
                hasher.update(key.as_bytes());
                hasher.update((value.len() as u32).to_le_bytes());
                hasher.update(value);
            }
        } else {
            hasher.update([0u8]); // Not present flag
        }

        // Hash encryption parameters
        hasher.update((self.metadata.enc_params.iv.len() as u32).to_le_bytes());
        hasher.update(&self.metadata.enc_params.iv);
        hasher.update((self.metadata.enc_params.tag.len() as u32).to_le_bytes());
        hasher.update(&self.metadata.enc_params.tag);
        // Hash enc params map deterministically
        let mut sorted_params: Vec<_> = self.metadata.enc_params.params.iter().collect();
        sorted_params.sort_by(|a, b| a.0.cmp(b.0));
        hasher.update((sorted_params.len() as u32).to_le_bytes());
        for (key, value) in sorted_params {
            hasher.update((key.len() as u32).to_le_bytes());
            hasher.update(key.as_bytes());
            hasher.update((value.len() as u32).to_le_bytes());
            hasher.update(value);
        }

        // Hash compression params if present
        if let Some(ref comp_params) = self.metadata.compression_params {
            hasher.update([1u8]); // Present flag
            hasher.update((comp_params.algorithm.len() as u32).to_le_bytes());
            hasher.update(comp_params.algorithm.as_bytes());
            hasher.update(comp_params.level.to_le_bytes());
            hasher.update(comp_params.original_size.to_le_bytes());
        } else {
            hasher.update([0u8]); // Not present flag
        }

        // Hash custom fields deterministically by sorting keys
        let mut sorted_custom: Vec<_> = self.metadata.custom.iter().collect();
        sorted_custom.sort_by(|a, b| a.0.cmp(b.0));
        hasher.update((sorted_custom.len() as u32).to_le_bytes());
        for (key, value) in sorted_custom {
            hasher.update((key.len() as u32).to_le_bytes());
            hasher.update(key.as_bytes());
            hasher.update((value.len() as u32).to_le_bytes());
            hasher.update(value);
        }
    }

    /// Get format flags
    #[must_use]
    pub fn flags(&self) -> FormatFlags {
        FormatFlags(self.flags)
    }

    /// Get algorithm
    #[must_use]
    pub const fn algorithm(&self) -> Algorithm {
        self.algorithm
    }

    /// Get encrypted data
    #[must_use]
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// Get metadata
    #[must_use]
    pub const fn metadata(&self) -> &PqcMetadata {
        &self.metadata
    }

    /// Get total size of the binary format when serialized
    #[must_use]
    pub fn total_size(&self) -> usize {
        self.to_bytes().map_or(0, |bytes| bytes.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EncParameters;
    use std::collections::HashMap;

    #[test]
    fn test_format_flags() {
        let flags = FormatFlags::new().with_compression().with_streaming();

        assert!(flags.has_compression());
        assert!(flags.has_streaming());
        assert!(!flags.has_additional_auth());
        assert!(!flags.has_experimental());
    }

    #[test]
    fn test_binary_format_roundtrip() {
        let metadata = PqcMetadata {
            enc_params: EncParameters {
                iv: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
                tag: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
                params: HashMap::new(),
            },
            ..Default::default()
        };

        let original = PqcBinaryFormat::new(Algorithm::Hybrid, metadata, vec![1, 2, 3, 4, 5]);

        let bytes = original.to_bytes().unwrap();
        let deserialized = PqcBinaryFormat::from_bytes(&bytes).unwrap();

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_checksum_validation() {
        let metadata = PqcMetadata {
            enc_params: EncParameters {
                iv: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
                tag: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
                params: HashMap::new(),
            },
            ..Default::default()
        };

        let format = PqcBinaryFormat::new(Algorithm::PostQuantum, metadata, vec![1, 2, 3, 4, 5]);

        let mut bytes = format.to_bytes().unwrap();

        // Corrupt the data
        if let Some(byte) = bytes.last_mut() {
            *byte = byte.wrapping_add(1);
        }

        // Should fail checksum validation
        assert!(PqcBinaryFormat::from_bytes(&bytes).is_err());
    }

    #[test]
    fn test_flags_roundtrip() {
        let metadata = PqcMetadata {
            enc_params: EncParameters {
                iv: vec![1; 12],
                tag: vec![1; 16],
                params: HashMap::new(),
            },
            ..Default::default()
        };

        let flags = FormatFlags::new()
            .with_compression()
            .with_streaming()
            .with_additional_auth();

        let format =
            PqcBinaryFormat::with_flags(Algorithm::QuadLayer, flags, metadata, vec![1, 2, 3]);

        let bytes = format.to_bytes().unwrap();
        let recovered = PqcBinaryFormat::from_bytes(&bytes).unwrap();

        assert!(recovered.flags().has_compression());
        assert!(recovered.flags().has_streaming());
        assert!(recovered.flags().has_additional_auth());
        assert!(!recovered.flags().has_experimental());
    }
}
