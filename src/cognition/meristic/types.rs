//! --------------------------
//! INFORMATION
//! --------------------------
//! Title: Meristic Types (M_M)
//! Author: Carlos Eduardo Favini
//! Date: 2025-12-30
//! Version: 1.1.0
//! Description: Defines vocabulary for the Canonical Meristic Meta-Motor (M_M).
//!              Strictly structural and deterministic.
//!              No natural language or semantic interpretation allowed.
//!              Uses Uuid for identification and u64 for scaling.
//! Layer: Community
//! Dependencies: serde, uuid
//! Affected Components: cognition::meristic::motor
//!
//! --------------------------
//! CHANGE LOG
//! --------------------------
//! 2025-12-30 - Removed natural language and semantic enums (Strict Structuralism).
//! 2025-12-30 - Migrated to Community with MERISTIC_SCALE.
//! --------------------------

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Canonical Scaling Factor for Deterministic Math in Meristic Motor.
/// 1.0 = 1,000,000.
pub const MERISTIC_SCALE: u64 = 1_000_000;

/// The topological scale at which a pattern is detected.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PatternScale {
    Micro, // Local topology
    Meso,  // Regional topology
    Macro, // Global topology
}

/// A structural metric triggering the hypothesis generation.
/// Replaces semantic "HypothesisSource".
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StructuralTrigger {
    EntropyDelta(u64),
    PatternDivergence(u64),
    CrossModalVariance(u64),
    Unknown(u64),
}

/// A condition required to verify a hypothesis structurally.
/// Replaces semantic "TestCondition".
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VerificationConstraint {
    SampleCount(u64),
    CoherenceThreshold(u64),
    TopologyConsistency,
}

/// A proposed alternative structural variant.
/// Description strings are replaced by unique IDs for Enterprise lookup.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantHypothesis {
    /// Unique identifier for this structural variant.
    pub variant_id: Uuid,
    
    pub scale: PatternScale,
    
    /// Confidence scaled by MERISTIC_SCALE.
    pub confidence: u64,
    
    /// Feasibility score scaled by MERISTIC_SCALE.
    pub feasibility_score: u64,
    
    pub requires_expansion: bool,
}

/// A hypothesis generated to explain a structural anomaly.
/// Replaces "InterpretationHypothesis" with strictly structural data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuralHypothesis {
    /// ID linking to the target signal or anomaly.
    pub target_id: Uuid,
    
    /// Estimated plausibility scaled by MERISTIC_SCALE.
    pub plausibility: u64,
    
    /// The structural cause that triggered this hypothesis.
    pub trigger: StructuralTrigger,
    
    /// Structural conditions required to verify this hypothesis.
    pub constraints: Vec<VerificationConstraint>,
}

/// The final assessment from M_M.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeristicAssessment {
    /// Score of informational completeness scaled by MERISTIC_SCALE.
    pub vividness_score: u64,
    
    /// Enumerated hypotheses for action variants.
    pub variants: Vec<VariantHypothesis>,
    
    /// Enumerated hypotheses for structural anomalies.
    pub anomalies: Vec<StructuralHypothesis>,
    
    /// Flag indicating if the motor operated within structural safety bounds.
    pub containment_breach: bool,
}