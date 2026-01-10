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
            _ => None,
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
