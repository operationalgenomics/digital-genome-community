//! --------------------------
//! INFORMATION
//! --------------------------
//! Title: Selection Module (Golden Index)
//! Author: Carlos Eduardo Favini
//! Date: 2025-01-02
//! Version: 1.2.0
//! Description: Criteria for selecting Golden DNA. The selection module
//!              provides structural comparison without making decisions.
//!              It reveals which DNA has higher coherence - it does not
//!              choose or act. Selection functions (find_highest, rank)
//!              have been moved to Enterprise Edition.
//! Layer: Community
//! Dependencies: hierarchy/dna, math/craft
//! Affected Components: External consumers (Enterprise)
//!
//! --------------------------
//! CHANGE LOG
//! --------------------------
//! 2025-01-02 - Carlos Eduardo Favini - Initial creation
//! 2025-01-02 - Carlos Eduardo Favini - Moved selection to Enterprise
//! 2025-01-02 - Carlos Eduardo Favini - Canonical comparison threshold (v0.2.0)
//! --------------------------

use serde::{Deserialize, Serialize};

use crate::hierarchy::GoldenDna;

/// Comparison result between two DNA strands.
///
/// This is structural observation, not decision.
/// The comparison reveals relative coherence - it does not
/// select or prefer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComparisonResult {
    /// First DNA has higher CP.
    FirstHigher,

    /// Second DNA has higher CP.
    SecondHigher,

    /// Both DNA have equal CP.
    Equal,

    /// Comparison invalid (one or both vetoed).
    Invalid,
}

/// Golden Index for DNA comparison.
///
/// Provides methods to compare and rank DNA by Craft Performance.
/// All comparisons are structural observations - no decisions are made.
#[derive(Debug, Clone)]
pub struct GoldenIndex;

impl GoldenIndex {
    /// Compares two DNA strands by Craft Performance.
    ///
    /// # Returns
    /// ComparisonResult indicating relative CP.
    pub fn compare(dna_a: &GoldenDna, dna_b: &GoldenDna) -> ComparisonResult {
        // Check for vetoes
        if dna_a.is_vetoed() || dna_b.is_vetoed() {
            return ComparisonResult::Invalid;
        }

        let cp_a = dna_a.craft_performance;
        let cp_b = dna_b.craft_performance;

        // Use threshold derived from VETO_THRESHOLD for consistency
        // CP_EQUALITY_THRESHOLD = VETO_THRESHOLD^0.5 ≈ 3.16e-8
        // This provides a meaningful equality zone while remaining
        // well above numerical noise.
        const CP_EQUALITY_THRESHOLD: f64 = 3.16e-8; // sqrt(1e-15)

        if (cp_a - cp_b).abs() < CP_EQUALITY_THRESHOLD {
            ComparisonResult::Equal
        } else if cp_a > cp_b {
            ComparisonResult::FirstHigher
        } else {
            ComparisonResult::SecondHigher
        }
    }

    /// Calculates the CP difference between two DNA strands.
    ///
    /// # Returns
    /// Delta CP = CP_a - CP_b
    pub fn delta_cp(dna_a: &GoldenDna, dna_b: &GoldenDna) -> f64 {
        dna_a.craft_performance - dna_b.craft_performance
    }

    /// Calculates improvement factor (relative change).
    ///
    /// # Returns
    /// (CP_new - CP_old) / CP_old, or infinity if CP_old = 0
    pub fn improvement_factor(old: &GoldenDna, new: &GoldenDna) -> f64 {
        if old.craft_performance == 0.0 {
            if new.craft_performance > 0.0 {
                f64::INFINITY
            } else {
                0.0
            }
        } else {
            (new.craft_performance - old.craft_performance) / old.craft_performance
        }
    }

    // Note: find_highest() and rank() have been moved to Enterprise Edition.
    // Community Edition only performs pairwise comparisons, not selection.
    // See ENTERPRISE-BACKLOG.md for the moved functions.
}

/// Statistics about a collection of DNA.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionStats {
    /// Total number of DNA.
    pub total: usize,

    /// Number of vetoed DNA.
    pub vetoed_count: usize,

    /// Number of viable (non-vetoed) DNA.
    pub viable_count: usize,

    /// Minimum CP among viable DNA.
    pub min_cp: f64,

    /// Maximum CP among viable DNA.
    pub max_cp: f64,

    /// Mean CP among viable DNA.
    pub mean_cp: f64,

    /// CP standard deviation among viable DNA.
    pub std_cp: f64,
}

impl CollectionStats {
    /// Calculates statistics for a collection of DNA.
    pub fn from_collection(dnas: &[&GoldenDna]) -> Self {
        let total = dnas.len();
        let viable: Vec<&&GoldenDna> = dnas.iter().filter(|dna: &&&GoldenDna| !dna.is_vetoed()).collect();
        let viable_count = viable.len();
        let vetoed_count = total - viable_count;

        if viable_count == 0 {
            return Self {
                total,
                vetoed_count,
                viable_count: 0,
                min_cp: 0.0,
                max_cp: 0.0,
                mean_cp: 0.0,
                std_cp: 0.0,
            };
        }

        let cps: Vec<f64> = viable.iter().map(|dna| dna.craft_performance).collect();

        let min_cp = cps.iter().cloned().fold(f64::INFINITY, f64::min);
        let max_cp = cps.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let mean_cp = cps.iter().sum::<f64>() / viable_count as f64;

        let variance = cps.iter().map(|cp| (cp - mean_cp).powi(2)).sum::<f64>()
            / viable_count as f64;
        let std_cp = variance.sqrt();

        Self {
            total,
            vetoed_count,
            viable_count,
            min_cp,
            max_cp,
            mean_cp,
            std_cp,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hierarchy::{ActionSequence, MotorScores};

    fn create_test_dna(cp: f64) -> GoldenDna {
        // Create scores that multiply to cp
        let score = cp.powf(0.25);
        let scores = MotorScores::new(score, score, score, score);
        GoldenDna::primordial(ActionSequence::new(), scores)
    }

    #[test]
    fn test_compare_higher() {
        let dna_a = create_test_dna(0.8);
        let dna_b = create_test_dna(0.5);

        assert_eq!(GoldenIndex::compare(&dna_a, &dna_b), ComparisonResult::FirstHigher);
    }

    #[test]
    fn test_compare_lower() {
        let dna_a = create_test_dna(0.3);
        let dna_b = create_test_dna(0.7);

        assert_eq!(GoldenIndex::compare(&dna_a, &dna_b), ComparisonResult::SecondHigher);
    }

    #[test]
    fn test_compare_vetoed() {
        let dna_a = create_test_dna(0.0);
        let dna_b = create_test_dna(0.5);

        assert_eq!(GoldenIndex::compare(&dna_a, &dna_b), ComparisonResult::Invalid);
    }
}
