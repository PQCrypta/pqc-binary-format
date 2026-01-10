package pqc_binary_format

/*
#cgo LDFLAGS: -L../../target/release -lpqc_binary_format
#include "../../include/pqc_binary_format.h"
#include <stdlib.h>
*/
import "C"
import (
	"errors"
	"unsafe"
)

// Algorithm IDs
const (
	AlgorithmClassical         = 0x0050
	AlgorithmPasswordClassical = 0x0051
	AlgorithmHybrid            = 0x0100
	AlgorithmPostQuantum       = 0x0200
	AlgorithmMlKem1024         = 0x0202
	AlgorithmMultiKem          = 0x0203
	AlgorithmMultiKemTriple    = 0x0204
	AlgorithmQuadLayer         = 0x0205
	AlgorithmPq3Stack          = 0x0207
	AlgorithmLatticeCodeHybrid = 0x0208
)

// PqcBinaryFormat represents a PQC binary format structure
type PqcBinaryFormat struct {
	handle *C.PqcFormatHandle
}

// NewPqcBinaryFormat creates a new PQC binary format structure
func NewPqcBinaryFormat(algorithmID uint16, iv, tag, data []byte) (*PqcBinaryFormat, error) {
	if len(iv) == 0 || len(tag) == 0 || len(data) == 0 {
		return nil, errors.New("iv, tag, and data must not be empty")
	}

	handle := C.pqc_format_new(
		C.uint16_t(algorithmID),
		(*C.uchar)(unsafe.Pointer(&iv[0])),
		C.size_t(len(iv)),
		(*C.uchar)(unsafe.Pointer(&tag[0])),
		C.size_t(len(tag)),
		(*C.uchar)(unsafe.Pointer(&data[0])),
		C.size_t(len(data)),
	)

	if handle == nil {
		return nil, errors.New("failed to create PQC binary format")
	}

	return &PqcBinaryFormat{handle: handle}, nil
}

// NewPqcBinaryFormatWithKEM creates a new PQC binary format with KEM parameters
func NewPqcBinaryFormatWithKEM(algorithmID uint16, iv, tag, kemPublicKey, kemCiphertext, data []byte) (*PqcBinaryFormat, error) {
	if len(iv) == 0 || len(tag) == 0 || len(kemPublicKey) == 0 || len(kemCiphertext) == 0 || len(data) == 0 {
		return nil, errors.New("all parameters must not be empty")
	}

	handle := C.pqc_format_new_with_kem(
		C.uint16_t(algorithmID),
		(*C.uchar)(unsafe.Pointer(&iv[0])),
		C.size_t(len(iv)),
		(*C.uchar)(unsafe.Pointer(&tag[0])),
		C.size_t(len(tag)),
		(*C.uchar)(unsafe.Pointer(&kemPublicKey[0])),
		C.size_t(len(kemPublicKey)),
		(*C.uchar)(unsafe.Pointer(&kemCiphertext[0])),
		C.size_t(len(kemCiphertext)),
		(*C.uchar)(unsafe.Pointer(&data[0])),
		C.size_t(len(data)),
	)

	if handle == nil {
		return nil, errors.New("failed to create PQC binary format with KEM")
	}

	return &PqcBinaryFormat{handle: handle}, nil
}

// ToBytes serializes the PQC binary format to bytes
func (p *PqcBinaryFormat) ToBytes() ([]byte, error) {
	if p.handle == nil {
		return nil, errors.New("invalid handle")
	}

	buffer := C.pqc_format_to_bytes(p.handle)
	defer C.pqc_free_buffer(buffer)

	if buffer.data == nil {
		return nil, errors.New("serialization failed")
	}

	// Copy C buffer to Go slice
	result := C.GoBytes(unsafe.Pointer(buffer.data), C.int(buffer.len))
	return result, nil
}

// FromBytes deserializes PQC binary format from bytes
func FromBytes(data []byte) (*PqcBinaryFormat, error) {
	if len(data) == 0 {
		return nil, errors.New("data must not be empty")
	}

	handle := C.pqc_format_from_bytes(
		(*C.uchar)(unsafe.Pointer(&data[0])),
		C.size_t(len(data)),
	)

	if handle == nil {
		return nil, errors.New("deserialization failed")
	}

	return &PqcBinaryFormat{handle: handle}, nil
}

// GetAlgorithmID returns the algorithm ID
func (p *PqcBinaryFormat) GetAlgorithmID() uint16 {
	if p.handle == nil {
		return 0
	}
	return uint16(C.pqc_format_get_algorithm_id(p.handle))
}

// GetAlgorithmName returns the algorithm name
func (p *PqcBinaryFormat) GetAlgorithmName() string {
	if p.handle == nil {
		return ""
	}

	cname := C.pqc_format_get_algorithm_name(p.handle)
	if cname == nil {
		return ""
	}
	defer C.pqc_free_string(cname)

	return C.GoString(cname)
}

// GetData returns the encrypted data
func (p *PqcBinaryFormat) GetData() []byte {
	if p.handle == nil {
		return nil
	}

	buffer := C.pqc_format_get_data(p.handle)
	defer C.pqc_free_buffer(buffer)

	if buffer.data == nil {
		return nil
	}

	return C.GoBytes(unsafe.Pointer(buffer.data), C.int(buffer.len))
}

// Validate validates the format structure
func (p *PqcBinaryFormat) Validate() error {
	if p.handle == nil {
		return errors.New("invalid handle")
	}

	result := C.pqc_format_validate(p.handle)
	if result != 0 {
		return errors.New("validation failed")
	}

	return nil
}

// GetTotalSize returns the total serialized size
func (p *PqcBinaryFormat) GetTotalSize() int {
	if p.handle == nil {
		return 0
	}
	return int(C.pqc_format_get_total_size(p.handle))
}

// Free frees the PQC binary format handle
func (p *PqcBinaryFormat) Free() {
	if p.handle != nil {
		C.pqc_format_free(p.handle)
		p.handle = nil
	}
}

// GetVersion returns the library version
func GetVersion() string {
	cversion := C.pqc_get_version()
	if cversion == nil {
		return ""
	}
	defer C.pqc_free_string(cversion)

	return C.GoString(cversion)
}

// GetBinaryVersion returns the binary format version
func GetBinaryVersion() uint8 {
	return uint8(C.pqc_get_binary_version())
}
