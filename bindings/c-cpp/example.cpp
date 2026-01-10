/**
 * Example usage of PQC Binary Format C/C++ bindings
 */

#include "../../include/pqc_binary_format.h"
#include <iostream>
#include <vector>
#include <cstring>
#include <iomanip>

void print_separator(const std::string& title = "") {
    std::cout << "\n";
    if (!title.empty()) {
        std::cout << title << "\n";
    }
    std::cout << std::string(50, '-') << "\n";
}

void print_buffer(const char* label, const PqcByteBuffer& buffer) {
    std::cout << label << ": " << buffer.len << " bytes\n";
}

int main() {
    std::cout << "PQC Binary Format C++ Example\n";
    std::cout << std::string(50, '=') << "\n";

    // Get version information
    char* version = pqc_get_version();
    uint8_t binary_version = pqc_get_binary_version();
    std::cout << "Library version: " << version << "\n";
    std::cout << "Binary format version: " << static_cast<int>(binary_version) << "\n";
    pqc_free_string(version);

    // Example 1: Basic encryption format
    print_separator("1. Basic Encryption Format");

    // Create encryption parameters
    std::vector<uint8_t> iv = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12}; // 12-byte nonce
    std::vector<uint8_t> tag(16, 0); // 16-byte auth tag
    std::vector<uint8_t> encrypted_data = {1, 2, 3, 4, 5};

    // Create PQC Binary Format
    PqcFormatHandle* format = pqc_format_new(
        PQC_ALGORITHM_HYBRID,
        iv.data(),
        iv.size(),
        tag.data(),
        tag.size(),
        encrypted_data.data(),
        encrypted_data.size()
    );

    if (!format) {
        std::cerr << "Failed to create format\n";
        return 1;
    }

    // Get algorithm info
    uint16_t alg_id = pqc_format_get_algorithm_id(format);
    char* alg_name = pqc_format_get_algorithm_name(format);
    std::cout << "Algorithm: " << alg_name << " (ID: " << alg_id << ")\n";
    pqc_free_string(alg_name);

    // Serialize to bytes
    PqcByteBuffer serialized = pqc_format_to_bytes(format);
    if (!serialized.data) {
        std::cerr << "Serialization failed\n";
        pqc_format_free(format);
        return 1;
    }
    print_buffer("Serialized size", serialized);

    size_t total_size = pqc_format_get_total_size(format);
    std::cout << "Total format size: " << total_size << " bytes\n";

    // Deserialize from bytes
    PqcFormatHandle* deserialized = pqc_format_from_bytes(
        serialized.data,
        serialized.len
    );

    if (!deserialized) {
        std::cerr << "Deserialization failed\n";
        pqc_free_buffer(serialized);
        pqc_format_free(format);
        return 1;
    }

    char* deser_alg_name = pqc_format_get_algorithm_name(deserialized);
    std::cout << "Deserialized algorithm: " << deser_alg_name << "\n";
    pqc_free_string(deser_alg_name);

    PqcByteBuffer data = pqc_format_get_data(deserialized);
    print_buffer("Data length", data);
    pqc_free_buffer(data);

    // Validate
    int validation_result = pqc_format_validate(deserialized);
    if (validation_result == 0) {
        std::cout << "✓ Validation passed\n";
    } else {
        std::cout << "✗ Validation failed\n";
    }

    // Clean up
    pqc_free_buffer(serialized);
    pqc_format_free(deserialized);

    // Example 2: Format with KEM parameters
    print_separator("2. Format with KEM Parameters");

    std::vector<uint8_t> kem_public_key(1568, 0); // ML-KEM-1024 public key
    std::vector<uint8_t> kem_ciphertext(1568, 0); // Encapsulated key

    PqcFormatHandle* format_with_kem = pqc_format_new_with_kem(
        PQC_ALGORITHM_HYBRID,
        iv.data(),
        iv.size(),
        tag.data(),
        tag.size(),
        kem_public_key.data(),
        kem_public_key.size(),
        kem_ciphertext.data(),
        kem_ciphertext.size(),
        encrypted_data.data(),
        encrypted_data.size()
    );

    if (!format_with_kem) {
        std::cerr << "Failed to create format with KEM\n";
        pqc_format_free(format);
        return 1;
    }

    PqcByteBuffer serialized_with_kem = pqc_format_to_bytes(format_with_kem);
    if (serialized_with_kem.data) {
        print_buffer("Serialized size with KEM", serialized_with_kem);
        pqc_free_buffer(serialized_with_kem);
    }

    pqc_format_free(format_with_kem);

    // Example 3: Algorithm comparison
    print_separator("3. Algorithm Comparison");

    struct AlgorithmInfo {
        const char* name;
        uint16_t id;
    };

    AlgorithmInfo algorithms[] = {
        {"Classical",    PQC_ALGORITHM_CLASSICAL},
        {"Hybrid",       PQC_ALGORITHM_HYBRID},
        {"Post-Quantum", PQC_ALGORITHM_POST_QUANTUM},
        {"ML-KEM-1024",  PQC_ALGORITHM_ML_KEM_1024},
    };

    for (const auto& algo : algorithms) {
        PqcFormatHandle* fmt = pqc_format_new(
            algo.id,
            iv.data(),
            iv.size(),
            tag.data(),
            tag.size(),
            encrypted_data.data(),
            encrypted_data.size()
        );

        if (fmt) {
            size_t size = pqc_format_get_total_size(fmt);
            std::cout << std::left << std::setw(20) << algo.name
                      << " - " << size << " bytes\n";
            pqc_format_free(fmt);
        }
    }

    // Example 4: Cross-platform interoperability
    print_separator("4. Cross-Platform Interoperability");
    std::cout << "This binary format can be:\n";
    std::cout << "  • Created in C++ and read in Python\n";
    std::cout << "  • Created in Rust and read in Go\n";
    std::cout << "  • Created in JavaScript and read in C++\n";
    std::cout << "All using the same standardized binary format!\n";

    // Clean up
    pqc_format_free(format);

    std::cout << "\n✓ All examples completed successfully!\n";

    return 0;
}
