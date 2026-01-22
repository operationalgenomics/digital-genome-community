//! Cognitive Cycle - Complete GDC processing cycle
//!
//! This module orchestrates the full cognitive cycle:
//! 1. Receive frame from GDO
//! 2. Perceive (sensory transduction)
//! 3. Evaluate motors (Praxis, Nash, Chaos, Meristic)
//! 4. Calculate Craft Performance (CP)
//! 5. Emit DNA

use crate::sensory::{RawInput, SensoryCortex, CortexOutput};
use crate::motors::{
    CognitiveMotor,
    PraxisMotor, PraxisInput,
    NashMotor, NashInput,
    ChaosMotor, ChaosInput,
    MeristicMotor, MeristicInput,
};
use crate::math::craft::{CraftPerformance, CpResult};
use sha2::{Sha256, Digest};
use std::collections::BTreeMap;

/// Complete cognitive cycle output.
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
    /// Emitted DNA fingerprint
    pub dna_fingerprint: [u8; 32],
    /// Whether Nash was applicable (>=2 players)
    pub nash_applicable: bool,
}

/// All four motor scores.
#[derive(Debug, Clone, Copy)]
pub struct MotorScores {
    pub praxis: f64,
    pub nash: f64,
    pub chaos: f64,
    pub meristic: f64,
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

/// Executes a complete cognitive cycle.
pub struct CognitiveCycle {
    cortex: SensoryCortex,
    praxis: PraxisMotor,
    nash: NashMotor,
    chaos: ChaosMotor,
    meristic: MeristicMotor,
}

impl CognitiveCycle {
    pub fn new() -> Self {
        let cortex = SensoryCortex::new();
        let _ = cortex.perceive(&RawInput::from_bytes(vec![0u8; 64]));
        
        Self {
            cortex,
            praxis: PraxisMotor::new(),
            nash: NashMotor::new(),
            chaos: ChaosMotor::new(),
            meristic: MeristicMotor::new(),
        }
    }

    /// Execute complete cognitive cycle on input data.
    pub fn process(&self, data: &[u8], context: &MotorContext) -> CycleOutput {
        // 1. Perceive
        let input = RawInput::from_bytes(data.to_vec());
        let perception = self.cortex.perceive(&input);

        // 2. Evaluate Praxis
        let praxis_input = PraxisInput {
            proposed: context.proposed.clone(),
            necessary: context.necessary.clone(),
            context_vector: context.context_vector.clone(),
            history_centroid: context.history_centroid.clone(),
        };
        let praxis_output = self.praxis.evaluate(&praxis_input);

        // 3. Evaluate Nash (conditional)
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

        // 4. Evaluate Chaos
        let chaos_input = ChaosInput {
            reference_trajectory: context.reference_trajectory.clone(),
            perturbed_trajectory: context.perturbed_trajectory.clone(),
            delta_0: context.delta_0,
            dt: context.dt,
            epsilon_tolerance: None,
        };
        let chaos_output = self.chaos.evaluate(&chaos_input);

        // 5. Evaluate Meristic
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

        // 6. Calculate CP
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

        // 7. Generate DNA fingerprint
        let dna_fingerprint = Self::generate_dna(&perception, &motor_scores, cp_value);

        CycleOutput {
            perception,
            motor_scores,
            cp_value,
            vetoed,
            dna_fingerprint,
            nash_applicable,
        }
    }

    fn generate_dna(perception: &CortexOutput, motors: &MotorScores, cp: f64) -> [u8; 32] {
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
        let cycle = CognitiveCycle::new();
        let ctx = MotorContext::default();
        let output = cycle.process(&[1, 2, 3, 4, 5], &ctx);
        
        assert!(output.motor_scores.praxis >= 0.0 && output.motor_scores.praxis <= 1.0);
        assert!(output.motor_scores.chaos >= 0.0 && output.motor_scores.chaos <= 1.0);
        assert!(!output.nash_applicable);
    }

    #[test]
    fn test_dna_determinism() {
        let cycle = CognitiveCycle::new();
        let ctx = MotorContext::default();
        let o1 = cycle.process(&[10, 20, 30], &ctx);
        let o2 = cycle.process(&[10, 20, 30], &ctx);
        
        assert_eq!(o1.dna_fingerprint, o2.dna_fingerprint);
    }
}
