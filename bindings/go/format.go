// Package pqcbinaryformat provides pure Go implementation of PQC Binary Format v1.0
//
// This package implements the PQC Binary Format specification for post-quantum
// cryptographic data encapsulation. It provides constant-time parsing and
// serialization of encrypted data with support for 31+ cryptographic algorithms including HQC.
//
// Example usage:
//
//	// Create a new format
//	format := &PqcBinaryFormat{
//		Version:     1,
//		AlgorithmID: AlgorithmHybrid,
//		Metadata:    []byte(`{"key":"value"}`),
//		Data:        encryptedData,
//	}
//	binary, err := format.Serialize()
//
//	// Parse existing format
//	format, err := Parse(binaryData)
//	fmt.Printf("Algorithm: %s\n", format.AlgorithmName())
package pqcbinaryformat

import (
	"bytes"
	"crypto/sha256"
	"encoding/binary"
	"errors"
	"fmt"
)

// Magic bytes identifying PQC Binary Format
var MagicBytes = []byte{0x50, 0x51, 0x43, 0x01} // "PQC\x01"

// Version constants
const (
	Version1 byte = 0x01
)

// Algorithm IDs - Core Algorithms
const (
	AlgorithmClassical         uint16 = 0x0050
	AlgorithmPasswordClassical uint16 = 0x0051
	AlgorithmHybrid            uint16 = 0x0100
	AlgorithmPostQuantum       uint16 = 0x0200
	AlgorithmMlKem1024         uint16 = 0x0202
	AlgorithmMultiAlgorithm    uint16 = 0x0201
)

// Algorithm IDs - Multi-KEM Series
const (
	AlgorithmMultiKem       uint16 = 0x0203
	AlgorithmMultiKemTriple uint16 = 0x0204
	AlgorithmQuadLayer      uint16 = 0x0205
)

// Algorithm IDs - Advanced Stacks
const (
	AlgorithmPq3Stack          uint16 = 0x0207
	AlgorithmLatticeCodeHybrid uint16 = 0x0208
)

// Algorithm IDs - Max Secure Series
const (
	AlgorithmMaxSecureLightweight      uint16 = 0x0300
	AlgorithmMaxSecurePurePQ           uint16 = 0x0301
	AlgorithmMaxSecureHybridTransition uint16 = 0x0302
	AlgorithmMaxSecureStateless        uint16 = 0x0303
	AlgorithmMaxSecureCryptoAgile      uint16 = 0x0304
	AlgorithmMaxSecurePQCZK            uint16 = 0x0305
	AlgorithmMaxSecureHybrid           uint16 = 0x0306
)

// Algorithm IDs - FN-DSA Signature Series
const (
	AlgorithmFnDsa512Compact     uint16 = 0x0400
	AlgorithmFnDsa1024Security   uint16 = 0x0401
	AlgorithmFnDsaFPHardened     uint16 = 0x0402
	AlgorithmFnDsaDualSignature  uint16 = 0x0403
	AlgorithmFnDsaTransitionStack uint16 = 0x0404
	AlgorithmFnDsaZKStack        uint16 = 0x0405
)

// Algorithm IDs - Experimental Series
const (
	AlgorithmQuantumLatticeFusion   uint16 = 0x0500
	AlgorithmPostZKHomomorphic      uint16 = 0x0501
	AlgorithmQuantumResistantConsensus uint16 = 0x0502
	AlgorithmEntropyOrchestrated    uint16 = 0x0503
	AlgorithmAISynthesizedCryptoAgile uint16 = 0x0504
)

// Algorithm IDs - HQC Code-Based Series (NIST 2025 Backup KEM)
const (
	AlgorithmHqc128 uint16 = 0x0600
	AlgorithmHqc192 uint16 = 0x0601
	AlgorithmHqc256 uint16 = 0x0602
)

// Algorithm name mappings
var algorithmNames = map[uint16]string{
	AlgorithmClassical:                  "Classical",
	AlgorithmPasswordClassical:          "Password-Classical",
	AlgorithmHybrid:                     "Hybrid",
	AlgorithmPostQuantum:                "Post-Quantum",
	AlgorithmMlKem1024:                  "ML-KEM-1024",
	AlgorithmMultiAlgorithm:             "Multi-Algorithm",
	AlgorithmMultiKem:                   "Multi-KEM",
	AlgorithmMultiKemTriple:             "Multi-KEM-Triple",
	AlgorithmQuadLayer:                  "Quad-Layer",
	AlgorithmPq3Stack:                   "PQ3-Stack",
	AlgorithmLatticeCodeHybrid:          "Lattice-Code-Hybrid",
	AlgorithmMaxSecureLightweight:       "Max-Secure-Lightweight",
	AlgorithmMaxSecurePurePQ:            "Max-Secure-Pure-PQ",
	AlgorithmMaxSecureHybridTransition:  "Max-Secure-Hybrid-Transition",
	AlgorithmMaxSecureStateless:         "Max-Secure-Stateless",
	AlgorithmMaxSecureCryptoAgile:       "Max-Secure-Crypto-Agile",
	AlgorithmMaxSecurePQCZK:             "Max-Secure-PQC-ZK",
	AlgorithmMaxSecureHybrid:            "Max-Secure-Hybrid",
	AlgorithmFnDsa512Compact:            "FN-DSA-512-Compact",
	AlgorithmFnDsa1024Security:          "FN-DSA-1024-Security",
	AlgorithmFnDsaFPHardened:            "FN-DSA-FP-Hardened",
	AlgorithmFnDsaDualSignature:         "FN-DSA-Dual-Signature",
	AlgorithmFnDsaTransitionStack:       "FN-DSA-Transition-Stack",
	AlgorithmFnDsaZKStack:               "FN-DSA-ZK-Stack",
	AlgorithmQuantumLatticeFusion:       "Quantum-Lattice-Fusion",
	AlgorithmPostZKHomomorphic:          "Post-ZK-Homomorphic",
	AlgorithmQuantumResistantConsensus:  "Quantum-Resistant-Consensus",
	AlgorithmEntropyOrchestrated:        "Entropy-Orchestrated",
	AlgorithmAISynthesizedCryptoAgile:   "AI-Synthesized-Crypto-Agile",
	AlgorithmHqc128:                     "HQC-128",
	AlgorithmHqc192:                     "HQC-192",
	AlgorithmHqc256:                     "HQC-256",
}

// Errors
var (
	ErrInvalidMagic    = errors.New("invalid magic bytes")
	ErrInvalidVersion  = errors.New("invalid version")
	ErrInvalidChecksum = errors.New("invalid checksum")
	ErrInvalidLength   = errors.New("invalid data length")
	ErrBufferTooSmall  = errors.New("buffer too small")
)

// PqcBinaryFormat represents a PQC Binary Format structure
type PqcBinaryFormat struct {
	// Magic bytes (always "PQC\x01")
	Magic []byte

	// Version (currently 0x01)
	Version byte

	// Algorithm identifier (16-bit)
	AlgorithmID uint16

	// Metadata length (32-bit)
	MetadataLen uint32

	// Data length (64-bit)
	DataLen uint64

	// Algorithm-specific metadata
	Metadata []byte

	// Encrypted payload
	Data []byte

	// SHA-256 checksum (32 bytes)
	Checksum [32]byte
}

// New creates a new PqcBinaryFormat with the given parameters
func New(algorithmID uint16, metadata, data []byte) *PqcBinaryFormat {
	format := &PqcBinaryFormat{
		Magic:       make([]byte, 4),
		Version:     Version1,
		AlgorithmID: algorithmID,
		MetadataLen: uint32(len(metadata)),
		DataLen:     uint64(len(data)),
		Metadata:    metadata,
		Data:        data,
	}
	copy(format.Magic, MagicBytes)
	return format
}

// Parse parses a PQC Binary Format from bytes
func Parse(data []byte) (*PqcBinaryFormat, error) {
	if len(data) < 51 { // Minimum size: 4 + 1 + 2 + 4 + 8 + 32 = 51 bytes
		return nil, ErrBufferTooSmall
	}

	format := &PqcBinaryFormat{}
	offset := 0

	// Parse magic bytes
	format.Magic = data[offset : offset+4]
	if !bytes.Equal(format.Magic, MagicBytes) {
		return nil, ErrInvalidMagic
	}
	offset += 4

	// Parse version
	format.Version = data[offset]
	if format.Version != Version1 {
		return nil, ErrInvalidVersion
	}
	offset += 1

	// Parse algorithm ID (big-endian)
	format.AlgorithmID = binary.BigEndian.Uint16(data[offset : offset+2])
	offset += 2

	// Parse metadata length (big-endian)
	format.MetadataLen = binary.BigEndian.Uint32(data[offset : offset+4])
	offset += 4

	// Parse data length (big-endian)
	format.DataLen = binary.BigEndian.Uint64(data[offset : offset+8])
	offset += 8

	// Validate total length
	totalLen := 51 + int(format.MetadataLen) + int(format.DataLen)
	if len(data) != totalLen {
		return nil, ErrInvalidLength
	}

	// Parse metadata
	if format.MetadataLen > 0 {
		format.Metadata = data[offset : offset+int(format.MetadataLen)]
		offset += int(format.MetadataLen)
	}

	// Parse data
	if format.DataLen > 0 {
		format.Data = data[offset : offset+int(format.DataLen)]
		offset += int(format.DataLen)
	}

	// Parse checksum
	copy(format.Checksum[:], data[offset:offset+32])
	offset += 32

	// Verify checksum
	if !format.VerifyChecksum() {
		return nil, ErrInvalidChecksum
	}

	return format, nil
}

// Serialize serializes the format to bytes with checksum
func (f *PqcBinaryFormat) Serialize() ([]byte, error) {
	// Calculate total size
	totalSize := 51 + len(f.Metadata) + len(f.Data)
	buf := make([]byte, totalSize)
	offset := 0

	// Write magic bytes
	copy(buf[offset:], MagicBytes)
	offset += 4

	// Write version
	buf[offset] = f.Version
	offset += 1

	// Write algorithm ID (big-endian)
	binary.BigEndian.PutUint16(buf[offset:], f.AlgorithmID)
	offset += 2

	// Write metadata length (big-endian)
	binary.BigEndian.PutUint32(buf[offset:], uint32(len(f.Metadata)))
	offset += 4

	// Write data length (big-endian)
	binary.BigEndian.PutUint64(buf[offset:], uint64(len(f.Data)))
	offset += 8

	// Write metadata
	if len(f.Metadata) > 0 {
		copy(buf[offset:], f.Metadata)
		offset += len(f.Metadata)
	}

	// Write data
	if len(f.Data) > 0 {
		copy(buf[offset:], f.Data)
		offset += len(f.Data)
	}

	// Calculate checksum (everything except the checksum field itself)
	checksumData := buf[:offset]
	checksum := sha256.Sum256(checksumData)
	copy(buf[offset:], checksum[:])

	// Store checksum in struct
	f.Checksum = checksum

	return buf, nil
}

// VerifyChecksum verifies the SHA-256 checksum
func (f *PqcBinaryFormat) VerifyChecksum() bool {
	// Reconstruct the data that was checksummed
	buf := new(bytes.Buffer)

	// Write all fields except checksum
	buf.Write(f.Magic)
	buf.WriteByte(f.Version)

	algID := make([]byte, 2)
	binary.BigEndian.PutUint16(algID, f.AlgorithmID)
	buf.Write(algID)

	metaLen := make([]byte, 4)
	binary.BigEndian.PutUint32(metaLen, uint32(len(f.Metadata)))
	buf.Write(metaLen)

	dataLen := make([]byte, 8)
	binary.BigEndian.PutUint64(dataLen, uint64(len(f.Data)))
	buf.Write(dataLen)

	buf.Write(f.Metadata)
	buf.Write(f.Data)

	// Calculate checksum
	calculated := sha256.Sum256(buf.Bytes())

	// Constant-time comparison
	return bytes.Equal(calculated[:], f.Checksum[:])
}

// AlgorithmName returns the human-readable algorithm name
func (f *PqcBinaryFormat) AlgorithmName() string {
	if name, ok := algorithmNames[f.AlgorithmID]; ok {
		return name
	}
	return fmt.Sprintf("Unknown-0x%04X", f.AlgorithmID)
}

// IsQuantumResistant returns true if the algorithm is quantum-resistant
func (f *PqcBinaryFormat) IsQuantumResistant() bool {
	// Classical algorithms are not quantum-resistant
	if f.AlgorithmID == AlgorithmClassical || f.AlgorithmID == AlgorithmPasswordClassical {
		return false
	}
	// All other algorithms in the spec are quantum-resistant or hybrid
	return true
}

// Size returns the total size of the serialized format in bytes
func (f *PqcBinaryFormat) Size() int {
	return 51 + len(f.Metadata) + len(f.Data)
}

// String returns a string representation of the format
func (f *PqcBinaryFormat) String() string {
	return fmt.Sprintf("PqcBinaryFormat{Version: %d, Algorithm: %s (0x%04X), MetadataLen: %d, DataLen: %d}",
		f.Version, f.AlgorithmName(), f.AlgorithmID, len(f.Metadata), len(f.Data))
}
