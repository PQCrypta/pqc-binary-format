//! C/C++ FFI bindings for PQC Binary Format
//!
//! This module provides C-compatible foreign function interface (FFI)
//! for use in C/C++ applications and Go (via cgo).

#![allow(missing_docs)]

use std::collections::HashMap;
use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_uchar};
use std::ptr;
use std::slice;

use crate::{Algorithm, EncParameters, KemParameters, PqcBinaryFormat, PqcMetadata};

/// Opaque handle to PqcBinaryFormat
#[repr(C)]
pub struct PqcFormatHandle {
    _private: [u8; 0],
}

/// C-compatible byte buffer
#[repr(C)]
pub struct ByteBuffer {
    pub data: *mut c_uchar,
    pub len: usize,
    pub capacity: usize,
}

impl ByteBuffer {
    fn from_vec(vec: Vec<u8>) -> Self {
        let mut vec = vec;
        let data = vec.as_mut_ptr();
        let len = vec.len();
        let capacity = vec.capacity();
        std::mem::forget(vec);
        Self {
            data,
            len,
            capacity,
        }
    }

    #[allow(dead_code)]
    unsafe fn to_vec(&self) -> Vec<u8> {
        if self.data.is_null() {
            return Vec::new();
        }
        slice::from_raw_parts(self.data, self.len).to_vec()
    }
}

/// Free a byte buffer allocated by this library
///
/// # Safety
/// The buffer must have been allocated by this library
#[no_mangle]
pub unsafe extern "C" fn pqc_free_buffer(buffer: ByteBuffer) {
    if !buffer.data.is_null() {
        drop(Vec::from_raw_parts(
            buffer.data,
            buffer.len,
            buffer.capacity,
        ));
    }
}

/// Free a string allocated by this library
///
/// # Safety
/// The string must have been allocated by this library
#[no_mangle]
pub unsafe extern "C" fn pqc_free_string(s: *mut c_char) {
    if !s.is_null() {
        drop(CString::from_raw(s));
    }
}

/// Create a new PQC Binary Format structure
///
/// # Parameters
/// - `algorithm_id`: Algorithm identifier (see Algorithm enum)
/// - `iv`: IV/nonce bytes
/// - `iv_len`: Length of IV
/// - `tag`: Authentication tag bytes
/// - `tag_len`: Length of tag
/// - `data`: Encrypted data bytes
/// - `data_len`: Length of encrypted data
///
/// # Returns
/// Opaque handle to PqcBinaryFormat, or NULL on error
///
/// # Safety
/// All pointers must be valid. The returned handle must be freed with `pqc_format_free`.
#[no_mangle]
pub unsafe extern "C" fn pqc_format_new(
    algorithm_id: u16,
    iv: *const c_uchar,
    iv_len: usize,
    tag: *const c_uchar,
    tag_len: usize,
    data: *const c_uchar,
    data_len: usize,
) -> *mut PqcFormatHandle {
    // Validate pointers
    if iv.is_null() || tag.is_null() || data.is_null() {
        return ptr::null_mut();
    }

    // Convert algorithm ID
    let algorithm = match Algorithm::from_id(algorithm_id) {
        Some(a) => a,
        None => return ptr::null_mut(),
    };

    // Convert byte slices
    let iv_vec = slice::from_raw_parts(iv, iv_len).to_vec();
    let tag_vec = slice::from_raw_parts(tag, tag_len).to_vec();
    let data_vec = slice::from_raw_parts(data, data_len).to_vec();

    // Create metadata
    let metadata = PqcMetadata {
        enc_params: EncParameters {
            iv: iv_vec,
            tag: tag_vec,
            params: HashMap::new(),
        },
        kem_params: None,
        sig_params: None,
        compression_params: None,
        custom: HashMap::new(),
    };

    // Create format
    let format = PqcBinaryFormat::new(algorithm, metadata, data_vec);

    Box::into_raw(Box::new(format)) as *mut PqcFormatHandle
}

/// Create PQC Binary Format with KEM parameters
///
/// # Safety
/// All pointers must be valid
#[no_mangle]
pub unsafe extern "C" fn pqc_format_new_with_kem(
    algorithm_id: u16,
    iv: *const c_uchar,
    iv_len: usize,
    tag: *const c_uchar,
    tag_len: usize,
    kem_public_key: *const c_uchar,
    kem_public_key_len: usize,
    kem_ciphertext: *const c_uchar,
    kem_ciphertext_len: usize,
    data: *const c_uchar,
    data_len: usize,
) -> *mut PqcFormatHandle {
    if iv.is_null()
        || tag.is_null()
        || kem_public_key.is_null()
        || kem_ciphertext.is_null()
        || data.is_null()
    {
        return ptr::null_mut();
    }

    let algorithm = match Algorithm::from_id(algorithm_id) {
        Some(a) => a,
        None => return ptr::null_mut(),
    };

    let metadata = PqcMetadata {
        enc_params: EncParameters {
            iv: slice::from_raw_parts(iv, iv_len).to_vec(),
            tag: slice::from_raw_parts(tag, tag_len).to_vec(),
            params: HashMap::new(),
        },
        kem_params: Some(KemParameters {
            public_key: slice::from_raw_parts(kem_public_key, kem_public_key_len).to_vec(),
            ciphertext: slice::from_raw_parts(kem_ciphertext, kem_ciphertext_len).to_vec(),
            params: HashMap::new(),
        }),
        sig_params: None,
        compression_params: None,
        custom: HashMap::new(),
    };

    let data_vec = slice::from_raw_parts(data, data_len).to_vec();
    let format = PqcBinaryFormat::new(algorithm, metadata, data_vec);

    Box::into_raw(Box::new(format)) as *mut PqcFormatHandle
}

/// Serialize PQC Binary Format to bytes
///
/// # Parameters
/// - `handle`: Handle to PqcBinaryFormat
///
/// # Returns
/// ByteBuffer containing serialized data, or NULL buffer on error
///
/// # Safety
/// Handle must be valid. Returned buffer must be freed with `pqc_free_buffer`.
#[no_mangle]
pub unsafe extern "C" fn pqc_format_to_bytes(handle: *const PqcFormatHandle) -> ByteBuffer {
    if handle.is_null() {
        return ByteBuffer {
            data: ptr::null_mut(),
            len: 0,
            capacity: 0,
        };
    }

    let format = &*(handle as *const PqcBinaryFormat);

    match format.to_bytes() {
        Ok(bytes) => ByteBuffer::from_vec(bytes),
        Err(_) => ByteBuffer {
            data: ptr::null_mut(),
            len: 0,
            capacity: 0,
        },
    }
}

/// Deserialize PQC Binary Format from bytes
///
/// # Parameters
/// - `data`: Bytes to deserialize
/// - `len`: Length of data
///
/// # Returns
/// Handle to PqcBinaryFormat, or NULL on error
///
/// # Safety
/// Data pointer must be valid. Returned handle must be freed with `pqc_format_free`.
#[no_mangle]
pub unsafe extern "C" fn pqc_format_from_bytes(
    data: *const c_uchar,
    len: usize,
) -> *mut PqcFormatHandle {
    if data.is_null() {
        return ptr::null_mut();
    }

    let bytes = slice::from_raw_parts(data, len);

    match PqcBinaryFormat::from_bytes(bytes) {
        Ok(format) => Box::into_raw(Box::new(format)) as *mut PqcFormatHandle,
        Err(_) => ptr::null_mut(),
    }
}

/// Get algorithm ID from format
///
/// # Safety
/// Handle must be valid
#[no_mangle]
pub unsafe extern "C" fn pqc_format_get_algorithm_id(handle: *const PqcFormatHandle) -> u16 {
    if handle.is_null() {
        return 0;
    }

    let format = &*(handle as *const PqcBinaryFormat);
    format.algorithm().as_id()
}

/// Get algorithm name from format
///
/// # Returns
/// Null-terminated string. Must be freed with `pqc_free_string`.
///
/// # Safety
/// Handle must be valid
#[no_mangle]
pub unsafe extern "C" fn pqc_format_get_algorithm_name(
    handle: *const PqcFormatHandle,
) -> *mut c_char {
    if handle.is_null() {
        return ptr::null_mut();
    }

    let format = &*(handle as *const PqcBinaryFormat);
    let name = format.algorithm().name();

    match CString::new(name) {
        Ok(cstr) => cstr.into_raw(),
        Err(_) => ptr::null_mut(),
    }
}

/// Get encrypted data from format
///
/// # Returns
/// ByteBuffer containing data. Must be freed with `pqc_free_buffer`.
///
/// # Safety
/// Handle must be valid
#[no_mangle]
pub unsafe extern "C" fn pqc_format_get_data(handle: *const PqcFormatHandle) -> ByteBuffer {
    if handle.is_null() {
        return ByteBuffer {
            data: ptr::null_mut(),
            len: 0,
            capacity: 0,
        };
    }

    let format = &*(handle as *const PqcBinaryFormat);
    ByteBuffer::from_vec(format.data().to_vec())
}

/// Validate format structure
///
/// # Returns
/// 0 on success, -1 on error
///
/// # Safety
/// Handle must be valid
#[no_mangle]
pub unsafe extern "C" fn pqc_format_validate(handle: *const PqcFormatHandle) -> c_int {
    if handle.is_null() {
        return -1;
    }

    let format = &*(handle as *const PqcBinaryFormat);

    match format.validate() {
        Ok(()) => 0,
        Err(_) => -1,
    }
}

/// Get total size of serialized format
///
/// # Safety
/// Handle must be valid
#[no_mangle]
pub unsafe extern "C" fn pqc_format_get_total_size(handle: *const PqcFormatHandle) -> usize {
    if handle.is_null() {
        return 0;
    }

    let format = &*(handle as *const PqcBinaryFormat);
    format.total_size()
}

/// Free a PqcBinaryFormat handle
///
/// # Safety
/// Handle must have been allocated by this library and not previously freed
#[no_mangle]
pub unsafe extern "C" fn pqc_format_free(handle: *mut PqcFormatHandle) {
    if !handle.is_null() {
        drop(Box::from_raw(handle as *mut PqcBinaryFormat));
    }
}

/// Get library version string
///
/// # Returns
/// Null-terminated version string. Must be freed with `pqc_free_string`.
#[no_mangle]
pub extern "C" fn pqc_get_version() -> *mut c_char {
    match CString::new(crate::VERSION) {
        Ok(cstr) => cstr.into_raw(),
        Err(_) => ptr::null_mut(),
    }
}

/// Get binary format version
#[no_mangle]
pub extern "C" fn pqc_get_binary_version() -> u8 {
    crate::PQC_BINARY_VERSION
}

// Algorithm ID constants for C/C++ convenience

/// Algorithm ID for Classical (X25519 + Ed25519 + AES-256-GCM)
#[no_mangle]
pub static PQC_ALGORITHM_CLASSICAL: u16 = 0x0050;

/// Algorithm ID for Password-based Classical encryption
#[no_mangle]
pub static PQC_ALGORITHM_PASSWORD_CLASSICAL: u16 = 0x0051;

/// Algorithm ID for Hybrid (ML-KEM-1024 + X25519 + ML-DSA-87 + Ed25519)
#[no_mangle]
pub static PQC_ALGORITHM_HYBRID: u16 = 0x0100;

/// Algorithm ID for Post-Quantum (ML-KEM-1024 + ML-DSA-87)
#[no_mangle]
pub static PQC_ALGORITHM_POST_QUANTUM: u16 = 0x0200;

/// Algorithm ID for ML-KEM-1024 pure implementation
#[no_mangle]
pub static PQC_ALGORITHM_ML_KEM_1024: u16 = 0x0202;

/// Algorithm ID for Multi-KEM (multiple key encapsulation layers)
#[no_mangle]
pub static PQC_ALGORITHM_MULTI_KEM: u16 = 0x0203;

/// Algorithm ID for Multi-KEM Triple Layer
#[no_mangle]
pub static PQC_ALGORITHM_MULTI_KEM_TRIPLE: u16 = 0x0204;

/// Algorithm ID for Quad-Layer redundant encryption
#[no_mangle]
pub static PQC_ALGORITHM_QUAD_LAYER: u16 = 0x0205;

/// Algorithm ID for PQ3-Stack with forward secrecy
#[no_mangle]
pub static PQC_ALGORITHM_PQ3_STACK: u16 = 0x0207;

/// Algorithm ID for Lattice-Code Hybrid Stack
#[no_mangle]
pub static PQC_ALGORITHM_LATTICE_CODE_HYBRID: u16 = 0x0208;

/// Algorithm ID for HQC-128 (NIST Level 1, 128-bit security)
#[no_mangle]
pub static PQC_ALGORITHM_HQC_128: u16 = 0x0600;

/// Algorithm ID for HQC-192 (NIST Level 3, 192-bit security)
#[no_mangle]
pub static PQC_ALGORITHM_HQC_192: u16 = 0x0601;

/// Algorithm ID for HQC-256 (NIST Level 5, 256-bit security)
#[no_mangle]
pub static PQC_ALGORITHM_HQC_256: u16 = 0x0602;
