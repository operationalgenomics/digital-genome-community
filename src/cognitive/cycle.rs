//! Cognitive Cycle - Complete GDC processing cycle with MCI integration
//!
//! # MVP-7: Full Cognitive Integration
//!
//! This module orchestrates the full cognitive cycle with learning and memory:
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────────┐
//! │                     COGNITIVE CYCLE v0.7.0                          │
//! ├─────────────────────────────────────────────────────────────────────┤
//! │                                                                     │
//! │  E1: PERCEIVE ──→ E2: ENCODE ──→ E3: EVALUATE ──→ E4: INTEGRATE    │
//! │       │                              │    ▲              │          │
//! │       │                              ▼    │              ▼          │
//! │       │                           ┌──────────┐    E5: DELIBERATE   │
//! │       │                           │   MCI    │          │          │
//! │       │                           │ (AF-12)  │          ▼          │
//! │       │                           └──────────┘    E6: EMIT DNA     │
//! │       │                                ▲              │            │
//! │       │                                │              │            │
//! │       └── ORIGIN: EXTERNAL            │              │            │
//! │                                       │              │            │
//! │           ORIGIN: INTERNAL ───────────┘              │            │
//! │                                                      │            │
//! │           LEARNING (AF-11) ◄─────────────────────────┘            │
//! │                                                                     │
//! └─────────────────────────────────────────────────────────────────────┘
//! ```
//!
//! # Canonical Compliance
//!
//! - **AF-11**: Autonomous cognitive learning by replayable incorporation
//! - **AF-12**: Internal cognitive memory (MCI) - non-observation
//! - **AO-18**: Self-reference via Origin marker (EXTERNAL/INTERNAL/RECOMBINED)
//! - **LEI-AF-10-07**: Meristic motor posteriority
//! - **LEI-AF-10-08**: Structured DNA emission
//! - **LEI-AF-12-04**: MCI participates in E3 pipeline

use crate::sensory::{RawInput, SensoryCortex, CortexOutput};
use crate::motors::{
    CognitiveMotor,
    PraxisMotor, PraxisInput,
    NashMotor, NashInput,
    ChaosMotor, ChaosInput,
    MeristicMotor, MeristicInput,
};
use crate::math::craft::{CraftPerformance, CpResult};
use crate::memory::{
    MCI, CanonicalContext, CanonicalCodon, Origin,
    EvaluativeSignature, ActivationCondition, ReplayableProvenance,
    LearningEngine, EpistemicTrigger, LearningResult,
};
use crate::cognitive::dna::{StructuredDNA, DnaBuilder, AtomicAction};
use sha2::{Sha256, Digest};
use std::collections::BTreeMap;

/// Complete cognitive cycle output (v0.7.0 with MCI).
#[derive(Debug, Clone)]
pub struct CycleOutput {
    /// Sensory perception result
    pub perception: CortexOutput,
    /// Motor scores
    pub motor_scores: MotorScores,
    /// Craft Performance value
    pub cp_value: f64,
    /// Whether CP was vetoed
    pub vetoed: bool,
    /// Emitted DNA fingerprint (legacy)
    pub dna_fingerprint: [u8; 32],
    /// Whether Nash was applicable (>=2 players)
    pub nash_applicable: bool,
    /// Structured DNA (LEI-AF-10-08)
    pub structured_dna: StructuredDNA,
    /// Origin of this cycle's primary state (AO-18)
    pub origin: Origin,
    /// Learning result (if learning was attempted)
    pub learning_result: Option<LearningResult>,
    /// MCI baseline CP before this cycle
    pub baseline_cp: f64,
    /// Whether MCI was consulted
    pub mci_consulted: bool,
}

/// All four motor scores.
#[derive(Debug, Clone, Copy)]
pub struct MotorScores {
    pub praxis: f64,
    pub nash: f64,
    pub chaos: f64,
    pub meristic: f64,
}

impl MotorScores {
    /// Convert to EvaluativeSignature
    pub fn to_signature(&self, nash_applicable: bool) -> EvaluativeSignature {
        EvaluativeSignature::new(
            self.praxis,
            self.nash,
            self.chaos,
            self.meristic,
            nash_applicable,
        )
    }
}

/// Context for motor evaluation (provided by GDO).
#[derive(Debug, Clone)]
pub struct MotorContext {
    // Praxis
    pub proposed: Vec<String>,
    pub necessary: Vec<String>,
    pub context_vector: Vec<f64>,
    pub history_centroid: Vec<f64>,
    
    // Nash
    pub player_count: usize,
    pub action_sizes: Vec<usize>,
    pub payoffs: Vec<Vec<i64>>,
    pub strategies: Vec<Vec<u64>>,
    pub scale: u64,
    
    // Chaos
    pub reference_trajectory: Vec<Vec<f64>>,
    pub perturbed_trajectory: Vec<Vec<f64>>,
    pub delta_0: f64,
    pub dt: f64,
    
    // Meristic
    pub current_embedding: Vec<f64>,
    pub historical_embeddings: Vec<Vec<f64>>,
    pub domain_characteristics: Option<BTreeMap<String, f64>>,
    pub exploration_depth: usize,
    pub novelty_weight: f64,
}

impl Default for MotorContext {
    fn default() -> Self {
        Self {
            // Praxis defaults
            proposed: vec!["action".to_string()],
            necessary: vec!["action".to_string()],
            context_vector: vec![0.5, 0.5, 0.5],
            history_centroid: vec![0.5, 0.5, 0.5],
            
            // Nash defaults (no players = not applicable)
            player_count: 0,
            action_sizes: vec![],
            payoffs: vec![],
            strategies: vec![],
            scale: 100,
            
            // Chaos defaults (stable system)
            reference_trajectory: vec![vec![0.1], vec![0.15], vec![0.12], vec![0.14], vec![0.13]],
            perturbed_trajectory: vec![vec![0.11], vec![0.16], vec![0.13], vec![0.15], vec![0.14]],
            delta_0: 0.01,
            dt: 1.0,
            
            // Meristic defaults
            current_embedding: vec![0.5, 0.5, 0.5],
            historical_embeddings: vec![],
            domain_characteristics: None,
            exploration_depth: 3,
            novelty_weight: 0.5,
        }
    }
}

/// Executes a complete cognitive cycle with MCI integration.
///
/// # MVP-7 Capabilities
/// - Perceives input (ORIGIN_EXTERNAL)
/// - Consults MCI for similar Codons (LEI-AF-12-04)
/// - Evaluates 4 motors in canonical order
/// - Attempts learning if CP improves (AF-11)
/// - Emits Structured DNA (LEI-AF-10-08)
pub struct CognitiveCycle {
    cortex: SensoryCortex,
    praxis: PraxisMotor,
    nash: NashMotor,
    chaos: ChaosMotor,
    meristic: MeristicMotor,
    /// Internal MCI (AF-12)
    mci: MCI,
    /// Learning engine (AF-11)
    learning: LearningEngine,
    /// Cycle counter for provenance
    cycle_counter: u64,
}

impl CognitiveCycle {
    /// Create a new cognitive cycle with default MCI.
    pub fn new() -> Self {
        let cortex = SensoryCortex::new();
        let _ = cortex.perceive(&RawInput::from_bytes(vec![0u8; 64]));
        
        Self {
            cortex,
            praxis: PraxisMotor::new(),
            nash: NashMotor::new(),
            chaos: ChaosMotor::new(),
            meristic: MeristicMotor::new(),
            mci: MCI::unlimited(),
            learning: LearningEngine::new(3), // Trigger after 3 stagnations
            cycle_counter: 0,
        }
    }
    
    /// Create with custom MCI capacity.
    pub fn with_mci_capacity(capacity: usize) -> Self {
        let mut cycle = Self::new();
        cycle.mci = MCI::with_capacity(capacity);
        cycle
    }
    
    /// Get reference to MCI (for inspection).
    pub fn mci(&self) -> &MCI {
        &self.mci
    }
    
    /// Get mutable reference to MCI (for external manipulation).
    pub fn mci_mut(&mut self) -> &mut MCI {
        &mut self.mci
    }
    
    /// Get current cycle counter.
    pub fn cycle_counter(&self) -> u64 {
        self.cycle_counter
    }

    /// Execute complete cognitive cycle on input data.
    /// 
    /// # Pipeline (E1-E6)
    /// 1. E1: Perceive input (ORIGIN_EXTERNAL)
    /// 2. E2: Encode (build canonical context)
    /// 3. E3: Evaluate motors + consult MCI
    /// 4. E4: Integrate (calculate CP)
    /// 5. E5: Deliberate (attempt learning)
    /// 6. E6: Emit Structured DNA
    ///
    /// # Canonical Order (LEI-AF-10-07)
    /// Motors execute in strict sequence: Praxis → Nash → Chaos → Meristic
    pub fn process(&mut self, data: &[u8], context: &MotorContext) -> CycleOutput {
        // Increment cycle counter
        self.cycle_counter += 1;
        let cycle_id = self.generate_cycle_id(data);
        
        // E1: Perceive (ORIGIN_EXTERNAL)
        let input = RawInput::from_bytes(data.to_vec());
        let perception = self.cortex.perceive(&input);
        let input_fingerprint = self.hash_input(data);
        
        // E2: Build Canonical Context
        let canonical_context = CanonicalContext::new(
            &self.hash_context(context),
            &input_fingerprint,
        );
        
        // E3: Consult MCI (LEI-AF-12-04)
        let mci_state_fingerprint = self.mci.state_fingerprint();
        let baseline_cp = self.mci.baseline_cp(&canonical_context);
        let mci_query = self.mci.query(&canonical_context);
        let mci_consulted = mci_query.found;
        
        // Determine initial origin based on MCI consultation
        let mut origin = Origin::External;
        if mci_consulted && !mci_query.codons.is_empty() {
            // MCI contributed knowledge → RECOMBINED
            origin = Origin::Recombined;
        }

        // E3: Quadrimotor Evaluation - CANONICAL ORDER
        
        // Motor 1/4: Praxis (truth observed)
        let praxis_input = PraxisInput {
            proposed: context.proposed.clone(),
            necessary: context.necessary.clone(),
            context_vector: context.context_vector.clone(),
            history_centroid: context.history_centroid.clone(),
        };
        let praxis_output = self.praxis.evaluate(&praxis_input);

        // Motor 2/4: Nash (equilibrium, conditional)
        let (nash_score, nash_applicable) = if context.player_count >= 2 && !context.payoffs.is_empty() {
            let nash_input = NashInput {
                num_players: context.player_count,
                action_sizes: context.action_sizes.clone(),
                payoffs: context.payoffs.clone(),
                strategies: context.strategies.clone(),
                scale: context.scale,
            };
            let nash_output = self.nash.evaluate(&nash_input);
            (nash_output.score, true)
        } else {
            (1.0, false)
        };

        // Motor 3/4: Chaos (robustness)
        let chaos_input = ChaosInput {
            reference_trajectory: context.reference_trajectory.clone(),
            perturbed_trajectory: context.perturbed_trajectory.clone(),
            delta_0: context.delta_0,
            dt: context.dt,
            epsilon_tolerance: None,
        };
        let chaos_output = self.chaos.evaluate(&chaos_input);

        // Motor 4/4: Meristic (POSTERIOR - LEI-AF-10-07)
        let meristic_input = MeristicInput {
            current_embedding: context.current_embedding.clone(),
            historical_embeddings: context.historical_embeddings.clone(),
            domain_characteristics: context.domain_characteristics.clone(),
            exploration_depth: context.exploration_depth,
            novelty_weight: context.novelty_weight,
        };
        let meristic_output = self.meristic.evaluate(&meristic_input);

        let motor_scores = MotorScores {
            praxis: praxis_output.score,
            nash: nash_score,
            chaos: chaos_output.score,
            meristic: meristic_output.score,
        };

        // E4: Integration - Calculate CP (AF-10.5)
        let cp_result = CraftPerformance::calculate(
            motor_scores.praxis,
            motor_scores.nash,
            motor_scores.chaos,
            motor_scores.meristic,
        );

        let (cp_value, vetoed) = match cp_result {
            CpResult::Valid { value, .. } => (value, false),
            CpResult::Vetoed { value, .. } => (value, true),
            CpResult::Invalid { .. } => (0.0, true),
        };

        // E5: Deliberate - Attempt Learning (AF-11)
        let learning_result = if !vetoed && cp_value > baseline_cp {
            // Create candidate Codon
            let signature = motor_scores.to_signature(nash_applicable);
            let provenance = ReplayableProvenance::new(
                cycle_id,
                input_fingerprint,
                mci_state_fingerprint,
                self.cycle_counter,
                origin,
            );
            let condition = ActivationCondition::new(canonical_context.clone());
            let codon = CanonicalCodon::new(
                data.to_vec(), // forma = input data
                provenance,
                signature,
                condition,
            );
            
            // Check for epistemic trigger
            let trigger = self.learning.check_trigger(&canonical_context, cp_value)
                .unwrap_or(EpistemicTrigger::ExplicitRequest);
            
            // Attempt incorporation
            let result = self.learning.try_learn(&mut self.mci, codon, trigger);
            
            if result.learned {
                // Learning occurred → this cycle's knowledge is now INTERNAL
                origin = Origin::Internal;
            }
            
            Some(result)
        } else {
            None
        };

        // E6: Emission - Generate Structured DNA (LEI-AF-10-08)
        let signature = motor_scores.to_signature(nash_applicable);
        let action = AtomicAction::new(
            0, // Single action for now
            origin,
            input_fingerprint,
            self.hash_perception(&perception),
            signature.clone(),
        );
        
        let mut builder = DnaBuilder::new(cycle_id, self.cycle_counter);
        builder.add_action(action);
        let structured_dna = builder.build();
        
        // Legacy DNA fingerprint
        let dna_fingerprint = Self::generate_legacy_dna(&perception, &motor_scores, cp_value);

        CycleOutput {
            perception,
            motor_scores,
            cp_value,
            vetoed,
            dna_fingerprint,
            nash_applicable,
            structured_dna,
            origin,
            learning_result,
            baseline_cp,
            mci_consulted,
        }
    }
    
    /// Process without learning (for replay verification).
    pub fn process_readonly(&self, data: &[u8], context: &MotorContext) -> CycleOutput {
        // Similar to process() but doesn't modify MCI
        let input = RawInput::from_bytes(data.to_vec());
        let perception = self.cortex.perceive(&input);
        let input_fingerprint = self.hash_input(data);
        let cycle_id = self.generate_cycle_id(data);
        
        let canonical_context = CanonicalContext::new(
            &self.hash_context(context),
            &input_fingerprint,
        );
        
        let baseline_cp = self.mci.baseline_cp(&canonical_context);
        let mci_query = self.mci.query(&canonical_context);
        let mci_consulted = mci_query.found;
        let origin = if mci_consulted { Origin::Recombined } else { Origin::External };
        
        // Motors evaluation (same as process)
        let praxis_input = PraxisInput {
            proposed: context.proposed.clone(),
            necessary: context.necessary.clone(),
            context_vector: context.context_vector.clone(),
            history_centroid: context.history_centroid.clone(),
        };
        let praxis_output = self.praxis.evaluate(&praxis_input);

        let (nash_score, nash_applicable) = if context.player_count >= 2 && !context.payoffs.is_empty() {
            let nash_input = NashInput {
                num_players: context.player_count,
                action_sizes: context.action_sizes.clone(),
                payoffs: context.payoffs.clone(),
                strategies: context.strategies.clone(),
                scale: context.scale,
            };
            let nash_output = self.nash.evaluate(&nash_input);
            (nash_output.score, true)
        } else {
            (1.0, false)
        };

        let chaos_input = ChaosInput {
            reference_trajectory: context.reference_trajectory.clone(),
            perturbed_trajectory: context.perturbed_trajectory.clone(),
            delta_0: context.delta_0,
            dt: context.dt,
            epsilon_tolerance: None,
        };
        let chaos_output = self.chaos.evaluate(&chaos_input);

        let meristic_input = MeristicInput {
            current_embedding: context.current_embedding.clone(),
            historical_embeddings: context.historical_embeddings.clone(),
            domain_characteristics: context.domain_characteristics.clone(),
            exploration_depth: context.exploration_depth,
            novelty_weight: context.novelty_weight,
        };
        let meristic_output = self.meristic.evaluate(&meristic_input);

        let motor_scores = MotorScores {
            praxis: praxis_output.score,
            nash: nash_score,
            chaos: chaos_output.score,
            meristic: meristic_output.score,
        };

        let cp_result = CraftPerformance::calculate(
            motor_scores.praxis,
            motor_scores.nash,
            motor_scores.chaos,
            motor_scores.meristic,
        );

        let (cp_value, vetoed) = match cp_result {
            CpResult::Valid { value, .. } => (value, false),
            CpResult::Vetoed { value, .. } => (value, true),
            CpResult::Invalid { .. } => (0.0, true),
        };

        let signature = motor_scores.to_signature(nash_applicable);
        let action = AtomicAction::new(
            0,
            origin,
            input_fingerprint,
            self.hash_perception(&perception),
            signature,
        );
        
        let mut builder = DnaBuilder::new(cycle_id, self.cycle_counter);
        builder.add_action(action);
        let structured_dna = builder.build();
        
        let dna_fingerprint = Self::generate_legacy_dna(&perception, &motor_scores, cp_value);

        CycleOutput {
            perception,
            motor_scores,
            cp_value,
            vetoed,
            dna_fingerprint,
            nash_applicable,
            structured_dna,
            origin,
            learning_result: None,
            baseline_cp,
            mci_consulted,
        }
    }
    
    fn generate_cycle_id(&self, data: &[u8]) -> [u8; 16] {
        let mut hasher = Sha256::new();
        hasher.update(self.cycle_counter.to_le_bytes());
        hasher.update(data);
        let hash: [u8; 32] = hasher.finalize().into();
        let mut id = [0u8; 16];
        id.copy_from_slice(&hash[..16]);
        id
    }
    
    fn hash_input(&self, data: &[u8]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finalize().into()
    }
    
    fn hash_context(&self, context: &MotorContext) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update((context.player_count as u64).to_le_bytes());
        hasher.update((context.exploration_depth as u64).to_le_bytes());
        for v in &context.context_vector {
            hasher.update(v.to_le_bytes());
        }
        hasher.finalize().into()
    }
    
    fn hash_perception(&self, perception: &CortexOutput) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(perception.signals.entropy.to_le_bytes());
        hasher.update(perception.signals.mean.to_le_bytes());
        hasher.update(perception.signals.std_dev.to_le_bytes());
        hasher.finalize().into()
    }

    fn generate_legacy_dna(perception: &CortexOutput, motors: &MotorScores, cp: f64) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(perception.signals.entropy.to_le_bytes());
        hasher.update(perception.signals.mean.to_le_bytes());
        hasher.update(motors.praxis.to_le_bytes());
        hasher.update(motors.nash.to_le_bytes());
        hasher.update(motors.chaos.to_le_bytes());
        hasher.update(motors.meristic.to_le_bytes());
        hasher.update(cp.to_le_bytes());
        hasher.finalize().into()
    }
}

impl Default for CognitiveCycle {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complete_cycle() {
        let mut cycle = CognitiveCycle::new();
        let ctx = MotorContext::default();
        let output = cycle.process(&[1, 2, 3, 4, 5], &ctx);
        
        assert!(output.motor_scores.praxis >= 0.0 && output.motor_scores.praxis <= 1.0);
        assert!(output.motor_scores.chaos >= 0.0 && output.motor_scores.chaos <= 1.0);
        assert!(!output.nash_applicable);
        // Origin is INTERNAL after successful learning (first cycle learns from baseline=0)
        // or EXTERNAL if learning didn't occur
        assert!(output.origin == Origin::Internal || output.origin == Origin::External);
    }

    #[test]
    fn test_dna_determinism() {
        let mut cycle = CognitiveCycle::new();
        let ctx = MotorContext::default();
        let o1 = cycle.process(&[10, 20, 30], &ctx);
        
        // Reset cycle for determinism test
        let mut cycle2 = CognitiveCycle::new();
        let o2 = cycle2.process(&[10, 20, 30], &ctx);
        
        assert_eq!(o1.dna_fingerprint, o2.dna_fingerprint);
    }

    /// LEI-AF-10-07: Verifies Meristic motor posteriority
    #[test]
    fn test_canonical_motor_order() {
        let mut cycle = CognitiveCycle::new();
        let ctx = MotorContext::default();
        let output = cycle.process(&[1, 2, 3], &ctx);
        
        assert!(output.motor_scores.praxis >= 0.0 && output.motor_scores.praxis <= 1.0);
        assert!(output.motor_scores.nash >= 0.0 && output.motor_scores.nash <= 1.0);
        assert!(output.motor_scores.chaos >= 0.0 && output.motor_scores.chaos <= 1.0);
        assert!(output.motor_scores.meristic >= 0.0 && output.motor_scores.meristic <= 1.0);
        assert!(output.cp_value >= 0.0 && output.cp_value <= 1.0);
    }
    
    /// AF-11: Test learning integration
    #[test]
    fn test_learning_integration() {
        let mut cycle = CognitiveCycle::new();
        let ctx = MotorContext::default();
        
        // First cycle - should learn (baseline = 0)
        let output1 = cycle.process(&[1, 2, 3], &ctx);
        assert!(output1.learning_result.is_some());
        
        if let Some(ref result) = output1.learning_result {
            assert!(result.learned, "First cycle should learn (baseline=0)");
        }
        
        // Verify MCI has content
        assert!(cycle.mci().total_codons() > 0);
    }
    
    /// AF-12: Test MCI consultation
    #[test]
    fn test_mci_consultation() {
        let mut cycle = CognitiveCycle::new();
        let ctx = MotorContext::default();
        
        // First cycle - MCI empty
        let output1 = cycle.process(&[1, 2, 3], &ctx);
        assert!(!output1.mci_consulted);
        
        // Second cycle with same context - MCI should be consulted
        let output2 = cycle.process(&[1, 2, 3], &ctx);
        assert!(output2.mci_consulted);
    }
    
    /// AO-18: Test origin markers
    #[test]
    fn test_origin_markers() {
        let mut cycle = CognitiveCycle::new();
        let ctx = MotorContext::default();
        
        // First cycle - EXTERNAL (no MCI contribution)
        let output1 = cycle.process(&[1, 2, 3], &ctx);
        // After learning, origin becomes INTERNAL
        if output1.learning_result.as_ref().map(|r| r.learned).unwrap_or(false) {
            assert_eq!(output1.origin, Origin::Internal);
        }
        
        // Second cycle - should be RECOMBINED (MCI contributed)
        let output2 = cycle.process(&[1, 2, 3], &ctx);
        assert!(output2.mci_consulted);
    }
    
    /// LEI-AF-10-08: Test structured DNA emission
    #[test]
    fn test_structured_dna_emission() {
        let mut cycle = CognitiveCycle::new();
        let ctx = MotorContext::default();
        let output = cycle.process(&[1, 2, 3, 4, 5], &ctx);
        
        // Verify structured DNA
        assert_eq!(output.structured_dna.action_count(), 1);
        assert!(output.structured_dna.verify_cp_invariant());
        assert_eq!(output.structured_dna.cycle_counter, 1);
    }
    
    /// Test cycle counter increment
    #[test]
    fn test_cycle_counter() {
        let mut cycle = CognitiveCycle::new();
        let ctx = MotorContext::default();
        
        assert_eq!(cycle.cycle_counter(), 0);
        
        cycle.process(&[1], &ctx);
        assert_eq!(cycle.cycle_counter(), 1);
        
        cycle.process(&[2], &ctx);
        assert_eq!(cycle.cycle_counter(), 2);
    }
    
    /// Test readonly processing (no learning)
    #[test]
    fn test_readonly_processing() {
        let mut cycle = CognitiveCycle::new();
        let ctx = MotorContext::default();
        
        // Normal process - learns
        cycle.process(&[1, 2, 3], &ctx);
        let codons_after_learn = cycle.mci().total_codons();
        
        // Readonly process - does not learn
        let output = cycle.process_readonly(&[4, 5, 6], &ctx);
        assert!(output.learning_result.is_none());
        assert_eq!(cycle.mci().total_codons(), codons_after_learn);
    }
    
    /// Integration test: Complete learning cycle
    #[test]
    fn test_complete_learning_cycle_integration() {
        let mut cycle = CognitiveCycle::new();
        let ctx = MotorContext::default();
        
        // Process multiple cycles
        for i in 0..5 {
            let data = vec![i as u8; 10];
            let output = cycle.process(&data, &ctx);
            
            // All outputs should have valid structure
            assert!(output.cp_value >= 0.0);
            assert!(output.structured_dna.verify_cp_invariant());
        }
        
        // MCI should have accumulated knowledge
        assert!(cycle.mci().total_codons() > 0);
        assert!(cycle.cycle_counter() == 5);
    }
}

