//! Any bytes, as an untrusted envelope arrives: `from_bytes()` answers with a
//! value or an error and never panics, and a value it accepts serializes, and
//! that serialization parses back to the same content and serializes the same.
#![no_main]

use libfuzzer_sys::fuzz_target;
use pqc_binary_format::PqcBinaryFormat;

fuzz_target!(|data: &[u8]| {
    if let Ok(format) = PqcBinaryFormat::from_bytes(data) {
        round_trip(&format);
    }
});

fn round_trip(format: &PqcBinaryFormat) {
    let bytes = format.to_bytes().expect("an accepted envelope serializes");
    let again = PqcBinaryFormat::from_bytes(&bytes).expect("its serialization parses");
    assert_eq!(again.data(), format.data(), "the payload survives the round trip");
    assert_eq!(
        again.to_bytes().expect("and serializes again"),
        bytes,
        "serialization is a fixed point after one round"
    );
}
