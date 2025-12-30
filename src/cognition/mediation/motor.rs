//! --------------------------
//! INFORMATION
//! --------------------------
//! Title: Mediation Motor Engine (Nash)
//! Author: Carlos Eduardo Favini
//! Date: 2025-12-30
//! Version: 1.0.0
//! Description: Implements M_N (Canonical Mediation/Nash Motor).
//!              Calculates System Stress and resolves Cross-Pressures.
//!              Strictly deterministic: uses integer scaling (1.0 = 1,000,000).
//!              SANITIZED: Consumes NashTuning for all heuristics.
//! Layer: Community
//! Dependencies: types
//! Affected Components: cognition::meristic::motor
//!
//! --------------------------
//! CHANGE LOG
//! --------------------------
//! 2025-12-30 - Initial migration to Community. Logic sanitized with NashTuning.
//! --------------------------

use super::types::{OperationalTensionField, TensionDimension, CrossPressure, MEDIATION_SCALE};

/// Configuration struct for Mediation/Nash heuristics.
/// Injected to prevent magic numbers in the core logic.
#[derive(Debug, Clone)]
pub struct NashTuning {
    /// Multiplier for stress impact when rigidity is high.
    pub rigidity_stress_multiplier: u64,
    
    /// Damping factor for cross-pressure propagation (avoids infinite loops).
    pub pressure_damping_factor: u64,
    
    /// Maximum allowed iterations for equilibrium verification (if applicable).
    pub max_iterations: u64,
    
    /// Baseline stress value to normalize results.
    pub baseline_stress: u64,
}

/// The Canonical Mediation Motor (M_N).
/// Calculates the Operational Tension Field's stress state.
pub struct MediationMotor {
    tuning: NashTuning,
}

impl MediationMotor {
    /// Creates a new Mediation Motor with injected tuning.
    pub fn new(tuning: NashTuning) -> Self {
        Self { tuning }
    }

    /// Calculates the raw stress of a single dimension ignoring cross-pressures.
    /// Stress = |Target - Current| * (1.0 - Elasticity) * TuningFactor
    fn calculate_dimensional_stress(&self, dim: &TensionDimension) -> u64 {
        let delta = (dim.target_value - dim.current_value).abs() as u64;
        
        // Rigidity = SCALE - Elasticity
        let rigidity = MEDIATION_SCALE.saturating_sub(dim.elasticity);
        
        // Raw Impact = (Delta * Rigidity) / SCALE
        let raw_impact = (delta as u128 * rigidity as u128) / MEDIATION_SCALE as u128;
        
        // Tuned Impact = (Raw * Multiplier) / SCALE
        let tuned_impact = (raw_impact * self.tuning.rigidity_stress_multiplier as u128) 
            / MEDIATION_SCALE as u128;
            
        if tuned_impact > MEDIATION_SCALE as u128 {
            MEDIATION_SCALE
        } else {
            tuned_impact as u64
        }
    }

    /// Calculates additional stress derived from Cross-Pressures.
    /// If Source is stressed, it transmits pressure to Target based on Magnitude.
    fn calculate_pressure_impact(
        &self, 
        dim: &TensionDimension, 
        all_dims: &[TensionDimension], 
        pressures: &[CrossPressure]
    ) -> u64 {
        let mut total_pressure: i64 = 0;

        // Find all pressures targeting this dimension
        for p in pressures.iter().filter(|p| p.target == dim.name) {
            if let Some(source) = all_dims.iter().find(|d| d.name == p.source) {
                let source_stress = self.calculate_dimensional_stress(source);
                
                // Transmitted Pressure = (SourceStress * Magnitude) / SCALE
                let transmission = (source_stress as i128 * p.magnitude as i128) 
                    / MEDIATION_SCALE as i128;
                
                // Damping
                let damped = (transmission * self.tuning.pressure_damping_factor as i128) 
                    / MEDIATION_SCALE as i128;

                total_pressure = total_pressure.saturating_add(damped as i64);
            }
        }

        // Return absolute pressure impact clipped to positive u64 range
        total_pressure.abs().min(MEDIATION_SCALE as i64) as u64
    }

    /// Evaluates the Tension Field and returns a new field with updated Net System Stress.
    /// This represents the "Deep Thought" assessment of the current state.
    pub fn assess(&self, mut field: OperationalTensionField) -> OperationalTensionField {
        if field.dimensions.is_empty() {
            field.net_system_stress = 0;
            return field;
        }

        let mut total_stress: u128 = 0;
        let count = field.dimensions.len() as u128;

        // Clone dimensions for immutable reference during pressure calculation
        let ref_dims = field.dimensions.clone();

        for dim in &field.dimensions {
            let local_stress = self.calculate_dimensional_stress(dim);
            let pressure_stress = self.calculate_pressure_impact(dim, &ref_dims, &field.cross_pressures);
            
            // Total dimension stress = Local + Pressure
            let dim_total = local_stress.saturating_add(pressure_stress);
            total_stress += dim_total as u128;
        }

        // Average Stress
        let avg_stress = (total_stress / count) as u64;
        
        // Normalize against baseline (optional scaling logic could go here)
        // For now, we clamp to scale.
        field.net_system_stress = if avg_stress > MEDIATION_SCALE {
            MEDIATION_SCALE
        } else {
            avg_stress
        };

        field
    }
}