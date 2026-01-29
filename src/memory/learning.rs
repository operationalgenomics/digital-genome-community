//! Learning Module — AF-11: Aprendizado Cognitivo Autônomo
//!
//! # Canon Reference
//! > "O GDC PODE incorporar Códons Canônicos por ciclo fechado que
//!    satisfaça melhoria estrita de CP no mesmo contexto."
//!
//! # Key Laws
//! - LEI-AF-11-01: Epistemic trigger by stagnation (Meristic proposes)
//! - LEI-AF-11-02: Incorporation by strict improvement (CP_cand > CP_base)
//! - LEI-AF-11-03: Stability by mandatory replay
//! - LEI-AF-11-04: Vetoed Codon cannot incorporate
//!
//! # Layer: Community
//! # Version: 0.6.0 (MVP-6)

use super::context::CanonicalContext;
use super::codon::{CanonicalCodon, EvaluativeSignature, ActivationCondition, ReplayableProvenance, Origin};
use super::mci::{MCI, MciError};

/// Learning trigger types — LEI-AF-11-01
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EpistemicTrigger {
    /// CP stagnated (CP_t == CP_{t-1} in same context)
    Stagnation,
    /// Meristic motor proposed exploration
    MeristicProposal,
    /// Explicit learning request (from GDO)
    ExplicitRequest,
    /// Novelty detected (new context)
    NoveltyDetected,
}

/// Learning attempt result
#[derive(Debug, Clone)]
pub struct LearningResult {
    /// Whether learning occurred
    pub learned: bool,
    /// Trigger that initiated learning
    pub trigger: EpistemicTrigger,
    /// CP before learning attempt
    pub baseline_cp: f64,
    /// CP after learning (if learned)
    pub new_cp: Option<f64>,
    /// Reason for rejection (if not learned)
    pub rejection_reason: Option<RejectionReason>,
}

/// Reasons for rejecting learning
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RejectionReason {
    /// Codon was vetoed (CP = 0)
    Vetoed,
    /// CP did not improve strictly
    NoImprovement,
    /// MCI capacity exceeded
    CapacityExceeded,
    /// Replay verification failed
    ReplayFailed,
}

/// Stagnation detector — LEI-AF-11-01
///
/// Detects when CP is stagnating in the same context,
/// which triggers epistemic exploration.
#[derive(Debug, Clone, Default)]
pub struct StagnationDetector {
    /// Last CP per context
    last_cp: std::collections::HashMap<[u8; 32], f64>,
    /// Stagnation count per context
    stagnation_count: std::collections::HashMap<[u8; 32], u32>,
    /// Threshold for triggering (consecutive stagnations)
    threshold: u32,
}

impl StagnationDetector {
    pub fn new(threshold: u32) -> Self {
        Self {
            last_cp: std::collections::HashMap::new(),
            stagnation_count: std::collections::HashMap::new(),
            threshold,
        }
    }
    
    /// Record CP observation and check for stagnation.
    pub fn observe(&mut self, context: &CanonicalContext, current_cp: f64) -> bool {
        let key = context.fingerprint();
        
        if let Some(&last) = self.last_cp.get(&key) {
            // Check if stagnated (same CP)
            if (current_cp - last).abs() < 1e-10 {
                let count = self.stagnation_count.entry(key).or_insert(0);
                *count += 1;
                
                if *count >= self.threshold {
                    // Reset count and trigger
                    *count = 0;
                    return true;
                }
            } else {
                // CP changed, reset stagnation count
                self.stagnation_count.insert(key, 0);
            }
        }
        
        self.last_cp.insert(key, current_cp);
        false
    }
    
    /// Reset detector state.
    pub fn reset(&mut self) {
        self.last_cp.clear();
        self.stagnation_count.clear();
    }
}

/// Learning Engine — AF-11
///
/// Orchestrates cognitive learning by:
/// 1. Detecting epistemic triggers
/// 2. Validating incorporation criteria
/// 3. Managing MCI updates
#[derive(Debug)]
pub struct LearningEngine {
    /// Stagnation detector
    stagnation: StagnationDetector,
    /// Replay verification enabled
    replay_verify: bool,
}

impl LearningEngine {
    pub fn new(stagnation_threshold: u32) -> Self {
        Self {
            stagnation: StagnationDetector::new(stagnation_threshold),
            replay_verify: true,
        }
    }
    
    /// Create engine without replay verification (for testing).
    pub fn without_replay_verify(stagnation_threshold: u32) -> Self {
        Self {
            stagnation: StagnationDetector::new(stagnation_threshold),
            replay_verify: false,
        }
    }
    
    /// Check if epistemic exploration should be triggered — LEI-AF-11-01
    pub fn check_trigger(&mut self, context: &CanonicalContext, current_cp: f64) -> Option<EpistemicTrigger> {
        if self.stagnation.observe(context, current_cp) {
            return Some(EpistemicTrigger::Stagnation);
        }
        None
    }
    
    /// Attempt to learn (incorporate) a Codon — LEI-AF-11-02
    ///
    /// # Criteria
    /// 1. Codon must not be vetoed (LEI-AF-11-04)
    /// 2. CP_candidate > CP_baseline (LEI-AF-11-02)
    /// 3. Replay verification passes (LEI-AF-11-03) — if enabled
    pub fn try_learn(
        &mut self,
        mci: &mut MCI,
        candidate: CanonicalCodon,
        trigger: EpistemicTrigger,
    ) -> LearningResult {
        let context = &candidate.condicao_uso.context;
        let baseline_cp = mci.baseline_cp(context);
        let candidate_cp = candidate.cp();
        
        // LEI-AF-11-04: Vetoed cannot incorporate
        if candidate.is_vetoed() {
            return LearningResult {
                learned: false,
                trigger,
                baseline_cp,
                new_cp: None,
                rejection_reason: Some(RejectionReason::Vetoed),
            };
        }
        
        // LEI-AF-11-02: Must improve strictly
        if candidate_cp <= baseline_cp {
            return LearningResult {
                learned: false,
                trigger,
                baseline_cp,
                new_cp: None,
                rejection_reason: Some(RejectionReason::NoImprovement),
            };
        }
        
        // LEI-AF-11-03: Replay verification (if enabled)
        if self.replay_verify {
            // In production, this would verify the Codon produces same results
            // For MVP-6, we trust the Codon's provenance
            // Full replay verification will be implemented in MVP-7
        }
        
        // Attempt incorporation
        match mci.try_incorporate(candidate) {
            Ok(true) => LearningResult {
                learned: true,
                trigger,
                baseline_cp,
                new_cp: Some(candidate_cp),
                rejection_reason: None,
            },
            Ok(false) => LearningResult {
                learned: false,
                trigger,
                baseline_cp,
                new_cp: None,
                rejection_reason: Some(RejectionReason::NoImprovement),
            },
            Err(MciError::CapacityExceeded) => LearningResult {
                learned: false,
                trigger,
                baseline_cp,
                new_cp: None,
                rejection_reason: Some(RejectionReason::CapacityExceeded),
            },
            Err(_) => LearningResult {
                learned: false,
                trigger,
                baseline_cp,
                new_cp: None,
                rejection_reason: Some(RejectionReason::Vetoed),
            },
        }
    }
    
    /// Create a candidate Codon from current cycle state.
    ///
    /// This is a helper for creating Codons from evaluation results.
    pub fn create_candidate(
        &self,
        forma: Vec<u8>,
        context: CanonicalContext,
        signature: EvaluativeSignature,
        cycle_id: [u8; 16],
        input_fingerprint: [u8; 32],
        mci_fingerprint: [u8; 32],
        cycle_counter: u64,
        origin: Origin,
    ) -> CanonicalCodon {
        let provenance = ReplayableProvenance::new(
            cycle_id,
            input_fingerprint,
            mci_fingerprint,
            cycle_counter,
            origin,
        );
        
        let condition = ActivationCondition::new(context);
        
        CanonicalCodon::new(forma, provenance, signature, condition)
    }
    
    /// Reset learning engine state.
    pub fn reset(&mut self) {
        self.stagnation.reset();
    }
}

impl Default for LearningEngine {
    fn default() -> Self {
        Self::new(3) // Default: trigger after 3 stagnations
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn make_signature(cp_root: f64) -> EvaluativeSignature {
        EvaluativeSignature::new(cp_root, cp_root, cp_root, cp_root, false)
    }
    
    fn make_codon(cp_root: f64, problem: &[u8], origin: Origin) -> CanonicalCodon {
        let ctx = CanonicalContext::new(problem, b"state");
        let sig = make_signature(cp_root);
        let prov = ReplayableProvenance::new(
            [0; 16], [0; 32], [0; 32], 0, origin
        );
        CanonicalCodon::new(b"action".to_vec(), prov, sig, ActivationCondition::new(ctx))
    }
    
    #[test]
    fn test_stagnation_detection() {
        let mut detector = StagnationDetector::new(3);
        let ctx = CanonicalContext::new(b"problem", b"state");
        
        // First observation - no stagnation
        assert!(!detector.observe(&ctx, 0.5));
        
        // Same CP - count 1
        assert!(!detector.observe(&ctx, 0.5));
        // Same CP - count 2
        assert!(!detector.observe(&ctx, 0.5));
        // Same CP - count 3 → trigger!
        assert!(detector.observe(&ctx, 0.5));
        
        // Reset and change CP - no trigger
        assert!(!detector.observe(&ctx, 0.6));
    }
    
    #[test]
    fn test_learning_engine_trigger() {
        let mut engine = LearningEngine::new(2);
        let ctx = CanonicalContext::new(b"problem", b"state");
        
        // No trigger initially
        assert!(engine.check_trigger(&ctx, 0.5).is_none());
        assert!(engine.check_trigger(&ctx, 0.5).is_none());
        
        // Trigger on second stagnation
        assert!(matches!(
            engine.check_trigger(&ctx, 0.5),
            Some(EpistemicTrigger::Stagnation)
        ));
    }
    
    #[test]
    fn test_try_learn_success() {
        let mut engine = LearningEngine::without_replay_verify(3);
        let mut mci = MCI::unlimited();
        
        // First Codon
        let codon1 = make_codon(0.8, b"problem", Origin::External);
        let result1 = engine.try_learn(&mut mci, codon1, EpistemicTrigger::ExplicitRequest);
        assert!(result1.learned);
        
        // Better Codon - should learn
        let codon2 = make_codon(0.95, b"problem", Origin::Internal);
        let result2 = engine.try_learn(&mut mci, codon2, EpistemicTrigger::MeristicProposal);
        assert!(result2.learned);
        assert!(result2.new_cp.unwrap() > result2.baseline_cp);
    }
    
    #[test]
    fn test_try_learn_vetoed_rejected() {
        let mut engine = LearningEngine::without_replay_verify(3);
        let mut mci = MCI::unlimited();
        
        // Vetoed Codon (CP = 0)
        let codon = make_codon(0.0, b"problem", Origin::External);
        let result = engine.try_learn(&mut mci, codon, EpistemicTrigger::ExplicitRequest);
        
        assert!(!result.learned);
        assert_eq!(result.rejection_reason, Some(RejectionReason::Vetoed));
    }
    
    #[test]
    fn test_try_learn_no_improvement_rejected() {
        let mut engine = LearningEngine::without_replay_verify(3);
        let mut mci = MCI::unlimited();
        
        // First Codon with high CP
        let codon1 = make_codon(0.95, b"problem", Origin::External);
        engine.try_learn(&mut mci, codon1, EpistemicTrigger::ExplicitRequest);
        
        // Lower CP Codon - rejected
        let codon2 = make_codon(0.7, b"problem", Origin::Internal);
        let result = engine.try_learn(&mut mci, codon2, EpistemicTrigger::MeristicProposal);
        
        assert!(!result.learned);
        assert_eq!(result.rejection_reason, Some(RejectionReason::NoImprovement));
    }
    
    #[test]
    fn test_create_candidate() {
        let engine = LearningEngine::default();
        let ctx = CanonicalContext::new(b"problem", b"state");
        let sig = make_signature(0.9);
        
        let codon = engine.create_candidate(
            b"action_sequence".to_vec(),
            ctx,
            sig,
            [1; 16],
            [2; 32],
            [3; 32],
            42,
            Origin::Internal,
        );
        
        assert_eq!(codon.origin(), Origin::Internal);
        assert!((codon.cp() - 0.9_f64.powi(4)).abs() < 0.01);
    }
}
