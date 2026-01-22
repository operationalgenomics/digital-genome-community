//! --------------------------
//! INFORMATION
//! --------------------------
//! Title: Praxeological Motor (M_P)
//! Author: Carlos Eduardo Favini
//! Date: 2025-01-02
//! Version: 1.2.0
//! Description: Motor derived from Ludwig von Mises praxeology.
//! Recognizes reproducible action cycles through reverse
//! engineering of momentum. Analyzes frame-by-frame without
//! modifying the observed truth.
//! Layer: Community
//! Dependencies: Consagrated mathematics (set theory, measure theory)
//! Affected Components: math/craft
//!
//! --------------------------
//! CHANGE LOG
//! --------------------------
//! 2025-01-02 - Carlos Eduardo Favini - Initial creation
//! 2025-01-02 - Carlos Eduardo Favini - Added was_clamped signaling (v0.2.0)
//! --------------------------

use serde::{Deserialize, Serialize};

use super::CognitiveMotor;

/// Praxeological Motor (M_P).
///
/// Evaluates coherence between means and ends in action sequences.
/// Derived from Ludwig von Mises' study of human action, extended
/// to any observable system.
///
/// # Function
/// Recognizes reproducible action cycles or their absence.
/// Operates by reverse engineering the "momentum" of actions.
///
/// # Restrictions (Constitutional)
/// - Does NOT alter observed truth
/// - Does NOT correct
/// - Does NOT optimize
/// - Does NOT interpret intention
///
/// # Output
/// - Structure of observed action
/// - Coherence or rupture of cycles
/// - Continuous score [0.0, 1.0]
#[derive(Debug, Clone)]
pub struct PraxisMotor;

/// Input for the Praxeological Motor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PraxisInput {
    /// Elements proposed in the plan.
    pub proposed: Vec<String>,

    /// Elements necessary for completion.
    pub necessary: Vec<String>,

    /// Context vector for similarity calculation.
    pub context_vector: Vec<f64>,

    /// Historical centroid for coherence calculation.
    pub history_centroid: Vec<f64>,
}

/// Output from the Praxeological Motor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PraxisOutput {
    /// Completeness factor: φ_comp ∈ [0, 1]
    /// Measures how much of the necessary is present in proposed.
    pub phi_completeness: f64,

    /// Coherence factor: φ_coer ∈ [0, 1]
    /// Measures alignment with historical patterns.
    pub phi_coherence: f64,

    /// Adequacy factor: φ_adeq ∈ [0, 1]
    /// Measures contextual appropriateness.
    pub phi_adequacy: f64,

    /// Final motor score: M_P = φ_comp × φ_coer × φ_adeq
    pub score: f64,

    /// Whether the final score was clamped to [0, 1].
    pub was_clamped: bool,

    /// Original score before clamping (if was_clamped is true).
    pub unclamped_score: Option<f64>,
}

impl PraxisMotor {
    /// Creates a new Praxeological Motor.
    pub fn new() -> Self {
        Self
    }

    /// Calculates completeness: |proposed ∩ necessary| / |necessary|
    ///
    /// This is a pure set-theoretic measure with no arbitrary thresholds.
    fn calculate_completeness(proposed: &[String], necessary: &[String]) -> f64 {
        if necessary.is_empty() {
            return 1.0; // Nothing required = complete
        }

        let intersection_count = necessary
            .iter()
            .filter(|n| proposed.contains(n))
            .count();

        intersection_count as f64 / necessary.len() as f64
    }

    /// Calculates coherence: cosine similarity with history.
    ///
    /// Uses standard cosine similarity - no arbitrary parameters.
    fn calculate_coherence(context: &[f64], centroid: &[f64]) -> f64 {
        if context.is_empty() || centroid.is_empty() {
            return 1.0; // No history = coherent by default
        }

        if context.len() != centroid.len() {
            return 0.0; // Dimension mismatch
        }

        let dot_product: f64 = context
            .iter()
            .zip(centroid.iter())
            .map(|(a, b)| a * b)
            .sum();

        let norm_context: f64 = context.iter().map(|x| x * x).sum::<f64>().sqrt();
        let norm_centroid: f64 = centroid.iter().map(|x| x * x).sum::<f64>().sqrt();

        if norm_context == 0.0 || norm_centroid == 0.0 {
            return 0.0;
        }

        // Cosine similarity ∈ [-1, 1], normalize to [0, 1]
        let cosine = dot_product / (norm_context * norm_centroid);
        (cosine + 1.0) / 2.0
    }

    /// Calculates adequacy: inverse sigmoid of divergence magnitude.
    ///
    /// Uses the standard sigmoid function - no arbitrary thresholds.
    /// φ_adeq = 1 / (1 + ||context||)
    fn calculate_adequacy(context: &[f64]) -> f64 {
        if context.is_empty() {
            return 1.0; // No context = adequate by default
        }

        let magnitude: f64 = context.iter().map(|x| x * x).sum::<f64>().sqrt();
        1.0 / (1.0 + magnitude)
    }
}

impl Default for PraxisMotor {
    fn default() -> Self {
        Self::new()
    }
}

impl CognitiveMotor for PraxisMotor {
    type Input = PraxisInput;
    type Output = PraxisOutput;

    /// Evaluates the action and produces M_P score.
    ///
    /// Formula: M_P = φ_comp × φ_coer × φ_adeq
    fn evaluate(&self, input: &Self::Input) -> Self::Output {
        let phi_completeness = Self::calculate_completeness(&input.proposed, &input.necessary);
        let phi_coherence = Self::calculate_coherence(&input.context_vector, &input.history_centroid);
        let phi_adequacy = Self::calculate_adequacy(&input.context_vector);

        // Calculate raw score
        let raw_score = phi_completeness * phi_coherence * phi_adequacy;

        // Check if clamping is needed
        let needs_clamping = raw_score < 0.0 || raw_score > 1.0 || !raw_score.is_finite();
        let final_score = if needs_clamping {
            if raw_score.is_nan() { 0.0 } else { raw_score.clamp(0.0, 1.0) }
        } else {
            raw_score
        };

        PraxisOutput {
            phi_completeness,
            phi_coherence,
            phi_adequacy,
            score: final_score,
            was_clamped: needs_clamping,
            unclamped_score: if needs_clamping { Some(raw_score) } else { None },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complete_match() {
        let motor = PraxisMotor::new();

        let input = PraxisInput {
            proposed: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            necessary: vec!["a".to_string(), "b".to_string()],
            context_vector: vec![1.0, 0.0, 0.0],
            history_centroid: vec![1.0, 0.0, 0.0],
        };

        let output = motor.evaluate(&input);

        assert!((output.phi_completeness - 1.0).abs() < 1e-10);
        assert!(output.phi_coherence > 0.99);
        assert!(!output.was_clamped);
    }

    #[test]
    fn test_partial_match() {
        let motor = PraxisMotor::new();

        let input = PraxisInput {
            proposed: vec!["a".to_string()],
            necessary: vec!["a".to_string(), "b".to_string()],
            context_vector: vec![1.0, 0.0],
            history_centroid: vec![1.0, 0.0],
        };

        let output = motor.evaluate(&input);

        assert!((output.phi_completeness - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_no_match() {
        let motor = PraxisMotor::new();

        let input = PraxisInput {
            proposed: vec!["x".to_string()],
            necessary: vec!["a".to_string(), "b".to_string()],
            context_vector: vec![1.0, 0.0],
            history_centroid: vec![1.0, 0.0],
        };

        let output = motor.evaluate(&input);

        assert!((output.phi_completeness - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_orthogonal_vectors() {
        let motor = PraxisMotor::new();

        let input = PraxisInput {
            proposed: vec!["a".to_string()],
            necessary: vec!["a".to_string()],
            context_vector: vec![1.0, 0.0],
            history_centroid: vec![0.0, 1.0],
        };

        let output = motor.evaluate(&input);

        // Orthogonal = cosine 0 = normalized to 0.5
        assert!((output.phi_coherence - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_motor_determinism() {
        let motor = PraxisMotor::new();

        let input = PraxisInput {
            proposed: vec!["a".to_string(), "b".to_string()],
            necessary: vec!["a".to_string()],
            context_vector: vec![0.5, 0.5],
            history_centroid: vec![0.3, 0.7],
        };

        let output1 = motor.evaluate(&input);
        let output2 = motor.evaluate(&input.clone());

        assert!((output1.score - output2.score).abs() < 1e-10);
    }

    #[test]
    fn test_empty_necessary() {
        let motor = PraxisMotor::new();

        let input = PraxisInput {
            proposed: vec!["a".to_string()],
            necessary: vec![],
            context_vector: vec![1.0],
            history_centroid: vec![1.0],
        };

        let output = motor.evaluate(&input);

        assert!((output.phi_completeness - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_score_bounds() {
        let motor = PraxisMotor::new();

        let input = PraxisInput {
            proposed: vec!["a".to_string()],
            necessary: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            context_vector: vec![1.0, 2.0, 3.0],
            history_centroid: vec![-1.0, -2.0, -3.0],
        };

        let output = motor.evaluate(&input);

        assert!(output.score >= 0.0);
        assert!(output.score <= 1.0);
    }
}
