//! Algorithm identifiers for supported post-quantum cryptographic algorithms.

use serde::{Deserialize, Serialize};

/// Algorithm identifier type
pub type AlgorithmId = u16;

/// Supported cryptographic algorithms with unique identifiers
///
/// Each algorithm has a unique 16-bit identifier used in the binary format.
/// Identifiers are grouped by algorithm family for easy categorization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u16)]
pub enum Algorithm {
    // Classical algorithms (0x0050-0x00FF)
    /// Classical cryptography: X25519 + Ed25519 + AES-256-GCM
    Classical = 0x0050,
    /// Password-based classical encryption
    PasswordClassical = 0x0051,

    // Hybrid algorithms (0x0100-0x01FF)
    /// Hybrid: ML-KEM-1024 + X25519 + ML-DSA-87 + Ed25519 + AES-256-GCM
    Hybrid = 0x0100,

    // Pure post-quantum algorithms (0x0200-0x02FF)
    /// Post-quantum: ML-KEM-1024 + ML-DSA-87 + AES-256-GCM
    PostQuantum = 0x0200,
    /// Multi-algorithm runtime selection
    MultiAlgorithm = 0x0201,
    /// ML-KEM-1024 with AES-256-GCM
    MlKem1024 = 0x0202,
    /// Multi-KEM dual layer
    MultiKem = 0x0203,
    /// Multi-KEM triple layer
    MultiKemTriple = 0x0204,
    /// Quad-layer redundant security
    QuadLayer = 0x0205,
    /// Lattice-Code hybrid stack
    LatticeCodeHybrid = 0x0206,
    /// PQ3-Stack with forward secrecy
    Pq3Stack = 0x0207,

    // Max Secure series (0x0300-0x03FF)
    /// Max Secure: PQ Lightweight
    MaxSecureLightweight = 0x0300,
    /// Max Secure: Pure PQ
    MaxSecurePurePq = 0x0301,
    /// Max Secure: Hybrid Transition
    MaxSecureHybrid = 0x0302,
    /// Max Secure: Stateless
    MaxSecureStateless = 0x0303,
    /// Max Secure: Crypto-Agile
    MaxSecureCryptoAgile = 0x0304,
    /// Max Secure: PQC + Zero-Knowledge
    MaxSecurePqcZk = 0x0305,
    /// Max Secure: Hybrid Transition
    MaxSecureHybridTransition = 0x0306,

    // FN-DSA series (Falcon-based signatures) (0x0400-0x04FF)
    /// FN-DSA 512: Compact
    FnDsa512Compact = 0x0400,
    /// FN-DSA 1024: High-Security
    FnDsa1024Security = 0x0401,
    /// FN-DSA: Floating-Point Hardened
    FnDsaFpHardened = 0x0402,
    /// FN-DSA: Dual Signature
    FnDsaDualSignature = 0x0403,
    /// FN-DSA: Transition Stack
    FnDsaTransition = 0x0404,
    /// FN-DSA + Zero-Knowledge Stack
    FnDsaZk = 0x0405,
    /// FN-DSA + ZK Stack Enhanced
    FnDsaZkStack = 0x0406,
    /// FN-DSA: Transition Stack Enhanced
    FnDsaTransitionStack = 0x0407,

    // Experimental algorithms (0x0500-0x05FF)
    /// Quantum-Inspired Lattice Fusion
    QuantumLatticeFusion = 0x0500,
    /// Post-ZK Homomorphic with LFHE 2023
    PostZkHomomorphic = 0x0501,
    /// Quantum-Resistant Consensus
    QuantumResistantConsensus = 0x0502,
    /// Entropy-Orchestrated PQ Stack
    EntropyOrchestrated = 0x0503,
    /// Lattice-Code Hybrid FN
    LatticeCodeHybridFn = 0x0504,
    /// AI-Synthesized Crypto-Agile
    AiSynthesizedCryptoAgile = 0x0505,
    /// Experimental Engine (generic)
    Experimental = 0x0506,

    // HQC Code-Based series (NIST 2025 Backup KEM) (0x0600-0x06FF)
    /// HQC-128 (NIST Level 1, 128-bit security)
    Hqc128 = 0x0600,
    /// HQC-192 (NIST Level 3, 192-bit security)
    Hqc192 = 0x0601,
    /// HQC-256 (NIST Level 5, 256-bit security)
    Hqc256 = 0x0602,

    // NIST ML-KEM variants (FIPS 203) (0x0700-0x07FF)
    /// ML-KEM-512 (NIST Level 1, 128-bit security)
    MlKem512 = 0x0700,
    /// ML-KEM-768 (NIST Level 3, 192-bit security)
    MlKem768 = 0x0701,

    // NIST ML-DSA variants (FIPS 204) (0x0800-0x08FF)
    /// ML-DSA-44 (NIST Level 2, 128-bit security)
    MlDsa44 = 0x0800,
    /// ML-DSA-65 (NIST Level 3, 192-bit security)
    MlDsa65 = 0x0801,
    /// ML-DSA-87 (NIST Level 5, 256-bit security)
    MlDsa87 = 0x0802,

    // NIST SLH-DSA variants (FIPS 205) (0x0900-0x09FF)
    /// SLH-DSA-SHA2-128s (NIST Level 1, small signatures)
    SlhDsaSha2_128s = 0x0900,
    /// SLH-DSA-SHA2-128f (NIST Level 1, fast signatures)
    SlhDsaSha2_128f = 0x0901,
    /// SLH-DSA-SHA2-192s (NIST Level 3, small signatures)
    SlhDsaSha2_192s = 0x0902,
    /// SLH-DSA-SHA2-192f (NIST Level 3, fast signatures)
    SlhDsaSha2_192f = 0x0903,
    /// SLH-DSA-SHA2-256s (NIST Level 5, small signatures)
    SlhDsaSha2_256s = 0x0904,
    /// SLH-DSA-SHA2-256f (NIST Level 5, fast signatures)
    SlhDsaSha2_256f = 0x0905,
}

impl Algorithm {
    /// Convert u16 identifier to Algorithm enum
    ///
    /// # Example
    ///
    /// ```
    /// use pqc_binary_format::Algorithm;
    ///
    /// let algo = Algorithm::from_id(0x0100).unwrap();
    /// assert_eq!(algo, Algorithm::Hybrid);
    /// ```
    #[must_use]
    pub fn from_id(id: u16) -> Option<Self> {
        match id {
            0x0050 => Some(Self::Classical),
            0x0051 => Some(Self::PasswordClassical),
            0x0100 => Some(Self::Hybrid),
            0x0200 => Some(Self::PostQuantum),
            0x0201 => Some(Self::MultiAlgorithm),
            0x0202 => Some(Self::MlKem1024),
            0x0203 => Some(Self::MultiKem),
            0x0204 => Some(Self::MultiKemTriple),
            0x0205 => Some(Self::QuadLayer),
            0x0206 => Some(Self::LatticeCodeHybrid),
            0x0207 => Some(Self::Pq3Stack),
            0x0300 => Some(Self::MaxSecureLightweight),
            0x0301 => Some(Self::MaxSecurePurePq),
            0x0302 => Some(Self::MaxSecureHybrid),
            0x0303 => Some(Self::MaxSecureStateless),
            0x0304 => Some(Self::MaxSecureCryptoAgile),
            0x0305 => Some(Self::MaxSecurePqcZk),
            0x0306 => Some(Self::MaxSecureHybridTransition),
            0x0400 => Some(Self::FnDsa512Compact),
            0x0401 => Some(Self::FnDsa1024Security),
            0x0402 => Some(Self::FnDsaFpHardened),
            0x0403 => Some(Self::FnDsaDualSignature),
            0x0404 => Some(Self::FnDsaTransition),
            0x0405 => Some(Self::FnDsaZk),
            0x0406 => Some(Self::FnDsaZkStack),
            0x0407 => Some(Self::FnDsaTransitionStack),
            0x0500 => Some(Self::QuantumLatticeFusion),
            0x0501 => Some(Self::PostZkHomomorphic),
            0x0502 => Some(Self::QuantumResistantConsensus),
            0x0503 => Some(Self::EntropyOrchestrated),
            0x0504 => Some(Self::LatticeCodeHybridFn),
            0x0505 => Some(Self::AiSynthesizedCryptoAgile),
            0x0506 => Some(Self::Experimental),
            0x0600 => Some(Self::Hqc128),
            0x0601 => Some(Self::Hqc192),
            0x0602 => Some(Self::Hqc256),
            0x0700 => Some(Self::MlKem512),
            0x0701 => Some(Self::MlKem768),
            0x0800 => Some(Self::MlDsa44),
            0x0801 => Some(Self::MlDsa65),
            0x0802 => Some(Self::MlDsa87),
            0x0900 => Some(Self::SlhDsaSha2_128s),
            0x0901 => Some(Self::SlhDsaSha2_128f),
            0x0902 => Some(Self::SlhDsaSha2_192s),
            0x0903 => Some(Self::SlhDsaSha2_192f),
            0x0904 => Some(Self::SlhDsaSha2_256s),
            0x0905 => Some(Self::SlhDsaSha2_256f),
            _ => None,
        }
    }

    /// Resolve an algorithm from its name.
    ///
    /// Accepts the canonical kebab-case names used by the PQCrypta API and web
    /// UI (e.g. `"ml-kem-1024"`, `"max-secure-pure-pq"`) as well as compact
    /// aliases with separators stripped (e.g. `"mlkem1024"`). Matching is
    /// case-insensitive.
    ///
    /// # Example
    ///
    /// ```
    /// use pqc_binary_format::Algorithm;
    ///
    /// assert_eq!(Algorithm::from_name("ml-kem-1024"), Some(Algorithm::MlKem1024));
    /// assert_eq!(Algorithm::from_name("Max-Secure-Pure-PQ"), Some(Algorithm::MaxSecurePurePq));
    /// assert_eq!(Algorithm::from_name("nope"), None);
    /// ```
    #[must_use]
    pub fn from_name(name: &str) -> Option<Self> {
        let lower = name.to_lowercase();
        // Compact form: strip separators so "ml-kem-1024" == "mlkem1024".
        let compact: String = lower.chars().filter(|c| c.is_alphanumeric()).collect();
        Self::all().into_iter().find(|algo| {
            let canon = algo.canonical_name();
            let canon_compact: String = canon.chars().filter(char::is_ascii_alphanumeric).collect();
            lower == canon || compact == canon_compact
        })
    }

    /// Canonical kebab-case name matching the PQCrypta API / web UI identifier.
    ///
    /// This is the stable machine name (e.g. `"ml-kem-1024"`), distinct from
    /// [`Algorithm::name`] which is the human-readable display label.
    #[must_use]
    pub const fn canonical_name(self) -> &'static str {
        match self {
            Self::Classical => "classical",
            Self::PasswordClassical => "password-classical",
            Self::Hybrid => "hybrid",
            Self::PostQuantum => "post-quantum",
            Self::MultiAlgorithm => "multi-algorithm",
            Self::MlKem1024 => "ml-kem-1024",
            Self::MultiKem => "multi-kem",
            Self::MultiKemTriple => "multi-kem-triple",
            Self::QuadLayer => "quad-layer",
            Self::LatticeCodeHybrid => "lattice-code-hybrid",
            Self::Pq3Stack => "pq3-stack",
            Self::MaxSecureLightweight => "max-secure-lightweight",
            Self::MaxSecurePurePq => "max-secure-pure-pq",
            Self::MaxSecureHybrid => "max-secure-hybrid",
            Self::MaxSecureStateless => "max-secure-stateless",
            Self::MaxSecureCryptoAgile => "max-secure-crypto-agile",
            Self::MaxSecurePqcZk => "max-secure-pqc-zk",
            Self::MaxSecureHybridTransition => "max-secure-hybrid-transition",
            Self::FnDsa512Compact => "fn-dsa-512-compact",
            Self::FnDsa1024Security => "fn-dsa-1024-security",
            Self::FnDsaFpHardened => "fn-dsa-fp-hardened",
            Self::FnDsaDualSignature => "fn-dsa-dual-signature",
            Self::FnDsaTransition => "fn-dsa-transition",
            Self::FnDsaZk => "fn-dsa-zk",
            Self::FnDsaZkStack => "fn-dsa-zk-stack",
            Self::FnDsaTransitionStack => "fn-dsa-transition-stack",
            Self::QuantumLatticeFusion => "quantum-lattice-fusion",
            Self::PostZkHomomorphic => "post-zk-homomorphic",
            Self::QuantumResistantConsensus => "quantum-resistant-consensus",
            Self::EntropyOrchestrated => "entropy-orchestrated",
            Self::LatticeCodeHybridFn => "lattice-code-hybrid-fn",
            Self::AiSynthesizedCryptoAgile => "ai-synthesized-crypto-agile",
            Self::Experimental => "experimental",
            Self::Hqc128 => "hqc-128",
            Self::Hqc192 => "hqc-192",
            Self::Hqc256 => "hqc-256",
            Self::MlKem512 => "ml-kem-512",
            Self::MlKem768 => "ml-kem-768",
            Self::MlDsa44 => "ml-dsa-44",
            Self::MlDsa65 => "ml-dsa-65",
            Self::MlDsa87 => "ml-dsa-87",
            Self::SlhDsaSha2_128s => "slh-dsa-sha2-128s",
            Self::SlhDsaSha2_128f => "slh-dsa-sha2-128f",
            Self::SlhDsaSha2_192s => "slh-dsa-sha2-192s",
            Self::SlhDsaSha2_192f => "slh-dsa-sha2-192f",
            Self::SlhDsaSha2_256s => "slh-dsa-sha2-256s",
            Self::SlhDsaSha2_256f => "slh-dsa-sha2-256f",
        }
    }

    /// Get u16 identifier for this algorithm
    ///
    /// # Example
    ///
    /// ```
    /// use pqc_binary_format::Algorithm;
    ///
    /// assert_eq!(Algorithm::Hybrid.as_id(), 0x0100);
    /// ```
    #[must_use]
    pub const fn as_id(self) -> u16 {
        self as u16
    }

    /// Get human-readable name for this algorithm
    ///
    /// # Example
    ///
    /// ```
    /// use pqc_binary_format::Algorithm;
    ///
    /// assert_eq!(Algorithm::Hybrid.name(), "Hybrid");
    /// ```
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Classical => "Classical",
            Self::PasswordClassical => "Password Classical",
            Self::Hybrid => "Hybrid",
            Self::PostQuantum => "Post-Quantum",
            Self::MultiAlgorithm => "Multi-Algorithm",
            Self::MlKem1024 => "ML-KEM-1024",
            Self::MultiKem => "Multi-KEM Dual Layer",
            Self::MultiKemTriple => "Multi-KEM Triple Layer",
            Self::QuadLayer => "Quad-Layer",
            Self::LatticeCodeHybrid => "Lattice-Code Hybrid",
            Self::Pq3Stack => "PQ3-Stack",
            Self::MaxSecureLightweight => "Max Secure: PQ Lightweight",
            Self::MaxSecurePurePq => "Max Secure: Pure PQ",
            Self::MaxSecureHybrid => "Max Secure: Hybrid",
            Self::MaxSecureStateless => "Max Secure: Stateless",
            Self::MaxSecureCryptoAgile => "Max Secure: Crypto-Agile",
            Self::MaxSecurePqcZk => "Max Secure: PQC + ZK",
            Self::FnDsa512Compact => "FN-DSA 512: Compact",
            Self::FnDsa1024Security => "FN-DSA 1024: High-Security",
            Self::FnDsaFpHardened => "FN-DSA: Floating-Point Hardened",
            Self::FnDsaDualSignature => "FN-DSA: Dual Signature",
            Self::FnDsaTransition => "FN-DSA: Transition Stack",
            Self::FnDsaZk => "FN-DSA + ZK Stack",
            Self::FnDsaZkStack => "FN-DSA + ZK Stack Enhanced",
            Self::FnDsaTransitionStack => "FN-DSA: Transition Stack Enhanced",
            Self::MaxSecureHybridTransition => "Max Secure: Hybrid Transition",
            Self::QuantumLatticeFusion => "Quantum-Inspired Lattice Fusion",
            Self::PostZkHomomorphic => "Post-ZK Homomorphic",
            Self::QuantumResistantConsensus => "Quantum-Resistant Consensus",
            Self::EntropyOrchestrated => "Entropy-Orchestrated",
            Self::LatticeCodeHybridFn => "Lattice-Code Hybrid FN",
            Self::AiSynthesizedCryptoAgile => "AI-Synthesized Crypto-Agile",
            Self::Experimental => "Experimental Engine",
            Self::Hqc128 => "HQC-128",
            Self::Hqc192 => "HQC-192",
            Self::Hqc256 => "HQC-256",
            Self::MlKem512 => "ML-KEM-512",
            Self::MlKem768 => "ML-KEM-768",
            Self::MlDsa44 => "ML-DSA-44",
            Self::MlDsa65 => "ML-DSA-65",
            Self::MlDsa87 => "ML-DSA-87",
            Self::SlhDsaSha2_128s => "SLH-DSA-SHA2-128s",
            Self::SlhDsaSha2_128f => "SLH-DSA-SHA2-128f",
            Self::SlhDsaSha2_192s => "SLH-DSA-SHA2-192s",
            Self::SlhDsaSha2_192f => "SLH-DSA-SHA2-192f",
            Self::SlhDsaSha2_256s => "SLH-DSA-SHA2-256s",
            Self::SlhDsaSha2_256f => "SLH-DSA-SHA2-256f",
        }
    }

    /// Check if this algorithm is marked as experimental
    ///
    /// Experimental algorithms may have reduced security guarantees
    /// and are intended for research purposes.
    #[must_use]
    pub const fn is_experimental(self) -> bool {
        matches!(
            self,
            Self::QuantumLatticeFusion
                | Self::PostZkHomomorphic
                | Self::QuantumResistantConsensus
                | Self::EntropyOrchestrated
                | Self::LatticeCodeHybridFn
                | Self::AiSynthesizedCryptoAgile
                | Self::Experimental
        )
    }

    /// Get all defined algorithm identifiers
    ///
    /// Useful for iteration and testing.
    #[must_use]
    pub fn all() -> Vec<Self> {
        vec![
            Self::Classical,
            Self::PasswordClassical,
            Self::Hybrid,
            Self::PostQuantum,
            Self::MultiAlgorithm,
            Self::MlKem1024,
            Self::MultiKem,
            Self::MultiKemTriple,
            Self::QuadLayer,
            Self::LatticeCodeHybrid,
            Self::Pq3Stack,
            Self::MaxSecureLightweight,
            Self::MaxSecurePurePq,
            Self::MaxSecureHybrid,
            Self::MaxSecureStateless,
            Self::MaxSecureCryptoAgile,
            Self::MaxSecurePqcZk,
            Self::MaxSecureHybridTransition,
            Self::FnDsa512Compact,
            Self::FnDsa1024Security,
            Self::FnDsaFpHardened,
            Self::FnDsaDualSignature,
            Self::FnDsaTransition,
            Self::FnDsaZk,
            Self::FnDsaZkStack,
            Self::FnDsaTransitionStack,
            Self::QuantumLatticeFusion,
            Self::PostZkHomomorphic,
            Self::QuantumResistantConsensus,
            Self::EntropyOrchestrated,
            Self::LatticeCodeHybridFn,
            Self::AiSynthesizedCryptoAgile,
            Self::Experimental,
            Self::Hqc128,
            Self::Hqc192,
            Self::Hqc256,
            Self::MlKem512,
            Self::MlKem768,
            Self::MlDsa44,
            Self::MlDsa65,
            Self::MlDsa87,
            Self::SlhDsaSha2_128s,
            Self::SlhDsaSha2_128f,
            Self::SlhDsaSha2_192s,
            Self::SlhDsaSha2_192f,
            Self::SlhDsaSha2_256s,
            Self::SlhDsaSha2_256f,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_algorithm_roundtrip() {
        for algo in Algorithm::all() {
            let id = algo.as_id();
            let recovered = Algorithm::from_id(id).unwrap();
            assert_eq!(algo, recovered);
        }
    }

    #[test]
    fn test_invalid_algorithm_id() {
        assert!(Algorithm::from_id(0xFFFF).is_none());
    }

    #[test]
    fn test_experimental_detection() {
        assert!(Algorithm::QuantumLatticeFusion.is_experimental());
        assert!(!Algorithm::Hybrid.is_experimental());
    }
}
