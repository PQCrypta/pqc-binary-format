//! Algorithm identifiers for supported post-quantum cryptographic algorithms.
//!
//! One table, at the end of this file, is the registry: each algorithm's
//! variant, 16-bit identifier, canonical name, display name, family and C
//! constant. The enum, every lookup and the `PQC_ALGORITHM_*` constants the C
//! API exports are generated from it, so they cannot disagree; the C header
//! and the Go binding are held to it by `tests/registry.rs`.

use serde::{Deserialize, Serialize};

/// Algorithm identifier type
pub type AlgorithmId = u16;

/// The family an algorithm belongs to: what kind of construction it is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Family {
    /// Classical cryptography only
    Classical,
    /// Classical and post-quantum combined
    Hybrid,
    /// Post-quantum stacks and KEMs
    PostQuantum,
    /// The Max Secure series
    MaxSecure,
    /// FN-DSA (Falcon-based) signature stacks
    FnDsa,
    /// Research constructions with reduced guarantees
    Experimental,
    /// HQC code-based KEMs (NIST 2025 backup KEM)
    Hqc,
    /// NIST ML-KEM parameter sets (FIPS 203)
    MlKem,
    /// NIST ML-DSA parameter sets (FIPS 204)
    MlDsa,
    /// NIST SLH-DSA parameter sets (FIPS 205)
    SlhDsa,
}

impl Family {
    /// Human-readable name of the family
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Classical => "Classical",
            Self::Hybrid => "Hybrid",
            Self::PostQuantum => "Post-quantum",
            Self::MaxSecure => "Max Secure",
            Self::FnDsa => "FN-DSA",
            Self::Experimental => "Experimental",
            Self::Hqc => "HQC",
            Self::MlKem => "ML-KEM (FIPS 203)",
            Self::MlDsa => "ML-DSA (FIPS 204)",
            Self::SlhDsa => "SLH-DSA (FIPS 205)",
        }
    }
}

macro_rules! algorithms {
    ($(
        $(#[doc = $doc:literal])*
        $variant:ident = $id:literal, $canonical:literal, $name:literal, $family:ident, $c_name:ident;
    )*) => {
        /// Supported cryptographic algorithms with unique identifiers
        ///
        /// Each algorithm has a unique 16-bit identifier used in the binary
        /// format. New identifiers are added in minor releases, so a `match`
        /// on this enum needs a wildcard arm.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
        #[repr(u16)]
        #[non_exhaustive]
        pub enum Algorithm {
            $( $(#[doc = $doc])* $variant = $id, )*
        }

        impl Algorithm {
            /// Every algorithm in the registry, in identifier order.
            pub const ALL: &'static [Self] = &[ $( Self::$variant, )* ];

            /// Get algorithm from u16 identifier
            ///
            /// # Example
            ///
            /// ```
            /// use pqc_binary_format::Algorithm;
            ///
            /// assert_eq!(Algorithm::from_id(0x0100), Some(Algorithm::Hybrid));
            /// assert_eq!(Algorithm::from_id(0xFFFF), None);
            /// ```
            #[must_use]
            pub const fn from_id(id: u16) -> Option<Self> {
                match id {
                    $( $id => Some(Self::$variant), )*
                    _ => None,
                }
            }

            /// Canonical kebab-case name matching the PQCrypta API / web UI identifier.
            ///
            /// This is the stable machine name (e.g. `"ml-kem-1024"`), distinct from
            /// [`Algorithm::name`] which is the human-readable display label.
            #[must_use]
            pub const fn canonical_name(self) -> &'static str {
                match self {
                    $( Self::$variant => $canonical, )*
                }
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
                    $( Self::$variant => $name, )*
                }
            }

            /// The family this algorithm belongs to
            #[must_use]
            pub const fn family(self) -> Family {
                match self {
                    $( Self::$variant => Family::$family, )*
                }
            }

            /// The name of the constant the C API exports for this algorithm's
            /// identifier (`PQC_ALGORITHM_HYBRID`)
            #[must_use]
            pub const fn c_constant(self) -> &'static str {
                match self {
                    $( Self::$variant => stringify!($c_name), )*
                }
            }
        }

        /// The identifiers as the C API exports them, one constant per
        /// algorithm, declared in `include/pqc_binary_format.h`.
        pub mod c_constants {
            $(
                #[doc = concat!("Algorithm ID for ", $name, " (", stringify!($id), ")")]
                #[unsafe(no_mangle)]
                pub static $c_name: u16 = $id;
            )*
        }
    };
}

impl Algorithm {
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
        Self::ALL.iter().copied().find(|algo| {
            let canon = algo.canonical_name();
            let canon_compact: String = canon.chars().filter(char::is_ascii_alphanumeric).collect();
            lower == canon || compact == canon_compact
        })
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

    /// Check if this algorithm is marked as experimental
    ///
    /// Experimental algorithms may have reduced security guarantees
    /// and are intended for research purposes.
    #[must_use]
    pub const fn is_experimental(self) -> bool {
        matches!(self.family(), Family::Experimental)
    }

    /// Get all defined algorithm identifiers
    ///
    /// Useful for iteration and testing; [`Algorithm::ALL`] is the same list
    /// without allocating.
    #[must_use]
    pub fn all() -> Vec<Self> {
        Self::ALL.to_vec()
    }
}

// The registry: variant = identifier, canonical name, display name, family,
// C constant. Identifiers are grouped by range; the PQCrypta platform assigned
// 0x0208-0x020B in the post-quantum range, the X-Wing hybrid among them.
algorithms! {
    // Classical algorithms (0x0050-0x00FF)
    /// Classical cryptography: X25519 + Ed25519 + AES-256-GCM
    Classical = 0x0050, "classical", "Classical", Classical, PQC_ALGORITHM_CLASSICAL;
    /// Password-based classical encryption
    PasswordClassical = 0x0051, "password-classical", "Password Classical", Classical, PQC_ALGORITHM_PASSWORD_CLASSICAL;
    // Hybrid algorithms (0x0100-0x01FF)
    /// Hybrid: ML-KEM-1024 + X25519 + ML-DSA-87 + Ed25519 + AES-256-GCM
    Hybrid = 0x0100, "hybrid", "Hybrid", Hybrid, PQC_ALGORITHM_HYBRID;
    // Pure post-quantum algorithms (0x0200-0x02FF)
    /// Post-quantum: ML-KEM-1024 + ML-DSA-87 + AES-256-GCM
    PostQuantum = 0x0200, "post-quantum", "Post-Quantum", PostQuantum, PQC_ALGORITHM_POST_QUANTUM;
    /// Multi-algorithm runtime selection
    MultiAlgorithm = 0x0201, "multi-algorithm", "Multi-Algorithm", PostQuantum, PQC_ALGORITHM_MULTI_ALGORITHM;
    /// ML-KEM-1024 with AES-256-GCM
    MlKem1024 = 0x0202, "ml-kem-1024", "ML-KEM-1024", PostQuantum, PQC_ALGORITHM_ML_KEM_1024;
    /// Multi-KEM dual layer
    MultiKem = 0x0203, "multi-kem", "Multi-KEM Dual Layer", PostQuantum, PQC_ALGORITHM_MULTI_KEM;
    /// Multi-KEM triple layer
    MultiKemTriple = 0x0204, "multi-kem-triple", "Multi-KEM Triple Layer", PostQuantum, PQC_ALGORITHM_MULTI_KEM_TRIPLE;
    /// Quad-layer redundant security
    QuadLayer = 0x0205, "quad-layer", "Quad-Layer", PostQuantum, PQC_ALGORITHM_QUAD_LAYER;
    /// Lattice-Code hybrid stack
    LatticeCodeHybrid = 0x0206, "lattice-code-hybrid", "Lattice-Code Hybrid", PostQuantum, PQC_ALGORITHM_LATTICE_CODE_HYBRID;
    /// PQ3-Stack with forward secrecy
    Pq3Stack = 0x0207, "pq3-stack", "PQ3-Stack", PostQuantum, PQC_ALGORITHM_PQ3_STACK;
    /// X-Wing hybrid KEM: ML-KEM-768 (FIPS 203) combined with X25519, the pair TLS negotiates as X25519MLKEM768
    X25519MlKem768 = 0x0208, "x25519-mlkem768", "X25519MLKEM768", Hybrid, PQC_ALGORITHM_X25519_MLKEM768;
    /// FrodoKEM-1344: unstructured-lattice KEM, NIST level 5 (ISO/IEC 18033-2)
    FrodoKem1344 = 0x0209, "frodokem-1344", "FrodoKEM-1344", PostQuantum, PQC_ALGORITHM_FRODOKEM_1344;
    /// Classic McEliece 6960119: code-based KEM (ISO/IEC 18033-2)
    ClassicMcEliece = 0x020A, "classic-mceliece", "Classic-McEliece-6960119", PostQuantum, PQC_ALGORITHM_CLASSIC_MCELIECE;
    /// FrodoKEM-976: unstructured-lattice KEM, NIST level 3
    FrodoKem976 = 0x020B, "frodokem-976", "FrodoKEM-976", PostQuantum, PQC_ALGORITHM_FRODOKEM_976;
    // Max Secure series (0x0300-0x03FF)
    /// Max Secure: PQ Lightweight
    MaxSecureLightweight = 0x0300, "max-secure-lightweight", "Max Secure: PQ Lightweight", MaxSecure, PQC_ALGORITHM_MAX_SECURE_LIGHTWEIGHT;
    /// Max Secure: Pure PQ
    MaxSecurePurePq = 0x0301, "max-secure-pure-pq", "Max Secure: Pure PQ", MaxSecure, PQC_ALGORITHM_MAX_SECURE_PURE_PQ;
    /// Max Secure: Hybrid Transition
    MaxSecureHybrid = 0x0302, "max-secure-hybrid", "Max Secure: Hybrid", MaxSecure, PQC_ALGORITHM_MAX_SECURE_HYBRID;
    /// Max Secure: Stateless
    MaxSecureStateless = 0x0303, "max-secure-stateless", "Max Secure: Stateless", MaxSecure, PQC_ALGORITHM_MAX_SECURE_STATELESS;
    /// Max Secure: Crypto-Agile
    MaxSecureCryptoAgile = 0x0304, "max-secure-crypto-agile", "Max Secure: Crypto-Agile", MaxSecure, PQC_ALGORITHM_MAX_SECURE_CRYPTO_AGILE;
    /// Max Secure: PQC + Zero-Knowledge
    MaxSecurePqcZk = 0x0305, "max-secure-pqc-zk", "Max Secure: PQC + ZK", MaxSecure, PQC_ALGORITHM_MAX_SECURE_PQC_ZK;
    /// Max Secure: Hybrid Transition
    MaxSecureHybridTransition = 0x0306, "max-secure-hybrid-transition", "Max Secure: Hybrid Transition", MaxSecure, PQC_ALGORITHM_MAX_SECURE_HYBRID_TRANSITION;
    // FN-DSA series (Falcon-based signatures) (0x0400-0x04FF)
    /// FN-DSA 512: Compact
    FnDsa512Compact = 0x0400, "fn-dsa-512-compact", "FN-DSA 512: Compact", FnDsa, PQC_ALGORITHM_FN_DSA_512_COMPACT;
    /// FN-DSA 1024: High-Security
    FnDsa1024Security = 0x0401, "fn-dsa-1024-security", "FN-DSA 1024: High-Security", FnDsa, PQC_ALGORITHM_FN_DSA_1024_SECURITY;
    /// FN-DSA: Floating-Point Hardened
    FnDsaFpHardened = 0x0402, "fn-dsa-fp-hardened", "FN-DSA: Floating-Point Hardened", FnDsa, PQC_ALGORITHM_FN_DSA_FP_HARDENED;
    /// FN-DSA: Dual Signature
    FnDsaDualSignature = 0x0403, "fn-dsa-dual-signature", "FN-DSA: Dual Signature", FnDsa, PQC_ALGORITHM_FN_DSA_DUAL_SIGNATURE;
    /// FN-DSA: Transition Stack
    FnDsaTransition = 0x0404, "fn-dsa-transition", "FN-DSA: Transition Stack", FnDsa, PQC_ALGORITHM_FN_DSA_TRANSITION;
    /// FN-DSA + Zero-Knowledge Stack
    FnDsaZk = 0x0405, "fn-dsa-zk", "FN-DSA + ZK Stack", FnDsa, PQC_ALGORITHM_FN_DSA_ZK;
    /// FN-DSA + ZK Stack Enhanced
    FnDsaZkStack = 0x0406, "fn-dsa-zk-stack", "FN-DSA + ZK Stack Enhanced", FnDsa, PQC_ALGORITHM_FN_DSA_ZK_STACK;
    /// FN-DSA: Transition Stack Enhanced
    FnDsaTransitionStack = 0x0407, "fn-dsa-transition-stack", "FN-DSA: Transition Stack Enhanced", FnDsa, PQC_ALGORITHM_FN_DSA_TRANSITION_STACK;
    // Experimental algorithms (0x0500-0x05FF)
    /// Quantum-Inspired Lattice Fusion
    QuantumLatticeFusion = 0x0500, "quantum-lattice-fusion", "Quantum-Inspired Lattice Fusion", Experimental, PQC_ALGORITHM_QUANTUM_LATTICE_FUSION;
    /// Post-ZK Homomorphic with LFHE 2023
    PostZkHomomorphic = 0x0501, "post-zk-homomorphic", "Post-ZK Homomorphic", Experimental, PQC_ALGORITHM_POST_ZK_HOMOMORPHIC;
    /// Quantum-Resistant Consensus
    QuantumResistantConsensus = 0x0502, "quantum-resistant-consensus", "Quantum-Resistant Consensus", Experimental, PQC_ALGORITHM_QUANTUM_RESISTANT_CONSENSUS;
    /// Entropy-Orchestrated PQ Stack
    EntropyOrchestrated = 0x0503, "entropy-orchestrated", "Entropy-Orchestrated", Experimental, PQC_ALGORITHM_ENTROPY_ORCHESTRATED;
    /// Lattice-Code Hybrid FN
    LatticeCodeHybridFn = 0x0504, "lattice-code-hybrid-fn", "Lattice-Code Hybrid FN", Experimental, PQC_ALGORITHM_LATTICE_CODE_HYBRID_FN;
    /// AI-Synthesized Crypto-Agile
    AiSynthesizedCryptoAgile = 0x0505, "ai-synthesized-crypto-agile", "AI-Synthesized Crypto-Agile", Experimental, PQC_ALGORITHM_AI_SYNTHESIZED_CRYPTO_AGILE;
    /// Experimental Engine (generic)
    Experimental = 0x0506, "experimental", "Experimental Engine", Experimental, PQC_ALGORITHM_EXPERIMENTAL;
    // HQC Code-Based series (NIST 2025 Backup KEM) (0x0600-0x06FF)
    /// HQC-128 (NIST Level 1, 128-bit security)
    Hqc128 = 0x0600, "hqc-128", "HQC-128", Hqc, PQC_ALGORITHM_HQC_128;
    /// HQC-192 (NIST Level 3, 192-bit security)
    Hqc192 = 0x0601, "hqc-192", "HQC-192", Hqc, PQC_ALGORITHM_HQC_192;
    /// HQC-256 (NIST Level 5, 256-bit security)
    Hqc256 = 0x0602, "hqc-256", "HQC-256", Hqc, PQC_ALGORITHM_HQC_256;
    // NIST ML-KEM variants (FIPS 203) (0x0700-0x07FF)
    /// ML-KEM-512 (NIST Level 1, 128-bit security)
    MlKem512 = 0x0700, "ml-kem-512", "ML-KEM-512", MlKem, PQC_ALGORITHM_ML_KEM_512;
    /// ML-KEM-768 (NIST Level 3, 192-bit security)
    MlKem768 = 0x0701, "ml-kem-768", "ML-KEM-768", MlKem, PQC_ALGORITHM_ML_KEM_768;
    // NIST ML-DSA variants (FIPS 204) (0x0800-0x08FF)
    /// ML-DSA-44 (NIST Level 2, 128-bit security)
    MlDsa44 = 0x0800, "ml-dsa-44", "ML-DSA-44", MlDsa, PQC_ALGORITHM_ML_DSA_44;
    /// ML-DSA-65 (NIST Level 3, 192-bit security)
    MlDsa65 = 0x0801, "ml-dsa-65", "ML-DSA-65", MlDsa, PQC_ALGORITHM_ML_DSA_65;
    /// ML-DSA-87 (NIST Level 5, 256-bit security)
    MlDsa87 = 0x0802, "ml-dsa-87", "ML-DSA-87", MlDsa, PQC_ALGORITHM_ML_DSA_87;
    // NIST SLH-DSA variants (FIPS 205) (0x0900-0x09FF)
    /// SLH-DSA-SHA2-128s (NIST Level 1, small signatures)
    SlhDsaSha2_128s = 0x0900, "slh-dsa-sha2-128s", "SLH-DSA-SHA2-128s", SlhDsa, PQC_ALGORITHM_SLH_DSA_SHA2_128S;
    /// SLH-DSA-SHA2-128f (NIST Level 1, fast signatures)
    SlhDsaSha2_128f = 0x0901, "slh-dsa-sha2-128f", "SLH-DSA-SHA2-128f", SlhDsa, PQC_ALGORITHM_SLH_DSA_SHA2_128F;
    /// SLH-DSA-SHA2-192s (NIST Level 3, small signatures)
    SlhDsaSha2_192s = 0x0902, "slh-dsa-sha2-192s", "SLH-DSA-SHA2-192s", SlhDsa, PQC_ALGORITHM_SLH_DSA_SHA2_192S;
    /// SLH-DSA-SHA2-192f (NIST Level 3, fast signatures)
    SlhDsaSha2_192f = 0x0903, "slh-dsa-sha2-192f", "SLH-DSA-SHA2-192f", SlhDsa, PQC_ALGORITHM_SLH_DSA_SHA2_192F;
    /// SLH-DSA-SHA2-256s (NIST Level 5, small signatures)
    SlhDsaSha2_256s = 0x0904, "slh-dsa-sha2-256s", "SLH-DSA-SHA2-256s", SlhDsa, PQC_ALGORITHM_SLH_DSA_SHA2_256S;
    /// SLH-DSA-SHA2-256f (NIST Level 5, fast signatures)
    SlhDsaSha2_256f = 0x0905, "slh-dsa-sha2-256f", "SLH-DSA-SHA2-256f", SlhDsa, PQC_ALGORITHM_SLH_DSA_SHA2_256F;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_algorithm_roundtrip() {
        for &algo in Algorithm::ALL {
            assert_eq!(Algorithm::from_id(algo.as_id()), Some(algo));
            assert_eq!(Algorithm::from_name(algo.canonical_name()), Some(algo));
        }
    }

    #[test]
    fn identifiers_and_names_are_unique_and_ordered() {
        let ids: Vec<u16> = Algorithm::ALL.iter().map(|a| a.as_id()).collect();
        assert!(
            ids.windows(2).all(|w| w[0] < w[1]),
            "ALL is in identifier order: {ids:x?}"
        );
        let mut canon: Vec<&str> = Algorithm::ALL.iter().map(|a| a.canonical_name()).collect();
        canon.sort_unstable();
        canon.dedup();
        assert_eq!(canon.len(), Algorithm::ALL.len());
    }

    /// The C constant is the canonical name in capitals, so the C API's
    /// names follow from the registry.
    #[test]
    fn c_constants_follow_the_canonical_names() {
        for &algo in Algorithm::ALL {
            let expected = format!(
                "PQC_ALGORITHM_{}",
                algo.canonical_name().to_uppercase().replace('-', "_")
            );
            assert_eq!(algo.c_constant(), expected);
        }
        assert_eq!(c_constants::PQC_ALGORITHM_LATTICE_CODE_HYBRID, 0x0206);
        assert_eq!(c_constants::PQC_ALGORITHM_X25519_MLKEM768, 0x0208);
    }

    #[test]
    fn test_invalid_algorithm_id() {
        assert!(Algorithm::from_id(0xFFFF).is_none());
    }

    #[test]
    fn test_experimental_detection() {
        assert!(Algorithm::QuantumLatticeFusion.is_experimental());
        assert!(!Algorithm::Hybrid.is_experimental());
        assert_eq!(
            Algorithm::ALL
                .iter()
                .filter(|a| a.is_experimental())
                .count(),
            Algorithm::ALL
                .iter()
                .filter(|a| (0x0500..0x0600).contains(&a.as_id()))
                .count()
        );
    }

    #[test]
    fn the_platform_assignments_resolve() {
        for (id, name) in [
            (0x0208, "x25519-mlkem768"),
            (0x0209, "frodokem-1344"),
            (0x020A, "classic-mceliece"),
            (0x020B, "frodokem-976"),
        ] {
            assert_eq!(
                Algorithm::from_id(id).map(Algorithm::canonical_name),
                Some(name)
            );
        }
    }
}
