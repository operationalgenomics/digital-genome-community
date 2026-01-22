//! --------------------------
//! INFORMATION
//! --------------------------
//! Title: Chaotic Motor (M_C)
//! Author: Carlos Eduardo Favini
//! Date: 2025-01-02
//! Version: 1.2.0
//! Description: Motor based on chaos theory and dynamical systems.
//! Maps structural possibilities from observed data.
//! Uses trajectory divergence analysis with dual trajectories.
//! Layer: Community
//! Dependencies: Consagrated mathematics (dynamical systems, sensitivity analysis)
//! Affected Components: math/craft
//!
//! --------------------------
//! NOMENCLATURE NOTE (v0.2.0)
//! --------------------------
//! This motor uses "trajectory divergence" terminology instead of "Lyapunov exponent"
//! for epistemological honesty. While the mathematical method is inspired by Lyapunov
//! stability theory, the implementation is a simplified trajectory divergence measure
//! suitable for operational contexts, not a rigorous Lyapunov exponent calculation.
//!
//! --------------------------
//! CHANGE LOG
//! --------------------------
//! 2025-01-02 - Carlos Eduardo Favini - Initial creation
//! 2025-01-02 - Carlos Eduardo Favini - Canonical implementation with dual trajectories
//! 2025-01-02 - Carlos Eduardo Favini - Renamed to trajectory_divergence (v0.2.0)
//! 2025-01-02 - Carlos Eduardo Favini - Added was_clamped signaling (v0.2.0)
//! --------------------------

use serde::{Deserialize, Serialize};

use super::CognitiveMotor;

/// Chaotic Motor (M_C).
///
/// Evaluates structural sensitivity and stability using chaos theory concepts.
/// Works with consagrated mathematical foundations.
///
/// # Function
/// Maps structural possibilities from observed data:
/// - Instability regions
/// - Volatility patterns
/// - Sensitivity to initial conditions (butterfly effect)
///
/// # Restrictions (Constitutional)
/// - Does NOT predict events
/// - Does NOT choose scenarios
/// - Does NOT classify results
/// - Does NOT filter possibilities
///
/// # Output
/// - Maps of structural dependency
/// - Instability regions
/// - Fields of possible variation
/// - Continuous score [0.0, 1.0]
#[derive(Debug, Clone)]
pub struct ChaosMotor;

/// Input for the Chaotic Motor.
///
/// Requires two trajectories: reference and perturbed.
/// The perturbed trajectory starts with a small perturbation (delta_0)
/// from the reference initial condition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChaosInput {
    /// Reference trajectory of observed states.
    /// Each Vec<f64> is a state vector at a time step.
    pub reference_trajectory: Vec<Vec<f64>>,

    /// Perturbed trajectory starting from x_0 + delta_0.
    /// Must have same length and dimensions as reference.
    pub perturbed_trajectory: Vec<Vec<f64>>,

    /// Initial perturbation magnitude.
    /// Must be > 0.
    pub delta_0: f64,

    /// Time step between observations (dt).
    /// Must be > 0.
    pub dt: f64,

    /// Error tolerance for predictability horizon.
    /// If None, derived as 10 * delta_0.
    pub epsilon_tolerance: Option<f64>,
}

/// Validation error for Chaos input.
#[derive(Debug, Clone, PartialEq)]
pub enum ChaosValidationError {
    /// Reference trajectory is empty.
    EmptyReferenceTrajectory,
    /// Perturbed trajectory is empty.
    EmptyPerturbedTrajectory,
    /// Trajectories have different lengths.
    TrajectoryLengthMismatch { reference: usize, perturbed: usize },
    /// State dimensions don't match.
    DimensionMismatch { time_step: usize, reference: usize, perturbed: usize },
    /// delta_0 must be > 0.
    InvalidDelta0,
    /// dt must be > 0.
    InvalidDt,
    /// State dimension is zero.
    ZeroDimensionState { time_step: usize },
}

impl std::fmt::Display for ChaosValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyReferenceTrajectory => {
                write!(f, "Reference trajectory is empty")
            }
            Self::EmptyPerturbedTrajectory => {
                write!(f, "Perturbed trajectory is empty")
            }
            Self::TrajectoryLengthMismatch { reference, perturbed } => {
                write!(f, "Trajectory length mismatch: reference={}, perturbed={}", reference, perturbed)
            }
            Self::DimensionMismatch { time_step, reference, perturbed } => {
                write!(f, "Dimension mismatch at t={}: reference={}, perturbed={}", time_step, reference, perturbed)
            }
            Self::InvalidDelta0 => write!(f, "delta_0 must be > 0"),
            Self::InvalidDt => write!(f, "dt must be > 0"),
            Self::ZeroDimensionState { time_step } => {
                write!(f, "Zero dimension state at t={}", time_step)
            }
        }
    }
}

impl std::error::Error for ChaosValidationError {}

/// Output from the Chaotic Motor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChaosOutput {
    /// Whether the input was valid.
    pub valid: bool,

    /// Validation error if invalid.
    pub validation_error: Option<String>,

    /// Trajectory divergence rate (λ).
    /// λ > 0 indicates divergent/chaotic behavior.
    /// λ < 0 indicates convergent/stable behavior.
    /// λ ≈ 0 indicates marginal stability.
    ///
    /// Note: This is a trajectory divergence measure, not a rigorous
    /// Lyapunov exponent. See module documentation for details.
    pub trajectory_divergence_rate: f64,

    /// Divergence magnitude at each time step.
    pub divergence_history: Vec<f64>,

    /// Predictability horizon (time until error exceeds epsilon).
    /// Infinite for stable systems (λ ≤ 0).
    pub predictability_horizon: f64,

    /// Trajectory variance (measure of volatility).
    pub variance: f64,

    /// Stability factor: ψ_stab = 1 / (1 + max(0, λ))
    pub psi_stability: f64,

    /// Volatility factor: ψ_vol = 1 / (1 + σ)
    pub psi_volatility: f64,

    /// Final motor score: M_C = ψ_stab × ψ_vol
    pub score: f64,

    /// Whether the final score was clamped to [0, 1].
    /// If true, `unclamped_score` contains the original value.
    pub was_clamped: bool,

    /// Original score before clamping (if was_clamped is true).
    pub unclamped_score: Option<f64>,
}

impl ChaosOutput {
    /// Creates an invalid output with zero score.
    fn invalid(error: ChaosValidationError) -> Self {
        Self {
            valid: false,
            validation_error: Some(error.to_string()),
            trajectory_divergence_rate: 0.0,
            divergence_history: Vec::new(),
            predictability_horizon: 0.0,
            variance: 0.0,
            psi_stability: 0.0,
            psi_volatility: 0.0,
            score: 0.0,
            was_clamped: false,
            unclamped_score: None,
        }
    }
}

impl ChaosMotor {
    /// Creates a new Chaotic Motor.
    pub fn new() -> Self {
        Self
    }

    /// Validates input dimensions and constraints.
    fn validate(input: &ChaosInput) -> Result<(), ChaosValidationError> {
        if input.reference_trajectory.is_empty() {
            return Err(ChaosValidationError::EmptyReferenceTrajectory);
        }

        if input.perturbed_trajectory.is_empty() {
            return Err(ChaosValidationError::EmptyPerturbedTrajectory);
        }

        if input.reference_trajectory.len() != input.perturbed_trajectory.len() {
            return Err(ChaosValidationError::TrajectoryLengthMismatch {
                reference: input.reference_trajectory.len(),
                perturbed: input.perturbed_trajectory.len(),
            });
        }

        if input.delta_0 <= 0.0 || !input.delta_0.is_finite() {
            return Err(ChaosValidationError::InvalidDelta0);
        }

        if input.dt <= 0.0 || !input.dt.is_finite() {
            return Err(ChaosValidationError::InvalidDt);
        }

        // Validate dimensions at each time step
        for (t, (ref_state, pert_state)) in input
            .reference_trajectory
            .iter()
            .zip(input.perturbed_trajectory.iter())
            .enumerate()
        {
            if ref_state.is_empty() {
                return Err(ChaosValidationError::ZeroDimensionState { time_step: t });
            }

            if ref_state.len() != pert_state.len() {
                return Err(ChaosValidationError::DimensionMismatch {
                    time_step: t,
                    reference: ref_state.len(),
                    perturbed: pert_state.len(),
                });
            }
        }

        Ok(())
    }

    /// Calculates Euclidean distance between two state vectors.
    fn euclidean_distance(a: &[f64], b: &[f64]) -> f64 {
        a.iter()
            .zip(b.iter())
            .map(|(x, y)| (x - y).powi(2))
            .sum::<f64>()
            .sqrt()
    }

    /// Estimates the trajectory divergence rate.
    ///
    /// # Method
    /// 1. Track separation between reference and perturbed trajectories
    /// 2. Calculate local divergence rates at each step
    /// 3. Average over the trajectory
    ///
    /// # Formula
    /// λ = (1/T) × Σ ln(d_i / d_{i-1})
    ///
    /// where d_i is the separation at time step i.
    ///
    /// # Note
    /// This is a trajectory divergence measure inspired by Lyapunov stability
    /// theory, but simplified for operational contexts. It measures how quickly
    /// nearby trajectories separate, without the full mathematical rigor of
    /// Lyapunov exponent calculation (which requires infinite-time limits and
    /// renormalization).
    fn estimate_trajectory_divergence(
        reference: &[Vec<f64>],
        perturbed: &[Vec<f64>],
        delta_0: f64,
        dt: f64,
    ) -> (f64, Vec<f64>) {
        let n = reference.len();
        if n < 2 {
            return (0.0, vec![delta_0]);
        }

        // Calculate divergence at each time step
        let mut divergences = Vec::with_capacity(n);
        for (ref_state, pert_state) in reference.iter().zip(perturbed.iter()) {
            let d = Self::euclidean_distance(ref_state, pert_state);
            divergences.push(if d > 0.0 { d } else { delta_0 });
        }

        // Calculate local divergence rates and average
        let mut lambda_sum = 0.0;
        let mut count = 0;

        for i in 1..n {
            let d_prev = divergences[i - 1];
            let d_curr = divergences[i];

            if d_prev > 0.0 && d_curr > 0.0 {
                // Local divergence rate: λ_i = (1/dt) × ln(d_curr / d_prev)
                let lambda_i = (d_curr / d_prev).ln() / dt;
                
                // Filter out infinite values from numerical issues
                if lambda_i.is_finite() {
                    lambda_sum += lambda_i;
                    count += 1;
                }
            }
        }

        let lambda = if count > 0 {
            lambda_sum / (count as f64)
        } else {
            0.0
        };

        (lambda, divergences)
    }

    /// Calculates predictability horizon.
    ///
    /// # Formula
    /// t_h = (1/λ) × ln(ε / δ_0)
    ///
    /// where ε is the error tolerance.
    /// If λ ≤ 0, system is stable and horizon is infinite.
    fn calculate_predictability_horizon(
        divergence_rate: f64,
        delta_0: f64,
        epsilon: f64,
    ) -> f64 {
        if divergence_rate <= 0.0 {
            return f64::INFINITY; // Stable system - infinite predictability
        }

        if delta_0 <= 0.0 || epsilon <= 0.0 {
            return 0.0;
        }

        if epsilon <= delta_0 {
            return 0.0; // Already exceeded tolerance
        }

        (1.0 / divergence_rate) * (epsilon / delta_0).ln()
    }

    /// Calculates trajectory variance (volatility measure).
    fn calculate_variance(trajectory: &[Vec<f64>]) -> f64 {
        if trajectory.is_empty() {
            return 0.0;
        }

        let n = trajectory.len() as f64;
        let dim = trajectory[0].len();

        if dim == 0 {
            return 0.0;
        }

        // Calculate mean for each dimension
        let mut means = vec![0.0; dim];
        for state in trajectory {
            for (i, &val) in state.iter().enumerate() {
                means[i] += val / n;
            }
        }

        // Calculate variance
        let mut variance = 0.0;
        for state in trajectory {
            for (i, &val) in state.iter().enumerate() {
                variance += (val - means[i]).powi(2);
            }
        }

        variance / (n * dim as f64)
    }

    /// Calculates stability factor from divergence rate.
    ///
    /// ψ_stab = 1 / (1 + max(0, λ))
    ///
    /// Interpretation:
    /// - λ < 0: stable system, ψ_stab ≈ 1
    /// - λ = 0: marginally stable, ψ_stab = 1
    /// - λ > 0: divergent/chaotic, ψ_stab < 1
    fn calculate_psi_stability(divergence_rate: f64) -> f64 {
        1.0 / (1.0 + divergence_rate.max(0.0))
    }

    /// Calculates volatility factor from variance.
    ///
    /// ψ_vol = 1 / (1 + σ)
    ///
    /// Lower variance = higher score (more predictable).
    fn calculate_psi_volatility(variance: f64) -> f64 {
        1.0 / (1.0 + variance.max(0.0))
    }
}

impl Default for ChaosMotor {
    fn default() -> Self {
        Self::new()
    }
}

impl CognitiveMotor for ChaosMotor {
    type Input = ChaosInput;
    type Output = ChaosOutput;

    /// Evaluates the trajectory pair and produces M_C score.
    ///
    /// Formula: M_C = ψ_stab × ψ_vol
    fn evaluate(&self, input: &Self::Input) -> Self::Output {
        // Validate input
        if let Err(e) = Self::validate(input) {
            return ChaosOutput::invalid(e);
        }

        // Derive epsilon from delta_0 if not provided
        let epsilon = input.epsilon_tolerance.unwrap_or(10.0 * input.delta_0);

        // Calculate trajectory divergence rate
        let (trajectory_divergence_rate, divergence_history) = Self::estimate_trajectory_divergence(
            &input.reference_trajectory,
            &input.perturbed_trajectory,
            input.delta_0,
            input.dt,
        );

        // Calculate predictability horizon
        let predictability_horizon = Self::calculate_predictability_horizon(
            trajectory_divergence_rate,
            input.delta_0,
            epsilon,
        );

        // Calculate variance of reference trajectory
        let variance = Self::calculate_variance(&input.reference_trajectory);

        // Calculate factors
        let psi_stability = Self::calculate_psi_stability(trajectory_divergence_rate);
        let psi_volatility = Self::calculate_psi_volatility(variance);

        // Calculate raw score
        let raw_score = psi_stability * psi_volatility;

        // Check if clamping is needed
        let needs_clamping = raw_score < 0.0 || raw_score > 1.0 || !raw_score.is_finite();
        let final_score = if needs_clamping {
            if raw_score.is_nan() { 0.0 } else { raw_score.clamp(0.0, 1.0) }
        } else {
            raw_score
        };

        ChaosOutput {
            valid: true,
            validation_error: None,
            trajectory_divergence_rate,
            divergence_history,
            predictability_horizon,
            variance,
            psi_stability,
            psi_volatility,
            score: final_score,
            was_clamped: needs_clamping,
            unclamped_score: if needs_clamping { Some(raw_score) } else { None },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Creates an exponentially diverging trajectory pair.
    fn exponential_divergence(n: usize, lambda: f64, delta_0: f64, dt: f64) -> (Vec<Vec<f64>>, Vec<Vec<f64>>) {
        let reference: Vec<Vec<f64>> = (0..n).map(|i| vec![i as f64]).collect();
        let perturbed: Vec<Vec<f64>> = (0..n)
            .map(|i| {
                let t = (i as f64) * dt;
                vec![(i as f64) + delta_0 * (lambda * t).exp()]
            })
            .collect();
        (reference, perturbed)
    }

    #[test]
    fn test_validation_empty_trajectory() {
        let motor = ChaosMotor::new();

        let input = ChaosInput {
            reference_trajectory: vec![],
            perturbed_trajectory: vec![vec![1.0]],
            delta_0: 0.1,
            dt: 1.0,
            epsilon_tolerance: None,
        };

        let output = motor.evaluate(&input);
        assert!(!output.valid);
    }

    #[test]
    fn test_validation_invalid_delta() {
        let motor = ChaosMotor::new();

        let input = ChaosInput {
            reference_trajectory: vec![vec![1.0]],
            perturbed_trajectory: vec![vec![1.0]],
            delta_0: 0.0, // Invalid
            dt: 1.0,
            epsilon_tolerance: None,
        };

        let output = motor.evaluate(&input);
        assert!(!output.valid);
    }

    #[test]
    fn test_stable_system() {
        let motor = ChaosMotor::new();

        // Parallel trajectories (no divergence)
        let input = ChaosInput {
            reference_trajectory: vec![
                vec![0.0], vec![1.0], vec![2.0], vec![3.0],
            ],
            perturbed_trajectory: vec![
                vec![0.1], vec![1.1], vec![2.1], vec![3.1],
            ],
            delta_0: 0.1,
            dt: 1.0,
            epsilon_tolerance: None,
        };

        let output = motor.evaluate(&input);

        assert!(output.valid);
        // Parallel trajectories should have λ ≈ 0 (stable)
        assert!(output.trajectory_divergence_rate.abs() < 0.01);
        assert!(output.psi_stability > 0.99);
        assert!(!output.was_clamped);
    }

    #[test]
    fn test_divergent_system() {
        let motor = ChaosMotor::new();

        // Exponentially diverging trajectories
        let (reference, perturbed) = exponential_divergence(10, 0.5, 0.1, 1.0);

        let input = ChaosInput {
            reference_trajectory: reference,
            perturbed_trajectory: perturbed,
            delta_0: 0.1,
            dt: 1.0,
            epsilon_tolerance: None,
        };

        let output = motor.evaluate(&input);

        assert!(output.valid);
        // Should detect positive divergence rate
        assert!(output.trajectory_divergence_rate > 0.0);
        // Lower stability factor for divergent system
        assert!(output.psi_stability < 1.0);
    }

    #[test]
    fn test_constant_system() {
        let motor = ChaosMotor::new();

        // Constant trajectories
        let input = ChaosInput {
            reference_trajectory: vec![
                vec![1.0, 1.0], vec![1.0, 1.0], vec![1.0, 1.0],
            ],
            perturbed_trajectory: vec![
                vec![1.1, 1.0], vec![1.1, 1.0], vec![1.1, 1.0],
            ],
            delta_0: 0.1,
            dt: 1.0,
            epsilon_tolerance: None,
        };

        let output = motor.evaluate(&input);

        assert!(output.valid);
        // Zero variance for constant trajectory
        assert!((output.variance - 0.0).abs() < 1e-10);
        // Maximum volatility factor
        assert!((output.psi_volatility - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_motor_determinism() {
        let motor = ChaosMotor::new();

        let input = ChaosInput {
            reference_trajectory: vec![
                vec![0.0, 0.0], vec![1.0, 1.0], vec![2.0, 2.0],
            ],
            perturbed_trajectory: vec![
                vec![0.1, 0.1], vec![1.2, 1.2], vec![2.4, 2.4],
            ],
            delta_0: 0.1,
            dt: 1.0,
            epsilon_tolerance: Some(1.0),
        };

        let output1 = motor.evaluate(&input);
        let output2 = motor.evaluate(&input.clone());

        assert!((output1.score - output2.score).abs() < 1e-10);
        assert!((output1.trajectory_divergence_rate - output2.trajectory_divergence_rate).abs() < 1e-10);
    }

    #[test]
    fn test_high_variance_low_score() {
        let motor = ChaosMotor::new();

        // High variance trajectory
        let input = ChaosInput {
            reference_trajectory: vec![
                vec![0.0], vec![100.0], vec![-50.0], vec![200.0],
            ],
            perturbed_trajectory: vec![
                vec![0.1], vec![100.1], vec![-49.9], vec![200.1],
            ],
            delta_0: 0.1,
            dt: 1.0,
            epsilon_tolerance: None,
        };

        let output = motor.evaluate(&input);

        assert!(output.valid);
        // High variance -> low volatility factor
        assert!(output.psi_volatility < 0.1);
    }

    #[test]
    fn test_was_clamped_signaling() {
        let motor = ChaosMotor::new();

        // Normal case - should not be clamped
        let input = ChaosInput {
            reference_trajectory: vec![vec![0.0], vec![1.0]],
            perturbed_trajectory: vec![vec![0.1], vec![1.1]],
            delta_0: 0.1,
            dt: 1.0,
            epsilon_tolerance: None,
        };

        let output = motor.evaluate(&input);
        assert!(output.valid);
        // Normal values should not be clamped
        assert!(!output.was_clamped);
        assert!(output.unclamped_score.is_none());
    }

    #[test]
    fn test_score_bounds() {
        let motor = ChaosMotor::new();

        let input = ChaosInput {
            reference_trajectory: vec![vec![0.0], vec![1.0], vec![2.0]],
            perturbed_trajectory: vec![vec![0.1], vec![1.2], vec![2.5]],
            delta_0: 0.1,
            dt: 1.0,
            epsilon_tolerance: None,
        };

        let output = motor.evaluate(&input);

        assert!(output.valid);
        assert!(output.score >= 0.0);
        assert!(output.score <= 1.0);
    }
}
