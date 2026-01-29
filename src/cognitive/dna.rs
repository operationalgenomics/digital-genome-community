//! Structured DNA — LEI-AF-10-08
//!
//! # Canon Reference
//! > "O DNA Sintético emitido pelo GDC é uma sequência de termos UNL atômicos"
//!
//! # DNA Structure (6 components)
//! 1. Discovered actions (minimal A→B transformations)
//! 2. CP per action (4 motors decomposed)
//! 3. Aggregated CP (∏ cp_i)
//! 4. CP vector (granular visibility)
//! 5. Uncertainties (UNL metadata)
//! 6. Meristic suggestions
//!
//! # Prohibitions
//! - GDC does NOT persist DNA
//! - GDC does NOT execute CRISP
//! - GDC does NOT emit in human format
//!
//! # Layer: Community
//! # Version: 0.6.0 (MVP-6)

use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};

use crate::memory::{Origin, EvaluativeSignature};

/// Atomic action in DNA — LEI-AF-2-10
///
/// Represents the minimal operational unit that cannot be
/// subdivided without loss of effect.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtomicAction {
    /// Action identifier (UNL code)
    pub action_id: u32,
    /// Origin marker (AO-18)
    pub origin: Origin,
    /// Input state fingerprint
    pub state_before: [u8; 32],
    /// Output state fingerprint
    pub state_after: [u8; 32],
    /// Evaluative signature (4 motors)
    pub signature: EvaluativeSignature,
}

impl AtomicAction {
    pub fn new(
        action_id: u32,
        origin: Origin,
        state_before: [u8; 32],
        state_after: [u8; 32],
        signature: EvaluativeSignature,
    ) -> Self {
        Self {
            action_id,
            origin,
            state_before,
            state_after,
            signature,
        }
    }
    
    /// Get the CP of this action
    pub fn cp(&self) -> f64 {
        self.signature.cp
    }
    
    /// Check if this action is vetoed
    pub fn is_vetoed(&self) -> bool {
        self.signature.is_vetoed()
    }
}

/// Uncertainty metadata — Part of LEI-AF-10-08
///
/// Represents questions/uncertainties that emerged during processing.
/// Stored as UNL codes, NOT human strings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Uncertainty {
    /// Uncertainty type code (UNL)
    pub code: u16,
    /// Related action index (if any)
    pub related_action: Option<usize>,
    /// Confidence level [0,1]
    pub confidence: f64,
}

/// Meristic suggestion — Part of LEI-AF-10-08
///
/// Proposals from the Meristic motor for improvement or extension.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeristicSuggestion {
    /// Suggestion type code (UNL)
    pub code: u16,
    /// Target action index (if modifying existing)
    pub target_action: Option<usize>,
    /// Proposed action (if new)
    pub proposed_action: Option<AtomicAction>,
    /// Expected CP improvement
    pub expected_improvement: f64,
    /// Exploration depth used
    pub depth: usize,
}

/// Structured DNA — LEI-AF-10-08
///
/// The complete output of a cognitive cycle.
///
/// # Components
/// 1. **actions**: Discovered atomic actions
/// 2. **cp_vector**: CP per action [cp₁, cp₂, ..., cpₙ]
/// 3. **cp_task**: Aggregated CP (∏ cpᵢ)
/// 4. **weak_actions**: Indices of actions with low CP
/// 5. **uncertainties**: UNL metadata for questions
/// 6. **suggestions**: Meristic proposals
///
/// # Invariants
/// - cp_task == ∏ cp_vector[i]
/// - All components in UNL, not human format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredDNA {
    /// 1. Discovered actions
    pub actions: Vec<AtomicAction>,
    
    /// 2/3. CP vector and aggregated CP
    pub cp_vector: Vec<f64>,
    pub cp_task: f64,
    
    /// 4. Weak action indices (below threshold)
    pub weak_actions: Vec<usize>,
    
    /// 5. Uncertainties (UNL metadata)
    pub uncertainties: Vec<Uncertainty>,
    
    /// 6. Meristic suggestions
    pub suggestions: Vec<MeristicSuggestion>,
    
    /// DNA fingerprint
    pub fingerprint: [u8; 32],
    
    /// Cycle metadata
    pub cycle_id: [u8; 16],
    pub cycle_counter: u64,
}

impl StructuredDNA {
    /// Weakness threshold for identifying weak actions
    pub const WEAKNESS_THRESHOLD: f64 = 0.7;
    
    /// Build structured DNA from actions.
    pub fn build(
        actions: Vec<AtomicAction>,
        uncertainties: Vec<Uncertainty>,
        suggestions: Vec<MeristicSuggestion>,
        cycle_id: [u8; 16],
        cycle_counter: u64,
    ) -> Self {
        // Extract CP vector
        let cp_vector: Vec<f64> = actions.iter().map(|a| a.cp()).collect();
        
        // Calculate aggregated CP (multiplicative)
        let cp_task = if cp_vector.is_empty() {
            1.0
        } else {
            cp_vector.iter().product()
        };
        
        // Identify weak actions
        let weak_actions: Vec<usize> = cp_vector.iter()
            .enumerate()
            .filter(|(_, &cp)| cp < Self::WEAKNESS_THRESHOLD)
            .map(|(i, _)| i)
            .collect();
        
        // Generate fingerprint
        let fingerprint = Self::compute_fingerprint(&actions, cp_task, cycle_id);
        
        Self {
            actions,
            cp_vector,
            cp_task,
            weak_actions,
            uncertainties,
            suggestions,
            fingerprint,
            cycle_id,
            cycle_counter,
        }
    }
    
    /// Build from single evaluation (backward compatibility).
    pub fn from_single_evaluation(
        signature: EvaluativeSignature,
        cycle_id: [u8; 16],
        cycle_counter: u64,
    ) -> Self {
        let action = AtomicAction::new(
            0, // Single action
            Origin::External,
            [0; 32],
            [0; 32],
            signature,
        );
        
        Self::build(vec![action], vec![], vec![], cycle_id, cycle_counter)
    }
    
    /// Get number of actions
    pub fn action_count(&self) -> usize {
        self.actions.len()
    }
    
    /// Get number of weak actions
    pub fn weak_count(&self) -> usize {
        self.weak_actions.len()
    }
    
    /// Check if DNA is vetoed (any action CP = 0)
    pub fn is_vetoed(&self) -> bool {
        self.cp_task == 0.0 || self.actions.iter().any(|a| a.is_vetoed())
    }
    
    /// Get weakest action (lowest CP)
    pub fn weakest_action(&self) -> Option<(usize, f64)> {
        self.cp_vector.iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, &cp)| (i, cp))
    }
    
    /// Verify CP invariant (cp_task == ∏ cp_vector)
    pub fn verify_cp_invariant(&self) -> bool {
        if self.cp_vector.is_empty() {
            return self.cp_task == 1.0;
        }
        
        let computed: f64 = self.cp_vector.iter().product();
        (self.cp_task - computed).abs() < 1e-10
    }
    
    fn compute_fingerprint(
        actions: &[AtomicAction],
        cp_task: f64,
        cycle_id: [u8; 16],
    ) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(&cycle_id);
        hasher.update(cp_task.to_le_bytes());
        hasher.update((actions.len() as u64).to_le_bytes());
        
        for action in actions {
            hasher.update(action.action_id.to_le_bytes());
            hasher.update(&action.state_before);
            hasher.update(&action.state_after);
            hasher.update(action.cp().to_le_bytes());
        }
        
        hasher.finalize().into()
    }
}

/// DNA Builder for incremental construction
#[derive(Debug, Default)]
pub struct DnaBuilder {
    actions: Vec<AtomicAction>,
    uncertainties: Vec<Uncertainty>,
    suggestions: Vec<MeristicSuggestion>,
    cycle_id: [u8; 16],
    cycle_counter: u64,
}

impl DnaBuilder {
    pub fn new(cycle_id: [u8; 16], cycle_counter: u64) -> Self {
        Self {
            actions: Vec::new(),
            uncertainties: Vec::new(),
            suggestions: Vec::new(),
            cycle_id,
            cycle_counter,
        }
    }
    
    /// Add an atomic action
    pub fn add_action(&mut self, action: AtomicAction) -> &mut Self {
        self.actions.push(action);
        self
    }
    
    /// Add uncertainty
    pub fn add_uncertainty(&mut self, uncertainty: Uncertainty) -> &mut Self {
        self.uncertainties.push(uncertainty);
        self
    }
    
    /// Add meristic suggestion
    pub fn add_suggestion(&mut self, suggestion: MeristicSuggestion) -> &mut Self {
        self.suggestions.push(suggestion);
        self
    }
    
    /// Build the DNA
    pub fn build(self) -> StructuredDNA {
        StructuredDNA::build(
            self.actions,
            self.uncertainties,
            self.suggestions,
            self.cycle_id,
            self.cycle_counter,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn make_signature(cp_root: f64) -> EvaluativeSignature {
        EvaluativeSignature::new(cp_root, cp_root, cp_root, cp_root, false)
    }
    
    fn make_action(id: u32, cp_root: f64) -> AtomicAction {
        AtomicAction::new(
            id,
            Origin::External,
            [0; 32],
            [1; 32],
            make_signature(cp_root),
        )
    }
    
    #[test]
    fn test_structured_dna_creation() {
        let actions = vec![
            make_action(1, 0.9),
            make_action(2, 0.8),
            make_action(3, 0.7),
        ];
        
        let dna = StructuredDNA::build(
            actions,
            vec![],
            vec![],
            [1; 16],
            1,
        );
        
        assert_eq!(dna.action_count(), 3);
        assert_eq!(dna.cp_vector.len(), 3);
        assert!(dna.verify_cp_invariant());
    }
    
    #[test]
    fn test_cp_vector_calculation() {
        let actions = vec![
            make_action(1, 0.9), // CP = 0.9^4 ≈ 0.6561
            make_action(2, 0.8), // CP = 0.8^4 ≈ 0.4096
        ];
        
        let dna = StructuredDNA::build(actions, vec![], vec![], [0; 16], 0);
        
        // CP vector contains individual CPs
        assert!((dna.cp_vector[0] - 0.6561).abs() < 0.01);
        assert!((dna.cp_vector[1] - 0.4096).abs() < 0.01);
        
        // cp_task is product
        let expected_task = 0.6561 * 0.4096;
        assert!((dna.cp_task - expected_task).abs() < 0.01);
    }
    
    #[test]
    fn test_weak_actions_identification() {
        let actions = vec![
            make_action(1, 0.95),  // High CP
            make_action(2, 0.7),   // Below threshold (0.7^4 ≈ 0.24)
            make_action(3, 0.6),   // Below threshold (0.6^4 ≈ 0.13)
        ];
        
        let dna = StructuredDNA::build(actions, vec![], vec![], [0; 16], 0);
        
        // Actions 2 and 3 are weak
        assert_eq!(dna.weak_count(), 2);
        assert!(dna.weak_actions.contains(&1));
        assert!(dna.weak_actions.contains(&2));
    }
    
    #[test]
    fn test_vetoed_dna() {
        let actions = vec![
            make_action(1, 0.9),
            make_action(2, 0.0), // Vetoed
        ];
        
        let dna = StructuredDNA::build(actions, vec![], vec![], [0; 16], 0);
        
        assert!(dna.is_vetoed());
        assert_eq!(dna.cp_task, 0.0);
    }
    
    #[test]
    fn test_weakest_action() {
        let actions = vec![
            make_action(1, 0.9),
            make_action(2, 0.5), // Weakest
            make_action(3, 0.7),
        ];
        
        let dna = StructuredDNA::build(actions, vec![], vec![], [0; 16], 0);
        
        let (idx, cp) = dna.weakest_action().unwrap();
        assert_eq!(idx, 1);
        assert!((cp - 0.5_f64.powi(4)).abs() < 0.01);
    }
    
    #[test]
    fn test_dna_builder() {
        let mut builder = DnaBuilder::new([1; 16], 42);
        
        builder
            .add_action(make_action(1, 0.9))
            .add_action(make_action(2, 0.8))
            .add_uncertainty(Uncertainty {
                code: 0x0100,
                related_action: Some(1),
                confidence: 0.7,
            })
            .add_suggestion(MeristicSuggestion {
                code: 0x0200,
                target_action: Some(1),
                proposed_action: None,
                expected_improvement: 0.1,
                depth: 3,
            });
        
        let dna = builder.build();
        
        assert_eq!(dna.action_count(), 2);
        assert_eq!(dna.uncertainties.len(), 1);
        assert_eq!(dna.suggestions.len(), 1);
        assert_eq!(dna.cycle_counter, 42);
    }
    
    #[test]
    fn test_fingerprint_determinism() {
        let actions = vec![make_action(1, 0.9)];
        
        let dna1 = StructuredDNA::build(actions.clone(), vec![], vec![], [1; 16], 1);
        let dna2 = StructuredDNA::build(actions, vec![], vec![], [1; 16], 1);
        
        assert_eq!(dna1.fingerprint, dna2.fingerprint);
    }
    
    #[test]
    fn test_from_single_evaluation() {
        let sig = make_signature(0.9);
        let dna = StructuredDNA::from_single_evaluation(sig, [1; 16], 1);
        
        assert_eq!(dna.action_count(), 1);
        assert!(dna.verify_cp_invariant());
    }
}
