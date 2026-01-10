#!/usr/bin/env python3
"""
Example usage of PQC Binary Format Python bindings
"""

from pqc_binary_format import (
    Algorithm,
    EncParameters,
    KemParameters,
    PqcMetadata,
    PqcBinaryFormat,
    FormatFlags,
)


def main():
    print("PQC Binary Format Python Example")
    print("=" * 50)

    # Example 1: Basic encryption format
    print("\n1. Basic Encryption Format")
    print("-" * 50)

    # Create algorithm
    algorithm = Algorithm("hybrid")
    print(f"Algorithm: {algorithm.name()} (ID: {algorithm.id()})")

    # Create encryption parameters
    enc_params = EncParameters(
        iv=bytes([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]),  # 12-byte nonce
        tag=bytes([0] * 16),  # 16-byte auth tag
    )

    # Create metadata
    metadata = PqcMetadata(
        enc_params=enc_params,
        kem_params=None,
        sig_params=None,
        compression_params=None,
    )

    # Create encrypted data (simulated)
    encrypted_data = bytes([1, 2, 3, 4, 5])

    # Create PQC Binary Format
    pqc_format = PqcBinaryFormat(algorithm, metadata, encrypted_data)

    # Serialize to bytes
    serialized = pqc_format.to_bytes()
    print(f"Serialized size: {len(serialized)} bytes")
    print(f"Total format size: {pqc_format.total_size()} bytes")

    # Deserialize from bytes
    deserialized = PqcBinaryFormat.from_bytes(serialized)
    print(f"Deserialized algorithm: {deserialized.algorithm().name()}")
    print(f"Data length: {len(deserialized.data())} bytes")

    # Validate
    deserialized.validate()
    print("✓ Validation passed")

    # Example 2: Format with KEM parameters
    print("\n2. Format with KEM Parameters")
    print("-" * 50)

    # Create KEM parameters
    kem_params = KemParameters(
        public_key=bytes([0] * 1568),  # ML-KEM-1024 public key
        ciphertext=bytes([0] * 1568),  # Encapsulated key
    )

    # Create metadata with KEM
    metadata_with_kem = PqcMetadata(
        enc_params=enc_params,
        kem_params=kem_params,
        sig_params=None,
        compression_params=None,
    )

    # Create format
    pqc_with_kem = PqcBinaryFormat(algorithm, metadata_with_kem, encrypted_data)
    serialized_with_kem = pqc_with_kem.to_bytes()
    print(f"Serialized size with KEM: {len(serialized_with_kem)} bytes")

    # Example 3: Format with flags
    print("\n3. Format with Feature Flags")
    print("-" * 50)

    # Create flags
    flags = FormatFlags().with_compression().with_streaming()
    print(f"Compression enabled: {flags.has_compression()}")
    print(f"Streaming enabled: {flags.has_streaming()}")
    print(f"Additional auth: {flags.has_additional_auth()}")

    # Create format with flags
    pqc_with_flags = PqcBinaryFormat.with_flags(
        algorithm, flags, metadata, encrypted_data
    )
    print(f"Format with flags size: {pqc_with_flags.total_size()} bytes")

    # Example 4: Algorithm comparison
    print("\n4. Algorithm Comparison")
    print("-" * 50)

    algorithms = ["classical", "hybrid", "post-quantum", "ml-kem-1024"]

    for alg_name in algorithms:
        alg = Algorithm(alg_name)
        fmt = PqcBinaryFormat(alg, metadata, encrypted_data)
        size = fmt.total_size()
        print(f"{alg.name():<20} - {size} bytes")

    print("\n✓ All examples completed successfully!")


if __name__ == "__main__":
    main()
