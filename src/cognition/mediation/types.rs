//! --------------------------
//! INFORMATION
//! --------------------------
//! Title: Mediation/Nash Types (M_N)
//! Author: Carlos Eduardo Favini
//! Date: 2025-12-30
//! Version: 1.0.0
//! Description: Defines vocabulary for the Canonical Mediation/Nash Motor (M_N).
//!              Includes OperationalTensionField, TensionDimension, and CrossPressure.
//!              Strictly deterministic: uses integer scaling (1.0 = 1,000,000).
//! Layer: Community
//! Dependencies: serde
//! Affected Components: cognition::mediation::motor
//!
//! --------------------------
//! CHANGE LOG
//! --------------------------
//! 2025-12-30 - Migrated to Community. Replaced f64 with u64/i64 scale.
//! 2025-12-30 - Initial creation for Phase 2.4
//! --------------------------

use serde::{Deserialize, Serialize};

/// Canonical Scaling Factor for Deterministic Math in Mediation Motor.
/// 1.0 = 1,000,000.
pub const MEDIATION_SCALE: u64 = 1_000_000;

/// Represents a single dimension of tension within the operational field.
/// E.g., "Budget", "Latency", "Compliance".
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TensionDimension {
    pub name: String,
    
    /// Elasticity scaled by MEDIATION_SCALE (0 to 1,000,000).
    /// 0.0 = Rigid (Fragile), 1.0 = Highly Elastic (Adaptive).
    pub elasticity: u64,
    
    /// Current measured value scaled by MEDIATION_SCALE.
    /// Can be relative to a baseline.
    pub current_value: i64,
    
    /// Target ideal value scaled by MEDIATION_SCALE.
    pub target_value: i64,
}

/// Represents a pressure vector between two dimensions.
/// "If I pull on A, does B break?"
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossPressure {
    pub source: String,
    pub target: String,
    
    /// Magnitude of the pressure scaled by MEDIATION_SCALE.
    /// Negative values imply trade-offs (inverse correlation).
    /// Positive values imply synergy (direct correlation).
    pub magnitude: i64,
}

/// The core structural context for decision making.
/// Represents the "shape" of the problem space (Factor C).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationalTensionField {
    /// The dimensions defining the field.
    pub dimensions: Vec<TensionDimension>,
    
    /// The relationships and pressures between dimensions.
    pub cross_pressures: Vec<CrossPressure>,
    
    /// Net system stress scaled by MEDIATION_SCALE.
    /// Aggregate measure of how far the system is from equilibrium.
    pub net_system_stress: u64,
}