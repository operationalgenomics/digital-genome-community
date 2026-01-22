//! --------------------------
//! INFORMATION
//! --------------------------
//! Title: Carrier Analysis (Level 0)
//! Author: Carlos Eduardo Favini
//! Date: 2025-01-02
//! Version: 1.2.0
//! Description: Level 0 of the abstraction hierarchy.
//! Computes basic statistical properties of raw signal.
//! Shannon entropy, basic statistics, byte distribution.
//! Layer: Community
//! Dependencies: None (pure math)
//! Affected Components: sensory/signals
//!
//! --------------------------
//! MATHEMATICAL BASIS
//! --------------------------
//! Shannon Entropy: H = -Σ p(x) log₂ p(x)
//! Normalized: H_norm = H / log₂(N) where N = number of unique values
//!
//! This measures the "surprise" or "uncertainty" in the signal.
//! Low entropy = ordered, predictable
//! High entropy = disordered, unpredictable
//!
//! --------------------------
//! CHANGE LOG
//! --------------------------
//! 2025-01-02 - Carlos Eduardo Favini - Initial creation (v1.1.0)
//! --------------------------

use std::collections::BTreeMap;

/// Result of carrier-level analysis
#[derive(Debug, Clone)]
pub struct CarrierAnalysis {
    /// Shannon entropy (normalized 0-1)
    pub entropy: f64,

    /// Number of samples
    pub sample_count: usize,

    /// Number of unique values
    pub unique_count: usize,

    /// Minimum value
    pub min: f64,

    /// Maximum value
    pub max: f64,

    /// Mean value
    pub mean: f64,

    /// Standard deviation
    pub std_dev: f64,

    /// Zero-crossing rate
    pub zero_crossing_rate: f64,
}

impl CarrierAnalysis {
    /// Analyzes raw bytes as unsigned 8-bit values
    pub fn from_bytes(data: &[u8]) -> Self {
        if data.is_empty() {
            return Self::empty();
        }

        // Convert to f64 for calculations
        let values: Vec<f64> = data.iter().map(|&b| b as f64).collect();
        Self::from_values(&values)
    }

    /// Analyzes a slice of f64 values
    pub fn from_values(values: &[f64]) -> Self {
        if values.is_empty() {
            return Self::empty();
        }

        let n = values.len();

        // Basic statistics
        let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let sum: f64 = values.iter().sum();
        let mean = sum / n as f64;

        // Standard deviation
        let variance: f64 = values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / n as f64;
        let std_dev = variance.sqrt();

        // Zero-crossing rate
        let zero_crossings = values
            .windows(2)
            .filter(|w| (w[0] - mean) * (w[1] - mean) < 0.0)
            .count();
        let zero_crossing_rate = if n > 1 {
            zero_crossings as f64 / (n - 1) as f64
        } else {
            0.0
        };

        // Entropy calculation
        let (entropy, unique_count) = Self::calculate_entropy(values);

        Self {
            entropy,
            sample_count: n,
            unique_count,
            min,
            max,
            mean,
            std_dev,
            zero_crossing_rate,
        }
    }

    /// Creates an empty analysis (for empty input)
    fn empty() -> Self {
        Self {
            entropy: 0.0,
            sample_count: 0,
            unique_count: 0,
            min: 0.0,
            max: 0.0,
            mean: 0.0,
            std_dev: 0.0,
            zero_crossing_rate: 0.0,
        }
    }

    /// Calculates Shannon entropy of the signal
    /// Returns (normalized_entropy, unique_count)
    fn calculate_entropy(values: &[f64]) -> (f64, usize) {
        if values.is_empty() {
            return (0.0, 0);
        }

        // Discretize to histogram bins for continuous data
        // Use 256 bins (like byte values) for consistency
        let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

        if (max - min).abs() < f64::EPSILON {
            // All values are the same - zero entropy
            return (0.0, 1);
        }

        let num_bins = 256usize;
        let bin_width = (max - min) / num_bins as f64;

        // Count frequency of each bin
        let mut counts: BTreeMap<usize, usize> = BTreeMap::new();
        for &v in values {
            let bin = ((v - min) / bin_width).floor() as usize;
            let bin = bin.min(num_bins - 1); // Clamp to valid range
            *counts.entry(bin).or_insert(0) += 1;
        }

        let n = values.len() as f64;
        let unique_count = counts.len();

        // Shannon entropy: H = -Σ p(x) log₂ p(x)
        let mut entropy = 0.0;
        for &count in counts.values() {
            if count > 0 {
                let p = count as f64 / n;
                entropy -= p * p.log2();
            }
        }

        // Normalize by maximum possible entropy (log₂ of bin count)
        let max_entropy = (num_bins as f64).log2();
        let normalized_entropy = if max_entropy > 0.0 {
            entropy / max_entropy
        } else {
            0.0
        };

        (normalized_entropy.clamp(0.0, 1.0), unique_count)
    }
}

/// Converts raw bytes to f64 values using different interpretations
pub struct ByteInterpreter;

impl ByteInterpreter {
    /// Interpret bytes as unsigned 8-bit values [0, 255]
    pub fn as_u8(data: &[u8]) -> Vec<f64> {
        data.iter().map(|&b| b as f64).collect()
    }

    /// Interpret bytes as signed 8-bit values [-128, 127]
    pub fn as_i8(data: &[u8]) -> Vec<f64> {
        data.iter().map(|&b| (b as i8) as f64).collect()
    }

    /// Interpret bytes as 16-bit unsigned (big-endian)
    pub fn as_u16_be(data: &[u8]) -> Vec<f64> {
        data.chunks_exact(2)
            .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]) as f64)
            .collect()
    }

    /// Interpret bytes as 16-bit signed (big-endian)
    pub fn as_i16_be(data: &[u8]) -> Vec<f64> {
        data.chunks_exact(2)
            .map(|chunk| i16::from_be_bytes([chunk[0], chunk[1]]) as f64)
            .collect()
    }

    /// Interpret bytes as 32-bit float (big-endian)
    pub fn as_f32_be(data: &[u8]) -> Vec<f64> {
        data.chunks_exact(4)
            .map(|chunk| {
                f32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]) as f64
            })
            .collect()
    }

    /// Interpret bytes as ASCII text values
    pub fn as_ascii(data: &[u8]) -> Vec<f64> {
        data.iter()
            .filter(|&&b| b.is_ascii())
            .map(|&b| b as f64)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constant_signal_zero_entropy() {
        let data = vec![128u8; 1000];
        let analysis = CarrierAnalysis::from_bytes(&data);
        assert_eq!(analysis.entropy, 0.0);
        assert_eq!(analysis.unique_count, 1);
    }

    #[test]
    fn test_uniform_distribution_high_entropy() {
        // All possible byte values equally distributed
        let data: Vec<u8> = (0..=255).cycle().take(256 * 100).collect();
        let analysis = CarrierAnalysis::from_bytes(&data);
        // Should be close to 1.0 (maximum entropy)
        assert!(analysis.entropy > 0.99);
    }

    #[test]
    fn test_basic_statistics() {
        let data = vec![0u8, 50, 100, 150, 200, 250];
        let analysis = CarrierAnalysis::from_bytes(&data);
        assert_eq!(analysis.sample_count, 6);
        assert_eq!(analysis.min, 0.0);
        assert_eq!(analysis.max, 250.0);
        assert!((analysis.mean - 125.0).abs() < 0.01);
    }

    #[test]
    fn test_empty_input() {
        let data: Vec<u8> = vec![];
        let analysis = CarrierAnalysis::from_bytes(&data);
        assert_eq!(analysis.sample_count, 0);
        assert_eq!(analysis.entropy, 0.0);
    }

    #[test]
    fn test_zero_crossing_rate() {
        // Signal that crosses zero frequently
        let values: Vec<f64> = vec![1.0, -1.0, 1.0, -1.0, 1.0, -1.0];
        let analysis = CarrierAnalysis::from_values(&values);
        // All transitions cross zero (around mean of 0)
        assert!(analysis.zero_crossing_rate > 0.9);
    }
}
