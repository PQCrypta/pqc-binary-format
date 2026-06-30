//! Core binary format implementation with serialization and validation.

use crate::{
    algorithm::Algorithm, error::CryptoError, metadata::PqcMetadata, Result, PQC_BINARY_VERSION,
    PQC_MAGIC,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// Fixed header size in bytes: magic(4) + version(1) + algorithm_id(2) +
/// flags(1) + metadata_len(4) + data_len(8) is split across the metadata
/// section, so the bytes preceding the metadata section total 12.
const HEADER_SIZE: usize = 12;

/// SHA-256 checksum size in bytes.
const CHECKSUM_SIZE: usize = 32;

/// Branch-free, fixed-length equality for two 32-byte checksums.
///
/// Avoids leaking checksum-correctness information through timing.
fn constant_time_eq(a: &[u8; 32], b: &[u8; 32]) -> bool {
    let mut diff = 0u8;
    for i in 0..32 {
        diff |= a[i] ^ b[i];
    }
    diff == 0
}

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

        // Build the packed little-endian layout defined by the PQC Binary
        // Format specification (draft-riddel-pqc-binary-format-00, Section 3):
        //   magic[4] | version[1] | algorithm_id[2 LE] | flags[1]
        //   | metadata_len[4 LE] | metadata(JSON) | data_len[8 LE] | data
        //   | checksum[32]
        let mut buf = self.serialize_prefix();
        let checksum: [u8; 32] = Sha256::digest(&buf).into();
        buf.extend_from_slice(&checksum);
        Ok(buf)
    }

    /// Serialize every field preceding the checksum, in spec order. The SHA-256
    /// of this byte sequence is the integrity checksum.
    ///
    /// Lengths are written as the spec's fixed-width little-endian integers. A
    /// metadata section larger than `u32::MAX` (4 GB) is saturated to `u32::MAX`;
    /// such inputs are rejected by the practical size limits in any caller.
    fn serialize_prefix(&self) -> Vec<u8> {
        let metadata = self.metadata.to_json_bytes();
        let mut buf = Vec::with_capacity(HEADER_SIZE + metadata.len() + self.data.len());
        buf.extend_from_slice(&self.magic);
        buf.push(self.version);
        buf.extend_from_slice(&self.algorithm.as_id().to_le_bytes());
        buf.push(self.flags);
        let metadata_len = u32::try_from(metadata.len()).unwrap_or(u32::MAX);
        buf.extend_from_slice(&metadata_len.to_le_bytes());
        buf.extend_from_slice(&metadata);
        buf.extend_from_slice(&(self.data.len() as u64).to_le_bytes());
        buf.extend_from_slice(&self.data);
        buf
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
        // Smallest valid envelope: 12-byte pre-metadata header + 0-byte metadata
        // + 8-byte data length + 0-byte data + 32-byte checksum = 52 bytes.
        const MIN_SIZE: usize = HEADER_SIZE + 8 + CHECKSUM_SIZE;
        if data.len() < MIN_SIZE {
            return Err(CryptoError::BinaryFormatError(format!(
                "Buffer too small: {} bytes, minimum {MIN_SIZE}",
                data.len(),
            )));
        }

        // Magic bytes
        let magic: [u8; 4] = [data[0], data[1], data[2], data[3]];
        if magic != PQC_MAGIC {
            return Err(CryptoError::InvalidMagic);
        }

        // Version
        let version = data[4];
        if version != PQC_BINARY_VERSION {
            return Err(CryptoError::UnsupportedVersion(version));
        }

        // Algorithm ID (little-endian) + flags
        let algorithm_id = u16::from_le_bytes([data[5], data[6]]);
        let algorithm = Algorithm::from_id(algorithm_id).ok_or_else(|| {
            CryptoError::UnknownAlgorithm(format!("Invalid algorithm ID: {algorithm_id:#x}"))
        })?;
        let flags = data[7];

        // Metadata length (little-endian) + metadata section
        let metadata_len = u32::from_le_bytes([data[8], data[9], data[10], data[11]]) as usize;
        let meta_start = HEADER_SIZE;
        let meta_end = meta_start
            .checked_add(metadata_len)
            .ok_or_else(|| CryptoError::BinaryFormatError("Metadata length overflow".into()))?;
        if meta_end + CHECKSUM_SIZE > data.len() {
            return Err(CryptoError::BinaryFormatError(
                "Metadata length exceeds buffer".into(),
            ));
        }
        let metadata = PqcMetadata::from_json_bytes(&data[meta_start..meta_end])?;

        // Data length (little-endian) + data section
        let len_end = meta_end + 8;
        if len_end + CHECKSUM_SIZE > data.len() {
            return Err(CryptoError::BinaryFormatError(
                "Truncated before data length".into(),
            ));
        }
        let mut data_len_bytes = [0u8; 8];
        data_len_bytes.copy_from_slice(&data[meta_end..len_end]);
        let data_len = usize::try_from(u64::from_le_bytes(data_len_bytes))
            .map_err(|_| CryptoError::BinaryFormatError("Data length exceeds usize".into()))?;
        let data_end = len_end
            .checked_add(data_len)
            .ok_or_else(|| CryptoError::BinaryFormatError("Data length overflow".into()))?;
        if data_end + CHECKSUM_SIZE != data.len() {
            return Err(CryptoError::BinaryFormatError(format!(
                "Length mismatch: expected {} bytes, got {}",
                data_end + CHECKSUM_SIZE,
                data.len()
            )));
        }
        let payload = data[len_end..data_end].to_vec();

        // Verify checksum over everything preceding it (constant-time compare).
        // The exact-length check above guarantees 32 trailing bytes.
        let mut stored = [0u8; 32];
        stored.copy_from_slice(&data[data_end..]);
        let calculated: [u8; 32] = Sha256::digest(&data[..data_end]).into();
        if !constant_time_eq(&stored, &calculated) {
            return Err(CryptoError::ChecksumMismatch);
        }

        let format = Self {
            magic,
            version,
            algorithm,
            flags,
            metadata,
            data: payload,
            checksum: stored,
        };
        format.validate()?;
        Ok(format)
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

    /// Calculate the SHA-256 integrity checksum over all fields preceding it.
    ///
    /// Equivalent to `SHA-256(serialize_prefix())`, where the metadata is the
    /// canonical (sorted-key) JSON section — matching the deterministic checksum
    /// definition in Section 3.3 of the specification.
    fn calculate_checksum(&self) -> [u8; 32] {
        Sha256::digest(self.serialize_prefix()).into()
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
