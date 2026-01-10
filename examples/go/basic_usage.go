// Basic usage example for PQC Binary Format Go bindings
// Demonstrates encryption format creation, serialization, and deserialization
//
// Usage:
//   First build the Rust library:
//     cd ../.. && cargo build --release
//   Then run this example:
//     cd examples/go && go run basic_usage.go

package main

import (
	"bytes"
	"fmt"
	"log"
	"strings"

	pqc "github.com/PQCrypta/pqcrypta-community/bindings/go"
)

func main() {
	fmt.Println(strings.Repeat("=", 60))
	fmt.Println("PQC Binary Format - Go Basic Usage Example")
	fmt.Println(strings.Repeat("=", 60))
	fmt.Println()

	// Print version information
	fmt.Printf("Library version: %s\n", pqc.GetVersion())
	fmt.Printf("Binary format version: %d\n", pqc.GetBinaryVersion())
	fmt.Println()

	// Step 1: Create encryption parameters
	fmt.Println("Step 1: Creating encryption parameters...")
	iv := []byte{1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12} // 12-byte nonce
	tag := make([]byte, 16)                              // 16-byte authentication tag
	fmt.Printf("  ✓ IV: %d bytes\n", len(iv))
	fmt.Printf("  ✓ Tag: %d bytes\n", len(tag))
	fmt.Println()

	// Step 2: Create encrypted data
	fmt.Println("Step 2: Preparing encrypted data...")
	encryptedData := []byte{1, 2, 3, 4, 5}
	fmt.Printf("  ✓ Data: %d bytes\n", len(encryptedData))
	fmt.Println()

	// Step 3: Create PQC Binary Format
	fmt.Println("Step 3: Creating PQC Binary Format...")
	format, err := pqc.NewPqcBinaryFormat(pqc.AlgorithmHybrid, iv, tag, encryptedData)
	if err != nil {
		log.Fatalf("Failed to create format: %v", err)
	}
	defer format.Free()
	fmt.Println("  ✓ Format created successfully")
	fmt.Println()

	// Step 4: Get algorithm information
	fmt.Println("Step 4: Retrieving algorithm information...")
	algName := format.GetAlgorithmName()
	algID := format.GetAlgorithmID()
	fmt.Printf("  ✓ Algorithm: %s\n", algName)
	fmt.Printf("  ✓ Algorithm ID: 0x%04X\n", algID)
	fmt.Println()

	// Step 5: Serialize to bytes
	fmt.Println("Step 5: Serializing to binary format...")
	serialized, err := format.ToBytes()
	if err != nil {
		log.Fatalf("Failed to serialize: %v", err)
	}
	totalSize := format.GetTotalSize()
	fmt.Printf("  ✓ Serialized size: %d bytes\n", len(serialized))
	fmt.Printf("  ✓ Total format size: %d bytes\n", totalSize)
	fmt.Println()

	// Step 6: Deserialize from bytes
	fmt.Println("Step 6: Deserializing from binary format...")
	deserialized, err := pqc.FromBytes(serialized)
	if err != nil {
		log.Fatalf("Failed to deserialize: %v", err)
	}
	defer deserialized.Free()

	deserAlgName := deserialized.GetAlgorithmName()
	deserData := deserialized.GetData()
	fmt.Printf("  ✓ Deserialized algorithm: %s\n", deserAlgName)
	fmt.Printf("  ✓ Data length: %d bytes\n", len(deserData))
	fmt.Println()

	// Step 7: Validate integrity
	fmt.Println("Step 7: Validating format integrity...")
	if err := deserialized.Validate(); err != nil {
		log.Fatalf("Validation failed: %v", err)
	}
	fmt.Println("  ✓ Validation passed - checksum verified")
	fmt.Println()

	// Step 8: Verify roundtrip
	fmt.Println("Step 8: Verifying roundtrip integrity...")
	originalData := format.GetData()
	recoveredData := deserialized.GetData()

	if bytes.Equal(originalData, recoveredData) {
		fmt.Println("  ✓ Roundtrip successful - data matches!")
	} else {
		log.Fatal("  ✗ Roundtrip failed - data mismatch!")
	}
	fmt.Println()

	fmt.Println(strings.Repeat("=", 60))
	fmt.Println("✅ All steps completed successfully!")
	fmt.Println(strings.Repeat("=", 60))
	fmt.Println()
	fmt.Println("Summary:")
	fmt.Printf("  • Algorithm: %s (0x%04X)\n", algName, algID)
	fmt.Printf("  • Data size: %d bytes\n", len(encryptedData))
	fmt.Printf("  • Serialized size: %d bytes\n", len(serialized))
	fmt.Printf("  • Overhead: %d bytes\n", len(serialized)-len(encryptedData))
	fmt.Println()
}
