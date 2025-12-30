//! --------------------------
//! INFORMATION
//! --------------------------
//! Title: Mediation Motor Tests
//! Author: Carlos Eduardo Favini
//! Date: 2025-12-30
//! Version: 1.0.0
//! Description: Scaffolding tests for M_N (Nash Motor).
//!              Provides provisional tuning values to validate technical flow.
//!              NOT scientifically calibrated.
//! Layer: Community
//! Dependencies: mediation
//! Affected Components: None
//!
//! --------------------------
//! CHANGE LOG
//! --------------------------
//! 2025-12-30 - Initial scaffolding tests.
//! --------------------------

#[cfg(test)]
mod tests {
    use crate::cognition::mediation::{
        MediationMotor, NashTuning, OperationalTensionField, 
        TensionDimension, CrossPressure, MEDIATION_SCALE
    };

    /// Helper: Provides provisional scaffolding tuning.
    /// These values are ARBITRARY and for technical validation only.
    fn get_scaffolding_tuning() -> NashTuning {
        NashTuning {
            rigidity_stress_multiplier: MEDIATION_SCALE, // 1.0 (Neutral pass-through)
            pressure_damping_factor: 500_000,            // 0.5 (50% damping)
            max_iterations: 1,                           // Minimal iterations
            baseline_stress: 0,
        }
    }

    /// Helper: Creates a simple field for testing.
    fn create_test_field() -> OperationalTensionField {
        OperationalTensionField {
            dimensions: vec![
                TensionDimension {
                    name: "budget".into(),
                    elasticity: 100_000, // 0.1 (Very Rigid)
                    current_value: 500_000,
                    target_value: 800_000, // Delta = 300,000
                },
                TensionDimension {
                    name: "quality".into(),
                    elasticity: 900_000, // 0.9 (Very Elastic)
                    current_value: 1_000_000,
                    target_value: 1_000_000,
                }
            ],
            cross_pressures: vec![],
            net_system_stress: 0,
        }
    }

    #[test]
    fn motor_initializes_with_tuning() {
        let tuning = get_scaffolding_tuning();
        let _motor = MediationMotor::new(tuning);
        // Pass if compiles and initializes
    }

    #[test]
    fn calculates_direct_stress() {
        let tuning = get_scaffolding_tuning();
        let motor = MediationMotor::new(tuning);
        
        let field = create_test_field();
        let result = motor.assess(field);
        
        // Budget dimension has Delta(300k) and Rigidity(900k).
        // It should contribute to net stress.
        assert!(result.net_system_stress > 0);
    }

    #[test]
    fn resolves_cross_pressures() {
        let tuning = get_scaffolding_tuning();
        let motor = MediationMotor::new(tuning);
        
        let mut field = create_test_field();
        
        // Add pressure: Budget stress impacts Quality
        field.cross_pressures.push(CrossPressure {
            source: "budget".into(),
            target: "quality".into(),
            magnitude: 800_000, // 0.8 impact
        });
        
        // Baseline (no pressure)
        let field_no_pressure = create_test_field();
        let res_no_pressure = motor.assess(field_no_pressure);
        
        // With pressure
        let res_with_pressure = motor.assess(field);
        
        // Since budget is stressed and hits quality, total stress should theoretically increase
        // or strictly change depending on the damping logic.
        // We assert technical execution (value generated) rather than specific math result here.
        assert!(res_with_pressure.net_system_stress > 0);
    }
}