//! --------------------------
//! INFORMATION
//! --------------------------
//! Title: Structure Analysis (Level 2)
//! Author: Carlos Eduardo Favini
//! Date: 2025-01-02
//! Version: 1.2.0
//! Description: Level 2 of the abstraction hierarchy.
//! Detects organization and low-entropy structure.
//! Compares local vs global properties.
//! Layer: Community
//! Dependencies: None (pure math)
//! Affected Components: sensory/signals
//!
//! --------------------------
//! MATHEMATICAL BASIS
//! --------------------------
//! Local vs Global Entropy: H_local / H_global
//! - If ratio < 1, local structure exists (ordered regions)
//! - If ratio ≈ 1, homogeneous signal
//!
//! Compressibility (Kolmogorov proxy):
//! - Higher compressibility = more structure
//! - We use run-length encoding as proxy
//!
//! Stationarity:
//! - Stationary signals have consistent statistics over time
//! - Non-stationarity suggests structural changes
//!
//! --------------------------
//! CHANGE LOG
//! --------------------------
//! 2025-01-02 - Carlos Eduardo Favini - Initial creation (v1.1.0)
//! --------------------------

use std::collections::BTreeMap;

/// Result of structure-level analysis
#[derive(Debug, Clone)]
pub struct StructureAnalysis {
    /// Local entropy / Global entropy ratio
    /// < 1 means local structure exists
    pub local_global_entropy_ratio: f64,

    /// Compressibility (0 = incompressible, 1 = highly compressible)
    pub compressibility: f64,

    /// Variance ratio (local variances / global variance)
    pub variance_ratio: f64,

    /// Stationarity test passed
    pub stationarity_test_passed: bool,

    /// Number of structural segments detected
    pub segment_count: usize,

    /// Mean segment length
    pub mean_segment_length: f64,
}

impl StructureAnalysis {
    /// Analyzes structure in a signal
    pub fn analyze(values: &[f64]) -> Self {
        if values.len() < 16 {
            return Self::empty();
        }

        // Compute global entropy
        let global_entropy = Self::compute_entropy(values);

        // Compute local entropy (average over windows)
        let local_entropy = Self::compute_local_entropy(values, 16);

        // Entropy ratio
        let local_global_entropy_ratio = if global_entropy > f64::EPSILON {
            local_entropy / global_entropy
        } else {
            1.0
        };

        // Compressibility
        let compressibility = Self::compute_compressibility(values);

        // Variance ratio
        let variance_ratio = Self::compute_variance_ratio(values, 16);

        // Stationarity test
        let stationarity_test_passed = Self::test_stationarity(values);

        // Segment detection
        let (segment_count, mean_segment_length) = Self::detect_segments(values);

        Self {
            local_global_entropy_ratio,
            compressibility,
            variance_ratio,
            stationarity_test_passed,
            segment_count,
            mean_segment_length,
        }
    }

    /// Creates empty analysis
    fn empty() -> Self {
        Self {
            local_global_entropy_ratio: 1.0,
            compressibility: 0.0,
            variance_ratio: 1.0,
            stationarity_test_passed: true,
            segment_count: 1,
            mean_segment_length: 0.0,
        }
    }

    /// Computes Shannon entropy of values
    fn compute_entropy(values: &[f64]) -> f64 {
        if values.is_empty() {
            return 0.0;
        }

        let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

        if (max - min).abs() < f64::EPSILON {
            return 0.0;
        }

        let num_bins = 64usize;
        let bin_width = (max - min) / num_bins as f64;

        let mut counts: BTreeMap<usize, usize> = BTreeMap::new();
        for &v in values {
            let bin = ((v - min) / bin_width).floor() as usize;
            let bin = bin.min(num_bins - 1);
            *counts.entry(bin).or_insert(0) += 1;
        }

        let n = values.len() as f64;
        let mut entropy = 0.0;
        for &count in counts.values() {
            if count > 0 {
                let p = count as f64 / n;
                entropy -= p * p.log2();
            }
        }

        entropy
    }

    /// Computes average local entropy over windows
    fn compute_local_entropy(values: &[f64], window_size: usize) -> f64 {
        if values.len() < window_size {
            return Self::compute_entropy(values);
        }

        let num_windows = values.len() / window_size;
        if num_windows == 0 {
            return Self::compute_entropy(values);
        }

        let mut total_entropy = 0.0;
        for i in 0..num_windows {
            let start = i * window_size;
            let end = start + window_size;
            total_entropy += Self::compute_entropy(&values[start..end]);
        }

        total_entropy / num_windows as f64
    }

    /// Computes compressibility as a proxy for Kolmogorov complexity
    /// Uses run-length encoding on quantized values
    fn compute_compressibility(values: &[f64]) -> f64 {
        if values.len() < 2 {
            return 0.0;
        }

        // Quantize to 16 levels
        let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

        if (max - min).abs() < f64::EPSILON {
            // Constant signal - maximally compressible
            return 1.0;
        }

        let levels = 16u8;
        let scale = (levels - 1) as f64 / (max - min);

        let quantized: Vec<u8> = values
            .iter()
            .map(|&v| ((v - min) * scale).round() as u8)
            .collect();

        // Run-length encode
        let mut runs = 0usize;
        let mut prev = quantized[0];
        for &q in &quantized[1..] {
            if q != prev {
                runs += 1;
                prev = q;
            }
        }
        runs += 1; // Last run

        // Compressibility: 1 - (runs / length)
        // More runs = less compressible
        let compressibility = 1.0 - (runs as f64 / values.len() as f64);
        compressibility.clamp(0.0, 1.0)
    }

    /// Computes variance ratio (heteroscedasticity measure)
    fn compute_variance_ratio(values: &[f64], window_size: usize) -> f64 {
        if values.len() < window_size * 2 {
            return 1.0;
        }

        // Global variance
        let n = values.len() as f64;
        let mean: f64 = values.iter().sum::<f64>() / n;
        let global_variance: f64 = values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / n;

        if global_variance < f64::EPSILON {
            return 1.0;
        }

        // Local variances
        let num_windows = values.len() / window_size;
        let mut local_variances = Vec::with_capacity(num_windows);

        for i in 0..num_windows {
            let start = i * window_size;
            let end = start + window_size;
            let window = &values[start..end];

            let local_mean: f64 = window.iter().sum::<f64>() / window_size as f64;
            let local_var: f64 = window.iter().map(|v| (v - local_mean).powi(2)).sum::<f64>()
                / window_size as f64;

            local_variances.push(local_var);
        }

        // Variance of local variances
        let var_mean: f64 = local_variances.iter().sum::<f64>() / local_variances.len() as f64;
        let var_of_vars: f64 = local_variances
            .iter()
            .map(|v| (v - var_mean).powi(2))
            .sum::<f64>()
            / local_variances.len() as f64;

        // Ratio: higher means more heteroscedastic
        (var_of_vars.sqrt() / global_variance).clamp(0.0, 10.0) / 10.0
    }

    /// Simple stationarity test (compares first half vs second half)
    fn test_stationarity(values: &[f64]) -> bool {
        if values.len() < 20 {
            return true;
        }

        let mid = values.len() / 2;
        let first_half = &values[..mid];
        let second_half = &values[mid..];

        // Compare means
        let mean1: f64 = first_half.iter().sum::<f64>() / first_half.len() as f64;
        let mean2: f64 = second_half.iter().sum::<f64>() / second_half.len() as f64;

        // Compare variances
        let var1: f64 =
            first_half.iter().map(|v| (v - mean1).powi(2)).sum::<f64>() / first_half.len() as f64;
        let var2: f64 = second_half
            .iter()
            .map(|v| (v - mean2).powi(2))
            .sum::<f64>()
            / second_half.len() as f64;

        // Relative differences
        let global_mean = (mean1 + mean2) / 2.0;
        let global_var = (var1 + var2) / 2.0;

        let mean_diff = if global_mean.abs() > f64::EPSILON {
            (mean1 - mean2).abs() / global_mean.abs()
        } else {
            (mean1 - mean2).abs()
        };

        let var_diff = if global_var > f64::EPSILON {
            (var1 - var2).abs() / global_var
        } else {
            (var1 - var2).abs()
        };

        // Stationary if differences are small
        mean_diff < 0.5 && var_diff < 1.0
    }

    /// Detects structural segments using change point detection
    fn detect_segments(values: &[f64]) -> (usize, f64) {
        if values.len() < 10 {
            return (1, values.len() as f64);
        }

        let window = 5;
        let mut change_points = Vec::new();

        for i in window..values.len().saturating_sub(window) {
            let before = &values[i - window..i];
            let after = &values[i..i + window];

            let mean_before: f64 = before.iter().sum::<f64>() / before.len() as f64;
            let mean_after: f64 = after.iter().sum::<f64>() / after.len() as f64;

            let var_before: f64 = before.iter().map(|v| (v - mean_before).powi(2)).sum::<f64>()
                / before.len() as f64;
            let var_after: f64 =
                after.iter().map(|v| (v - mean_after).powi(2)).sum::<f64>() / after.len() as f64;

            let pooled_std = ((var_before + var_after) / 2.0).sqrt();

            // Significant change if mean difference > 2 standard deviations
            if pooled_std > f64::EPSILON && (mean_before - mean_after).abs() > 2.0 * pooled_std {
                change_points.push(i);
            }
        }

        // Remove adjacent change points
        let mut filtered_points = Vec::new();
        for &cp in &change_points {
            if filtered_points.is_empty()
                || cp - *filtered_points.last().unwrap() > window * 2
            {
                filtered_points.push(cp);
            }
        }

        let segment_count = filtered_points.len() + 1;
        let mean_segment_length = values.len() as f64 / segment_count as f64;

        (segment_count, mean_segment_length)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constant_signal_high_compressibility() {
        let values = vec![0.5; 100];
        let analysis = StructureAnalysis::analyze(&values);
        assert!(
            analysis.compressibility > 0.9,
            "Constant signal should be highly compressible"
        );
    }

    #[test]
    fn test_random_signal_low_compressibility() {
        // Pseudo-random signal
        let values: Vec<f64> = (0..200)
            .map(|i| ((i * 17 + 31) % 100) as f64 / 100.0)
            .collect();

        let analysis = StructureAnalysis::analyze(&values);
        assert!(
            analysis.compressibility < 0.5,
            "Random signal should have low compressibility"
        );
    }

    #[test]
    fn test_structured_signal() {
        // Signal with clear structure (step function)
        let mut values = vec![0.0; 50];
        values.extend(vec![1.0; 50]);
        values.extend(vec![0.0; 50]);

        let analysis = StructureAnalysis::analyze(&values);

        // Should detect segments
        assert!(analysis.segment_count >= 2, "Should detect multiple segments");

        // Should have structure (low local/global ratio)
        assert!(analysis.local_global_entropy_ratio < 1.0);
    }

    #[test]
    fn test_stationarity() {
        // Stationary signal (consistent statistics) - uniform random-ish
        // Using a deterministic pseudo-random pattern that has same mean/variance throughout
        let values: Vec<f64> = (0..200)
            .map(|i| ((i * 7 + 3) % 10) as f64 / 10.0)
            .collect();

        let analysis = StructureAnalysis::analyze(&values);
        assert!(analysis.stationarity_test_passed, 
            "Uniform signal should be stationary");

        // Non-stationary signal (mean shift)
        let mut non_stationary = vec![0.0; 100];
        non_stationary.extend(vec![10.0; 100]);

        let analysis2 = StructureAnalysis::analyze(&non_stationary);
        assert!(!analysis2.stationarity_test_passed,
            "Mean-shifted signal should be non-stationary");
    }

    #[test]
    fn test_empty_signal() {
        let values: Vec<f64> = vec![];
        let analysis = StructureAnalysis::analyze(&values);
        assert_eq!(analysis.segment_count, 1);
    }
}
