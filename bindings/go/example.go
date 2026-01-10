package main

import (
	"fmt"
	"log"

	pqc "github.com/PQCrypta/pqcrypta-community/bindings/go"
)

func main() {
	fmt.Println("PQC Binary Format Go Example")
	fmt.Println("==================================================")
	fmt.Printf("Library version: %s\n", pqc.GetVersion())
	fmt.Printf("Binary format version: %d\n", pqc.GetBinaryVersion())

	// Example 1: Basic encryption format
	fmt.Println("\n1. Basic Encryption Format")
	fmt.Println("--------------------------------------------------")

	// Create encryption parameters
	iv := []byte{1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12} // 12-byte nonce
	tag := make([]byte, 16)                             // 16-byte auth tag
	encryptedData := []byte{1, 2, 3, 4, 5}

	// Create PQC Binary Format
	format, err := pqc.NewPqcBinaryFormat(pqc.AlgorithmHybrid, iv, tag, encryptedData)
	if err != nil {
		log.Fatalf("Failed to create format: %v", err)
	}
	defer format.Free()

	fmt.Printf("Algorithm: %s (ID: %d)\n", format.GetAlgorithmName(), format.GetAlgorithmID())

	// Serialize to bytes
	serialized, err := format.ToBytes()
	if err != nil {
		log.Fatalf("Failed to serialize: %v", err)
	}
	fmt.Printf("Serialized size: %d bytes\n", len(serialized))
	fmt.Printf("Total format size: %d bytes\n", format.GetTotalSize())

	// Deserialize from bytes
	deserialized, err := pqc.FromBytes(serialized)
	if err != nil {
		log.Fatalf("Failed to deserialize: %v", err)
	}
	defer deserialized.Free()

	fmt.Printf("Deserialized algorithm: %s\n", deserialized.GetAlgorithmName())
	data := deserialized.GetData()
	fmt.Printf("Data length: %d bytes\n", len(data))

	// Validate
	if err := deserialized.Validate(); err != nil {
		log.Fatalf("Validation failed: %v", err)
	}
	fmt.Println("✓ Validation passed")

	// Example 2: Format with KEM parameters
	fmt.Println("\n2. Format with KEM Parameters")
	fmt.Println("--------------------------------------------------")

	// Create KEM parameters
	kemPublicKey := make([]byte, 1568) // ML-KEM-1024 public key
	kemCiphertext := make([]byte, 1568) // Encapsulated key

	formatWithKEM, err := pqc.NewPqcBinaryFormatWithKEM(
		pqc.AlgorithmHybrid,
		iv,
		tag,
		kemPublicKey,
		kemCiphertext,
		encryptedData,
	)
	if err != nil {
		log.Fatalf("Failed to create format with KEM: %v", err)
	}
	defer formatWithKEM.Free()

	serializedWithKEM, err := formatWithKEM.ToBytes()
	if err != nil {
		log.Fatalf("Failed to serialize with KEM: %v", err)
	}
	fmt.Printf("Serialized size with KEM: %d bytes\n", len(serializedWithKEM))

	// Example 3: Algorithm comparison
	fmt.Println("\n3. Algorithm Comparison")
	fmt.Println("--------------------------------------------------")

	algorithms := map[string]uint16{
		"Classical":    pqc.AlgorithmClassical,
		"Hybrid":       pqc.AlgorithmHybrid,
		"Post-Quantum": pqc.AlgorithmPostQuantum,
		"ML-KEM-1024":  pqc.AlgorithmMlKem1024,
	}

	for name, algID := range algorithms {
		fmt := pqc.NewPqcBinaryFormat(algID, iv, tag, encryptedData)
		if fmt != nil {
			size := fmt.GetTotalSize()
			fmt.Printf("%-20s - %d bytes\n", name, size)
			fmt.Free()
		}
	}

	// Example 4: Cross-platform interoperability
	fmt.Println("\n4. Cross-Platform Interoperability")
	fmt.Println("--------------------------------------------------")
	fmt.Println("This binary format can be:")
	fmt.Println("  • Created in Go and read in Python")
	fmt.Println("  • Created in Rust and read in JavaScript")
	fmt.Println("  • Created in C++ and read in Go")
	fmt.Println("All using the same standardized binary format!")

	fmt.Println("\n✓ All examples completed successfully!")
}
