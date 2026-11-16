//! A correctly framed envelope around the input: magic, version, lengths and
//! the SHA-256 checksum right, so the input reaches the algorithm, the
//! metadata and `validate()` instead of stopping at the checksum. The input is
//! the algorithm ID (2 bytes), the flags (1), the metadata's length (2), then
//! the metadata and the payload.
#![no_main]

use libfuzzer_sys::fuzz_target;
use pqc_binary_format::{PqcBinaryFormat, PQC_BINARY_VERSION, PQC_MAGIC};
use sha2::{Digest, Sha256};

fuzz_target!(|input: &[u8]| {
    if input.len() < 5 {
        return;
    }
    let (head, rest) = input.split_at(5);
    let meta_len = usize::from(u16::from_le_bytes([head[3], head[4]])).min(rest.len());
    let (metadata, payload) = rest.split_at(meta_len);

    let mut buf = Vec::with_capacity(12 + metadata.len() + 8 + payload.len() + 32);
    buf.extend_from_slice(&PQC_MAGIC);
    buf.push(PQC_BINARY_VERSION);
    buf.extend_from_slice(&head[..2]);
    buf.push(head[2]);
    buf.extend_from_slice(&u32::try_from(metadata.len()).unwrap().to_le_bytes());
    buf.extend_from_slice(metadata);
    buf.extend_from_slice(&u64::try_from(payload.len()).unwrap().to_le_bytes());
    buf.extend_from_slice(payload);
    let checksum: [u8; 32] = Sha256::digest(&buf).into();
    buf.extend_from_slice(&checksum);

    if let Ok(format) = PqcBinaryFormat::from_bytes(&buf) {
        assert_eq!(format.data(), payload, "the payload is the one framed");
        let bytes = format.to_bytes().expect("an accepted envelope serializes");
        let again = PqcBinaryFormat::from_bytes(&bytes).expect("its serialization parses");
        assert_eq!(again.data(), payload, "the payload survives the round trip");
        assert_eq!(
            again.to_bytes().expect("and serializes again"),
            bytes,
            "serialization is a fixed point after one round"
        );
    }
});
