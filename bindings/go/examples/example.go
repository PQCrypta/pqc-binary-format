package main

import (
	"encoding/hex"
	"fmt"
	"log"

	pqc "github.com/PQCrypta/pqcrypta-community/bindings/go"
)

func main() {
	fmt.Println("=== PQC Binary Format v1.0 - Pure Go Example ===\n")

	// Example 1: Create and serialize a Hybrid algorithm format
	fmt.Println("Example 1: Creating Hybrid Algorithm Format")
	fmt.Println("--------------------------------------------")

	metadata := []byte(`{
		"algorithm": "Hybrid",
		"kem": "ML-KEM-1024",
		"signature": "ML-DSA-87",
		"encryption": "AES-256-GCM",
		"timestamp": "2026-01-10T00:00:00Z"
	}`)

	// Simulated encrypted data (in practice, this would be real encrypted data)
	encryptedData := []byte("This is simulated encrypted data from a hybrid post-quantum algorithm")

	// Create new format
	format := pqc.New(pqc.AlgorithmHybrid, metadata, encryptedData)
	fmt.Printf("Created format: %s\n", format)
	fmt.Printf("Quantum Resistant: %v\n", format.IsQuantumResistant())
	fmt.Printf("Total Size: %d bytes\n\n", format.Size())

	// Serialize to binary
	binary, err := format.Serialize()
	if err != nil {
		log.Fatalf("Serialization failed: %v", err)
	}

	fmt.Printf("Serialized to %d bytes\n", len(binary))
	fmt.Printf("First 64 bytes (hex): %s\n\n", hex.EncodeToString(binary[:min(64, len(binary))]))

	// Example 2: Parse the serialized data
	fmt.Println("Example 2: Parsing Binary Format")
	fmt.Println("---------------------------------")

	parsed, err := pqc.Parse(binary)
	if err != nil {
		log.Fatalf("Parsing failed: %v", err)
	}

	fmt.Printf("Parsed format: %s\n", parsed)
	fmt.Printf("Algorithm: %s (ID: 0x%04X)\n", parsed.AlgorithmName(), parsed.AlgorithmID)
	fmt.Printf("Version: %d\n", parsed.Version)
	fmt.Printf("Metadata length: %d bytes\n", len(parsed.Metadata))
	fmt.Printf("Data length: %d bytes\n", len(parsed.Data))
	fmt.Printf("Checksum valid: %v\n\n", parsed.VerifyChecksum())

	// Example 3: Demonstrate all algorithm types
	fmt.Println("Example 3: All Supported Algorithms")
	fmt.Println("------------------------------------")

	algorithms := []struct {
		id   uint16
		desc string
	}{
		{pqc.AlgorithmClassical, "Classical cryptography (X25519 + Ed25519)"},
		{pqc.AlgorithmHybrid, "Hybrid classical + post-quantum"},
		{pqc.AlgorithmPostQuantum, "Pure post-quantum (ML-KEM + ML-DSA)"},
		{pqc.AlgorithmMlKem1024, "ML-KEM-1024 only"},
		{pqc.AlgorithmMultiKem, "Multiple KEM layers"},
		{pqc.AlgorithmQuadLayer, "Four-layer redundant security"},
		{pqc.AlgorithmMaxSecurePurePQ, "Maximum security pure PQ"},
		{pqc.AlgorithmFnDsa1024Security, "FN-DSA high-security signatures"},
		{pqc.AlgorithmQuantumLatticeFusion, "Quantum-inspired lattice fusion"},
	}

	for _, alg := range algorithms {
		f := pqc.New(alg.id, nil, nil)
		fmt.Printf("• %s - %s (QR: %v)\n", f.AlgorithmName(), alg.desc, f.IsQuantumResistant())
	}

	// Example 4: Performance demonstration
	fmt.Println("\nExample 4: Performance Test")
	fmt.Println("----------------------------")

	testData := make([]byte, 1024*1024) // 1 MB of data
	testMeta := []byte(`{"test":"performance"}`)

	perfFormat := pqc.New(pqc.AlgorithmPostQuantum, testMeta, testData)
	perfBinary, _ := perfFormat.Serialize()

	fmt.Printf("Serialized 1 MB of data: %d bytes total\n", len(perfBinary))
	fmt.Printf("Overhead: %d bytes (%.2f%%)\n",
		len(perfBinary)-len(testData),
		float64(len(perfBinary)-len(testData))/float64(len(testData))*100)

	// Parse it back
	reparsed, err := pqc.Parse(perfBinary)
	if err != nil {
		log.Fatalf("Reparsing failed: %v", err)
	}

	fmt.Printf("Checksum verification: %v\n", reparsed.VerifyChecksum())

	// Example 5: Cross-platform interoperability
	fmt.Println("\nExample 5: Cross-Platform Interoperability")
	fmt.Println("-------------------------------------------")
	fmt.Println("This binary format can be:")
	fmt.Println("  • Created in Go and read in Python")
	fmt.Println("  • Created in Rust and read in JavaScript")
	fmt.Println("  • Created in C++ and read in Go")
	fmt.Println("  • Created in Python and read in Rust")
	fmt.Println("All using the same standardized binary format!")

	fmt.Println("\n=== All Examples Completed Successfully ===")
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}
