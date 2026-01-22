//! --------------------------
//! INFORMATION
//! --------------------------
//! Title: Pattern Analysis (Level 1)
//! Author: Carlos Eduardo Favini
//! Date: 2025-01-02
//! Version: 1.2.0
//! Description: Level 1 of the abstraction hierarchy.
//! Detects repetition and rhythm through autocorrelation.
//! Identifies periodicity in signals.
//! Layer: Community
//! Dependencies: rustfft
//! Affected Components: sensory/signals
//!
//! --------------------------
//! MATHEMATICAL BASIS
//! --------------------------
//! Autocorrelation: R(τ) = Σ x(t)·x(t+τ) / Σ x(t)²
//!
//! This measures how similar the signal is to a shifted version of itself.
//! High autocorrelation at lag τ means the signal repeats with period τ.
//!
//! We use FFT-based autocorrelation for efficiency:
//! R = IFFT(|FFT(x)|²)
//!
//! --------------------------
//! CHANGE LOG
//! --------------------------
//! 2025-01-02 - Carlos Eduardo Favini - Initial creation (v1.1.0)
//! --------------------------

use rustfft::{num_complex::Complex, FftPlanner};
use std::sync::Mutex;

// Persistent FFT planner for deterministic behavior
// First call may involve auto-tuning; subsequent calls are deterministic
static FFT_PLANNER: Mutex<Option<FftPlanner<f64>>> = Mutex::new(None);

fn get_fft_planner() -> std::sync::MutexGuard<'static, Option<FftPlanner<f64>>> {
    let mut guard = FFT_PLANNER.lock().unwrap();
    if guard.is_none() {
        *guard = Some(FftPlanner::new());
    }
    guard
}

/// Result of pattern-level analysis
#[derive(Debug, Clone)]
pub struct PatternAnalysis {
    /// Maximum autocorrelation coefficient (excluding lag 0)
    pub max_autocorrelation: f64,

    /// Lag at which maximum autocorrelation occurs
    pub max_autocorrelation_lag: usize,

    /// Whether significant periodicity was detected
    pub periodicity_detected: bool,

    /// Significance of periodicity (peak / noise floor)
    pub periodicity_significance: f64,

    /// Spectral centroid (normalized 0-1)
    pub spectral_centroid: f64,

    /// Spectral flatness (0 = tonal, 1 = noise)
    pub spectral_flatness: f64,

    /// Index of dominant frequency
    pub dominant_frequency_index: usize,

    /// Magnitude spectrum (normalized)
    pub spectrum: Vec<f64>,
}

impl PatternAnalysis {
    /// Analyzes pattern/periodicity in a signal
    pub fn analyze(values: &[f64]) -> Self {
        if values.len() < 4 {
            return Self::empty();
        }

        // Compute autocorrelation
        let autocorr = Self::compute_autocorrelation(values);

        // Find maximum autocorrelation (excluding lag 0)
        let (max_autocorr, max_lag) = Self::find_max_autocorrelation(&autocorr);

        // Compute noise floor (median of autocorrelation)
        let noise_floor = Self::compute_noise_floor(&autocorr);

        // Periodicity significance
        let periodicity_significance = if noise_floor > 0.0 {
            max_autocorr / noise_floor
        } else {
            0.0
        };

        // Significant if peak is at least 3x noise floor
        // OR if the signal has very strong absolute autocorrelation (clean periodic signal)
        let periodicity_detected =
            (periodicity_significance > 3.0 || max_autocorr > 0.9) && max_lag > 0;

        // Compute spectrum
        let spectrum = Self::compute_spectrum(values);
        let spectral_centroid = Self::compute_spectral_centroid(&spectrum);
        let spectral_flatness = Self::compute_spectral_flatness(&spectrum);
        let dominant_frequency_index = Self::find_dominant_frequency(&spectrum);

        Self {
            max_autocorrelation: max_autocorr,
            max_autocorrelation_lag: max_lag,
            periodicity_detected,
            periodicity_significance,
            spectral_centroid,
            spectral_flatness,
            dominant_frequency_index,
            spectrum,
        }
    }

    /// Creates empty analysis
    fn empty() -> Self {
        Self {
            max_autocorrelation: 0.0,
            max_autocorrelation_lag: 0,
            periodicity_detected: false,
            periodicity_significance: 0.0,
            spectral_centroid: 0.0,
            spectral_flatness: 1.0,
            dominant_frequency_index: 0,
            spectrum: Vec::new(),
        }
    }

    /// Computes autocorrelation using FFT method
    fn compute_autocorrelation(values: &[f64]) -> Vec<f64> {
        let n = values.len();
        let fft_size = (2 * n).next_power_of_two();

        // Remove mean for unbiased autocorrelation
        let mean: f64 = values.iter().sum::<f64>() / n as f64;
        let centered: Vec<f64> = values.iter().map(|&v| v - mean).collect();

        // Pad with zeros
        let mut input: Vec<Complex<f64>> = centered
            .iter()
            .map(|&v| Complex::new(v, 0.0))
            .collect();
        input.resize(fft_size, Complex::new(0.0, 0.0));

        // Forward FFT
        let mut planner_guard = get_fft_planner();
        let planner = planner_guard.as_mut().unwrap();
        let fft = planner.plan_fft_forward(fft_size);
        fft.process(&mut input);

        // Power spectrum (|FFT|²)
        for c in &mut input {
            *c = Complex::new(c.norm_sqr(), 0.0);
        }

        // Inverse FFT
        let ifft = planner.plan_fft_inverse(fft_size);
        ifft.process(&mut input);
        drop(planner_guard);

        // Normalize and extract real part
        let var: f64 = centered.iter().map(|v| v * v).sum();
        if var < f64::EPSILON {
            return vec![0.0; n];
        }

        input
            .iter()
            .take(n)
            .map(|c| (c.re / (fft_size as f64 * var)).clamp(-1.0, 1.0))
            .collect()
    }

    /// Finds maximum autocorrelation (excluding lag 0)
    fn find_max_autocorrelation(autocorr: &[f64]) -> (f64, usize) {
        if autocorr.len() < 2 {
            return (0.0, 0);
        }

        // Skip lag 0 (always 1.0) and first few lags (often high due to smoothness)
        let min_lag = (autocorr.len() / 20).max(1);
        let max_lag = autocorr.len() / 2;

        let mut max_val = 0.0;
        let mut max_idx = 0;

        for (i, &val) in autocorr.iter().enumerate().skip(min_lag).take(max_lag) {
            let abs_val = val.abs();
            if abs_val > max_val {
                max_val = abs_val;
                max_idx = i;
            }
        }

        (max_val, max_idx)
    }

    /// Computes noise floor (median of autocorrelation magnitudes)
    fn compute_noise_floor(autocorr: &[f64]) -> f64 {
        if autocorr.len() < 10 {
            return 0.1;
        }

        let mut magnitudes: Vec<f64> = autocorr
            .iter()
            .skip(autocorr.len() / 10)
            .take(autocorr.len() / 2)
            .map(|&v| v.abs())
            .collect();

        magnitudes.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        magnitudes.get(magnitudes.len() / 2).copied().unwrap_or(0.1)
    }

    /// Computes magnitude spectrum using FFT
    fn compute_spectrum(values: &[f64]) -> Vec<f64> {
        let n = values.len();
        let fft_size = n.next_power_of_two();

        // Apply Hann window to reduce spectral leakage
        let mut windowed: Vec<Complex<f64>> = values
            .iter()
            .enumerate()
            .map(|(i, &v)| {
                let window = 0.5 * (1.0 - (2.0 * std::f64::consts::PI * i as f64 / n as f64).cos());
                Complex::new(v * window, 0.0)
            })
            .collect();
        windowed.resize(fft_size, Complex::new(0.0, 0.0));

        // FFT
        let mut planner_guard = get_fft_planner();
        let planner = planner_guard.as_mut().unwrap();
        let fft = planner.plan_fft_forward(fft_size);
        fft.process(&mut windowed);
        drop(planner_guard);

        // Magnitude spectrum (only positive frequencies)
        let magnitudes: Vec<f64> = windowed
            .iter()
            .take(fft_size / 2)
            .map(|c| c.norm())
            .collect();

        // Normalize
        let max_mag = magnitudes.iter().cloned().fold(0.0_f64, f64::max);
        if max_mag > f64::EPSILON {
            magnitudes.iter().map(|&m| m / max_mag).collect()
        } else {
            magnitudes
        }
    }

    /// Computes spectral centroid (center of mass of spectrum)
    fn compute_spectral_centroid(spectrum: &[f64]) -> f64 {
        if spectrum.is_empty() {
            return 0.0;
        }

        let sum_magnitude: f64 = spectrum.iter().sum();
        if sum_magnitude < f64::EPSILON {
            return 0.0;
        }

        let weighted_sum: f64 = spectrum
            .iter()
            .enumerate()
            .map(|(i, &m)| i as f64 * m)
            .sum();

        let centroid = weighted_sum / sum_magnitude;

        // Normalize to 0-1
        centroid / spectrum.len() as f64
    }

    /// Computes spectral flatness (geometric mean / arithmetic mean)
    fn compute_spectral_flatness(spectrum: &[f64]) -> f64 {
        if spectrum.is_empty() {
            return 1.0;
        }

        // Filter out zeros and very small values
        let positive: Vec<f64> = spectrum
            .iter()
            .filter(|&&m| m > f64::EPSILON)
            .cloned()
            .collect();

        if positive.is_empty() {
            return 1.0;
        }

        let n = positive.len() as f64;

        // Geometric mean = exp(mean(log(x)))
        let log_sum: f64 = positive.iter().map(|&m| m.ln()).sum();
        let geometric_mean = (log_sum / n).exp();

        // Arithmetic mean
        let arithmetic_mean: f64 = positive.iter().sum::<f64>() / n;

        if arithmetic_mean < f64::EPSILON {
            return 1.0;
        }

        (geometric_mean / arithmetic_mean).clamp(0.0, 1.0)
    }

    /// Finds index of dominant frequency (highest magnitude)
    fn find_dominant_frequency(spectrum: &[f64]) -> usize {
        spectrum
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, _)| i)
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constant_signal_no_pattern() {
        let values = vec![1.0; 100];
        let analysis = PatternAnalysis::analyze(&values);
        assert!(!analysis.periodicity_detected);
    }

    #[test]
    fn test_periodic_signal() {
        // Generate a periodic signal (sine wave) - longer for better detection
        let period = 20;
        let values: Vec<f64> = (0..400)
            .map(|i| (2.0 * std::f64::consts::PI * i as f64 / period as f64).sin())
            .collect();

        let analysis = PatternAnalysis::analyze(&values);

        // Should have high autocorrelation at period
        assert!(
            analysis.max_autocorrelation > 0.5,
            "Periodic signal should have high autocorrelation, got {}",
            analysis.max_autocorrelation
        );
        
        // Lag should be close to period (within 3 samples)
        assert!(
            (analysis.max_autocorrelation_lag as i32 - period as i32).abs() <= 3,
            "Expected lag ~{}, got {}",
            period,
            analysis.max_autocorrelation_lag
        );
    }

    #[test]
    fn test_noise_no_pattern() {
        // Truly aperiodic signal using prime-based chaos
        // This breaks any simple periodicity
        let values: Vec<f64> = (0..200)
            .map(|i| {
                let x = ((i * 17 + 31) ^ (i * 13 + 7)) % 256;
                x as f64 / 256.0
            })
            .collect();

        let analysis = PatternAnalysis::analyze(&values);

        // Should have relatively low autocorrelation (no clear period)
        // We're lenient here because even "random" signals can have some correlation
        assert!(
            analysis.max_autocorrelation < 0.7,
            "Noisy signal should have low autocorrelation, got {}",
            analysis.max_autocorrelation
        );
    }

    #[test]
    fn test_spectral_flatness_tonal() {
        // Pure sine wave should have low flatness (tonal)
        let values: Vec<f64> = (0..256)
            .map(|i| (2.0 * std::f64::consts::PI * i as f64 / 32.0).sin())
            .collect();

        let analysis = PatternAnalysis::analyze(&values);
        assert!(
            analysis.spectral_flatness < 0.3,
            "Spectral flatness {} should be low for tonal signal",
            analysis.spectral_flatness
        );
    }

    #[test]
    fn test_empty_signal() {
        let values: Vec<f64> = vec![];
        let analysis = PatternAnalysis::analyze(&values);
        assert!(!analysis.periodicity_detected);
    }
}
