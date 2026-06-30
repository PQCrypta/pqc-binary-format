//! Metadata structures for algorithm-specific parameters.

use crate::{error::CryptoError, Result};
use base64::{engine::general_purpose::STANDARD, Engine};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
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

    /// Serialize the metadata to the canonical UTF-8 JSON object used in the
    /// metadata section of the binary format (Section 5 of the specification).
    ///
    /// Byte-valued fields (IV, tag, public keys, ciphertext, signature, custom
    /// values) are base64-encoded so the section stays valid UTF-8. Object keys
    /// serialize in sorted order (serde_json's map is a `BTreeMap`), giving a
    /// deterministic encoding suitable for the integrity checksum.
    #[must_use]
    pub fn to_json_bytes(&self) -> Vec<u8> {
        let mut root = Map::new();

        let mut enc = Map::new();
        enc.insert(
            "iv".into(),
            Value::String(STANDARD.encode(&self.enc_params.iv)),
        );
        enc.insert(
            "tag".into(),
            Value::String(STANDARD.encode(&self.enc_params.tag)),
        );
        enc.insert("params".into(), bytes_map_to_json(&self.enc_params.params));
        root.insert("encryption_params".into(), Value::Object(enc));

        if let Some(ref kem) = self.kem_params {
            let mut m = Map::new();
            m.insert(
                "public_key".into(),
                Value::String(STANDARD.encode(&kem.public_key)),
            );
            m.insert(
                "ciphertext".into(),
                Value::String(STANDARD.encode(&kem.ciphertext)),
            );
            m.insert("params".into(), bytes_map_to_json(&kem.params));
            root.insert("kem_params".into(), Value::Object(m));
        }

        if let Some(ref sig) = self.sig_params {
            let mut m = Map::new();
            m.insert(
                "public_key".into(),
                Value::String(STANDARD.encode(&sig.public_key)),
            );
            m.insert(
                "signature".into(),
                Value::String(STANDARD.encode(&sig.signature)),
            );
            m.insert("params".into(), bytes_map_to_json(&sig.params));
            root.insert("signature_params".into(), Value::Object(m));
        }

        if let Some(ref comp) = self.compression_params {
            let mut m = Map::new();
            m.insert("algorithm".into(), Value::String(comp.algorithm.clone()));
            m.insert("level".into(), Value::Number(comp.level.into()));
            m.insert(
                "original_size".into(),
                Value::Number(comp.original_size.into()),
            );
            m.insert("params".into(), bytes_map_to_json(&comp.params));
            root.insert("compression_params".into(), Value::Object(m));
        }

        if !self.custom.is_empty() {
            root.insert("custom".into(), bytes_map_to_json(&self.custom));
        }

        // serde_json never fails serializing an in-memory Value built from
        // owned data, so the conversion is infallible here.
        serde_json::to_vec(&Value::Object(root)).unwrap_or_default()
    }

    /// Reconstruct metadata from the canonical JSON object produced by
    /// [`PqcMetadata::to_json_bytes`].
    ///
    /// # Errors
    ///
    /// Returns [`CryptoError::MetadataError`] if the bytes are not valid UTF-8
    /// JSON, the required `encryption_params` object is missing, or any
    /// base64-encoded field cannot be decoded.
    pub fn from_json_bytes(bytes: &[u8]) -> Result<Self> {
        let root: Value = serde_json::from_slice(bytes)
            .map_err(|e| CryptoError::MetadataError(format!("Invalid metadata JSON: {e}")))?;
        let obj = root
            .as_object()
            .ok_or_else(|| CryptoError::MetadataError("Metadata must be a JSON object".into()))?;

        let enc_obj = obj
            .get("encryption_params")
            .and_then(Value::as_object)
            .ok_or_else(|| CryptoError::MetadataError("Missing encryption_params".into()))?;
        let enc_params = EncParameters {
            iv: decode_field(enc_obj, "iv")?,
            tag: decode_field(enc_obj, "tag")?,
            params: json_to_bytes_map(enc_obj.get("params"))?,
        };

        let kem_params = match obj.get("kem_params").and_then(Value::as_object) {
            Some(m) => Some(KemParameters {
                public_key: decode_field(m, "public_key")?,
                ciphertext: decode_field(m, "ciphertext")?,
                params: json_to_bytes_map(m.get("params"))?,
            }),
            None => None,
        };

        let sig_params = match obj.get("signature_params").and_then(Value::as_object) {
            Some(m) => Some(SigParameters {
                public_key: decode_field(m, "public_key")?,
                signature: decode_field(m, "signature")?,
                params: json_to_bytes_map(m.get("params"))?,
            }),
            None => None,
        };

        let compression_params = match obj.get("compression_params").and_then(Value::as_object) {
            Some(m) => Some(CompressionParameters {
                algorithm: m
                    .get("algorithm")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_string(),
                level: m
                    .get("level")
                    .and_then(Value::as_u64)
                    .and_then(|n| u8::try_from(n).ok())
                    .unwrap_or(0),
                original_size: m.get("original_size").and_then(Value::as_u64).unwrap_or(0),
                params: json_to_bytes_map(m.get("params"))?,
            }),
            None => None,
        };

        Ok(Self {
            kem_params,
            sig_params,
            enc_params,
            compression_params,
            custom: json_to_bytes_map(obj.get("custom"))?,
        })
    }
}

/// Encode a `HashMap<String, Vec<u8>>` as a JSON object of base64 strings.
fn bytes_map_to_json(map: &HashMap<String, Vec<u8>>) -> Value {
    let mut out = Map::new();
    for (k, v) in map {
        out.insert(k.clone(), Value::String(STANDARD.encode(v)));
    }
    Value::Object(out)
}

/// Decode a JSON object of base64 strings back into a `HashMap<String, Vec<u8>>`.
fn json_to_bytes_map(value: Option<&Value>) -> Result<HashMap<String, Vec<u8>>> {
    let mut out = HashMap::new();
    if let Some(Value::Object(m)) = value {
        for (k, v) in m {
            let s = v.as_str().ok_or_else(|| {
                CryptoError::MetadataError(format!("Param '{k}' is not a string"))
            })?;
            let decoded = STANDARD
                .decode(s)
                .map_err(|e| CryptoError::MetadataError(format!("Param '{k}' base64: {e}")))?;
            out.insert(k.clone(), decoded);
        }
    }
    Ok(out)
}

/// Decode a required base64 string field from a JSON object.
fn decode_field(obj: &Map<String, Value>, key: &str) -> Result<Vec<u8>> {
    let s = obj
        .get(key)
        .and_then(Value::as_str)
        .ok_or_else(|| CryptoError::MetadataError(format!("Missing metadata field '{key}'")))?;
    STANDARD
        .decode(s)
        .map_err(|e| CryptoError::MetadataError(format!("Field '{key}' base64: {e}")))
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
