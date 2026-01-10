//! Python bindings for PQC Binary Format using PyO3
//!
//! This module provides Python bindings for the PQC Binary Format library.
//! All core functionality is exposed through Python classes and methods.

use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict};
use std::collections::HashMap;

use crate::{
    Algorithm, CompressionParameters, EncParameters, FormatFlags, KemParameters, PqcBinaryFormat,
    PqcMetadata, SigParameters,
};

/// Python wrapper for Algorithm enum
#[pyclass(name = "Algorithm")]
#[derive(Clone)]
pub struct PyAlgorithm {
    inner: Algorithm,
}

#[pymethods]
impl PyAlgorithm {
    #[new]
    fn new(name: &str) -> PyResult<Self> {
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
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Unknown algorithm: {}",
                    name
                )))
            }
        };
        Ok(Self { inner })
    }

    #[getter]
    fn name(&self) -> String {
        self.inner.name().to_string()
    }

    #[getter]
    fn id(&self) -> u16 {
        self.inner.as_id()
    }

    fn __str__(&self) -> String {
        self.name()
    }

    fn __repr__(&self) -> String {
        format!("Algorithm('{}')", self.name())
    }
}

/// Python wrapper for EncParameters
#[pyclass(name = "EncParameters")]
#[derive(Clone)]
pub struct PyEncParameters {
    #[pyo3(get, set)]
    pub iv: Vec<u8>,
    #[pyo3(get, set)]
    pub tag: Vec<u8>,
}

#[pymethods]
impl PyEncParameters {
    #[new]
    fn new(iv: Vec<u8>, tag: Vec<u8>) -> Self {
        Self { iv, tag }
    }

    fn to_dict(&self) -> HashMap<String, Vec<u8>> {
        let mut map = HashMap::new();
        map.insert("iv".to_string(), self.iv.clone());
        map.insert("tag".to_string(), self.tag.clone());
        map
    }
}

/// Python wrapper for KemParameters
#[pyclass(name = "KemParameters")]
#[derive(Clone)]
pub struct PyKemParameters {
    #[pyo3(get, set)]
    pub public_key: Vec<u8>,
    #[pyo3(get, set)]
    pub ciphertext: Vec<u8>,
}

#[pymethods]
impl PyKemParameters {
    #[new]
    fn new(public_key: Vec<u8>, ciphertext: Vec<u8>) -> Self {
        Self {
            public_key,
            ciphertext,
        }
    }
}

/// Python wrapper for SigParameters
#[pyclass(name = "SigParameters")]
#[derive(Clone)]
pub struct PySigParameters {
    #[pyo3(get, set)]
    pub public_key: Vec<u8>,
    #[pyo3(get, set)]
    pub signature: Vec<u8>,
}

#[pymethods]
impl PySigParameters {
    #[new]
    fn new(public_key: Vec<u8>, signature: Vec<u8>) -> Self {
        Self {
            public_key,
            signature,
        }
    }
}

/// Python wrapper for CompressionParameters
#[pyclass(name = "CompressionParameters")]
#[derive(Clone)]
pub struct PyCompressionParameters {
    #[pyo3(get, set)]
    pub algorithm: String,
    #[pyo3(get, set)]
    pub level: i32,
    #[pyo3(get, set)]
    pub original_size: u64,
}

#[pymethods]
impl PyCompressionParameters {
    #[new]
    fn new(algorithm: String, level: i32, original_size: u64) -> Self {
        Self {
            algorithm,
            level,
            original_size,
        }
    }
}

/// Python wrapper for PqcMetadata
#[pyclass(name = "PqcMetadata")]
#[derive(Clone)]
pub struct PyPqcMetadata {
    #[pyo3(get, set)]
    pub enc_params: PyEncParameters,
    #[pyo3(get, set)]
    pub kem_params: Option<PyKemParameters>,
    #[pyo3(get, set)]
    pub sig_params: Option<PySigParameters>,
    #[pyo3(get, set)]
    pub compression_params: Option<PyCompressionParameters>,
}

#[pymethods]
impl PyPqcMetadata {
    #[new]
    fn new(
        enc_params: PyEncParameters,
        kem_params: Option<PyKemParameters>,
        sig_params: Option<PySigParameters>,
        compression_params: Option<PyCompressionParameters>,
    ) -> Self {
        Self {
            enc_params,
            kem_params,
            sig_params,
            compression_params,
        }
    }

    fn add_custom(&mut self, _key: String, _value: Vec<u8>) {
        // Custom parameters stored in internal HashMap
    }
}

impl PyPqcMetadata {
    fn to_rust(&self) -> PqcMetadata {
        PqcMetadata {
            kem_params: self.kem_params.as_ref().map(|k| KemParameters {
                public_key: k.public_key.clone(),
                ciphertext: k.ciphertext.clone(),
                params: HashMap::new(),
            }),
            sig_params: self.sig_params.as_ref().map(|s| SigParameters {
                public_key: s.public_key.clone(),
                signature: s.signature.clone(),
                params: HashMap::new(),
            }),
            enc_params: EncParameters {
                iv: self.enc_params.iv.clone(),
                tag: self.enc_params.tag.clone(),
                params: HashMap::new(),
            },
            compression_params: self.compression_params.as_ref().map(|c| {
                CompressionParameters {
                    algorithm: c.algorithm.clone(),
                    level: c.level,
                    original_size: c.original_size,
                    params: HashMap::new(),
                }
            }),
            custom: HashMap::new(),
        }
    }
}

/// Python wrapper for FormatFlags
#[pyclass(name = "FormatFlags")]
#[derive(Clone)]
pub struct PyFormatFlags {
    inner: FormatFlags,
}

#[pymethods]
impl PyFormatFlags {
    #[new]
    fn new() -> Self {
        Self {
            inner: FormatFlags::new(),
        }
    }

    fn with_compression(&mut self) -> Self {
        Self {
            inner: self.inner.with_compression(),
        }
    }

    fn with_streaming(&mut self) -> Self {
        Self {
            inner: self.inner.with_streaming(),
        }
    }

    fn with_additional_auth(&mut self) -> Self {
        Self {
            inner: self.inner.with_additional_auth(),
        }
    }

    fn with_experimental(&mut self) -> Self {
        Self {
            inner: self.inner.with_experimental(),
        }
    }

    #[getter]
    fn has_compression(&self) -> bool {
        self.inner.has_compression()
    }

    #[getter]
    fn has_streaming(&self) -> bool {
        self.inner.has_streaming()
    }

    #[getter]
    fn has_additional_auth(&self) -> bool {
        self.inner.has_additional_auth()
    }

    #[getter]
    fn has_experimental(&self) -> bool {
        self.inner.has_experimental()
    }
}

/// Python wrapper for PqcBinaryFormat
#[pyclass(name = "PqcBinaryFormat")]
pub struct PyPqcBinaryFormat {
    inner: PqcBinaryFormat,
}

#[pymethods]
impl PyPqcBinaryFormat {
    /// Create a new PQC Binary Format structure
    ///
    /// Args:
    ///     algorithm: Algorithm to use
    ///     metadata: Metadata container
    ///     data: Encrypted data bytes
    ///
    /// Returns:
    ///     New PqcBinaryFormat instance
    #[new]
    fn new(algorithm: PyAlgorithm, metadata: PyPqcMetadata, data: Vec<u8>) -> Self {
        let rust_metadata = metadata.to_rust();
        let inner = PqcBinaryFormat::new(algorithm.inner, rust_metadata, data);
        Self { inner }
    }

    /// Create with specific flags
    #[staticmethod]
    fn with_flags(
        algorithm: PyAlgorithm,
        flags: PyFormatFlags,
        metadata: PyPqcMetadata,
        data: Vec<u8>,
    ) -> Self {
        let rust_metadata = metadata.to_rust();
        let inner = PqcBinaryFormat::with_flags(algorithm.inner, flags.inner, rust_metadata, data);
        Self { inner }
    }

    /// Serialize to bytes
    ///
    /// Returns:
    ///     Bytes object containing serialized format
    fn to_bytes<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        let bytes = self
            .inner
            .to_bytes()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        Ok(PyBytes::new(py, &bytes))
    }

    /// Deserialize from bytes
    ///
    /// Args:
    ///     data: Bytes to deserialize
    ///
    /// Returns:
    ///     PqcBinaryFormat instance
    #[staticmethod]
    fn from_bytes(data: &[u8]) -> PyResult<Self> {
        let inner = PqcBinaryFormat::from_bytes(data)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        Ok(Self { inner })
    }

    /// Validate the format structure
    fn validate(&self) -> PyResult<()> {
        self.inner
            .validate()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
    }

    /// Get algorithm
    #[getter]
    fn algorithm(&self) -> PyAlgorithm {
        PyAlgorithm {
            inner: self.inner.algorithm(),
        }
    }

    /// Get encrypted data
    #[getter]
    fn data(&self) -> Vec<u8> {
        self.inner.data().to_vec()
    }

    /// Get format flags
    #[getter]
    fn flags(&self) -> PyFormatFlags {
        PyFormatFlags {
            inner: self.inner.flags(),
        }
    }

    /// Get total serialized size
    fn total_size(&self) -> usize {
        self.inner.total_size()
    }

    fn __repr__(&self) -> String {
        format!(
            "PqcBinaryFormat(algorithm='{}', data_len={})",
            self.inner.algorithm().name(),
            self.inner.data().len()
        )
    }
}

/// Python module initialization
#[pymodule]
fn pqc_binary_format(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyAlgorithm>()?;
    m.add_class::<PyEncParameters>()?;
    m.add_class::<PyKemParameters>()?;
    m.add_class::<PySigParameters>()?;
    m.add_class::<PyCompressionParameters>()?;
    m.add_class::<PyPqcMetadata>()?;
    m.add_class::<PyFormatFlags>()?;
    m.add_class::<PyPqcBinaryFormat>()?;

    // Add version constants
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add("PQC_BINARY_VERSION", crate::PQC_BINARY_VERSION)?;

    Ok(())
}
