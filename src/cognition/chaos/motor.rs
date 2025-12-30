//! --------------------------
//! INFORMATION
//! --------------------------
//! Title: Chaotic Motor Engine
//! Author: Carlos Eduardo Favini
//! Date: 2025-12-30
//! Version: 1.2.0
//! Description: Implements the M_C logic using deterministic integer math.
//!              NO FLOATING POINT OPERATIONS ALLOWED.
//! Layer: Community
//! Dependencies: types
//! Affected Components: reactor
//!
//! --------------------------
//! CHANGE LOG
//! --------------------------
//! 2025-12-30 - Enforced Absolute Determinism (Integer Math).
//! 2025-12-30 - Refactored for Community.
//! --------------------------

use super::types::{
    ChaoticAssessment, ContractualProximity, PerturbationPath, ChaoticTensionField, CHAOS_SCALE
};

// --- STRUCTURAL CONSTANTS (SCALED) ---
// Scale: 1,000,000 = 1.0

const PROXIMITY_ALIGNED_LIMIT: u64 = 100_000;    // 0.1
const PROXIMITY_DIVERGENT_LIMIT: u64 = 400_000;  // 0.4
const PROXIMITY_CRITICAL_LIMIT: u64 = 800_000;   // 0.8

const PERTURBATION_PROBABILITY_THRESHOLD: u64 = 300_000; // 0.3
// Stress Multiplier 0.5 becomes division by 2 or multiplication by 500,000/SCALE
const STRESS_SENSITIVITY_NUMERATOR: u64 = 5; 
const STRESS_SENSITIVITY_DENOMINATOR: u64 = 10;

/// The Chaotic Motor (M_C).
/// Evaluates structure robustness using fixed-point arithmetic.
pub struct ChaoticMotor;

impl ChaoticMotor {
    pub fn new() -> Self {
        Self
    }

    /// Calculates aggregate sensitivity based on Inelasticity.
    /// Uses integer math.
    fn calculate_sensitivity(&self, field: &ChaoticTensionField) -> u64 {
        if field.dimensions.is_empty() {
            return 0;
        }

        let count = field.dimensions.len() as u64;
        let total_rigidity: u64 = field.dimensions.iter()
            .map(|d| CHAOS_SCALE.saturating_sub(d.elasticity))
            .sum();
            
        let avg_rigidity = total_rigidity / count;
        
        // Stress Factor Calculation
        // Factor = 1.0 + (Stress * 0.5)
        // Scaled: SCALE + (Stress * 5 / 10)
        let stress_part = (field.net_system_stress * STRESS_SENSITIVITY_NUMERATOR) / STRESS_SENSITIVITY_DENOMINATOR;
        let stress_factor = CHAOS_SCALE + stress_part;
        
        // Result = (AvgRigidity * StressFactor) / SCALE
        // Use u128 for intermediate multiplication to avoid overflow
        let result = (avg_rigidity as u128 * stress_factor as u128) / CHAOS_SCALE as u128;
        
        // Clamp to SCALE
        if result > CHAOS_SCALE as u128 {
            CHAOS_SCALE
        } else {
            result as u64
        }
    }

    /// Determines contractual proximity.
    fn calculate_proximity(&self, stress: u64) -> ContractualProximity {
        if stress < PROXIMITY_ALIGNED_LIMIT { return ContractualProximity::Aligned; }
        if stress < PROXIMITY_DIVERGENT_LIMIT { return ContractualProximity::SlightlyDivergent; }
        if stress < PROXIMITY_CRITICAL_LIMIT { return ContractualProximity::PotentiallyCritical; }
        ContractualProximity::OutOfEnvelope
    }

    /// Maps potential butterfly effects.
    fn map_perturbations(&self, field: &ChaoticTensionField) -> Vec<PerturbationPath> {
        let mut paths = Vec::new();

        for pressure in &field.cross_pressures {
            if let Some(source_dim) = field.dimensions.iter().find(|d| d.name == pressure.source) {
                let rigidity = CHAOS_SCALE.saturating_sub(source_dim.elasticity);
                
                // Probability = Rigidity * |Magnitude|
                let abs_magnitude = pressure.magnitude.abs() as u64;
                
                // Scaled Multiplication: (R * M) / SCALE
                let probability = (rigidity as u128 * abs_magnitude as u128) / CHAOS_SCALE as u128;
                let probability_u64 = probability as u64;

                if probability_u64 > PERTURBATION_PROBABILITY_THRESHOLD {
                    let terminal = if pressure.magnitude < 0 {
                        format!("Collapse of {}", pressure.target)
                    } else {
                        format!("Over-saturation of {}", pressure.target)
                    };

                    paths.push(PerturbationPath {
                        trigger: pressure.source.clone(),
                        propagation: vec![pressure.target.clone()],
                        terminal_state: terminal,
                        probability: if probability_u64 > CHAOS_SCALE { CHAOS_SCALE } else { probability_u64 },
                    });
                }
            }
        }
        
        paths
    }

    pub fn assess(&self, context: &ChaoticTensionField) -> ChaoticAssessment {
        let sensitivity = self.calculate_sensitivity(context);
        let proximity = self.calculate_proximity(context.net_system_stress);
        let paths = self.map_perturbations(context);
        
        ChaoticAssessment {
            sensitivity_score: sensitivity,
            attractor_distance: proximity,
            perturbation_paths: paths,
        }
    }
}