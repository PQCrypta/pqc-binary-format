//! WebAssembly bindings for PQC Binary Format
//!
//! This module provides WASM bindings for use in JavaScript/TypeScript environments.

use wasm_bindgen::prelude::*;
use std::collections::HashMap;

use crate::{
    Algorithm, CompressionParameters, EncParameters, FormatFlags, KemParameters, PqcBinaryFormat,
    PqcMetadata, SigParameters,
};

/// WebAssembly wrapper for Algorithm
#[wasm_bindgen]
pub struct WasmAlgorithm {
    inner: Algorithm,
}

#[wasm_bindgen]
impl WasmAlgorithm {
    /// Create a new algorithm from name
    #[wasm_bindgen(constructor)]
    pub fn new(name: &str) -> Result<WasmAlgorithm, JsValue> {
        let inner = match name.to_lowercase().as_str() {
            "classical" => Algorithm::Classical,
            "hybrid" => Algorithm::Hybrid,
            "post-quantum" | "postquantum" => Algorithm::PostQuantum,
            "ml-kem-1024" | "mlkem1024" => Algorithm::MlKem1024,
            "multi-algorithm" | "multialgorithm" => Algorithm::MultiAlgorithm,
            "multi-kem" | "multikem" => Algorithm::MultiKem,
            "multi-kem-triple" | "multikemtriple" => Algorithm::MultiKemTriple,
            "quad-layer" | "quadlayer" => Algorithm::QuadLayer,
            "pq3-stack" | "pq3stack" => Algorithm::Pq3Stack,
            "lattice-code-hybrid" | "latticecodehybrid" => Algorithm::LatticeCodeHybrid,
            _ => {
                return Err(JsValue::from_str(&format!("Unknown algorithm: {}", name)));
            }
        };
        Ok(Self { inner })
    }

    /// Get algorithm name
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.inner.name().to_string()
    }

    /// Get algorithm ID
    #[wasm_bindgen(getter)]
    pub fn id(&self) -> u16 {
        self.inner.as_id()
    }

    /// Get all supported algorithm names
    #[wasm_bindgen(js_name = supportedAlgorithms)]
    pub fn supported_algorithms() -> Vec<JsValue> {
        vec![
            "classical",
            "hybrid",
            "post-quantum",
            "ml-kem-1024",
            "multi-algorithm",
            "multi-kem",
            "multi-kem-triple",
            "quad-layer",
            "pq3-stack",
            "lattice-code-hybrid",
        ]
        .into_iter()
        .map(JsValue::from_str)
        .collect()
    }
}

/// WebAssembly wrapper for EncParameters
#[wasm_bindgen]
pub struct WasmEncParameters {
    inner: EncParameters,
}

#[wasm_bindgen]
impl WasmEncParameters {
    /// Create new encryption parameters
    #[wasm_bindgen(constructor)]
    pub fn new(iv: Vec<u8>, tag: Vec<u8>) -> Self {
        Self {
            inner: EncParameters {
                iv,
                tag,
                params: HashMap::new(),
            },
        }
    }

    /// Get IV/nonce
    #[wasm_bindgen(getter)]
    pub fn iv(&self) -> Vec<u8> {
        self.inner.iv.clone()
    }

    /// Set IV/nonce
    #[wasm_bindgen(setter)]
    pub fn set_iv(&mut self, iv: Vec<u8>) {
        self.inner.iv = iv;
    }

    /// Get authentication tag
    #[wasm_bindgen(getter)]
    pub fn tag(&self) -> Vec<u8> {
        self.inner.tag.clone()
    }

    /// Set authentication tag
    #[wasm_bindgen(setter)]
    pub fn set_tag(&mut self, tag: Vec<u8>) {
        self.inner.tag = tag;
    }
}

/// WebAssembly wrapper for KemParameters
#[wasm_bindgen]
pub struct WasmKemParameters {
    inner: KemParameters,
}

#[wasm_bindgen]
impl WasmKemParameters {
    /// Create new KEM parameters
    #[wasm_bindgen(constructor)]
    pub fn new(public_key: Vec<u8>, ciphertext: Vec<u8>) -> Self {
        Self {
            inner: KemParameters {
                public_key,
                ciphertext,
                params: HashMap::new(),
            },
        }
    }

    /// Get public key
    #[wasm_bindgen(getter)]
    pub fn public_key(&self) -> Vec<u8> {
        self.inner.public_key.clone()
    }

    /// Get ciphertext
    #[wasm_bindgen(getter)]
    pub fn ciphertext(&self) -> Vec<u8> {
        self.inner.ciphertext.clone()
    }
}

/// WebAssembly wrapper for SigParameters
#[wasm_bindgen]
pub struct WasmSigParameters {
    inner: SigParameters,
}

#[wasm_bindgen]
impl WasmSigParameters {
    /// Create new signature parameters
    #[wasm_bindgen(constructor)]
    pub fn new(public_key: Vec<u8>, signature: Vec<u8>) -> Self {
        Self {
            inner: SigParameters {
                public_key,
                signature,
                params: HashMap::new(),
            },
        }
    }

    /// Get public key
    #[wasm_bindgen(getter)]
    pub fn public_key(&self) -> Vec<u8> {
        self.inner.public_key.clone()
    }

    /// Get signature
    #[wasm_bindgen(getter)]
    pub fn signature(&self) -> Vec<u8> {
        self.inner.signature.clone()
    }
}

/// WebAssembly wrapper for CompressionParameters
#[wasm_bindgen]
pub struct WasmCompressionParameters {
    inner: CompressionParameters,
}

#[wasm_bindgen]
impl WasmCompressionParameters {
    /// Create new compression parameters
    #[wasm_bindgen(constructor)]
    pub fn new(algorithm: String, level: u8, original_size: u64) -> Self {
        Self {
            inner: CompressionParameters {
                algorithm,
                level,
                original_size,
                params: HashMap::new(),
            },
        }
    }

    /// Get compression algorithm
    #[wasm_bindgen(getter)]
    pub fn algorithm(&self) -> String {
        self.inner.algorithm.clone()
    }

    /// Get compression level
    #[wasm_bindgen(getter)]
    pub fn level(&self) -> u8 {
        self.inner.level
    }

    /// Get original size
    #[wasm_bindgen(getter)]
    pub fn original_size(&self) -> u64 {
        self.inner.original_size
    }
}

/// WebAssembly wrapper for PqcMetadata
#[wasm_bindgen]
pub struct WasmPqcMetadata {
    enc_params: WasmEncParameters,
    kem_params: Option<WasmKemParameters>,
    sig_params: Option<WasmSigParameters>,
    compression_params: Option<WasmCompressionParameters>,
}

#[wasm_bindgen]
impl WasmPqcMetadata {
    /// Create new metadata
    #[wasm_bindgen(constructor)]
    pub fn new(enc_params: WasmEncParameters) -> Self {
        Self {
            enc_params,
            kem_params: None,
            sig_params: None,
            compression_params: None,
        }
    }

    /// Set KEM parameters
    #[wasm_bindgen(js_name = setKemParams)]
    pub fn set_kem_params(&mut self, kem_params: WasmKemParameters) {
        self.kem_params = Some(kem_params);
    }

    /// Set signature parameters
    #[wasm_bindgen(js_name = setSigParams)]
    pub fn set_sig_params(&mut self, sig_params: WasmSigParameters) {
        self.sig_params = Some(sig_params);
    }

    /// Set compression parameters
    #[wasm_bindgen(js_name = setCompressionParams)]
    pub fn set_compression_params(&mut self, compression_params: WasmCompressionParameters) {
        self.compression_params = Some(compression_params);
    }

    fn to_rust(&self) -> PqcMetadata {
        PqcMetadata {
            kem_params: self.kem_params.as_ref().map(|k| k.inner.clone()),
            sig_params: self.sig_params.as_ref().map(|s| s.inner.clone()),
            enc_params: self.enc_params.inner.clone(),
            compression_params: self
                .compression_params
                .as_ref()
                .map(|c| c.inner.clone()),
            custom: HashMap::new(),
        }
    }
}

/// WebAssembly wrapper for FormatFlags
#[wasm_bindgen]
pub struct WasmFormatFlags {
    inner: FormatFlags,
}

#[wasm_bindgen]
impl WasmFormatFlags {
    /// Create new empty flags
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: FormatFlags::new(),
        }
    }

    /// Enable compression flag
    #[wasm_bindgen(js_name = withCompression)]
    pub fn with_compression(&self) -> WasmFormatFlags {
        Self {
            inner: self.inner.with_compression(),
        }
    }

    /// Enable streaming flag
    #[wasm_bindgen(js_name = withStreaming)]
    pub fn with_streaming(&self) -> WasmFormatFlags {
        Self {
            inner: self.inner.with_streaming(),
        }
    }

    /// Enable additional auth flag
    #[wasm_bindgen(js_name = withAdditionalAuth)]
    pub fn with_additional_auth(&self) -> WasmFormatFlags {
        Self {
            inner: self.inner.with_additional_auth(),
        }
    }

    /// Enable experimental features flag
    #[wasm_bindgen(js_name = withExperimental)]
    pub fn with_experimental(&self) -> WasmFormatFlags {
        Self {
            inner: self.inner.with_experimental(),
        }
    }

    /// Check if compression is enabled
    #[wasm_bindgen(js_name = hasCompression)]
    pub fn has_compression(&self) -> bool {
        self.inner.has_compression()
    }

    /// Check if streaming is enabled
    #[wasm_bindgen(js_name = hasStreaming)]
    pub fn has_streaming(&self) -> bool {
        self.inner.has_streaming()
    }

    /// Check if additional auth is enabled
    #[wasm_bindgen(js_name = hasAdditionalAuth)]
    pub fn has_additional_auth(&self) -> bool {
        self.inner.has_additional_auth()
    }

    /// Check if experimental features are enabled
    #[wasm_bindgen(js_name = hasExperimental)]
    pub fn has_experimental(&self) -> bool {
        self.inner.has_experimental()
    }
}

/// WebAssembly wrapper for PqcBinaryFormat
#[wasm_bindgen]
pub struct WasmPqcBinaryFormat {
    inner: PqcBinaryFormat,
}

#[wasm_bindgen]
impl WasmPqcBinaryFormat {
    /// Create a new PQC Binary Format structure
    ///
    /// @param {WasmAlgorithm} algorithm - Algorithm to use
    /// @param {WasmPqcMetadata} metadata - Metadata container
    /// @param {Uint8Array} data - Encrypted data bytes
    /// @returns {WasmPqcBinaryFormat} New instance
    #[wasm_bindgen(constructor)]
    pub fn new(
        algorithm: WasmAlgorithm,
        metadata: WasmPqcMetadata,
        data: Vec<u8>,
    ) -> WasmPqcBinaryFormat {
        let rust_metadata = metadata.to_rust();
        let inner = PqcBinaryFormat::new(algorithm.inner, rust_metadata, data);
        Self { inner }
    }

    /// Create with specific flags
    ///
    /// @param {WasmAlgorithm} algorithm - Algorithm to use
    /// @param {WasmFormatFlags} flags - Format flags
    /// @param {WasmPqcMetadata} metadata - Metadata container
    /// @param {Uint8Array} data - Encrypted data bytes
    /// @returns {WasmPqcBinaryFormat} New instance
    #[wasm_bindgen(js_name = withFlags)]
    pub fn with_flags(
        algorithm: WasmAlgorithm,
        flags: WasmFormatFlags,
        metadata: WasmPqcMetadata,
        data: Vec<u8>,
    ) -> WasmPqcBinaryFormat {
        let rust_metadata = metadata.to_rust();
        let inner = PqcBinaryFormat::with_flags(algorithm.inner, flags.inner, rust_metadata, data);
        Self { inner }
    }

    /// Serialize to bytes
    ///
    /// @returns {Uint8Array} Serialized bytes
    /// @throws {Error} If serialization fails
    #[wasm_bindgen(js_name = toBytes)]
    pub fn to_bytes(&self) -> Result<Vec<u8>, JsValue> {
        self.inner
            .to_bytes()
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Deserialize from bytes
    ///
    /// @param {Uint8Array} data - Bytes to deserialize
    /// @returns {WasmPqcBinaryFormat} Deserialized instance
    /// @throws {Error} If deserialization fails
    #[wasm_bindgen(js_name = fromBytes)]
    pub fn from_bytes(data: &[u8]) -> Result<WasmPqcBinaryFormat, JsValue> {
        let inner = PqcBinaryFormat::from_bytes(data)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(Self { inner })
    }

    /// Validate the format structure
    ///
    /// @throws {Error} If validation fails
    pub fn validate(&self) -> Result<(), JsValue> {
        self.inner
            .validate()
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Get algorithm
    ///
    /// @returns {WasmAlgorithm} Algorithm used
    #[wasm_bindgen(getter)]
    pub fn algorithm(&self) -> WasmAlgorithm {
        WasmAlgorithm {
            inner: self.inner.algorithm(),
        }
    }

    /// Get encrypted data
    ///
    /// @returns {Uint8Array} Encrypted data
    #[wasm_bindgen(getter)]
    pub fn data(&self) -> Vec<u8> {
        self.inner.data().to_vec()
    }

    /// Get format flags
    ///
    /// @returns {WasmFormatFlags} Format flags
    #[wasm_bindgen(getter)]
    pub fn flags(&self) -> WasmFormatFlags {
        WasmFormatFlags {
            inner: self.inner.flags(),
        }
    }

    /// Get total serialized size
    ///
    /// @returns {number} Size in bytes
    #[wasm_bindgen(js_name = totalSize)]
    pub fn total_size(&self) -> usize {
        self.inner.total_size()
    }
}

/// Get the PQC Binary Format version
#[wasm_bindgen(js_name = getVersion)]
pub fn get_version() -> String {
    crate::VERSION.to_string()
}

/// Get the PQC Binary Format spec version
#[wasm_bindgen(js_name = getBinaryVersion)]
pub fn get_binary_version() -> u8 {
    crate::PQC_BINARY_VERSION
}
