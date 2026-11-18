//! The public API, through every path a caller or a hostile buffer can take.

use std::collections::HashMap;

use pqc_binary_format::{
    Algorithm, CompressionParameters, CryptoError, EncParameters, FormatFlags, KemParameters,
    PQC_BINARY_VERSION, PQC_MAGIC, PqcBinaryFormat, PqcMetadata, SigParameters,
};
use sha2::{Digest, Sha256};

const HYBRID: u16 = Algorithm::Hybrid.as_id();

fn metadata() -> PqcMetadata {
    PqcMetadata {
        enc_params: EncParameters {
            iv: vec![1; 12],
            tag: vec![2; 16],
            params: HashMap::new(),
        },
        ..Default::default()
    }
}

fn full_metadata() -> PqcMetadata {
    let params = HashMap::from([("p".to_string(), vec![9, 8, 7])]);
    let mut full = PqcMetadata {
        kem_params: Some(KemParameters {
            public_key: vec![3; 32],
            ciphertext: vec![4; 48],
            params: params.clone(),
        }),
        sig_params: Some(SigParameters {
            public_key: vec![5; 32],
            signature: vec![6; 64],
            params: params.clone(),
        }),
        enc_params: EncParameters {
            iv: vec![1; 12],
            tag: vec![2; 16],
            params: params.clone(),
        },
        compression_params: Some(CompressionParameters {
            algorithm: "zstd".to_string(),
            level: 3,
            original_size: 4096,
            params,
        }),
        custom: HashMap::new(),
    };
    full.add_custom("note".to_string(), vec![1, 2, 3]);
    full
}

/// The bytes before the checksum, laid out by hand from the specification.
fn prefix(algorithm: u16, metadata: &[u8], data: &[u8]) -> Vec<u8> {
    let mut out = PQC_MAGIC.to_vec();
    out.push(PQC_BINARY_VERSION);
    out.extend_from_slice(&algorithm.to_le_bytes());
    out.push(0);
    out.extend_from_slice(&u32::try_from(metadata.len()).unwrap().to_le_bytes());
    out.extend_from_slice(metadata);
    out.extend_from_slice(&(data.len() as u64).to_le_bytes());
    out.extend_from_slice(data);
    out
}

/// A complete envelope with a correct checksum over whatever it is given.
fn envelope(algorithm: u16, metadata: &[u8], data: &[u8]) -> Vec<u8> {
    let mut out = prefix(algorithm, metadata, data);
    let checksum = Sha256::digest(&out);
    out.extend_from_slice(&checksum);
    out
}

fn parse_error(bytes: &[u8]) -> CryptoError {
    PqcBinaryFormat::from_bytes(bytes).expect_err("the buffer must be refused")
}

fn metadata_error(json: &str) -> String {
    match PqcMetadata::from_json_bytes(json.as_bytes()) {
        Err(CryptoError::MetadataError(message)) => message,
        other => panic!("{json}: expected a metadata error, got {other:?}"),
    }
}

#[test]
fn every_algorithm_round_trips() {
    for algorithm in Algorithm::all() {
        let format = PqcBinaryFormat::new(algorithm, metadata(), vec![7; 64]);
        let parsed = PqcBinaryFormat::from_bytes(&format.to_bytes().unwrap()).unwrap();
        assert_eq!(parsed, format, "{}", algorithm.name());
        assert_eq!(parsed.algorithm(), algorithm);
    }
}

#[test]
fn every_metadata_section_round_trips() {
    let format = PqcBinaryFormat::new(Algorithm::Hybrid, full_metadata(), vec![7; 8]);
    let parsed = PqcBinaryFormat::from_bytes(&format.to_bytes().unwrap()).unwrap();
    assert_eq!(parsed.metadata(), &full_metadata());
    assert_eq!(parsed.metadata().get_custom("note"), Some(&[1, 2, 3][..]));
    assert_eq!(parsed.metadata().get_custom("absent"), None);
}

#[test]
fn the_layout_is_the_specifications() {
    let format = PqcBinaryFormat::new(Algorithm::Hybrid, metadata(), vec![7; 8]);
    let by_hand = envelope(HYBRID, &metadata().to_json_bytes(), &[7; 8]);
    assert_eq!(format.to_bytes().unwrap(), by_hand);
    assert_eq!(format.total_size(), by_hand.len());
}

#[test]
fn an_empty_payload_is_valid() {
    let format = PqcBinaryFormat::new(Algorithm::Hybrid, metadata(), Vec::new());
    let parsed = PqcBinaryFormat::from_bytes(&format.to_bytes().unwrap()).unwrap();
    assert!(parsed.data().is_empty());
}

#[test]
fn a_short_buffer_is_refused() {
    assert!(matches!(
        parse_error(&[0; 51]),
        CryptoError::BinaryFormatError(_)
    ));
}

#[test]
fn wrong_magic_is_refused() {
    let mut bytes = envelope(HYBRID, &metadata().to_json_bytes(), &[7]);
    bytes[0] ^= 0xFF;
    assert_eq!(parse_error(&bytes), CryptoError::InvalidMagic);
}

#[test]
fn an_unknown_version_is_refused() {
    let mut bytes = envelope(HYBRID, &metadata().to_json_bytes(), &[7]);
    bytes[4] = PQC_BINARY_VERSION + 1;
    assert_eq!(
        parse_error(&bytes),
        CryptoError::UnsupportedVersion(PQC_BINARY_VERSION + 1)
    );
}

#[test]
fn an_unknown_algorithm_is_refused() {
    let bytes = envelope(0xFFFF, &metadata().to_json_bytes(), &[7]);
    assert!(matches!(
        parse_error(&bytes),
        CryptoError::UnknownAlgorithm(_)
    ));
}

#[test]
fn metadata_longer_than_the_buffer_is_refused() {
    let mut bytes = envelope(HYBRID, &metadata().to_json_bytes(), &[7]);
    bytes[8..12].copy_from_slice(&u32::MAX.to_le_bytes());
    assert!(matches!(
        parse_error(&bytes),
        CryptoError::BinaryFormatError(_)
    ));
}

#[test]
fn a_buffer_ending_before_the_data_length_is_refused() {
    let json = metadata().to_json_bytes();
    let mut bytes = prefix(HYBRID, &json, &[]);
    bytes.truncate(12 + json.len());
    bytes.extend_from_slice(&[0; 32]);
    assert!(matches!(
        parse_error(&bytes),
        CryptoError::BinaryFormatError(_)
    ));
}

/// Writes `length` over the envelope's data length field.
fn with_data_length(length: u64) -> Vec<u8> {
    let json = metadata().to_json_bytes();
    let mut bytes = envelope(HYBRID, &json, &[7; 4]);
    let at = 12 + json.len();
    bytes[at..at + 8].copy_from_slice(&length.to_le_bytes());
    bytes
}

#[test]
fn a_data_length_past_the_address_space_is_refused() {
    assert!(matches!(
        parse_error(&with_data_length(u64::MAX)),
        CryptoError::BinaryFormatError(_)
    ));
}

/// The end of the data fits in a usize here, and adding the checksum to it
/// does not.
#[test]
fn a_data_length_ending_at_the_top_of_the_address_space_is_refused() {
    let at = 12 + metadata().to_json_bytes().len() as u64 + 8;
    for below_top in 0..32 {
        assert!(matches!(
            parse_error(&with_data_length(u64::MAX - at - below_top)),
            CryptoError::BinaryFormatError(_)
        ));
    }
}

#[test]
fn a_data_length_that_disagrees_with_the_buffer_is_refused() {
    for length in [0, 3, 5] {
        assert!(matches!(
            parse_error(&with_data_length(length)),
            CryptoError::BinaryFormatError(_)
        ));
    }
}

#[test]
fn any_changed_payload_byte_fails_the_checksum() {
    let json = metadata().to_json_bytes();
    let clean = envelope(HYBRID, &json, &[7; 4]);
    for offset in 0..4 {
        let mut bytes = clean.clone();
        bytes[12 + json.len() + 8 + offset] ^= 1;
        assert_eq!(parse_error(&bytes), CryptoError::ChecksumMismatch);
    }
}

#[test]
fn a_changed_checksum_is_refused() {
    let mut bytes = envelope(HYBRID, &metadata().to_json_bytes(), &[7]);
    *bytes.last_mut().unwrap() ^= 1;
    assert_eq!(parse_error(&bytes), CryptoError::ChecksumMismatch);
}

#[test]
fn an_empty_iv_is_refused_after_the_checksum_passes() {
    let mut empty = metadata();
    empty.enc_params.iv.clear();
    let bytes = envelope(HYBRID, &empty.to_json_bytes(), &[7]);
    assert!(matches!(parse_error(&bytes), CryptoError::MetadataError(_)));
}

#[test]
fn malformed_metadata_is_refused() {
    assert!(metadata_error("not json").contains("Invalid metadata JSON"));
    assert!(metadata_error("[]").contains("JSON object"));
    assert!(metadata_error("{}").contains("encryption_params"));
    assert!(metadata_error(r#"{"encryption_params":{"tag":""}}"#).contains("'iv'"));
    assert!(metadata_error(r#"{"encryption_params":{"iv":"!","tag":""}}"#).contains("base64"));
    assert!(
        metadata_error(r#"{"encryption_params":{"iv":"","tag":"","params":{"k":1}}}"#)
            .contains("not a string")
    );
    assert!(
        metadata_error(r#"{"encryption_params":{"iv":"","tag":"","params":{"k":"!"}}}"#)
            .contains("base64")
    );
}

#[test]
fn malformed_optional_sections_are_refused() {
    let enc = r#""encryption_params":{"iv":"AQ==","tag":""}"#;
    for section in [
        r#""kem_params":{"ciphertext":""}"#,
        r#""kem_params":{"public_key":""}"#,
        r#""signature_params":{"signature":""}"#,
        r#""signature_params":{"public_key":""}"#,
        r#""compression_params":{"params":{"k":1}}"#,
        r#""custom":{"k":"!"}"#,
    ] {
        metadata_error(&format!("{{{enc},{section}}}"));
    }
}

#[test]
fn absent_compression_fields_take_their_defaults() {
    let json = r#"{"encryption_params":{"iv":"AQ==","tag":""},"compression_params":{"level":300}}"#;
    let parsed = PqcMetadata::from_json_bytes(json.as_bytes()).unwrap();
    let compression = parsed.compression_params.unwrap();
    assert_eq!(compression.algorithm, "");
    assert_eq!(compression.level, 0);
    assert_eq!(compression.original_size, 0);
}

#[test]
fn an_invalid_structure_does_not_serialize() {
    let mut format = PqcBinaryFormat::new(Algorithm::Hybrid, metadata(), vec![7]);
    format.magic = *b"NOPE";
    assert_eq!(format.validate(), Err(CryptoError::InvalidMagic));
    assert_eq!(format.to_bytes(), Err(CryptoError::InvalidMagic));
    assert_eq!(format.total_size(), 0);

    let mut format = PqcBinaryFormat::new(Algorithm::Hybrid, metadata(), vec![7]);
    format.version = 9;
    assert_eq!(format.validate(), Err(CryptoError::UnsupportedVersion(9)));

    let format = PqcBinaryFormat::new(Algorithm::Hybrid, PqcMetadata::new(), vec![7]);
    assert!(matches!(
        format.validate(),
        Err(CryptoError::MetadataError(_))
    ));
}

#[test]
fn the_checksum_follows_the_fields() {
    let mut format = PqcBinaryFormat::new(Algorithm::Hybrid, metadata(), vec![7]);
    let before = format.checksum;
    format.data.push(8);
    format.update_checksum();
    assert_ne!(format.checksum, before);
    assert_eq!(format.data(), &[7, 8]);
}

#[test]
fn every_flag_is_independent() {
    assert_eq!(FormatFlags::default(), FormatFlags::new());
    assert_eq!(FormatFlags::new().as_u8(), 0);
    let all = FormatFlags::new()
        .with_compression()
        .with_streaming()
        .with_additional_auth()
        .with_experimental();
    assert_eq!(all.as_u8(), 0x0F);
    assert_eq!(FormatFlags::from_u8(0x0F), all);
    let experimental = FormatFlags::new().with_experimental();
    assert!(experimental.has_experimental());
    assert!(!experimental.has_compression());
    assert!(!experimental.has_streaming());
    assert!(!experimental.has_additional_auth());

    let format = PqcBinaryFormat::with_flags(Algorithm::Hybrid, all, metadata(), vec![7]);
    let parsed = PqcBinaryFormat::from_bytes(&format.to_bytes().unwrap()).unwrap();
    assert_eq!(parsed.flags(), all);
}

#[test]
fn every_error_says_what_went_wrong() {
    for (error, text) in [
        (
            CryptoError::BinaryFormatError("a".into()),
            "Binary format error: a",
        ),
        (CryptoError::InvalidInput("b".into()), "Invalid input: b"),
        (
            CryptoError::SerializationFailed("c".into()),
            "Serialization failed: c",
        ),
        (
            CryptoError::DeserializationFailed("d".into()),
            "Deserialization failed: d",
        ),
        (
            CryptoError::UnknownAlgorithm("e".into()),
            "Unknown algorithm: e",
        ),
        (CryptoError::ChecksumMismatch, "Checksum validation failed"),
        (CryptoError::InvalidMagic, "Invalid magic bytes"),
        (
            CryptoError::UnsupportedVersion(2),
            "Unsupported version: 0x2",
        ),
        (
            CryptoError::MetadataError("f".into()),
            "Metadata validation error: f",
        ),
    ] {
        assert_eq!(error.to_string(), text);
    }
}

#[test]
fn the_registry_answers_for_every_algorithm() {
    for algorithm in Algorithm::all() {
        assert_eq!(Algorithm::from_id(algorithm.as_id()), Some(algorithm));
        assert_eq!(
            Algorithm::from_name(algorithm.canonical_name()),
            Some(algorithm)
        );
        assert!(!algorithm.name().is_empty());
        assert!(!algorithm.family().name().is_empty());
        assert!(algorithm.c_constant().starts_with("PQC_ALGORITHM_"));
    }
    assert_eq!(Algorithm::from_id(0xFFFF), None);
    assert_eq!(Algorithm::from_name("no-such-algorithm"), None);
}
