//! --------------------------
//! INFORMATION
//! --------------------------
//! Title: Chaotic Motor Tests
//! Author: Carlos Eduardo Favini
//! Date: 2025-12-30
//! Version: 1.2.0
//! Description: Verifies sensitivity analysis using deterministic integers.
//! Layer: Community
//! Dependencies: chaos
//! Affected Components: None
//!
//! --------------------------
//! CHANGE LOG
//! --------------------------
//! 2025-12-30 - Adapted for Deterministic Integers.
//! 2025-12-27 - Initial creation
//! --------------------------

#[cfg(test)]
mod tests {
    use crate::cognition::chaos::{
        ChaoticMotor, ContractualProximity, 
        ChaoticTensionField, ChaoticTensionDimension, ChaoticCrossPressure,
        CHAOS_SCALE
    };

    /// Helper to create a specific tension field using scaled integers.
    /// rigidity_factor: 0.0 to 1.0 represented as 0 to CHAOS_SCALE
    fn create_field(rigidity_scaled: u64, stress_scaled: u64) -> ChaoticTensionField {
        let elasticity = CHAOS_SCALE.saturating_sub(rigidity_scaled);
        
        let dim = ChaoticTensionDimension {
            name: "test_dim".into(),
            // Values are arbitrary scaled units
            current_value: 500_000 + (stress_scaled as i64 / 2),
            target_value: 500_000,
            elasticity,
        };
        
        ChaoticTensionField {
            dimensions: vec![dim],
            cross_pressures: vec![],
            net_system_stress: stress_scaled,
        }
    }

    #[test]
    fn high_rigidity_increases_sensitivity() {
        let motor = ChaoticMotor::new();
        
        // Case 1: High Rigidity (0.9 -> 900,000), Low Stress
        let rigid_field = create_field(900_000, 0);
        let assessment_rigid = motor.assess(&rigid_field);
        
        // Case 2: Low Rigidity (0.1 -> 100,000), Low Stress
        let flexible_field = create_field(100_000, 0);
        let assessment_flex = motor.assess(&flexible_field);
        
        // Expect rigid system to be more sensitive
        assert!(assessment_rigid.sensitivity_score > assessment_flex.sensitivity_score);
    }

    #[test]
    fn stress_shifts_attractor_distance() {
        let motor = ChaoticMotor::new();
        
        // Case 1: Low Stress (0.05 -> 50,000)
        let safe_field = create_field(500_000, 50_000);
        let safe = motor.assess(&safe_field);
        assert_eq!(safe.attractor_distance, ContractualProximity::Aligned);
        
        // Case 2: Critical Stress (0.9 -> 900,000)
        let critical_field = create_field(500_000, 900_000);
        let critical = motor.assess(&critical_field);
        assert_eq!(critical.attractor_distance, ContractualProximity::OutOfEnvelope);
    }

    #[test]
    fn detects_butterfly_effect() {
        let motor = ChaoticMotor::new();
        
        let dimensions = vec![
            ChaoticTensionDimension {
                name: "time".into(),
                current_value: 500_000, target_value: 500_000,
                elasticity: 100_000, // Rigid (0.1)
            },
            ChaoticTensionDimension {
                name: "cost".into(),
                current_value: 500_000, target_value: 500_000,
                elasticity: 500_000, // 0.5
            }
        ];
        
        // Magnitude -0.9 -> -900,000
        let pressures = vec![
            ChaoticCrossPressure {
                source: "time".into(),
                target: "cost".into(),
                magnitude: -900_000, 
            }
        ];
        
        let field = ChaoticTensionField {
            dimensions,
            cross_pressures: pressures,
            net_system_stress: 0,
        };
        
        let assessment = motor.assess(&field);
        
        assert!(!assessment.perturbation_paths.is_empty());
        let path = &assessment.perturbation_paths[0];
        
        assert_eq!(path.trigger, "time");
        assert_eq!(path.propagation[0], "cost");
        
        // Prob = Rigidity(0.9) * Magnitude(0.9) = 0.81 (810,000)
        // Threshold is 0.3 (300,000)
        assert!(path.probability > 300_000);
    }
}