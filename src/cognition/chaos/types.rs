//! --------------------------
//! INFORMATION
//! --------------------------
//! Title: Chaotic Motor Data Types
//! Author: Carlos Eduardo Favini
//! Date: 2025-12-30
//! Version: 1.2.0
//! Description: Defines the vocabulary for M_C (Canonical Chaotic Motor).
//!              Strictly deterministic: uses integer scaling (Fixed Point).
//!              Scale: 1.0 = 1,000,000.
//! Layer: Community
//! Dependencies: serde
//! Affected Components: cognition::chaos::motor
//!
//! --------------------------
//! CHANGE LOG
//! --------------------------
//! 2025-12-30 - Enforced Absolute Determinism (Removed f64).
//! 2025-12-30 - Refactored for Community.
//! 2025-12-27 - Initial creation for Phase 2
//! --------------------------

use serde::{Deserialize, Serialize};

/// Canonical Scaling Factor for Deterministic Math.
/// 1.0 = 1,000,000.
pub const CHAOS_SCALE: u64 = 1_000_000;

/// Distance from the contractual attractor (Stability).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContractualProximity {
    Aligned,
    SlightlyDivergent,
    PotentiallyCritical,
    OutOfEnvelope,
}

/// A mapped path of potential failure propagation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerturbationPath {
    pub trigger: String,
    pub propagation: Vec<String>,
    pub terminal_state: String,
    
    /// Probability scaled by CHAOS_SCALE (0 to 1,000,000).
    pub probability: u64,
}

/// The result of a canonical chaotic assessment (M_C).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChaoticAssessment {
    /// Sensitivity scaled by CHAOS_SCALE (0 = Robust, 1,000,000 = Fragile).
    pub sensitivity_score: u64,
    
    pub attractor_distance: ContractualProximity,
    
    pub perturbation_paths: Vec<PerturbationPath>,
}

// --- LOCAL STRUCTURAL DEFINITIONS ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChaoticTensionDimension {
    pub name: String,
    
    /// Elasticity scaled by CHAOS_SCALE (0 to 1,000,000).
    pub elasticity: u64,
    
    /// Value scaled by CHAOS_SCALE.
    pub current_value: i64,
    
    /// Target scaled by CHAOS_SCALE.
    pub target_value: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChaoticCrossPressure {
    pub source: String,
    pub target: String,
    
    /// Magnitude scaled by CHAOS_SCALE (can be negative).
    pub magnitude: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChaoticTensionField {
    pub dimensions: Vec<ChaoticTensionDimension>,
    pub cross_pressures: Vec<ChaoticCrossPressure>,
    
    /// System stress scaled by CHAOS_SCALE.
    pub net_system_stress: u64,
}