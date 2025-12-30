//! --------------------------
//! INFORMATION
//! --------------------------
//! Title: Meristic Types (M_M)
//! Author: Carlos Eduardo Favini
//! Date: 2025-12-30
//! Version: 1.1.1
//! Description: Defines vocabulary for the Canonical Meristic Meta-Motor (M_M).
//!              Strictly structural and deterministic.
//!              Updated to make identification fields optional to support
//!              initial migration phases where IDs are not yet propagated.
//! Layer: Community
//! Dependencies: serde, uuid
//! Affected Components: cognition::meristic::motor
//!
//! --------------------------
//! CHANGE LOG
//! --------------------------
//! 2025-12-30 - HOTFIX: Made variant_id and target_id optional (Option<Uuid>).
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StructuralTrigger {
    EntropyDelta(u64),
    PatternDivergence(u64),
    CrossModalVariance(u64),
    Unknown(u64),
}

/// A condition required to verify a hypothesis structurally.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VerificationConstraint {
    SampleCount(u64),
    CoherenceThreshold(u64),
    TopologyConsistency,
}

/// A proposed alternative structural variant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantHypothesis {
    /// Unique identifier for this structural variant.
    /// Optional during initial migration phases.
    pub variant_id: Option<Uuid>,
    
    pub scale: PatternScale,
    
    /// Confidence scaled by MERISTIC_SCALE.
    pub confidence: u64,
    
    /// Feasibility score scaled by MERISTIC_SCALE.
    pub feasibility_score: u64,
    
    pub requires_expansion: bool,
}

/// A hypothesis generated to explain a structural anomaly.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuralHypothesis {
    /// ID linking to the target signal or anomaly.
    /// Optional during initial migration phases.
    pub target_id: Option<Uuid>,
    
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