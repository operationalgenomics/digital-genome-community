//! --------------------------
//! INFORMATION
//! --------------------------
//! Title: Craft Performance Formula
//! Author: Carlos Eduardo Favini
//! Date: 2025-01-02
//! Version: 1.2.0
//! Description: The canonical Craft Performance (CP) formula.
//!              CP = M_P × M_N × M_C × M_M
//!              Implements the non-compensatory principle with absolute veto.
//! Layer: Community
//! Dependencies: motors
//! Affected Components: hierarchy/dna, selection
//!
//! --------------------------
//! VETO THRESHOLD SPECIFICATION (v0.2.0)
//! --------------------------
//! The VETO_THRESHOLD constant defines the boundary between "viable" and "vetoed" DNA.
//!
//! Value: 1e-15 (0.000000000000001)
//!
//! Derivation:
//! - f64 has ~15-17 significant decimal digits
//! - f64::EPSILON ≈ 2.22e-16 (smallest difference from 1.0)
//! - VETO_THRESHOLD = 1e-15 ≈ 4.5 × f64::EPSILON
//! - This provides a margin above machine epsilon while remaining
//!   effectively indistinguishable from zero for operational purposes
//!
//! Constitutional Meaning:
//! - CP < VETO_THRESHOLD triggers ABSOLUTE VETO
//! - This threshold is the single non-mathematical axiom of the Digital Genome
//! - It converts continuous mathematics into discrete viability decisions
//!
//! Alternatives Considered:
//! - f64::EPSILON: Too tight, causes false positives from numerical noise
//! - 1e-10: Too loose, allows effectively-zero values to pass
//! - 1e-15: Optimal balance between precision and operational meaning
//!
//! --------------------------
//! CHANGE LOG
//! --------------------------
//! 2025-01-02 - Carlos Eduardo Favini - Initial creation
//! 2025-01-02 - Carlos Eduardo Favini - Canonical veto threshold
//! 2025-01-02 - Carlos Eduardo Favini - Added was_clamped signaling (v0.2.0)
//! 2025-01-02 - Carlos Eduardo Favini - Formal VETO_THRESHOLD documentation (v0.2.0)
//! --------------------------

use serde::{Deserialize, Serialize};

/// Canonical veto threshold for Craft Performance.
///
/// # Value
/// 1e-15 (one quadrillionth)
///
/// # Derivation
/// - Approximately 4.5 × f64::EPSILON
/// - Provides 15 orders of magnitude for valid CP values
/// - Effectively indistinguishable from zero for operational purposes
///
/// # Constitutional Rule
/// CP < VETO_THRESHOLD triggers ABSOLUTE VETO.
/// This is the ONLY non-mathematical axiom in the Digital Genome.
///
/// See module documentation for full specification.
pub const VETO_THRESHOLD: f64 = 1e-15;

/// Craft Performance (CP) calculator.
///
/// The formula CP = M_P × M_N × M_C × M_M implements the
/// non-compensatory principle: if any motor is below threshold, CP is vetoed.
///
/// # Constitutional Rule
/// CP < VETO_THRESHOLD triggers ABSOLUTE VETO.
///
/// # Interpretation
/// - CP ≈ 1.0: Near-perfect coherence (Platonic ideal - unreachable)
/// - CP ≥ VETO_THRESHOLD: Viable DNA
/// - CP < VETO_THRESHOLD: VETO - DNA is rejected
#[derive(Debug, Clone, Copy)]
pub struct CraftPerformance;

/// Result type for CP calculation.
#[derive(Debug, Clone, PartialEq)]
pub enum CpResult {
    /// Valid CP value.
    Valid {
        /// The computed CP value.
        value: f64,
        /// Whether the value was clamped to [0, 1].
        was_clamped: bool,
        /// Original value before clamping (if clamped).
        unclamped_value: Option<f64>,
    },
    /// Vetoed due to low value.
    Vetoed {
        /// The computed CP value (below threshold).
        value: f64,
        /// Which motor(s) caused the veto.
        cause: VetoCause,
    },
    /// Invalid input (NaN, negative, or > 1).
    Invalid {
        /// Description of the invalidity.
        reason: InvalidReason,
        /// Which motor had invalid input.
        source: Option<VetoCause>,
    },
}

/// Reason for veto.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VetoCause {
    /// Praxeological motor below threshold.
    Praxeological,
    /// Nash motor below threshold.
    Nash,
    /// Chaotic motor below threshold.
    Chaotic,
    /// Meristic motor below threshold.
    Meristic,
    /// Multiple motors below threshold.
    Multiple,
    /// Final CP below threshold (product of valid inputs).
    FinalProduct,
}

/// Reason for invalid input.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InvalidReason {
    /// Input is NaN.
    NaN,
    /// Input is negative.
    Negative,
    /// Input is greater than 1.
    GreaterThanOne,
    /// Input is infinite.
    Infinite,
}

impl CraftPerformance {
    /// Validates a single motor score.
    fn validate_score(score: f64) -> Result<f64, InvalidReason> {
        if score.is_nan() {
            Err(InvalidReason::NaN)
        } else if score.is_infinite() {
            Err(InvalidReason::Infinite)
        } else if score < 0.0 {
            Err(InvalidReason::Negative)
        } else if score > 1.0 {
            Err(InvalidReason::GreaterThanOne)
        } else {
            Ok(score)
        }
    }

    /// Calculates CP from individual motor scores with full validation.
    ///
    /// # Arguments
    /// * `m_p` - Praxeological motor score [0, 1]
    /// * `m_n` - Nash motor score [0, 1]
    /// * `m_c` - Chaotic motor score [0, 1]
    /// * `m_m` - Meristic motor score [0, 1]
    ///
    /// # Returns
    /// CpResult indicating valid CP, veto, or invalid input.
    pub fn calculate(m_p: f64, m_n: f64, m_c: f64, m_m: f64) -> CpResult {
        // Validate all inputs
        let validated = [
            (Self::validate_score(m_p), VetoCause::Praxeological),
            (Self::validate_score(m_n), VetoCause::Nash),
            (Self::validate_score(m_c), VetoCause::Chaotic),
            (Self::validate_score(m_m), VetoCause::Meristic),
        ];

        // Check for invalid inputs
        for (result, cause) in &validated {
            if let Err(reason) = result {
                return CpResult::Invalid {
                    reason: *reason,
                    source: Some(*cause),
                };
            }
        }

        // Check for individual vetoes
        let vetoed_motors: Vec<VetoCause> = validated
            .iter()
            .filter_map(|(result, cause)| {
                if let Ok(score) = result {
                    if *score < VETO_THRESHOLD {
                        Some(*cause)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        if !vetoed_motors.is_empty() {
            let cause = if vetoed_motors.len() == 1 {
                vetoed_motors[0]
            } else {
                VetoCause::Multiple
            };
            
            let value = m_p * m_n * m_c * m_m;
            return CpResult::Vetoed { value, cause };
        }

        // Calculate CP
        let cp = m_p * m_n * m_c * m_m;

        // Check final product against threshold
        if cp < VETO_THRESHOLD {
            return CpResult::Vetoed {
                value: cp,
                cause: VetoCause::FinalProduct,
            };
        }

        // Check if clamping is needed (should be rare with validated inputs)
        let needs_clamping = cp < 0.0 || cp > 1.0;
        let final_cp = if needs_clamping { cp.clamp(0.0, 1.0) } else { cp };

        CpResult::Valid {
            value: final_cp,
            was_clamped: needs_clamping,
            unclamped_value: if needs_clamping { Some(cp) } else { None },
        }
    }

    /// Simplified calculation returning just the numeric value.
    /// Returns 0.0 for vetoed or invalid inputs.
    pub fn calculate_value(m_p: f64, m_n: f64, m_c: f64, m_m: f64) -> f64 {
        match Self::calculate(m_p, m_n, m_c, m_m) {
            CpResult::Valid { value, .. } => value,
            CpResult::Vetoed { .. } => 0.0,
            CpResult::Invalid { .. } => 0.0,
        }
    }

    /// Calculates CP from a MotorScoreSet.
    pub fn from_scores(scores: &MotorScoreSet) -> CpResult {
        Self::calculate(
            scores.praxeological,
            scores.nash,
            scores.chaotic,
            scores.meristic,
        )
    }

    /// Checks if any motor is below the veto threshold.
    pub fn has_veto(m_p: f64, m_n: f64, m_c: f64, m_m: f64) -> bool {
        m_p < VETO_THRESHOLD
            || m_n < VETO_THRESHOLD
            || m_c < VETO_THRESHOLD
            || m_m < VETO_THRESHOLD
    }

    /// Identifies which motor(s) are below the veto threshold.
    pub fn veto_sources(m_p: f64, m_n: f64, m_c: f64, m_m: f64) -> Vec<&'static str> {
        let mut sources = Vec::new();
        if m_p < VETO_THRESHOLD {
            sources.push("Praxeological");
        }
        if m_n < VETO_THRESHOLD {
            sources.push("Nash");
        }
        if m_c < VETO_THRESHOLD {
            sources.push("Chaotic");
        }
        if m_m < VETO_THRESHOLD {
            sources.push("Meristic");
        }
        sources
    }

    /// Returns the canonical veto threshold.
    pub const fn threshold() -> f64 {
        VETO_THRESHOLD
    }
}

/// Complete set of motor scores.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MotorScoreSet {
    /// Praxeological motor score (M_P).
    pub praxeological: f64,

    /// Nash motor score (M_N).
    pub nash: f64,

    /// Chaotic motor score (M_C).
    pub chaotic: f64,

    /// Meristic motor score (M_M).
    pub meristic: f64,
}

impl MotorScoreSet {
    /// Creates a new motor score set with validation.
    ///
    /// Scores are clamped to [0, 1] range.
    /// For strict validation, use `try_new`.
    pub fn new(praxeological: f64, nash: f64, chaotic: f64, meristic: f64) -> Self {
        Self {
            praxeological: praxeological.clamp(0.0, 1.0),
            nash: nash.clamp(0.0, 1.0),
            chaotic: chaotic.clamp(0.0, 1.0),
            meristic: meristic.clamp(0.0, 1.0),
        }
    }

    /// Creates a new motor score set with strict validation.
    ///
    /// Returns None if any score is outside [0, 1] or is NaN/Infinite.
    pub fn try_new(praxeological: f64, nash: f64, chaotic: f64, meristic: f64) -> Option<Self> {
        let validate = |s: f64| !s.is_nan() && !s.is_infinite() && (0.0..=1.0).contains(&s);

        if validate(praxeological) && validate(nash) && validate(chaotic) && validate(meristic) {
            Some(Self {
                praxeological,
                nash,
                chaotic,
                meristic,
            })
        } else {
            None
        }
    }

    /// Calculates the Craft Performance for this score set.
    pub fn craft_performance(&self) -> CpResult {
        CraftPerformance::from_scores(self)
    }

    /// Returns the numeric CP value (0.0 if vetoed/invalid).
    pub fn cp_value(&self) -> f64 {
        CraftPerformance::calculate_value(
            self.praxeological,
            self.nash,
            self.chaotic,
            self.meristic,
        )
    }

    /// Checks if this score set triggers a veto.
    pub fn is_vetoed(&self) -> bool {
        CraftPerformance::has_veto(
            self.praxeological,
            self.nash,
            self.chaotic,
            self.meristic,
        )
    }

    /// Returns the minimum score among all motors.
    pub fn min_score(&self) -> f64 {
        self.praxeological
            .min(self.nash)
            .min(self.chaotic)
            .min(self.meristic)
    }

    /// Returns the motor with the lowest score.
    pub fn weakest_motor(&self) -> (&'static str, f64) {
        let mut min = ("Praxeological", self.praxeological);

        if self.nash < min.1 {
            min = ("Nash", self.nash);
        }
        if self.chaotic < min.1 {
            min = ("Chaotic", self.chaotic);
        }
        if self.meristic < min.1 {
            min = ("Meristic", self.meristic);
        }

        min
    }
}

impl Default for MotorScoreSet {
    fn default() -> Self {
        // Default to zero scores (will be vetoed)
        Self {
            praxeological: 0.0,
            nash: 0.0,
            chaotic: 0.0,
            meristic: 0.0,
        }
    }
}

/// Result of a Craft Performance evaluation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraftPerformanceResult {
    /// Individual motor scores.
    pub scores: MotorScoreSet,

    /// Final Craft Performance value (0.0 if vetoed/invalid).
    pub cp: f64,

    /// Whether a veto was triggered.
    pub vetoed: bool,

    /// Motors that caused the veto (if any).
    pub veto_sources: Vec<String>,

    /// Whether the input was valid.
    pub valid: bool,

    /// Whether the CP value was clamped.
    pub was_clamped: bool,
}

impl CraftPerformanceResult {
    /// Creates a new CP result from motor scores.
    pub fn from_scores(scores: MotorScoreSet) -> Self {
        let result = scores.craft_performance();

        match result {
            CpResult::Valid { value, was_clamped, .. } => Self {
                scores,
                cp: value,
                vetoed: false,
                veto_sources: Vec::new(),
                valid: true,
                was_clamped,
            },
            CpResult::Vetoed { value: _, cause: _ } => {
                let sources = CraftPerformance::veto_sources(
                    scores.praxeological,
                    scores.nash,
                    scores.chaotic,
                    scores.meristic,
                )
                .iter()
                .map(|s| s.to_string())
                .collect();

                Self {
                    scores,
                    cp: 0.0,
                    vetoed: true,
                    veto_sources: sources,
                    valid: true,
                    was_clamped: false,
                }
            }
            CpResult::Invalid { .. } => Self {
                scores,
                cp: 0.0,
                vetoed: false,
                veto_sources: Vec::new(),
                valid: false,
                was_clamped: false,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perfect_scores() {
        let result = CraftPerformance::calculate(1.0, 1.0, 1.0, 1.0);
        match result {
            CpResult::Valid { value, was_clamped, .. } => {
                assert!((value - 1.0).abs() < 1e-10);
                assert!(!was_clamped);
            }
            _ => panic!("Expected Valid result"),
        }
    }

    #[test]
    fn test_veto_exact_zero() {
        let result = CraftPerformance::calculate(1.0, 0.0, 1.0, 1.0);
        assert!(matches!(result, CpResult::Vetoed { cause: VetoCause::Nash, .. }));
    }

    #[test]
    fn test_veto_below_threshold() {
        let result = CraftPerformance::calculate(1.0, 1e-16, 1.0, 1.0);
        assert!(matches!(result, CpResult::Vetoed { cause: VetoCause::Nash, .. }));
    }

    #[test]
    fn test_valid_near_threshold() {
        // Just above threshold should be valid
        let result = CraftPerformance::calculate(1.0, 1.0, 1.0, 1e-14);
        assert!(matches!(result, CpResult::Valid { .. }));
    }

    #[test]
    fn test_invalid_nan() {
        let result = CraftPerformance::calculate(1.0, f64::NAN, 1.0, 1.0);
        match result {
            CpResult::Invalid { reason, source } => {
                assert_eq!(reason, InvalidReason::NaN);
                assert_eq!(source, Some(VetoCause::Nash));
            }
            _ => panic!("Expected Invalid result"),
        }
    }

    #[test]
    fn test_invalid_negative() {
        let result = CraftPerformance::calculate(1.0, -0.5, 1.0, 1.0);
        assert!(matches!(result, CpResult::Invalid { reason: InvalidReason::Negative, .. }));
    }

    #[test]
    fn test_invalid_greater_than_one() {
        let result = CraftPerformance::calculate(1.0, 1.5, 1.0, 1.0);
        assert!(matches!(result, CpResult::Invalid { reason: InvalidReason::GreaterThanOne, .. }));
    }

    #[test]
    fn test_veto_sources() {
        let sources = CraftPerformance::veto_sources(0.0, 1.0, 0.0, 1.0);
        assert_eq!(sources.len(), 2);
        assert!(sources.contains(&"Praxeological"));
        assert!(sources.contains(&"Chaotic"));
    }

    #[test]
    fn test_non_compensatory() {
        // High scores in 3 motors cannot compensate for veto in one
        let result = CraftPerformance::calculate(0.99, 0.99, 1e-20, 0.99);
        assert!(matches!(result, CpResult::Vetoed { .. }));
    }

    #[test]
    fn test_score_set() {
        let scores = MotorScoreSet::new(0.8, 0.7, 0.9, 0.6);
        let cp = scores.cp_value();

        // CP = 0.8 × 0.7 × 0.9 × 0.6 = 0.3024
        assert!((cp - 0.3024).abs() < 1e-10);

        let (weakest, value) = scores.weakest_motor();
        assert_eq!(weakest, "Meristic");
        assert!((value - 0.6).abs() < 1e-10);
    }

    #[test]
    fn test_threshold_value() {
        assert_eq!(CraftPerformance::threshold(), 1e-15);
        assert_eq!(VETO_THRESHOLD, 1e-15);
    }

    #[test]
    fn test_try_new_validation() {
        // Valid
        assert!(MotorScoreSet::try_new(0.5, 0.5, 0.5, 0.5).is_some());
        // Invalid - negative
        assert!(MotorScoreSet::try_new(-0.1, 0.5, 0.5, 0.5).is_none());
        // Invalid - > 1
        assert!(MotorScoreSet::try_new(0.5, 1.1, 0.5, 0.5).is_none());
        // Invalid - NaN
        assert!(MotorScoreSet::try_new(0.5, f64::NAN, 0.5, 0.5).is_none());
    }

    #[test]
    fn test_was_clamped_in_result() {
        let scores = MotorScoreSet::new(0.8, 0.7, 0.9, 0.6);
        let result = CraftPerformanceResult::from_scores(scores);
        
        assert!(result.valid);
        assert!(!result.vetoed);
        assert!(!result.was_clamped);
    }
}
