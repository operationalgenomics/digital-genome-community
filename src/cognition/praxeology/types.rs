//! --------------------------
//! INFORMATION
//! --------------------------
//! Title: Praxeological Types (M_P)
//! Author: Carlos Eduardo Favini
//! Date: 2025-12-30
//! Version: 1.2.0
//! Description: Defines the vocabulary for the Canonical Praxeological Motor (M_P).
//!              Strictly structural and deterministic.
//!              Removes all semantic classification (CarrierType) and interpretation (Intent).
//!              Focuses on functional asymmetry and means-ends coherence.
//! Layer: Community
//! Dependencies: serde
//! Affected Components: cognition::praxeology::motor
//!
//! --------------------------
//! CHANGE LOG
//! --------------------------
//! 2025-12-30 - Purified Ontology: Removed SemanticExtraction and CarrierType.
//! 2025-12-30 - Migrated to Community with PRAX_SCALE.
//! 2025-12-27 - Initial creation for Phase 2
//! --------------------------

use serde::{Deserialize, Serialize};

/// Canonical Scaling Factor for Deterministic Math in Praxeology.
/// 1.0 = 1,000,000.
pub const PRAX_SCALE: u64 = 1_000_000;

/// Represents the structural properties of an action relevant to praxeology.
/// Replaces the semantic "PerceptionResult".
/// Focuses on measuring the "shape" of the action (Means) rather than its category.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PraxeologicalStructure {
    /// Measures the functional asymmetry of the action.
    /// High asymmetry often indicates higher proto-agency (neg-entropy injection).
    /// Scaled by PRAX_SCALE.
    pub asymmetry_factor: u64,

    /// Represents the "cost" or "effort" magnitude expended in the action.
    /// Derived from context vectors (e.g., latency, constraints).
    /// Scaled by PRAX_SCALE.
    pub effort_magnitude: u64,

    /// Represents the structural complexity of the payload/content.
    /// Scaled by PRAX_SCALE.
    pub complexity_factor: u64,
}

/// The result of a canonical praxeological assessment (M_P).
/// Strictly structural evaluation of agency and coherence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PraxeologicalAssessment {
    /// The structural analysis of the action used for the assessment.
    pub structure: PraxeologicalStructure,

    /// The calculated Proto-Agency score.
    /// 0 = Random/Noise, 1,000,000 = Highly Agentic.
    /// Derived from asymmetry and complexity.
    pub proto_agency_score: u64,

    /// The structural coherence of the action.
    /// Evaluates if the Effort (Means) structurally aligns with the Complexity (Ends).
    /// A mismatch (high effort, zero complexity) implies incoherence.
    pub coherence_score: u64,
}