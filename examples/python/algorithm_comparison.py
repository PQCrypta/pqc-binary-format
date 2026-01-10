#!/usr/bin/env python3
"""
Algorithm comparison example for PQC Binary Format Python bindings
Demonstrates different cryptographic algorithms and their characteristics
"""

from pqc_binary_format import Algorithm, EncParameters, PqcMetadata, PqcBinaryFormat


def main():
    print("=" * 70)
    print("PQC Binary Format - Algorithm Comparison Example")
    print("=" * 70)
    print()

    # Common encryption parameters
    iv = bytes([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12])
    tag = bytes([0] * 16)
    enc_params = EncParameters(iv=iv, tag=tag)
    metadata = PqcMetadata(
        enc_params=enc_params,
        kem_params=None,
        sig_params=None,
        compression_params=None,
    )
    encrypted_data = bytes([1, 2, 3, 4, 5])

    # List of algorithms to test
    algorithms = [
        ("classical", "Classical (X25519 + Ed25519 + AES-256-GCM)"),
        ("hybrid", "Hybrid (ML-KEM-1024 + X25519 + ML-DSA-87 + Ed25519)"),
        ("post-quantum", "Post-Quantum (ML-KEM-1024 + ML-DSA-87)"),
        ("ml-kem-1024", "ML-KEM-1024 (Pure ML-KEM + AES-256-GCM)"),
        ("multi-algorithm", "Multi-Algorithm (Runtime Selection)"),
    ]

    print(f"{'Algorithm':<20} {'ID':<8} {'Size':<10} {'Description'}")
    print("-" * 70)

    for alg_name, description in algorithms:
        try:
            # Create algorithm
            algorithm = Algorithm(alg_name)

            # Create format
            pqc_format = PqcBinaryFormat(algorithm, metadata, encrypted_data)

            # Serialize
            serialized = pqc_format.to_bytes()

            # Display results
            print(
                f"{algorithm.name():<20} "
                f"0x{algorithm.id():04X}   "
                f"{len(serialized):<10} "
                f"{description}"
            )

        except Exception as e:
            print(f"{alg_name:<20} ERROR    N/A        {str(e)}")

    print("-" * 70)
    print()
    print("Key Observations:")
    print("  • All algorithms use the same binary format structure")
    print("  • Metadata overhead is consistent across algorithms")
    print("  • Algorithm selection doesn't affect serialization size (for same data)")
    print("  • Cross-platform compatibility guaranteed by format specification")
    print()
    print("✅ Algorithm comparison complete!")
    print()


if __name__ == "__main__":
    main()
