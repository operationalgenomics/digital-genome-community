//! Canonical Codon — LEI-AF-12-01
//!
//! # Canon Reference
//! > "Códon Canônico = { forma, evidência, assinatura_avaliativa, condição_de_uso }"
//!
//! # Purpose
//! The atomic unit of knowledge in MCI. Each Codon represents a
//! replayable cognitive action with its evaluative signature.
//!
//! # Layer: Community
//! # Version: 0.6.0 (MVP-6)

use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use super::context::CanonicalContext;

/// Origin marker — AO-18: Autorreferência Cognitiva
///
/// Distinguishes EXTERNAL (from perception) from INTERNAL (from MCI/Meristic).
/// Must be deterministically assigned and consistent under replay.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum Origin {
    /// State originated from external perception (E1)
    External,
    /// State originated internally (MCI recall, Meristic proposal)
    Internal,
    /// State resulted from recombination of External + Internal
    Recombined,
}

impl Default for Origin {
    fn default() -> Self {
        Self::External
    }
}

/// Evaluative Signature — Part of LEI-AF-12-01
///
/// Contains the 4 motor scores and the resulting CP at the time
/// the Codon was created.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct EvaluativeSignature {
    /// Praxeological motor score [0,1]
    pub m_p: f64,
    /// Nash motor score [0,1]
    pub m_n: f64,
    /// Chaotic motor score [0,1]
    pub m_c: f64,
    /// Meristic motor score [0,1]
    pub m_m: f64,
    /// Craft Performance = M_P × M_N × M_C × M_M
    pub cp: f64,
    /// Whether Nash was applicable (≥2 players)
    pub nash_applicable: bool,
}

impl EvaluativeSignature {
    /// Create new signature from motor scores.
    pub fn new(m_p: f64, m_n: f64, m_c: f64, m_m: f64, nash_applicable: bool) -> Self {
        let cp = m_p * m_n * m_c * m_m;
        Self { m_p, m_n, m_c, m_m, cp, nash_applicable }
    }
    
    /// Check if any motor vetoed (score = 0).
    pub fn is_vetoed(&self) -> bool {
        self.cp == 0.0 || self.m_p == 0.0 || self.m_n == 0.0 || self.m_c == 0.0 || self.m_m == 0.0
    }
    
    /// Get CP as vector [cp_action] for single-action Codon.
    pub fn cp_vector(&self) -> Vec<f64> {
        vec![self.cp]
    }
}

impl Default for EvaluativeSignature {
    fn default() -> Self {
        Self {
            m_p: 1.0,
            m_n: 1.0,
            m_c: 1.0,
            m_m: 1.0,
            cp: 1.0,
            nash_applicable: false,
        }
    }
}

/// Activation Condition — Part of LEI-AF-12-01
///
/// Specifies when this Codon is applicable.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ActivationCondition {
    /// Context in which this Codon was created
    pub context: CanonicalContext,
    /// Minimum CP threshold for activation
    pub min_cp_threshold: u64,  // Stored as fixed-point (× 1_000_000)
    /// Whether this Codon requires specific problem class
    pub problem_class_bound: bool,
}

impl ActivationCondition {
    pub fn new(context: CanonicalContext) -> Self {
        Self {
            context,
            min_cp_threshold: 0,
            problem_class_bound: true,
        }
    }
    
    /// Check if condition is satisfied for given context and CP.
    pub fn is_satisfied(&self, current_context: &CanonicalContext, current_cp: f64) -> bool {
        let cp_ok = (current_cp * 1_000_000.0) as u64 >= self.min_cp_threshold;
        let context_ok = if self.problem_class_bound {
            self.context.same_problem_class(current_context)
        } else {
            true
        };
        cp_ok && context_ok
    }
}

impl Default for ActivationCondition {
    fn default() -> Self {
        Self {
            context: CanonicalContext::default(),
            min_cp_threshold: 0,
            problem_class_bound: false,
        }
    }
}

/// Replayable Provenance — Part of LEI-AF-12-01
///
/// Evidence chain that allows replay verification.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReplayableProvenance {
    /// Cycle ID when this Codon was created
    pub cycle_id: [u8; 16],
    /// Input fingerprint at creation time
    pub input_fingerprint: [u8; 32],
    /// MCI state fingerprint at creation time
    pub mci_state_fingerprint: [u8; 32],
    /// Timestamp (replay-safe: cycle counter, not wall clock)
    pub cycle_counter: u64,
    /// Origin marker
    pub origin: Origin,
}

impl ReplayableProvenance {
    pub fn new(cycle_id: [u8; 16], input_fp: [u8; 32], mci_fp: [u8; 32], counter: u64, origin: Origin) -> Self {
        Self {
            cycle_id,
            input_fingerprint: input_fp,
            mci_state_fingerprint: mci_fp,
            cycle_counter: counter,
            origin,
        }
    }
}

impl Default for ReplayableProvenance {
    fn default() -> Self {
        Self {
            cycle_id: [0; 16],
            input_fingerprint: [0; 32],
            mci_state_fingerprint: [0; 32],
            cycle_counter: 0,
            origin: Origin::External,
        }
    }
}

/// Canonical Codon — LEI-AF-12-01
///
/// The atomic unit of knowledge in MCI.
///
/// # Structure (4 mandatory fields)
/// 1. **forma**: Structure in UNL/GD-QMN (action sequence)
/// 2. **evidência**: Replayable provenance chain
/// 3. **assinatura_avaliativa**: 4 motor scores + CP
/// 4. **condição_de_uso**: Activation condition
///
/// # Invariants
/// - Codon with vetoed signature (CP=0) CANNOT be incorporated (LEI-AF-11-04)
/// - Same context + higher CP = dominance (LEI-AF-12-02)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonicalCodon {
    /// 1. Form — UNL action sequence (stored as fingerprint + raw bytes)
    pub forma_fingerprint: [u8; 32],
    pub forma_bytes: Vec<u8>,
    
    /// 2. Evidence — Replayable provenance
    pub evidencia: ReplayableProvenance,
    
    /// 3. Evaluative Signature — 4 motors + CP
    pub assinatura: EvaluativeSignature,
    
    /// 4. Activation Condition — When applicable
    pub condicao_uso: ActivationCondition,
}

impl CanonicalCodon {
    /// Create a new Canonical Codon.
    ///
    /// # Arguments
    /// * `forma` - UNL action sequence bytes
    /// * `evidencia` - Provenance chain
    /// * `assinatura` - Motor scores at creation
    /// * `condicao_uso` - Activation condition
    pub fn new(
        forma: Vec<u8>,
        evidencia: ReplayableProvenance,
        assinatura: EvaluativeSignature,
        condicao_uso: ActivationCondition,
    ) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(&forma);
        let forma_fingerprint: [u8; 32] = hasher.finalize().into();
        
        Self {
            forma_fingerprint,
            forma_bytes: forma,
            evidencia,
            assinatura,
            condicao_uso,
        }
    }
    
    /// Get the Craft Performance of this Codon.
    pub fn cp(&self) -> f64 {
        self.assinatura.cp
    }
    
    /// Get the origin of this Codon.
    pub fn origin(&self) -> Origin {
        self.evidencia.origin
    }
    
    /// Check if this Codon is vetoed (CP = 0).
    pub fn is_vetoed(&self) -> bool {
        self.assinatura.is_vetoed()
    }
    
    /// Check if this Codon dominates another in the same context.
    ///
    /// Dominance: same context + higher CP.
    pub fn dominates(&self, other: &Self) -> bool {
        self.condicao_uso.context == other.condicao_uso.context
            && self.cp() > other.cp()
    }
    
    /// Generate unique fingerprint for this Codon.
    pub fn fingerprint(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(&self.forma_fingerprint);
        hasher.update(&self.evidencia.cycle_id);
        hasher.update(self.assinatura.cp.to_le_bytes());
        hasher.finalize().into()
    }
    
    /// Check if this Codon can be incorporated (LEI-AF-11-04).
    ///
    /// A vetoed Codon CANNOT be incorporated.
    pub fn can_incorporate(&self) -> bool {
        !self.is_vetoed()
    }
}

impl PartialEq for CanonicalCodon {
    fn eq(&self, other: &Self) -> bool {
        self.fingerprint() == other.fingerprint()
    }
}

impl Eq for CanonicalCodon {}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn make_codon(cp: f64, context: &[u8]) -> CanonicalCodon {
        let ctx = CanonicalContext::new(context, b"state");
        // Approximate CP = (cp^0.25)^4 = cp, so we need 4th root
        let root = cp.powf(0.25);
        let sig = EvaluativeSignature::new(root, root, root, root, false);
        
        CanonicalCodon::new(
            b"action_sequence".to_vec(),
            ReplayableProvenance::default(),
            sig,
            ActivationCondition::new(ctx),
        )
    }
    
    #[test]
    fn test_codon_creation() {
        let codon = make_codon(0.81, b"test_problem");
        assert!((codon.cp() - 0.81).abs() < 0.01);
        assert!(codon.can_incorporate());
    }
    
    #[test]
    fn test_vetoed_codon_cannot_incorporate() {
        let ctx = CanonicalContext::new(b"problem", b"state");
        let sig = EvaluativeSignature::new(0.0, 1.0, 1.0, 1.0, false); // M_P = 0 → veto
        
        let codon = CanonicalCodon::new(
            b"action".to_vec(),
            ReplayableProvenance::default(),
            sig,
            ActivationCondition::new(ctx),
        );
        
        assert!(codon.is_vetoed());
        assert!(!codon.can_incorporate());
    }
    
    #[test]
    fn test_dominance() {
        let codon_high = make_codon(0.9, b"problem_a");
        let codon_low = make_codon(0.5, b"problem_a");
        let codon_other = make_codon(0.95, b"problem_b");
        
        assert!(codon_high.dominates(&codon_low));
        assert!(!codon_low.dominates(&codon_high));
        assert!(!codon_high.dominates(&codon_other)); // Different context
    }
    
    #[test]
    fn test_origin_marker() {
        let mut prov = ReplayableProvenance::default();
        prov.origin = Origin::Internal;
        
        let codon = CanonicalCodon::new(
            b"internal_action".to_vec(),
            prov,
            EvaluativeSignature::default(),
            ActivationCondition::default(),
        );
        
        assert_eq!(codon.origin(), Origin::Internal);
    }
    
    #[test]
    fn test_codon_fingerprint_determinism() {
        let codon1 = make_codon(0.8, b"problem");
        let codon2 = make_codon(0.8, b"problem");
        
        // Same parameters → same fingerprint
        assert_eq!(codon1.fingerprint(), codon2.fingerprint());
    }
}
