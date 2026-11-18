//! The C interface: what a C, C++ or Go caller gets, and what it gets for a
//! null pointer or a buffer that does not parse.

use std::ffi::CStr;
use std::ptr;

use pqc_binary_format::ffi::{
    ByteBuffer, PqcFormatHandle, pqc_format_free, pqc_format_from_bytes,
    pqc_format_get_algorithm_id, pqc_format_get_algorithm_name, pqc_format_get_data,
    pqc_format_get_total_size, pqc_format_new, pqc_format_new_with_kem, pqc_format_to_bytes,
    pqc_format_validate, pqc_free_buffer, pqc_free_string, pqc_get_binary_version, pqc_get_version,
};
use pqc_binary_format::{Algorithm, PQC_BINARY_VERSION, PqcBinaryFormat, VERSION};

const HYBRID: u16 = Algorithm::Hybrid.as_id();
const IV: [u8; 12] = [1; 12];
const TAG: [u8; 16] = [2; 16];
const KEY: [u8; 32] = [3; 32];
const CIPHERTEXT: [u8; 48] = [4; 48];
const DATA: [u8; 5] = [5, 6, 7, 8, 9];

fn new_format(algorithm: u16) -> *mut PqcFormatHandle {
    unsafe {
        pqc_format_new(
            algorithm,
            IV.as_ptr(),
            IV.len(),
            TAG.as_ptr(),
            TAG.len(),
            DATA.as_ptr(),
            DATA.len(),
        )
    }
}

/// Copies a buffer the library allocated, then gives it back.
fn take(buffer: ByteBuffer) -> Vec<u8> {
    assert!(!buffer.data.is_null());
    unsafe {
        let copy = std::slice::from_raw_parts(buffer.data, buffer.len).to_vec();
        pqc_free_buffer(buffer);
        copy
    }
}

/// Copies a string the library allocated, then gives it back.
fn take_string(string: *mut std::os::raw::c_char) -> String {
    assert!(!string.is_null());
    unsafe {
        let copy = CStr::from_ptr(string).to_str().unwrap().to_string();
        pqc_free_string(string);
        copy
    }
}

#[test]
fn a_format_survives_the_c_interface() {
    let handle = new_format(HYBRID);
    assert!(!handle.is_null());
    unsafe {
        assert_eq!(pqc_format_validate(handle), 0);
        assert_eq!(pqc_format_get_algorithm_id(handle), HYBRID);
        assert_eq!(
            take_string(pqc_format_get_algorithm_name(handle)),
            Algorithm::Hybrid.name()
        );
        assert_eq!(take(pqc_format_get_data(handle)), DATA);

        let bytes = take(pqc_format_to_bytes(handle));
        assert_eq!(pqc_format_get_total_size(handle), bytes.len());

        let parsed = pqc_format_from_bytes(bytes.as_ptr(), bytes.len());
        assert!(!parsed.is_null());
        assert_eq!(take(pqc_format_get_data(parsed)), DATA);
        pqc_format_free(parsed);
        pqc_format_free(handle);
    }
}

#[test]
fn kem_parameters_reach_the_serialized_format() {
    unsafe {
        let handle = pqc_format_new_with_kem(
            HYBRID,
            IV.as_ptr(),
            IV.len(),
            TAG.as_ptr(),
            TAG.len(),
            KEY.as_ptr(),
            KEY.len(),
            CIPHERTEXT.as_ptr(),
            CIPHERTEXT.len(),
            DATA.as_ptr(),
            DATA.len(),
        );
        assert!(!handle.is_null());
        let bytes = take(pqc_format_to_bytes(handle));
        pqc_format_free(handle);

        let parsed = PqcBinaryFormat::from_bytes(&bytes).unwrap();
        let kem = parsed.metadata().kem_params.as_ref().unwrap();
        assert_eq!(kem.public_key, KEY);
        assert_eq!(kem.ciphertext, CIPHERTEXT);
        assert_eq!(parsed.data(), DATA);
    }
}

#[test]
fn an_unknown_algorithm_gives_no_handle() {
    assert!(new_format(0xFFFF).is_null());
    unsafe {
        let handle = pqc_format_new_with_kem(
            0xFFFF,
            IV.as_ptr(),
            IV.len(),
            TAG.as_ptr(),
            TAG.len(),
            KEY.as_ptr(),
            KEY.len(),
            CIPHERTEXT.as_ptr(),
            CIPHERTEXT.len(),
            DATA.as_ptr(),
            DATA.len(),
        );
        assert!(handle.is_null());
    }
}

#[test]
fn a_null_input_gives_no_handle() {
    let (iv, tag, data) = (IV.as_ptr(), TAG.as_ptr(), DATA.as_ptr());
    let null = ptr::null();
    unsafe {
        assert!(pqc_format_new(HYBRID, null, 0, tag, 16, data, 5).is_null());
        assert!(pqc_format_new(HYBRID, iv, 12, null, 0, data, 5).is_null());
        assert!(pqc_format_new(HYBRID, iv, 12, tag, 16, null, 0).is_null());
    }

    let (key, ciphertext) = (KEY.as_ptr(), CIPHERTEXT.as_ptr());
    for missing in 0..5 {
        let pick = |index, pointer| if index == missing { null } else { pointer };
        unsafe {
            let handle = pqc_format_new_with_kem(
                HYBRID,
                pick(0, iv),
                12,
                pick(1, tag),
                16,
                pick(2, key),
                32,
                pick(3, ciphertext),
                48,
                pick(4, data),
                5,
            );
            assert!(handle.is_null(), "pointer {missing} was null");
        }
    }
}

#[test]
fn a_null_handle_is_answered_not_dereferenced() {
    let null: *const PqcFormatHandle = ptr::null();
    unsafe {
        assert!(pqc_format_to_bytes(null).data.is_null());
        assert!(pqc_format_get_data(null).data.is_null());
        assert!(pqc_format_get_algorithm_name(null).is_null());
        assert_eq!(pqc_format_get_algorithm_id(null), 0);
        assert_eq!(pqc_format_get_total_size(null), 0);
        assert_eq!(pqc_format_validate(null), -1);
        assert!(pqc_format_from_bytes(ptr::null(), 0).is_null());
    }
}

#[test]
fn freeing_null_does_nothing() {
    unsafe {
        pqc_format_free(ptr::null_mut());
        pqc_free_string(ptr::null_mut());
        pqc_free_buffer(ByteBuffer {
            data: ptr::null_mut(),
            len: 0,
            capacity: 0,
        });
    }
}

#[test]
fn bytes_that_do_not_parse_give_no_handle() {
    let garbage = [0u8; 64];
    unsafe {
        assert!(pqc_format_from_bytes(garbage.as_ptr(), garbage.len()).is_null());
    }
}

#[test]
fn a_format_without_an_iv_does_not_validate_or_serialize() {
    unsafe {
        let handle = pqc_format_new(
            HYBRID,
            IV.as_ptr(),
            0,
            TAG.as_ptr(),
            TAG.len(),
            DATA.as_ptr(),
            DATA.len(),
        );
        assert!(!handle.is_null());
        assert_eq!(pqc_format_validate(handle), -1);
        assert!(pqc_format_to_bytes(handle).data.is_null());
        assert_eq!(pqc_format_get_total_size(handle), 0);
        pqc_format_free(handle);
    }
}

#[test]
fn the_versions_are_the_crates() {
    assert_eq!(take_string(pqc_get_version()), VERSION);
    assert_eq!(pqc_get_binary_version(), PQC_BINARY_VERSION);
}
