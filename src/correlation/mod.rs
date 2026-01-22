//! --------------------------
//! INFORMATION
//! --------------------------
//! Title: Correlation Module
//! Author: Carlos Eduardo Favini
//! Date: 2025-01-02
//! Version: 1.2.0
//! Description: Inference by correlation, not labeling.
//!              Patterns emerge from frequency, cooccurrence, and observed transformations.
//!              "The brain recognizes 'machine' before knowing the word 'machine'."
//! Layer: Community
//! Dependencies: sensory
//! Affected Components: cognitive output
//!
//! --------------------------
//! INSIGHT #3: INFERENCE BY CORRELATION
//! --------------------------
//! The brain does not classify first and then recognize.
//! It detects patterns through:
//! - Frequency: how often a pattern appears
//! - Cooccurrence: which patterns appear together
//! - Transformation: how patterns change over time
//!
//! This module provides the mathematical infrastructure for
//! pattern detection without semantic labels.
//!
//! --------------------------
//! CHANGE LOG
//! --------------------------
//! 2025-01-02 - Carlos Eduardo Favini - Initial creation (v1.2.0)
//! --------------------------

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

/// Hash of a detected pattern (content-addressable)
pub type PatternHash = String;

/// A correlation matrix between features.
/// No labels - just mathematical relationships.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorrelationMatrix {
    /// Pairwise correlation coefficients [i][j] = correlation between feature i and j
    /// Symmetric: correlations[i][j] == correlations[j][i]
    pub correlations: Vec<Vec<f64>>,

    /// Number of features being correlated
    pub dimension: usize,

    /// Number of observations used to compute correlations
    pub observation_count: usize,
}

impl CorrelationMatrix {
    /// Creates a new correlation matrix from observations.
    /// Each observation is a vector of feature values.
    pub fn from_observations(observations: &[Vec<f64>]) -> Option<Self> {
        if observations.is_empty() {
            return None;
        }

        let dimension = observations[0].len();
        if dimension == 0 {
            return None;
        }

        // Verify all observations have same dimension
        if !observations.iter().all(|o| o.len() == dimension) {
            return None;
        }

        let n = observations.len() as f64;

        // Calculate means
        let mut means = vec![0.0; dimension];
        for obs in observations {
            for (i, &val) in obs.iter().enumerate() {
                means[i] += val;
            }
        }
        for mean in &mut means {
            *mean /= n;
        }

        // Calculate correlation matrix
        let mut correlations = vec![vec![0.0; dimension]; dimension];

        for i in 0..dimension {
            for j in i..dimension {
                let mut cov = 0.0;
                let mut var_i = 0.0;
                let mut var_j = 0.0;

                for obs in observations {
                    let di = obs[i] - means[i];
                    let dj = obs[j] - means[j];
                    cov += di * dj;
                    if i == j {
                        var_i += di * di;
                    } else {
                        var_i += di * di;
                        var_j += dj * dj;
                    }
                }

                let correlation = if i == j {
                    1.0
                } else {
                    let denom = (var_i * var_j).sqrt();
                    if denom > f64::EPSILON {
                        cov / denom
                    } else {
                        0.0
                    }
                };

                correlations[i][j] = correlation;
                correlations[j][i] = correlation;
            }
        }

        Some(Self {
            correlations,
            dimension,
            observation_count: observations.len(),
        })
    }

    /// Returns the correlation between two features.
    pub fn get(&self, i: usize, j: usize) -> Option<f64> {
        if i < self.dimension && j < self.dimension {
            Some(self.correlations[i][j])
        } else {
            None
        }
    }

    /// Returns pairs of features with correlation above threshold.
    pub fn strong_correlations(&self, threshold: f64) -> Vec<(usize, usize, f64)> {
        let mut results = Vec::new();
        for i in 0..self.dimension {
            for j in (i + 1)..self.dimension {
                let corr = self.correlations[i][j];
                if corr.abs() >= threshold {
                    results.push((i, j, corr));
                }
            }
        }
        results
    }
}

/// Tracks cooccurrence of patterns without labels.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CooccurrenceTracker {
    /// How many times each pattern was observed
    pub pattern_counts: HashMap<PatternHash, usize>,

    /// How many times two patterns occurred together
    /// Key is (hash1, hash2) where hash1 < hash2 lexicographically
    pub pair_counts: HashMap<(PatternHash, PatternHash), usize>,

    /// Total observations
    pub total_observations: usize,
}

impl CooccurrenceTracker {
    /// Creates a new empty tracker.
    pub fn new() -> Self {
        Self::default()
    }

    /// Records observation of a set of patterns occurring together.
    pub fn observe(&mut self, patterns: &[PatternHash]) {
        self.total_observations += 1;

        // Count individual patterns
        for pattern in patterns {
            *self.pattern_counts.entry(pattern.clone()).or_insert(0) += 1;
        }

        // Count pairs
        for i in 0..patterns.len() {
            for j in (i + 1)..patterns.len() {
                let (p1, p2) = if patterns[i] < patterns[j] {
                    (patterns[i].clone(), patterns[j].clone())
                } else {
                    (patterns[j].clone(), patterns[i].clone())
                };
                *self.pair_counts.entry((p1, p2)).or_insert(0) += 1;
            }
        }
    }

    /// Returns the frequency of a pattern (0.0 to 1.0).
    pub fn frequency(&self, pattern: &PatternHash) -> f64 {
        if self.total_observations == 0 {
            return 0.0;
        }
        let count = self.pattern_counts.get(pattern).copied().unwrap_or(0);
        count as f64 / self.total_observations as f64
    }

    /// Returns the cooccurrence frequency of two patterns.
    pub fn cooccurrence_frequency(&self, p1: &PatternHash, p2: &PatternHash) -> f64 {
        if self.total_observations == 0 {
            return 0.0;
        }
        let key = if p1 < p2 {
            (p1.clone(), p2.clone())
        } else {
            (p2.clone(), p1.clone())
        };
        let count = self.pair_counts.get(&key).copied().unwrap_or(0);
        count as f64 / self.total_observations as f64
    }

    /// Returns the conditional probability P(p2 | p1).
    pub fn conditional_probability(&self, p1: &PatternHash, p2: &PatternHash) -> f64 {
        let p1_count = self.pattern_counts.get(p1).copied().unwrap_or(0);
        if p1_count == 0 {
            return 0.0;
        }

        let key = if p1 < p2 {
            (p1.clone(), p2.clone())
        } else {
            (p2.clone(), p1.clone())
        };
        let pair_count = self.pair_counts.get(&key).copied().unwrap_or(0);
        pair_count as f64 / p1_count as f64
    }

    /// Returns the pointwise mutual information between two patterns.
    /// PMI = log2(P(p1,p2) / (P(p1) * P(p2)))
    pub fn pointwise_mutual_information(&self, p1: &PatternHash, p2: &PatternHash) -> f64 {
        let p_p1 = self.frequency(p1);
        let p_p2 = self.frequency(p2);
        let p_joint = self.cooccurrence_frequency(p1, p2);

        if p_p1 < f64::EPSILON || p_p2 < f64::EPSILON || p_joint < f64::EPSILON {
            return 0.0;
        }

        (p_joint / (p_p1 * p_p2)).log2()
    }
}

/// An observed transformation from one pattern to another.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transformation {
    /// Pattern before transformation
    pub from: PatternHash,

    /// Pattern after transformation
    pub to: PatternHash,

    /// How many times this transformation was observed
    pub count: usize,

    /// Average time between observations (in arbitrary units)
    pub avg_interval: f64,
}

/// Tracks transformations between patterns.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TransformationTracker {
    /// All observed transformations
    pub transformations: HashMap<(PatternHash, PatternHash), Transformation>,

    /// Total transformations observed
    pub total_transformations: usize,
}

impl TransformationTracker {
    /// Creates a new empty tracker.
    pub fn new() -> Self {
        Self::default()
    }

    /// Records a transformation from one pattern to another.
    pub fn observe(&mut self, from: PatternHash, to: PatternHash, interval: f64) {
        self.total_transformations += 1;

        let key = (from.clone(), to.clone());
        let entry = self.transformations.entry(key).or_insert(Transformation {
            from,
            to,
            count: 0,
            avg_interval: 0.0,
        });

        // Update running average
        let old_count = entry.count as f64;
        let new_count = old_count + 1.0;
        entry.avg_interval = (entry.avg_interval * old_count + interval) / new_count;
        entry.count += 1;
    }

    /// Returns the probability of transformation from p1 to p2.
    pub fn transformation_probability(&self, from: &PatternHash, to: &PatternHash) -> f64 {
        let key = (from.clone(), to.clone());
        let count = self
            .transformations
            .get(&key)
            .map(|t| t.count)
            .unwrap_or(0);

        // Count all transformations FROM this pattern
        let from_count: usize = self
            .transformations
            .iter()
            .filter(|((f, _), _)| f == from)
            .map(|(_, t)| t.count)
            .sum();

        if from_count == 0 {
            return 0.0;
        }

        count as f64 / from_count as f64
    }

    /// Returns the most likely next patterns after observing a given pattern.
    pub fn predict_next(&self, from: &PatternHash, top_k: usize) -> Vec<(PatternHash, f64)> {
        let mut predictions: Vec<(PatternHash, f64)> = self
            .transformations
            .iter()
            .filter(|((f, _), _)| f == from)
            .map(|((_, to), _)| {
                (to.clone(), self.transformation_probability(from, to))
            })
            .collect();

        predictions.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        predictions.truncate(top_k);
        predictions
    }
}

/// Computes a content-addressable hash for a pattern.
pub fn hash_pattern(values: &[f64]) -> PatternHash {
    let mut hasher = Sha256::new();
    for v in values {
        hasher.update(v.to_le_bytes());
    }
    hex::encode(hasher.finalize())
}

/// Quantizes a value to a discrete level for pattern matching.
pub fn quantize(value: f64, levels: usize) -> usize {
    let clamped = value.clamp(0.0, 1.0);
    ((clamped * (levels - 1) as f64).round() as usize).min(levels - 1)
}

/// Discretizes a vector of values for pattern matching.
pub fn discretize(values: &[f64], levels: usize) -> Vec<usize> {
    values.iter().map(|&v| quantize(v, levels)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correlation_matrix() {
        // Two perfectly correlated features
        let observations = vec![
            vec![0.0, 0.0],
            vec![0.5, 0.5],
            vec![1.0, 1.0],
        ];

        let matrix = CorrelationMatrix::from_observations(&observations).unwrap();
        assert_eq!(matrix.dimension, 2);
        assert!((matrix.get(0, 1).unwrap() - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_correlation_negative() {
        // Two negatively correlated features
        let observations = vec![
            vec![0.0, 1.0],
            vec![0.5, 0.5],
            vec![1.0, 0.0],
        ];

        let matrix = CorrelationMatrix::from_observations(&observations).unwrap();
        assert!(matrix.get(0, 1).unwrap() < -0.9);
    }

    #[test]
    fn test_cooccurrence_tracker() {
        let mut tracker = CooccurrenceTracker::new();

        tracker.observe(&["A".to_string(), "B".to_string()]);
        tracker.observe(&["A".to_string(), "B".to_string()]);
        tracker.observe(&["A".to_string(), "C".to_string()]);

        assert_eq!(tracker.total_observations, 3);
        assert!((tracker.frequency(&"A".to_string()) - 1.0).abs() < 0.01);
        assert!((tracker.frequency(&"B".to_string()) - 2.0 / 3.0).abs() < 0.01);
    }

    #[test]
    fn test_conditional_probability() {
        let mut tracker = CooccurrenceTracker::new();

        // A appears 3 times, B appears with A 2 times
        tracker.observe(&["A".to_string(), "B".to_string()]);
        tracker.observe(&["A".to_string(), "B".to_string()]);
        tracker.observe(&["A".to_string(), "C".to_string()]);

        let p_b_given_a = tracker.conditional_probability(&"A".to_string(), &"B".to_string());
        assert!((p_b_given_a - 2.0 / 3.0).abs() < 0.01);
    }

    #[test]
    fn test_transformation_tracker() {
        let mut tracker = TransformationTracker::new();

        tracker.observe("A".to_string(), "B".to_string(), 1.0);
        tracker.observe("A".to_string(), "B".to_string(), 2.0);
        tracker.observe("A".to_string(), "C".to_string(), 1.0);

        let p_ab = tracker.transformation_probability(&"A".to_string(), &"B".to_string());
        assert!((p_ab - 2.0 / 3.0).abs() < 0.01);

        let predictions = tracker.predict_next(&"A".to_string(), 2);
        assert_eq!(predictions.len(), 2);
        assert_eq!(predictions[0].0, "B");
    }

    #[test]
    fn test_hash_pattern() {
        let values1 = vec![1.0, 2.0, 3.0];
        let values2 = vec![1.0, 2.0, 3.0];
        let values3 = vec![1.0, 2.0, 4.0];

        let hash1 = hash_pattern(&values1);
        let hash2 = hash_pattern(&values2);
        let hash3 = hash_pattern(&values3);

        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
    }

    #[test]
    fn test_quantize() {
        assert_eq!(quantize(0.0, 4), 0);
        assert_eq!(quantize(0.5, 4), 2);
        assert_eq!(quantize(1.0, 4), 3);
        assert_eq!(quantize(1.5, 4), 3); // Clamped
    }
}
