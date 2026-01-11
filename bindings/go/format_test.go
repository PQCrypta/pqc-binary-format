package pqcbinaryformat

import (
	"bytes"
	"crypto/sha256"
	"encoding/binary"
	"testing"
)

func TestNew(t *testing.T) {
	metadata := []byte(`{"key":"value"}`)
	data := []byte("encrypted data here")

	format := New(AlgorithmHybrid, metadata, data)

	if !bytes.Equal(format.Magic, MagicBytes) {
		t.Errorf("Magic bytes mismatch: got %v, want %v", format.Magic, MagicBytes)
	}

	if format.Version != Version1 {
		t.Errorf("Version mismatch: got %d, want %d", format.Version, Version1)
	}

	if format.AlgorithmID != AlgorithmHybrid {
		t.Errorf("AlgorithmID mismatch: got 0x%04X, want 0x%04X", format.AlgorithmID, AlgorithmHybrid)
	}

	if !bytes.Equal(format.Metadata, metadata) {
		t.Errorf("Metadata mismatch")
	}

	if !bytes.Equal(format.Data, data) {
		t.Errorf("Data mismatch")
	}
}

func TestSerializeAndParse(t *testing.T) {
	tests := []struct {
		name        string
		algorithmID uint16
		metadata    []byte
		data        []byte
	}{
		{
			name:        "Classical algorithm",
			algorithmID: AlgorithmClassical,
			metadata:    []byte(`{"encryption":"AES-256-GCM"}`),
			data:        []byte("test encrypted data"),
		},
		{
			name:        "Hybrid algorithm",
			algorithmID: AlgorithmHybrid,
			metadata:    []byte(`{"kem":"ML-KEM-1024","sig":"ML-DSA-87"}`),
			data:        []byte("quantum-resistant encrypted data"),
		},
		{
			name:        "Empty metadata",
			algorithmID: AlgorithmPostQuantum,
			metadata:    []byte{},
			data:        []byte("data with no metadata"),
		},
		{
			name:        "Large data",
			algorithmID: AlgorithmMlKem1024,
			metadata:    []byte(`{"test":"value"}`),
			data:        make([]byte, 10000),
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			// Create format
			original := New(tt.algorithmID, tt.metadata, tt.data)

			// Serialize
			serialized, err := original.Serialize()
			if err != nil {
				t.Fatalf("Serialize failed: %v", err)
			}

			// Parse
			parsed, err := Parse(serialized)
			if err != nil {
				t.Fatalf("Parse failed: %v", err)
			}

			// Compare
			if !bytes.Equal(parsed.Magic, original.Magic) {
				t.Errorf("Magic mismatch")
			}

			if parsed.Version != original.Version {
				t.Errorf("Version mismatch")
			}

			if parsed.AlgorithmID != original.AlgorithmID {
				t.Errorf("AlgorithmID mismatch: got 0x%04X, want 0x%04X", parsed.AlgorithmID, original.AlgorithmID)
			}

			if !bytes.Equal(parsed.Metadata, original.Metadata) {
				t.Errorf("Metadata mismatch")
			}

			if !bytes.Equal(parsed.Data, original.Data) {
				t.Errorf("Data mismatch")
			}

			if parsed.Checksum != original.Checksum {
				t.Errorf("Checksum mismatch")
			}
		})
	}
}

func TestParseInvalidMagic(t *testing.T) {
	data := make([]byte, 100)
	copy(data, []byte{0x00, 0x00, 0x00, 0x00}) // Invalid magic

	_, err := Parse(data)
	if err != ErrInvalidMagic {
		t.Errorf("Expected ErrInvalidMagic, got %v", err)
	}
}

func TestParseInvalidVersion(t *testing.T) {
	data := make([]byte, 100)
	copy(data, MagicBytes)
	data[4] = 0xFF // Invalid version

	_, err := Parse(data)
	if err != ErrInvalidVersion {
		t.Errorf("Expected ErrInvalidVersion, got %v", err)
	}
}

func TestParseBufferTooSmall(t *testing.T) {
	data := make([]byte, 10) // Too small

	_, err := Parse(data)
	if err != ErrBufferTooSmall {
		t.Errorf("Expected ErrBufferTooSmall, got %v", err)
	}
}

func TestParseInvalidChecksum(t *testing.T) {
	// Create valid format
	format := New(AlgorithmHybrid, []byte("metadata"), []byte("data"))
	serialized, _ := format.Serialize()

	// Corrupt checksum
	serialized[len(serialized)-1] ^= 0xFF

	_, err := Parse(serialized)
	if err != ErrInvalidChecksum {
		t.Errorf("Expected ErrInvalidChecksum, got %v", err)
	}
}

func TestVerifyChecksum(t *testing.T) {
	format := New(AlgorithmPostQuantum, []byte("test metadata"), []byte("test data"))

	// Calculate correct checksum
	buf := new(bytes.Buffer)
	buf.Write(format.Magic)
	buf.WriteByte(format.Version)

	algID := make([]byte, 2)
	binary.BigEndian.PutUint16(algID, format.AlgorithmID)
	buf.Write(algID)

	metaLen := make([]byte, 4)
	binary.BigEndian.PutUint32(metaLen, uint32(len(format.Metadata)))
	buf.Write(metaLen)

	dataLen := make([]byte, 8)
	binary.BigEndian.PutUint64(dataLen, uint64(len(format.Data)))
	buf.Write(dataLen)

	buf.Write(format.Metadata)
	buf.Write(format.Data)

	format.Checksum = sha256.Sum256(buf.Bytes())

	if !format.VerifyChecksum() {
		t.Error("Valid checksum failed verification")
	}

	// Corrupt checksum
	format.Checksum[0] ^= 0xFF

	if format.VerifyChecksum() {
		t.Error("Invalid checksum passed verification")
	}
}

func TestAlgorithmName(t *testing.T) {
	tests := []struct {
		algorithmID uint16
		expected    string
	}{
		{AlgorithmClassical, "Classical"},
		{AlgorithmHybrid, "Hybrid"},
		{AlgorithmPostQuantum, "Post-Quantum"},
		{AlgorithmMlKem1024, "ML-KEM-1024"},
		{AlgorithmMultiKem, "Multi-KEM"},
		{AlgorithmQuadLayer, "Quad-Layer"},
		{AlgorithmMaxSecurePurePQ, "Max-Secure-Pure-PQ"},
		{AlgorithmFnDsa1024Security, "FN-DSA-1024-Security"},
		{AlgorithmQuantumLatticeFusion, "Quantum-Lattice-Fusion"},
		{0x9999, "Unknown-0x9999"},
	}

	for _, tt := range tests {
		format := New(tt.algorithmID, nil, nil)
		got := format.AlgorithmName()
		if got != tt.expected {
			t.Errorf("AlgorithmName(0x%04X) = %s, want %s", tt.algorithmID, got, tt.expected)
		}
	}
}

func TestIsQuantumResistant(t *testing.T) {
	tests := []struct {
		algorithmID uint16
		expected    bool
	}{
		{AlgorithmClassical, false},
		{AlgorithmPasswordClassical, false},
		{AlgorithmHybrid, true},
		{AlgorithmPostQuantum, true},
		{AlgorithmMlKem1024, true},
		{AlgorithmMultiKem, true},
		{AlgorithmMaxSecurePurePQ, true},
	}

	for _, tt := range tests {
		format := New(tt.algorithmID, nil, nil)
		got := format.IsQuantumResistant()
		if got != tt.expected {
			t.Errorf("IsQuantumResistant(0x%04X) = %v, want %v", tt.algorithmID, got, tt.expected)
		}
	}
}

func TestSize(t *testing.T) {
	tests := []struct {
		name         string
		metadataLen  int
		dataLen      int
		expectedSize int
	}{
		{"Empty", 0, 0, 51},
		{"With metadata", 100, 0, 151},
		{"With data", 0, 500, 551},
		{"With both", 100, 500, 651},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			metadata := make([]byte, tt.metadataLen)
			data := make([]byte, tt.dataLen)
			format := New(AlgorithmHybrid, metadata, data)

			if format.Size() != tt.expectedSize {
				t.Errorf("Size() = %d, want %d", format.Size(), tt.expectedSize)
			}
		})
	}
}

func TestString(t *testing.T) {
	format := New(AlgorithmHybrid, []byte("metadata"), []byte("data"))
	str := format.String()

	expected := "PqcBinaryFormat{Version: 1, Algorithm: Hybrid (0x0100), MetadataLen: 8, DataLen: 4}"
	if str != expected {
		t.Errorf("String() = %s, want %s", str, expected)
	}
}

func BenchmarkSerialize(b *testing.B) {
	metadata := []byte(`{"algorithm":"hybrid","encryption":"AES-256-GCM"}`)
	data := make([]byte, 1024)
	format := New(AlgorithmHybrid, metadata, data)

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		_, _ = format.Serialize()
	}
}

func BenchmarkParse(b *testing.B) {
	metadata := []byte(`{"algorithm":"hybrid","encryption":"AES-256-GCM"}`)
	data := make([]byte, 1024)
	format := New(AlgorithmHybrid, metadata, data)
	serialized, _ := format.Serialize()

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		_, _ = Parse(serialized)
	}
}

func BenchmarkVerifyChecksum(b *testing.B) {
	format := New(AlgorithmHybrid, []byte("metadata"), make([]byte, 1024))
	format.Serialize()

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		format.VerifyChecksum()
	}
}
