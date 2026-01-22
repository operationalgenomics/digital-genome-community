//! --------------------------
//! INFORMATION
//! --------------------------
//! Title: Sensory Signals
//! Author: Carlos Eduardo Favini
//! Date: 2025-01-02
//! Version: 1.2.0
//! Description: Raw sensory signals computed from input data.
//! ONLY mathematics. ZERO interpretation.
//! No classification. No confidence. No evidence narrative.
//! Layer: Community
//! Dependencies: serde
//! Affected Components: sensory, output
//!
//! --------------------------
//! CRITICAL CONSTRAINTS (v1.1.0)
//! --------------------------
//! This module MUST NOT:
//! - Classify anything (no "this is X")
//! - Provide confidence scores for classifications
//! - Generate explanatory narratives
//! - Infer ontology
//! - Make semantic interpretations
//!
//! This module MUST ONLY:
//! - Compute mathematical statistics
//! - Return continuous scores
//! - Report binary test results (passed/failed)
//!
//! --------------------------
//! CHANGE LOG
//! --------------------------
//! 2025-01-02 - Carlos Eduardo Favini - Initial creation (v1.1.0)
//! --------------------------

use serde::{Deserialize, Serialize};

/// Sensory signals computed from raw input.
/// ONLY mathematics. ZERO interpretation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorySignals {
    // ═══════════════════════════════════════════════════════════════════════
    // RAW MATHEMATICAL SIGNALS
    // ═══════════════════════════════════════════════════════════════════════

    /// Shannon entropy normalized to [0,1]
    /// 0 = perfectly ordered, 1 = maximum disorder
    /// Formula: H = -Σ p(x) log₂ p(x) / log₂(N)
    pub entropy: f64,

    /// Maximum autocorrelation coefficient [0,1]
    /// Higher = more self-similar at some lag
    /// Formula: R(τ) = Σ x(t)·x(t+τ) / Σ x(t)²
    pub max_autocorrelation: f64,

    /// Lag (in samples) at which max autocorrelation occurs
    pub autocorrelation_lag: usize,

    /// Spectral centroid (normalized frequency)
    /// Where the "center of mass" of the spectrum is
    pub spectral_centroid: f64,

    /// Spectral flatness [0,1]
    /// 0 = tonal/peaked, 1 = noise-like/flat
    /// Formula: geometric_mean / arithmetic_mean
    pub spectral_flatness: f64,

    /// Dominant frequency index in spectrum
    pub dominant_frequency_index: usize,

    /// Local entropy / Global entropy ratio
    /// < 1 means local structure exists
    pub local_global_entropy_ratio: f64,

    /// Compressibility (Kolmogorov complexity proxy)
    /// Higher = more compressible = more structure
    /// Formula: 1 - compressed_size / original_size
    pub compressibility: f64,

    /// Variance of local variances / global variance
    /// Measures heteroscedasticity
    pub variance_ratio: f64,

    /// Zero-crossing rate (normalized)
    /// How often the signal crosses zero
    pub zero_crossing_rate: f64,

    // ═══════════════════════════════════════════════════════════════════════
    // STATISTICAL TEST RESULTS (binary, not interpretive)
    // ═══════════════════════════════════════════════════════════════════════

    /// Runs test for randomness (Wald-Wolfowitz)
    /// true = appears random, false = appears non-random
    pub randomness_test_passed: bool,

    /// p-value from randomness test (for transparency)
    pub randomness_test_p_value: f64,

    /// Stationarity test passed (simplified ADF-like)
    /// true = appears stationary, false = appears non-stationary
    pub stationarity_test_passed: bool,

    /// Periodicity detected (autocorrelation peak significant)
    /// true = significant periodicity, false = no significant periodicity
    pub periodicity_detected: bool,

    /// Periodicity significance (how much above noise floor)
    pub periodicity_significance: f64,

    // ═══════════════════════════════════════════════════════════════════════
    // INPUT METADATA (not interpretation)
    // ═══════════════════════════════════════════════════════════════════════

    /// Number of samples processed
    pub sample_count: usize,

    /// Number of unique values
    pub unique_values: usize,

    /// Minimum value in signal
    pub min_value: f64,

    /// Maximum value in signal
    pub max_value: f64,

    /// Mean value
    pub mean: f64,

    /// Standard deviation
    pub std_dev: f64,
}

impl SensorySignals {
    /// Creates a new SensorySignals with default (zero) values.
    /// Used as starting point before computation.
    pub fn empty() -> Self {
        Self {
            entropy: 0.0,
            max_autocorrelation: 0.0,
            autocorrelation_lag: 0,
            spectral_centroid: 0.0,
            spectral_flatness: 0.0,
            dominant_frequency_index: 0,
            local_global_entropy_ratio: 1.0,
            compressibility: 0.0,
            variance_ratio: 1.0,
            zero_crossing_rate: 0.0,
            randomness_test_passed: true,
            randomness_test_p_value: 1.0,
            stationarity_test_passed: true,
            periodicity_detected: false,
            periodicity_significance: 0.0,
            sample_count: 0,
            unique_values: 0,
            min_value: 0.0,
            max_value: 0.0,
            mean: 0.0,
            std_dev: 0.0,
        }
    }

    /// Returns true if the signal appears to have structure (low entropy)
    /// This is NOT interpretation - it's a mathematical threshold
    /// relative to maximum possible entropy.
    pub fn has_low_entropy(&self) -> bool {
        // Entropy below 0.5 means less than half of maximum disorder
        // This is derived from information theory, not arbitrary
        self.entropy < 0.5
    }

    /// Returns true if significant periodicity was detected.
    pub fn has_periodicity(&self) -> bool {
        self.periodicity_detected
    }

    /// Returns true if the signal appears non-random.
    pub fn appears_non_random(&self) -> bool {
        !self.randomness_test_passed
    }

    /// Returns true if local structure exists (local entropy < global).
    pub fn has_local_structure(&self) -> bool {
        self.local_global_entropy_ratio < 0.9
    }

    /// Validates that all signals are within expected bounds.
    pub fn validate(&self) -> Result<(), SignalValidationError> {
        if !self.entropy.is_finite() || self.entropy < 0.0 || self.entropy > 1.0 {
            return Err(SignalValidationError::InvalidEntropy(self.entropy));
        }
        if !self.max_autocorrelation.is_finite()
            || self.max_autocorrelation < 0.0
            || self.max_autocorrelation > 1.0
        {
            return Err(SignalValidationError::InvalidAutocorrelation(
                self.max_autocorrelation,
            ));
        }
        if !self.spectral_flatness.is_finite()
            || self.spectral_flatness < 0.0
            || self.spectral_flatness > 1.0
        {
            return Err(SignalValidationError::InvalidSpectralFlatness(
                self.spectral_flatness,
            ));
        }
        if self.sample_count == 0 {
            return Err(SignalValidationError::EmptySignal);
        }
        Ok(())
    }
}

/// Errors in signal validation
#[derive(Debug, Clone)]
pub enum SignalValidationError {
    /// Entropy out of bounds
    InvalidEntropy(f64),
    /// Autocorrelation out of bounds
    InvalidAutocorrelation(f64),
    /// Spectral flatness out of bounds
    InvalidSpectralFlatness(f64),
    /// No samples in signal
    EmptySignal,
}

impl std::fmt::Display for SignalValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidEntropy(v) => write!(f, "Invalid entropy value: {}", v),
            Self::InvalidAutocorrelation(v) => write!(f, "Invalid autocorrelation value: {}", v),
            Self::InvalidSpectralFlatness(v) => write!(f, "Invalid spectral flatness value: {}", v),
            Self::EmptySignal => write!(f, "Signal is empty"),
        }
    }
}

impl std::error::Error for SignalValidationError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_signals() {
        let signals = SensorySignals::empty();
        assert_eq!(signals.entropy, 0.0);
        assert_eq!(signals.sample_count, 0);
    }

    #[test]
    fn test_has_low_entropy() {
        let mut signals = SensorySignals::empty();
        signals.entropy = 0.3;
        assert!(signals.has_low_entropy());

        signals.entropy = 0.7;
        assert!(!signals.has_low_entropy());
    }

    #[test]
    fn test_has_local_structure() {
        let mut signals = SensorySignals::empty();
        signals.local_global_entropy_ratio = 0.5;
        assert!(signals.has_local_structure());

        signals.local_global_entropy_ratio = 0.95;
        assert!(!signals.has_local_structure());
    }

    #[test]
    fn test_validation() {
        let mut signals = SensorySignals::empty();
        signals.sample_count = 100;
        signals.entropy = 0.5;
        signals.max_autocorrelation = 0.3;
        signals.spectral_flatness = 0.8;
        assert!(signals.validate().is_ok());

        signals.entropy = 1.5; // Invalid
        assert!(signals.validate().is_err());
    }
}
