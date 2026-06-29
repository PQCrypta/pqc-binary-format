// Basic usage example for PQC Binary Format Go bindings
// Demonstrates encryption format creation, serialization, and deserialization
//
// Usage:
//   cd examples/go && go run basic_usage.go

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
	fmt.Printf("Magic bytes: % X\n", pqc.MagicBytes)
	fmt.Printf("Binary format version: %d\n", pqc.Version1)
	fmt.Println()

	// Step 1: Create encryption parameters (IV + tag travel in the metadata block)
	fmt.Println("Step 1: Creating encryption parameters...")
	iv := []byte{1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12} // 12-byte nonce
	tag := make([]byte, 16)                              // 16-byte authentication tag
	metadata := append(append([]byte{}, iv...), tag...)
	fmt.Printf("  ✓ IV: %d bytes\n", len(iv))
	fmt.Printf("  ✓ Tag: %d bytes\n", len(tag))
	fmt.Printf("  ✓ Metadata: %d bytes\n", len(metadata))
	fmt.Println()

	// Step 2: Create encrypted data
	fmt.Println("Step 2: Preparing encrypted data...")
	encryptedData := []byte{1, 2, 3, 4, 5}
	fmt.Printf("  ✓ Data: %d bytes\n", len(encryptedData))
	fmt.Println()

	// Step 3: Create PQC Binary Format
	fmt.Println("Step 3: Creating PQC Binary Format...")
	format := pqc.New(pqc.AlgorithmHybrid, metadata, encryptedData)
	fmt.Println("  ✓ Format created successfully")
	fmt.Println()

	// Step 4: Get algorithm information
	fmt.Println("Step 4: Retrieving algorithm information...")
	algName := format.AlgorithmName()
	algID := format.AlgorithmID
	fmt.Printf("  ✓ Algorithm: %s\n", algName)
	fmt.Printf("  ✓ Algorithm ID: 0x%04X\n", algID)
	fmt.Printf("  ✓ Quantum-resistant: %t\n", format.IsQuantumResistant())
	fmt.Println()

	// Step 5: Serialize to bytes
	fmt.Println("Step 5: Serializing to binary format...")
	serialized, err := format.Serialize()
	if err != nil {
		log.Fatalf("Failed to serialize: %v", err)
	}
	fmt.Printf("  ✓ Serialized size: %d bytes\n", len(serialized))
	fmt.Printf("  ✓ Total format size: %d bytes\n", format.Size())
	fmt.Println()

	// Step 6: Deserialize from bytes
	fmt.Println("Step 6: Deserializing from binary format...")
	deserialized, err := pqc.Parse(serialized)
	if err != nil {
		log.Fatalf("Failed to deserialize: %v", err)
	}
	fmt.Printf("  ✓ Deserialized algorithm: %s\n", deserialized.AlgorithmName())
	fmt.Printf("  ✓ Data length: %d bytes\n", len(deserialized.Data))
	fmt.Println()

	// Step 7: Validate integrity
	fmt.Println("Step 7: Validating format integrity...")
	if !deserialized.VerifyChecksum() {
		log.Fatal("Validation failed: checksum mismatch")
	}
	fmt.Println("  ✓ Validation passed - checksum verified")
	fmt.Println()

	// Step 8: Verify roundtrip
	fmt.Println("Step 8: Verifying roundtrip integrity...")
	if bytes.Equal(format.Data, deserialized.Data) {
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
