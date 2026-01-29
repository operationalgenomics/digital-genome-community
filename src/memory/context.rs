//! Canonical Context — LEI-AF-12-02
//!
//! # Canon Reference
//! > "Contexto Canônico = (classe de problema, condições iniciais serializadas)"
//!
//! # Purpose
//! Enables dominance comparison between Codons in the same context.
//! Two Codons are comparable only if they share the same CanonicalContext.
//!
//! # Layer: Community
//! # Version: 0.6.0 (MVP-6)

use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};

/// Canonical Context — LEI-AF-12-02
///
/// Defines the problem class and initial conditions under which
/// a Codon was created. Used for dominance comparison in MCI.
///
/// # Invariants
/// - Same context + higher CP = dominance
/// - Different contexts = incomparable
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CanonicalContext {
    /// Problem class identifier (UNL code sequence)
    /// This is NOT a human string — it's a hash of the problem signature
    pub problem_class: [u8; 32],
    
    /// Serialized initial conditions (hash)
    /// Includes: input fingerprint, motor context hash, MCI state hash
    pub initial_conditions: [u8; 32],
}

impl CanonicalContext {
    /// Create a new canonical context from problem signature and conditions.
    ///
    /// # Arguments
    /// * `problem_signature` - Bytes representing the problem class (e.g., data structure hash)
    /// * `initial_state` - Bytes representing initial conditions (e.g., input + MCI snapshot)
    pub fn new(problem_signature: &[u8], initial_state: &[u8]) -> Self {
        let mut problem_hasher = Sha256::new();
        problem_hasher.update(problem_signature);
        let problem_class: [u8; 32] = problem_hasher.finalize().into();
        
        let mut conditions_hasher = Sha256::new();
        conditions_hasher.update(initial_state);
        let initial_conditions: [u8; 32] = conditions_hasher.finalize().into();
        
        Self {
            problem_class,
            initial_conditions,
        }
    }
    
    /// Create context from raw hashes (for deserialization/replay).
    pub fn from_hashes(problem_class: [u8; 32], initial_conditions: [u8; 32]) -> Self {
        Self {
            problem_class,
            initial_conditions,
        }
    }
    
    /// Check if two contexts are in the same problem class.
    /// Used for partial comparison (same class, different conditions).
    pub fn same_problem_class(&self, other: &Self) -> bool {
        self.problem_class == other.problem_class
    }
    
    /// Generate a combined fingerprint for indexing.
    pub fn fingerprint(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(&self.problem_class);
        hasher.update(&self.initial_conditions);
        hasher.finalize().into()
    }
}

impl Default for CanonicalContext {
    fn default() -> Self {
        Self {
            problem_class: [0; 32],
            initial_conditions: [0; 32],
        }
    }
}

impl Ord for CanonicalContext {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.fingerprint().cmp(&other.fingerprint())
    }
}

impl PartialOrd for CanonicalContext {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_context_creation() {
        let ctx = CanonicalContext::new(b"turbine_analysis", b"initial_state_123");
        assert_ne!(ctx.problem_class, [0; 32]);
        assert_ne!(ctx.initial_conditions, [0; 32]);
    }
    
    #[test]
    fn test_same_problem_class() {
        let ctx1 = CanonicalContext::new(b"turbine_analysis", b"state_1");
        let ctx2 = CanonicalContext::new(b"turbine_analysis", b"state_2");
        let ctx3 = CanonicalContext::new(b"bearing_analysis", b"state_1");
        
        assert!(ctx1.same_problem_class(&ctx2));
        assert!(!ctx1.same_problem_class(&ctx3));
    }
    
    #[test]
    fn test_context_determinism() {
        let ctx1 = CanonicalContext::new(b"problem", b"state");
        let ctx2 = CanonicalContext::new(b"problem", b"state");
        
        assert_eq!(ctx1, ctx2);
        assert_eq!(ctx1.fingerprint(), ctx2.fingerprint());
    }
    
    #[test]
    fn test_context_ordering() {
        let ctx1 = CanonicalContext::new(b"aaa", b"state");
        let ctx2 = CanonicalContext::new(b"bbb", b"state");
        
        // Should have deterministic ordering (for BTreeMap)
        assert!(ctx1 != ctx2);
        assert!(ctx1 < ctx2 || ctx1 > ctx2);
    }
}
