//! --------------------------
//! INFORMATION
//! --------------------------
//! Title: Praxeological Motor Engine
//! Author: Carlos Eduardo Favini
//! Date: 2025-12-30
//! Version: 1.1.0
//! Description: Implements M_P (Canonical Praxeological Motor).
//!              Evaluates action structure based on pre-calculated factors.
//!              Strictly deterministic: uses integer scaling (1.0 = 1,000,000).
//!              No Enterprise dependencies (ObservedAction is external).
//! Layer: Community
//! Dependencies: types
//! Affected Components: reactor
//!
//! --------------------------
//! CHANGE LOG
//! --------------------------
//! 2025-12-30 - Migrated to Community. Replaced float logic with PRAX_SCALE summation.
//! 2025-12-27 - Initial creation
//! --------------------------

use super::types::{
    PraxeologicalAssessment, PraxeologicalStructure, PRAX_SCALE
};

/// The Canonical Praxeological Motor (M_P).
/// Evaluates action structure and coherence.
pub struct PraxeologicalMotor;

impl PraxeologicalMotor {
    pub fn new() -> Self {
        Self
    }

    /// Performs the Praxeological Assessment on a given structure.
    ///
    /// # Arguments
    /// * `structure` - The structural analysis of the action (Constructed upstream).
    pub fn assess(&self, structure: &PraxeologicalStructure) -> PraxeologicalAssessment {
        // Level 2.5 Analysis: Calculate Proto-Agency.
        // In the original implementation, this was a sum of weighted indicators.
        // Here, the factors are provided in the structure (already scaled), and we aggregate them.
        
        // Sum the factors: Asymmetry + Effort + Complexity
        // Use saturating_add to prevent overflow, though values should be within range.
        let raw_score = structure.asymmetry_factor
            .saturating_add(structure.effort_magnitude)
            .saturating_add(structure.complexity_factor);

        // Clamp to PRAX_SCALE (1.0)
        let proto_agency_score = if raw_score > PRAX_SCALE {
            PRAX_SCALE
        } else {
            raw_score
        };

        // Fulfillment/Coherence Score Calculation:
        // Originally: fulfillment_score = perception.proto_agency_score;
        // Identity mapping preserves the original logic byte-for-byte in intent.
        let coherence_score = proto_agency_score;

        PraxeologicalAssessment {
            structure: structure.clone(),
            proto_agency_score,
            coherence_score,
        }
    }
}