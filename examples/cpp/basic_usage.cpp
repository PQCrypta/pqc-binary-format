/**
 * Basic usage example for PQC Binary Format C++ bindings
 * Demonstrates encryption format creation, serialization, and deserialization
 *
 * Compilation:
 *   First build the Rust library:
 *     cd ../.. && cargo build --release
 *   Then compile this example:
 *     g++ -std=c++17 -o basic_usage basic_usage.cpp \
 *         -I../../include \
 *         -L../../target/release \
 *         -lpqc_binary_format \
 *         -ldl -lpthread -lm
 *   Run with:
 *     LD_LIBRARY_PATH=../../target/release ./basic_usage
 */

#include "../../include/pqc_binary_format.h"
#include <iostream>
#include <vector>
#include <string>
#include <iomanip>
#include <cstring>

void printSeparator(const std::string& title = "") {
    std::cout << "\n";
    if (!title.empty()) {
        std::cout << title << "\n";
    }
    std::cout << std::string(60, '=') << "\n\n";
}

int main() {
    printSeparator("PQC Binary Format - C++ Basic Usage Example");

    // Print version information
    char* version = pqc_get_version();
    uint8_t binaryVersion = pqc_get_binary_version();
    std::cout << "Library version: " << version << "\n";
    std::cout << "Binary format version: " << static_cast<int>(binaryVersion) << "\n\n";
    pqc_free_string(version);

    // Step 1: Create encryption parameters
    std::cout << "Step 1: Creating encryption parameters...\n";
    std::vector<uint8_t> iv = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12};
    std::vector<uint8_t> tag(16, 0);
    std::cout << "  ✓ IV: " << iv.size() << " bytes\n";
    std::cout << "  ✓ Tag: " << tag.size() << " bytes\n\n";

    // Step 2: Create encrypted data
    std::cout << "Step 2: Preparing encrypted data...\n";
    std::vector<uint8_t> encryptedData = {1, 2, 3, 4, 5};
    std::cout << "  ✓ Data: " << encryptedData.size() << " bytes\n\n";

    // Step 3: Create PQC Binary Format
    std::cout << "Step 3: Creating PQC Binary Format...\n";
    PqcFormatHandle* format = pqc_format_new(
        PQC_ALGORITHM_HYBRID,
        iv.data(), iv.size(),
        tag.data(), tag.size(),
        encryptedData.data(), encryptedData.size()
    );

    if (!format) {
        std::cerr << "  ✗ Failed to create format\n";
        return 1;
    }
    std::cout << "  ✓ Format created successfully\n\n";

    // Step 4: Get algorithm information
    std::cout << "Step 4: Retrieving algorithm information...\n";
    uint16_t algId = pqc_format_get_algorithm_id(format);
    char* algName = pqc_format_get_algorithm_name(format);
    std::cout << "  ✓ Algorithm: " << algName << "\n";
    std::cout << "  ✓ Algorithm ID: 0x" << std::hex << std::setw(4)
              << std::setfill('0') << algId << std::dec << "\n\n";
    pqc_free_string(algName);

    // Step 5: Serialize to bytes
    std::cout << "Step 5: Serializing to binary format...\n";
    ByteBuffer serialized = pqc_format_to_bytes(format);
    if (!serialized.data) {
        std::cerr << "  ✗ Serialization failed\n";
        pqc_format_free(format);
        return 1;
    }
    size_t totalSize = pqc_format_get_total_size(format);
    std::cout << "  ✓ Serialized size: " << serialized.len << " bytes\n";
    std::cout << "  ✓ Total format size: " << totalSize << " bytes\n\n";

    // Step 6: Deserialize from bytes
    std::cout << "Step 6: Deserializing from binary format...\n";
    PqcFormatHandle* deserialized = pqc_format_from_bytes(
        serialized.data,
        serialized.len
    );

    if (!deserialized) {
        std::cerr << "  ✗ Deserialization failed\n";
        pqc_free_buffer(serialized);
        pqc_format_free(format);
        return 1;
    }

    char* deserAlgName = pqc_format_get_algorithm_name(deserialized);
    ByteBuffer deserData = pqc_format_get_data(deserialized);
    std::cout << "  ✓ Deserialized algorithm: " << deserAlgName << "\n";
    std::cout << "  ✓ Data length: " << deserData.len << " bytes\n\n";
    pqc_free_string(deserAlgName);

    // Step 7: Validate integrity
    std::cout << "Step 7: Validating format integrity...\n";
    int validationResult = pqc_format_validate(deserialized);
    if (validationResult == 0) {
        std::cout << "  ✓ Validation passed - checksum verified\n\n";
    } else {
        std::cerr << "  ✗ Validation failed\n\n";
        pqc_free_buffer(deserData);
        pqc_free_buffer(serialized);
        pqc_format_free(deserialized);
        pqc_format_free(format);
        return 1;
    }

    // Step 8: Verify roundtrip
    std::cout << "Step 8: Verifying roundtrip integrity...\n";
    if (deserData.len == encryptedData.size() &&
        std::memcmp(deserData.data, encryptedData.data(), encryptedData.size()) == 0) {
        std::cout << "  ✓ Roundtrip successful - data matches!\n\n";
    } else {
        std::cerr << "  ✗ Roundtrip failed - data mismatch!\n\n";
        pqc_free_buffer(deserData);
        pqc_free_buffer(serialized);
        pqc_format_free(deserialized);
        pqc_format_free(format);
        return 1;
    }

    printSeparator("✅ All steps completed successfully!");
    std::cout << "Summary:\n";
    std::cout << "  • Algorithm: Hybrid (0x" << std::hex << std::setw(4)
              << std::setfill('0') << algId << std::dec << ")\n";
    std::cout << "  • Data size: " << encryptedData.size() << " bytes\n";
    std::cout << "  • Serialized size: " << serialized.len << " bytes\n";
    std::cout << "  • Overhead: " << (serialized.len - encryptedData.size()) << " bytes\n\n";

    // Clean up
    pqc_free_buffer(deserData);
    pqc_free_buffer(serialized);
    pqc_format_free(deserialized);
    pqc_format_free(format);

    return 0;
}
