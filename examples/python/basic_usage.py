#!/usr/bin/env python3
"""
Basic usage example for PQC Binary Format Python bindings
Demonstrates encryption format creation, serialization, and deserialization
"""

from pqc_binary_format import (
    Algorithm,
    EncParameters,
    PqcMetadata,
    PqcBinaryFormat,
)


def main():
    print("=" * 60)
    print("PQC Binary Format - Python Basic Usage Example")
    print("=" * 60)
    print()

    # Step 1: Create encryption parameters
    print("Step 1: Creating encryption parameters...")
    iv = bytes([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12])  # 12-byte nonce
    tag = bytes([0] * 16)  # 16-byte authentication tag
    enc_params = EncParameters(iv=iv, tag=tag)
    print(f"  ✓ IV: {len(iv)} bytes")
    print(f"  ✓ Tag: {len(tag)} bytes")
    print()

    # Step 2: Create metadata container
    print("Step 2: Creating metadata container...")
    metadata = PqcMetadata(
        enc_params=enc_params,
        kem_params=None,
        sig_params=None,
        compression_params=None,
    )
    print("  ✓ Metadata created with encryption parameters")
    print()

    # Step 3: Create algorithm
    print("Step 3: Selecting cryptographic algorithm...")
    algorithm = Algorithm("hybrid")
    print(f"  ✓ Algorithm: {algorithm.name()}")
    print(f"  ✓ Algorithm ID: 0x{algorithm.id():04X}")
    print()

    # Step 4: Create PQC Binary Format
    print("Step 4: Creating PQC Binary Format...")
    encrypted_data = bytes([1, 2, 3, 4, 5])  # Simulated encrypted data
    pqc_format = PqcBinaryFormat(algorithm, metadata, encrypted_data)
    print(f"  ✓ Format created with {len(encrypted_data)} bytes of data")
    print()

    # Step 5: Serialize to bytes
    print("Step 5: Serializing to binary format...")
    serialized = pqc_format.to_bytes()
    print(f"  ✓ Serialized size: {len(serialized)} bytes")
    print(f"  ✓ Total format size: {pqc_format.total_size()} bytes")
    print()

    # Step 6: Deserialize from bytes
    print("Step 6: Deserializing from binary format...")
    deserialized = PqcBinaryFormat.from_bytes(serialized)
    print(f"  ✓ Deserialized algorithm: {deserialized.algorithm().name()}")
    print(f"  ✓ Data length: {len(deserialized.data())} bytes")
    print()

    # Step 7: Validate integrity
    print("Step 7: Validating format integrity...")
    try:
        deserialized.validate()
        print("  ✓ Validation passed - checksum verified")
    except Exception as e:
        print(f"  ✗ Validation failed: {e}")
        return 1
    print()

    # Step 8: Verify roundtrip
    print("Step 8: Verifying roundtrip integrity...")
    original_data = pqc_format.data()
    recovered_data = deserialized.data()

    if original_data == recovered_data:
        print("  ✓ Roundtrip successful - data matches!")
    else:
        print("  ✗ Roundtrip failed - data mismatch!")
        return 1
    print()

    print("=" * 60)
    print("✅ All steps completed successfully!")
    print("=" * 60)
    print()
    print("Summary:")
    print(f"  • Algorithm: {algorithm.name()} (0x{algorithm.id():04X})")
    print(f"  • Data size: {len(encrypted_data)} bytes")
    print(f"  • Serialized size: {len(serialized)} bytes")
    print(f"  • Overhead: {len(serialized) - len(encrypted_data)} bytes")
    print()

    return 0


if __name__ == "__main__":
    exit(main())
