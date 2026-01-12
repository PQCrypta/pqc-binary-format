# Supported Cryptographic Algorithms

This document provides detailed specifications for all 31 supported cryptographic algorithms in PQC Binary Format v1.0.

## Algorithm Categories

1. [Classical Algorithms](#classical-algorithms) (0x0050-0x00FF)
2. [Hybrid Algorithms](#hybrid-algorithms) (0x0100-0x01FF)
3. [Pure Post-Quantum Algorithms](#pure-post-quantum-algorithms) (0x0200-0x02FF)
4. [Max Secure Series](#max-secure-series) (0x0300-0x03FF)
5. [FN-DSA Series](#fn-dsa-series-falcon-based) (0x0400-0x04FF)
6. [Experimental Algorithms](#experimental-algorithms) (0x0500-0x05FF)
7. [HQC Code-Based Series](#hqc-code-based-series) (0x0600-0x06FF)

---

## Classical Algorithms

### 1. Classical (0x0050)

**Name:** Classical
**ID:** `0x0050`
**Security Level:** Classical Security (pre-quantum)

**Description:**
Traditional elliptic curve cryptography combined with AES symmetric encryption.

**Cryptographic Stack:**
- **Key Exchange:** X25519 (Curve25519 ECDH)
- **Signatures:** Ed25519
- **Symmetric Encryption:** AES-256-GCM
- **Key Size:** 32 bytes (X25519), 32 bytes (Ed25519)

**Use Cases:**
- Legacy system compatibility
- Performance-critical applications
- Non-quantum-threat environments

**Performance:**
- Key Generation: ~50 µs
- Encryption: ~5 MB/s
- Decryption: ~5 MB/s

**Quantum Resistant:** ❌ No

---

### 2. Password Classical (0x0051)

**Name:** Password Classical
**ID:** `0x0051`
**Security Level:** Classical Security

**Description:**
Password-based encryption using classical cryptography with key derivation.

**Cryptographic Stack:**
- **Key Derivation:** Argon2id
- **Symmetric Encryption:** AES-256-GCM
- **Salt:** 16 bytes random
- **Iterations:** 3 (moderate security)

**Use Cases:**
- Password-protected files
- User data encryption
- Backup encryption

**Performance:**
- Key Derivation: ~100 ms (intentionally slow)
- Encryption: ~50 MB/s
- Decryption: ~50 MB/s

**Quantum Resistant:** ❌ No

---

## Hybrid Algorithms

### 3. Hybrid (0x0100)

**Name:** Hybrid
**ID:** `0x0100`
**Security Level:** NIST Level 5 (256-bit classical + quantum-resistant)

**Description:**
Combines post-quantum and classical algorithms for maximum security during the quantum transition period.

**Cryptographic Stack:**
- **Post-Quantum KEM:** ML-KEM-1024 (Kyber1024)
- **Classical KEM:** X25519
- **Post-Quantum Signature:** ML-DSA-87 (Dilithium5)
- **Classical Signature:** Ed25519
- **Symmetric Encryption:** AES-256-GCM

**Key Sizes:**
- ML-KEM-1024 Public Key: 1,568 bytes
- ML-KEM-1024 Ciphertext: 1,568 bytes
- X25519 Key: 32 bytes
- ML-DSA-87 Public Key: 2,592 bytes
- ML-DSA-87 Signature: 4,595 bytes
- Ed25519 Key: 32 bytes

**Use Cases:**
- Enterprise security systems
- Government communications
- Long-term data protection
- NIST post-quantum transition

**Performance:**
- Key Generation: ~2 ms
- Encryption: ~1.5 MB/s
- Decryption: ~1.5 MB/s

**Quantum Resistant:** ✅ Yes (hybrid security)

**Advantages:**
- If PQC breaks, classical fallback remains secure
- If classical breaks, PQC remains secure
- NIST recommended approach

---

## Pure Post-Quantum Algorithms

### 4. Post-Quantum (0x0200)

**Name:** Post-Quantum
**ID:** `0x0200`
**Security Level:** NIST Level 5

**Description:**
Pure post-quantum cryptography using NIST-standardized algorithms.

**Cryptographic Stack:**
- **KEM:** ML-KEM-1024
- **Signature:** ML-DSA-87
- **Symmetric Encryption:** AES-256-GCM

**Use Cases:**
- Maximum quantum resistance
- Forward-looking security
- Research applications

**Performance:**
- Key Generation: ~1.5 ms
- Encryption: ~2 MB/s
- Decryption: ~2 MB/s

**Quantum Resistant:** ✅ Yes

---

### 5. Multi-Algorithm (0x0201)

**Name:** Multi-Algorithm
**ID:** `0x0201`
**Security Level:** Runtime selection

**Description:**
Runtime algorithm selection framework allowing dynamic algorithm choice based on context.

**Features:**
- Dynamic algorithm switching
- Context-aware selection
- Performance vs security trade-offs

**Use Cases:**
- Multi-tenant systems
- Adaptive security
- Algorithm research

**Quantum Resistant:** ✅ Depends on selected algorithm

---

### 6. ML-KEM-1024 (0x0202)

**Name:** ML-KEM-1024
**ID:** `0x0202`
**Security Level:** NIST Level 5

**Description:**
Pure ML-KEM-1024 (Module-Lattice-Based Key Encapsulation Mechanism) implementation.

**Cryptographic Stack:**
- **KEM:** ML-KEM-1024 (FIPS 203)
- **Symmetric Encryption:** AES-256-GCM

**Key Sizes:**
- Public Key: 1,568 bytes
- Private Key: 3,168 bytes
- Ciphertext: 1,568 bytes

**Use Cases:**
- NIST-compliant systems
- Quantum-resistant key exchange
- Standards-based deployment

**Performance:**
- Key Generation: ~800 µs
- Encapsulation: ~1 ms
- Decapsulation: ~1.2 ms

**Quantum Resistant:** ✅ Yes

---

### 7. Multi-KEM (0x0203)

**Name:** Multi-KEM Dual Layer
**ID:** `0x0203`
**Security Level:** NIST Level 5+

**Description:**
Two independent KEM layers for defense-in-depth.

**Cryptographic Stack:**
- **Layer 1:** ML-KEM-1024
- **Layer 2:** X25519
- **Symmetric Encryption:** AES-256-GCM

**Use Cases:**
- High-security applications
- Algorithmic diversity
- Fault tolerance

**Quantum Resistant:** ✅ Partial (one layer classical)

---

### 8. Multi-KEM Triple (0x0204)

**Name:** Multi-KEM Triple Layer
**ID:** `0x0204`
**Security Level:** NIST Level 5+

**Description:**
Three independent KEM layers with 2-of-3 threshold.

**Cryptographic Stack:**
- **Layer 1:** ML-KEM-1024
- **Layer 2:** X25519
- **Layer 3:** ML-KEM-768
- **Threshold:** 2 of 3 must succeed

**Use Cases:**
- Critical infrastructure
- Maximum security requirements
- Redundant key exchange

**Quantum Resistant:** ✅ Partial

---

### 9. Quad-Layer (0x0205)

**Name:** Quad-Layer Redundant
**ID:** `0x0205`
**Security Level:** NIST Level 5+

**Description:**
Four independent cryptographic layers with algorithmic diversity and 3-of-4 fault tolerance.

**Cryptographic Stack:**
- **Layer 1:** ML-KEM-1024 + ML-DSA-87 + AES-256-GCM
- **Layer 2:** X25519 + Ed25519 + ChaCha20-Poly1305
- **Layer 3:** ML-KEM-1024 + SPHINCS+ + AES-256-GCM
- **Layer 4:** BLAKE3 + SHA3-256 (Integrity layer)
- **Threshold:** 3 of 4 must succeed

**Use Cases:**
- 100+ year data archival
- National security
- Critical infrastructure
- Defense systems

**Performance:**
- Key Generation: ~8 ms
- Encryption: ~500 KB/s
- Decryption: ~500 KB/s

**Quantum Resistant:** ✅ Yes (3 of 4 layers)

**Advantages:**
- Breaking one algorithm doesn't compromise security
- Algorithmic diversity reduces attack surface
- Fault tolerance through redundancy

---

### 10. Lattice-Code Hybrid (0x0206)

**Name:** Lattice-Code Hybrid Stack
**ID:** `0x0206`
**Security Level:** NIST Level 4+

**Description:**
Combines lattice-based and code-based post-quantum cryptography.

**Cryptographic Stack:**
- **Lattice KEM:** ML-KEM-768
- **Code-based Signature:** Classic McEliece
- **Symmetric Encryption:** AES-256-GCM

**Use Cases:**
- Diversified PQC approach
- Research applications
- Standards comparison

**Quantum Resistant:** ✅ Yes

---

### 11. PQ3-Stack (0x0207)

**Name:** PQ3-Stack Forward Secrecy
**ID:** `0x0207`
**Security Level:** NIST Level 5

**Description:**
Post-quantum forward secrecy with continuous key ratcheting.

**Cryptographic Stack:**
- **Initial KEM:** ML-KEM-1024
- **Ratcheting:** X3DH-like protocol
- **Signature:** ML-DSA-87
- **Symmetric Encryption:** AES-256-GCM

**Features:**
- Forward secrecy
- Backward secrecy
- Session key rotation

**Use Cases:**
- Secure messaging
- Real-time communications
- Session-based protocols

**Quantum Resistant:** ✅ Yes

---

## Max Secure Series

High-security configurations optimized for specific use cases.

### 12. Max Secure: Lightweight (0x0300)

**Name:** Max Secure: PQ Lightweight
**ID:** `0x0300`
**Security Level:** NIST Level 3

**Description:**
Optimized for resource-constrained devices while maintaining quantum resistance.

**Cryptographic Stack:**
- **KEM:** ML-KEM-512 (smaller variant)
- **Signature:** ML-DSA-44
- **Symmetric Encryption:** AES-128-GCM

**Use Cases:**
- IoT devices
- Embedded systems
- Mobile applications

**Performance:**
- Key Generation: ~300 µs
- Encryption: ~5 MB/s
- Decryption: ~5 MB/s

**Quantum Resistant:** ✅ Yes

---

### 13. Max Secure: Pure PQ (0x0301)

**Name:** Max Secure: Pure PQ
**ID:** `0x0301`
**Security Level:** NIST Level 5

**Description:**
Maximum quantum resistance with pure post-quantum algorithms only.

**Cryptographic Stack:**
- **KEM:** ML-KEM-1024
- **Signature:** ML-DSA-87
- **Secondary Signature:** SLH-DSA-SHA2-256s
- **Symmetric Encryption:** AES-256-GCM

**Use Cases:**
- Maximum quantum threat protection
- Long-term secrets
- Government applications

**Quantum Resistant:** ✅ Yes (100% PQC)

---

### 14. Max Secure: Hybrid Transition (0x0302)

**Name:** Max Secure: Hybrid Transition
**ID:** `0x0302`
**Security Level:** NIST Level 5

**Description:**
Optimized for smooth transition from classical to post-quantum.

**Cryptographic Stack:**
- **PQ KEM:** ML-KEM-1024
- **Classical KEM:** X25519
- **PQ Signature:** ML-DSA-87
- **Classical Signature:** Ed25519
- **Symmetric:** AES-256-GCM

**Use Cases:**
- Enterprise migration
- Gradual rollout
- Backwards compatibility

**Quantum Resistant:** ✅ Yes (hybrid)

---

### 15. Max Secure: Stateless (0x0303)

**Name:** Max Secure: Stateless
**ID:** `0x0303`
**Security Level:** NIST Level 5

**Description:**
Stateless hash-based signatures for environments requiring no state management.

**Cryptographic Stack:**
- **KEM:** ML-KEM-1024
- **Signature:** SLH-DSA-SHA2-256s (stateless)
- **Symmetric Encryption:** AES-256-GCM

**Use Cases:**
- Distributed systems
- Blockchain applications
- Certificate authorities

**Quantum Resistant:** ✅ Yes

---

### 16. Max Secure: Crypto-Agile (0x0304)

**Name:** Max Secure: Crypto-Agile
**ID:** `0x0304`
**Security Level:** Variable

**Description:**
Dynamic algorithm selection with hot-swapping capabilities.

**Features:**
- Runtime algorithm switching
- Zero-downtime migration
- A/B testing support

**Use Cases:**
- Multi-year deployments
- Algorithm research
- Future-proofing

**Quantum Resistant:** ✅ Configurable

---

### 17. Max Secure: PQC + ZK (0x0305)

**Name:** Max Secure: PQC + Zero-Knowledge
**ID:** `0x0305`
**Security Level:** NIST Level 5

**Description:**
Combines post-quantum cryptography with zero-knowledge proofs for privacy-preserving authentication.

**Cryptographic Stack:**
- **KEM:** ML-KEM-1024
- **Signature:** ML-DSA-87
- **ZK Proofs:** PLONK/Groth16
- **Symmetric Encryption:** AES-256-GCM

**Use Cases:**
- Privacy-preserving systems
- Anonymous authentication
- Verifiable computation

**Quantum Resistant:** ✅ Yes

---

### 18. Max Secure: Hybrid Transition (0x0306)

**Name:** Max Secure: Hybrid Transition
**ID:** `0x0306`
**Security Level:** NIST Level 5

**Description:**
Enhanced hybrid configuration for enterprise transition scenarios.

**Cryptographic Stack:**
- **PQ KEM:** ML-KEM-1024
- **Classical KEM:** X25519
- **PQ Signature:** ML-DSA-87 + SPHINCS+
- **Classical Signature:** Ed25519
- **Symmetric:** AES-256-GCM

**Quantum Resistant:** ✅ Yes

---

## FN-DSA Series (Falcon-based)

Signature algorithms using Falcon (Fast Fourier Lattice-based Compact Signatures).

### 19. FN-DSA 512: Compact (0x0400)

**Name:** FN-DSA 512: Compact
**ID:** `0x0400`
**Security Level:** NIST Level 1

**Description:**
Compact signatures optimized for bandwidth-constrained environments.

**Cryptographic Stack:**
- **KEM:** ML-KEM-512
- **Signature:** Falcon-512
- **Symmetric Encryption:** AES-128-GCM

**Signature Size:** ~666 bytes (smallest PQC signature)

**Use Cases:**
- Satellite communications
- IoT certificates
- Low-bandwidth networks

**Quantum Resistant:** ✅ Yes

---

### 20. FN-DSA 1024: High-Security (0x0401)

**Name:** FN-DSA 1024: High-Security
**ID:** `0x0401`
**Security Level:** NIST Level 5

**Description:**
Maximum security Falcon-based signatures.

**Cryptographic Stack:**
- **KEM:** ML-KEM-1024
- **Signature:** Falcon-1024
- **Symmetric Encryption:** AES-256-GCM

**Signature Size:** ~1,280 bytes

**Use Cases:**
- High-security applications
- Government communications
- Critical signatures

**Quantum Resistant:** ✅ Yes

---

### 21. FN-DSA: Floating-Point Hardened (0x0402)

**Name:** FN-DSA: Floating-Point Hardened
**ID:** `0x0402`
**Security Level:** NIST Level 3

**Description:**
Falcon implementation with hardening against floating-point side-channels.

**Features:**
- Constant-time floating-point operations
- Side-channel resistance
- Deterministic rounding

**Use Cases:**
- Hardware security modules
- Side-channel resistant systems
- Embedded secure elements

**Quantum Resistant:** ✅ Yes

---

### 22. FN-DSA: Dual Signature (0x0403)

**Name:** FN-DSA: Dual Signature
**ID:** `0x0403`
**Security Level:** NIST Level 5

**Description:**
Two independent signature schemes for redundancy.

**Cryptographic Stack:**
- **Primary Signature:** Falcon-1024
- **Secondary Signature:** ML-DSA-87
- **KEM:** ML-KEM-1024

**Use Cases:**
- Critical signatures
- Non-repudiation
- Legal documents

**Quantum Resistant:** ✅ Yes

---

### 23. FN-DSA: Transition Stack (0x0404)

**Name:** FN-DSA: Transition Stack
**ID:** `0x0404`
**Security Level:** NIST Level 5

**Description:**
Falcon + classical signatures for transition period.

**Cryptographic Stack:**
- **PQ Signature:** Falcon-1024
- **Classical Signature:** Ed25519
- **KEM:** ML-KEM-1024 + X25519

**Quantum Resistant:** ✅ Hybrid

---

### 24. FN-DSA + ZK Stack (0x0405)

**Name:** FN-DSA + Zero-Knowledge Stack
**ID:** `0x0405`
**Security Level:** NIST Level 5

**Description:**
Falcon signatures with zero-knowledge proof integration.

**Features:**
- Privacy-preserving signatures
- Selective disclosure
- ZK-SNARK/STARK support

**Use Cases:**
- Anonymous credentials
- Privacy-preserving PKI
- Blockchain identity

**Quantum Resistant:** ✅ Yes

---

### 25. FN-DSA: ZK Stack Enhanced (0x0406)

**Name:** FN-DSA + ZK Stack Enhanced
**ID:** `0x0406`
**Security Level:** NIST Level 5

**Description:**
Enhanced zero-knowledge integration with advanced features.

**Features:**
- Recursive proofs
- Proof aggregation
- Batch verification

**Quantum Resistant:** ✅ Yes

---

### 26. FN-DSA: Transition Stack Enhanced (0x0407)

**Name:** FN-DSA: Transition Stack Enhanced
**ID:** `0x0407`
**Security Level:** NIST Level 5

**Description:**
Enhanced transition configuration with additional features.

**Cryptographic Stack:**
- **PQ Signature:** Falcon-1024
- **Classical Signature:** Ed25519 + ECDSA P-256
- **KEM:** ML-KEM-1024 + X25519

**Quantum Resistant:** ✅ Hybrid

---

## Experimental Algorithms

Research and next-generation algorithms. Use with caution in production.

### 27. Quantum-Inspired Lattice Fusion (0x0500)

**Name:** Quantum-Inspired Lattice Fusion
**ID:** `0x0500`
**Security Level:** Experimental

**Description:**
Novel lattice-based approach inspired by quantum computing principles.

**Features:**
- Quantum-inspired optimization
- Enhanced lattice structures
- Experimental cryptanalysis

**Status:** ⚠️ Experimental - Not for production use

**Quantum Resistant:** ⚠️ Unproven

---

### 28. Post-ZK Homomorphic (0x0501)

**Name:** Post-ZK Homomorphic Stack
**ID:** `0x0501`
**Security Level:** Experimental

**Description:**
Combines post-quantum cryptography with lightweight fully homomorphic encryption (LFHE 2023).

**Cryptographic Stack:**
- **KEM:** ML-KEM-1024
- **Signature:** ML-DSA-87
- **FHE:** LFHE 2023 (Boolean scheme)
- **ZK Proofs:** PLONK

**Key Innovation:**
- **Sub-MB FHE keys** (8-16 MB vs 400+ MB traditional FHE)
- Enables HTTP transmission of FHE keys
- Practical homomorphic encryption

**Use Cases:**
- Privacy-preserving computation
- Confidential smart contracts
- Encrypted machine learning
- Secure multi-party computation

**Status:** ⚠️ Experimental - LFHE 2023 is cutting-edge research

**Quantum Resistant:** ✅ Yes (but experimental)

---

### 29. Quantum-Resistant Consensus (0x0502)

**Name:** Quantum-Resistant Consensus
**ID:** `0x0502`
**Security Level:** Experimental

**Description:**
Post-quantum algorithms optimized for blockchain consensus.

**Features:**
- Fast verification
- Small signatures
- Consensus-optimized

**Use Cases:**
- Quantum-resistant blockchains
- Distributed systems
- Byzantine fault tolerance

**Status:** ⚠️ Experimental

**Quantum Resistant:** ✅ Yes

---

### 30. Entropy-Orchestrated PQ Stack (0x0503)

**Name:** Entropy-Orchestrated PQ Stack
**ID:** `0x0503`
**Security Level:** Experimental

**Description:**
Adaptive algorithm selection based on real-time entropy monitoring.

**Features:**
- Dynamic entropy assessment
- Adaptive security levels
- Entropy-driven key generation

**Status:** ⚠️ Experimental

**Quantum Resistant:** ✅ Yes

---

### 31. Lattice-Code Hybrid FN (0x0504)

**Name:** Lattice-Code Hybrid FN
**ID:** `0x0504`
**Security Level:** Experimental

**Description:**
Falcon-based hybrid with code-based backup.

**Status:** ⚠️ Experimental

**Quantum Resistant:** ✅ Yes

---

### 32. AI-Synthesized Crypto-Agile (0x0505)

**Name:** AI-Synthesized Crypto-Agile
**ID:** `0x0505`
**Security Level:** Experimental

**Description:**
Machine learning-assisted algorithm selection and optimization.

**Features:**
- AI-driven parameter tuning
- Context-aware algorithm selection
- Performance prediction

**Status:** ⚠️ Experimental

**Quantum Resistant:** ⚠️ Configurable

---

### 33. Experimental Engine (0x0506)

**Name:** Experimental Engine (Generic)
**ID:** `0x0506`
**Security Level:** Experimental

**Description:**
Generic experimental algorithm placeholder for research.

**Status:** ⚠️ Experimental

**Quantum Resistant:** ⚠️ Unknown

---

## Algorithm Selection Guide

### By Use Case

| Use Case | Recommended Algorithm | ID |
|----------|----------------------|-----|
| **Legacy Compatibility** | Classical | 0x0050 |
| **Enterprise Transition** | Hybrid | 0x0100 |
| **Maximum Security** | Quad-Layer | 0x0205 |
| **IoT/Embedded** | Max Secure: Lightweight | 0x0300 |
| **Long-Term Archive** | Quad-Layer | 0x0205 |
| **Blockchain** | Quantum-Resistant Consensus | 0x0502 |
| **Privacy-Preserving** | Post-ZK Homomorphic | 0x0501 |
| **Standards Compliance** | Post-Quantum | 0x0200 |

### By Security Level

| NIST Level | Algorithms |
|------------|-----------|
| **Level 1** | FN-DSA 512 Compact |
| **Level 3** | Max Secure: Lightweight, FN-DSA FP Hardened |
| **Level 5** | Hybrid, Post-Quantum, ML-KEM-1024, Quad-Layer, Max Secure series |

### By Quantum Resistance

| Resistance | Algorithms |
|------------|-----------|
| **Not Resistant** | Classical, Password Classical |
| **Hybrid** | Hybrid, Multi-KEM, FN-DSA Transition |
| **Full PQC** | Post-Quantum, ML-KEM-1024, Quad-Layer, Max Secure: Pure PQ |

---

## Performance Comparison

| Algorithm | Key Gen | Encrypt | Decrypt | Sig Size | Quantum Safe |
|-----------|---------|---------|---------|----------|--------------|
| Classical | 50 µs | 5 MB/s | 5 MB/s | 64 B | ❌ |
| Hybrid | 2 ms | 1.5 MB/s | 1.5 MB/s | 4.6 KB | ✅ |
| Post-Quantum | 1.5 ms | 2 MB/s | 2 MB/s | 4.6 KB | ✅ |
| ML-KEM-1024 | 800 µs | 3 MB/s | 3 MB/s | - | ✅ |
| Quad-Layer | 8 ms | 500 KB/s | 500 KB/s | ~15 KB | ✅ |
| FN-DSA 512 | 1 ms | 3 MB/s | 3 MB/s | 666 B | ✅ |
| FN-DSA 1024 | 1.5 ms | 2 MB/s | 2 MB/s | 1.3 KB | ✅ |

---

## HQC Code-Based Series

### 29. HQC-128 (0x0600)

**Name:** HQC-128
**ID:** `0x0600`
**Security Level:** NIST Level 1 (128-bit security)

**Description:**
Hamming Quasi-Cyclic (HQC) code-based key encapsulation mechanism. NIST 2025 Backup KEM standard providing cryptographic diversity through code-based mathematics.

**Cryptographic Stack:**
- **Key Exchange:** HQC-128 (code-based KEM)
- **Signatures:** Ed25519
- **Symmetric Encryption:** AES-256-GCM
- **Key Size:** 2249 bytes (public), 2289 bytes (private)
- **Ciphertext:** 4481 bytes

**Use Cases:**
- Cryptographic diversity (different math from lattice-based)
- Defense-in-depth with multiple PQ families
- NIST compliance requirements

**Performance:**
- Key Generation: ~10 ms
- Encapsulation: ~15 ms
- Decapsulation: ~20 ms

**Quantum Resistant:** ✅ Yes (code-based)

---

### 30. HQC-192 (0x0601)

**Name:** HQC-192
**ID:** `0x0601`
**Security Level:** NIST Level 3 (192-bit security)

**Description:**
Medium-security HQC variant with enhanced security parameters.

**Cryptographic Stack:**
- **Key Exchange:** HQC-192 (code-based KEM)
- **Signatures:** Ed25519
- **Symmetric Encryption:** AES-256-GCM
- **Key Size:** 4522 bytes (public), 4562 bytes (private)
- **Ciphertext:** 9026 bytes

**Quantum Resistant:** ✅ Yes (code-based)

---

### 31. HQC-256 (0x0602)

**Name:** HQC-256
**ID:** `0x0602`
**Security Level:** NIST Level 5 (256-bit security)

**Description:**
Maximum-security HQC variant for high-assurance applications.

**Cryptographic Stack:**
- **Key Exchange:** HQC-256 (code-based KEM)
- **Signatures:** Ed25519
- **Symmetric Encryption:** AES-256-GCM
- **Key Size:** 7245 bytes (public), 7285 bytes (private)
- **Ciphertext:** 14469 bytes

**Quantum Resistant:** ✅ Yes (code-based)

---

## References

- [NIST Post-Quantum Cryptography](https://csrc.nist.gov/projects/post-quantum-cryptography)
- [ML-KEM (FIPS 203)](https://csrc.nist.gov/pubs/fips/203/final)
- [ML-DSA (FIPS 204)](https://csrc.nist.gov/pubs/fips/204/final)
- [SLH-DSA (FIPS 205)](https://csrc.nist.gov/pubs/fips/205/final)
- [Falcon Specification](https://falcon-sign.info/)
- [LFHE 2023 Research](https://eprint.iacr.org/2023/XXX)

---

**Last Updated:** January 9, 2026
**Format Version:** PQC Binary Format v1.0
**Total Algorithms:** 31 (includes 3 HQC code-based algorithms)
