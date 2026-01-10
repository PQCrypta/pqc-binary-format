/**
 * Basic usage example for PQC Binary Format C bindings
 * Demonstrates encryption format creation, serialization, and deserialization
 *
 * Compilation:
 *   First build the Rust library:
 *     cd ../.. && cargo build --release
 *   Then compile this example:
 *     gcc -o basic_usage basic_usage.c \
 *         -I../../include \
 *         -L../../target/release \
 *         -lpqc_binary_format \
 *         -ldl -lpthread -lm
 *   Run with:
 *     LD_LIBRARY_PATH=../../target/release ./basic_usage
 */

#include "../../include/pqc_binary_format.h"
#include <stdio.h>
#include <stdint.h>
#include <string.h>

void print_separator(const char* title) {
    printf("\n");
    if (title && strlen(title) > 0) {
        printf("%s\n", title);
    }
    for (int i = 0; i < 60; i++) printf("=");
    printf("\n\n");
}

int main(void) {
    print_separator("PQC Binary Format - C Basic Usage Example");

    // Print version information
    char* version = pqc_get_version();
    uint8_t binary_version = pqc_get_binary_version();
    printf("Library version: %s\n", version);
    printf("Binary format version: %d\n\n", binary_version);
    pqc_free_string(version);

    // Step 1: Create encryption parameters
    printf("Step 1: Creating encryption parameters...\n");
    uint8_t iv[] = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12};
    uint8_t tag[16] = {0};
    printf("  ✓ IV: %zu bytes\n", sizeof(iv));
    printf("  ✓ Tag: %zu bytes\n\n", sizeof(tag));

    // Step 2: Create encrypted data
    printf("Step 2: Preparing encrypted data...\n");
    uint8_t encrypted_data[] = {1, 2, 3, 4, 5};
    printf("  ✓ Data: %zu bytes\n\n", sizeof(encrypted_data));

    // Step 3: Create PQC Binary Format
    printf("Step 3: Creating PQC Binary Format...\n");
    PqcFormatHandle* format = pqc_format_new(
        PQC_ALGORITHM_HYBRID,
        iv, sizeof(iv),
        tag, sizeof(tag),
        encrypted_data, sizeof(encrypted_data)
    );

    if (!format) {
        fprintf(stderr, "  ✗ Failed to create format\n");
        return 1;
    }
    printf("  ✓ Format created successfully\n\n");

    // Step 4: Get algorithm information
    printf("Step 4: Retrieving algorithm information...\n");
    uint16_t alg_id = pqc_format_get_algorithm_id(format);
    char* alg_name = pqc_format_get_algorithm_name(format);
    printf("  ✓ Algorithm: %s\n", alg_name);
    printf("  ✓ Algorithm ID: 0x%04X\n\n", alg_id);
    pqc_free_string(alg_name);

    // Step 5: Serialize to bytes
    printf("Step 5: Serializing to binary format...\n");
    ByteBuffer serialized = pqc_format_to_bytes(format);
    if (!serialized.data) {
        fprintf(stderr, "  ✗ Serialization failed\n");
        pqc_format_free(format);
        return 1;
    }
    size_t total_size = pqc_format_get_total_size(format);
    printf("  ✓ Serialized size: %zu bytes\n", serialized.len);
    printf("  ✓ Total format size: %zu bytes\n\n", total_size);

    // Step 6: Deserialize from bytes
    printf("Step 6: Deserializing from binary format...\n");
    PqcFormatHandle* deserialized = pqc_format_from_bytes(
        serialized.data,
        serialized.len
    );

    if (!deserialized) {
        fprintf(stderr, "  ✗ Deserialization failed\n");
        pqc_free_buffer(serialized);
        pqc_format_free(format);
        return 1;
    }

    char* deser_alg_name = pqc_format_get_algorithm_name(deserialized);
    ByteBuffer deser_data = pqc_format_get_data(deserialized);
    printf("  ✓ Deserialized algorithm: %s\n", deser_alg_name);
    printf("  ✓ Data length: %zu bytes\n\n", deser_data.len);
    pqc_free_string(deser_alg_name);

    // Step 7: Validate integrity
    printf("Step 7: Validating format integrity...\n");
    int validation_result = pqc_format_validate(deserialized);
    if (validation_result == 0) {
        printf("  ✓ Validation passed - checksum verified\n\n");
    } else {
        fprintf(stderr, "  ✗ Validation failed\n\n");
        pqc_free_buffer(deser_data);
        pqc_free_buffer(serialized);
        pqc_format_free(deserialized);
        pqc_format_free(format);
        return 1;
    }

    // Step 8: Verify roundtrip
    printf("Step 8: Verifying roundtrip integrity...\n");
    if (deser_data.len == sizeof(encrypted_data) &&
        memcmp(deser_data.data, encrypted_data, sizeof(encrypted_data)) == 0) {
        printf("  ✓ Roundtrip successful - data matches!\n\n");
    } else {
        fprintf(stderr, "  ✗ Roundtrip failed - data mismatch!\n\n");
        pqc_free_buffer(deser_data);
        pqc_free_buffer(serialized);
        pqc_format_free(deserialized);
        pqc_format_free(format);
        return 1;
    }

    print_separator("✅ All steps completed successfully!");
    printf("Summary:\n");
    printf("  • Algorithm: Hybrid (0x%04X)\n", alg_id);
    printf("  • Data size: %zu bytes\n", sizeof(encrypted_data));
    printf("  • Serialized size: %zu bytes\n", serialized.len);
    printf("  • Overhead: %zu bytes\n\n", serialized.len - sizeof(encrypted_data));

    // Clean up
    pqc_free_buffer(deser_data);
    pqc_free_buffer(serialized);
    pqc_format_free(deserialized);
    pqc_format_free(format);

    return 0;
}
