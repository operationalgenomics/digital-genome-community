//! --------------------------
//! INFORMATION
//! --------------------------
//! Title: Perceptual Maturation (Insight A.5)
//! Author: Carlos Eduardo Favini
//! Date: 2025-01-02
//! Version: 1.5.0
//! Description: Implements perceptual maturation - multiple internal passes
//! during a single perceptual cycle, allowing the system to
//! "mature" its understanding before emitting output.
//!
//! Critical constraints from ALERT-012:
//! - Maturation is NOT learning (no persistent changes)
//! - Maturation is NOT memory (no recall of previous inputs)
//! - Maturation is NOT historical adaptation (no evolution across cycles)
//!
//! Maturation IS:
//! - Confined to the perceptual cycle
//! - Discarded entirely at the end
//! - Traceable via replay (for auditing)
//!
//! Analogy: "The baby matures in milliseconds - computational time."
//! Between perceive call and return, the system can refine
//! its perception through multiple passes, but forgets everything
//! once the function returns.
//!
//! Layer: Community
//! Dependencies: sensory/*
//! Affected Components: SensoryCortex
//!
//! --------------------------
//! Foundational Axiom B.1
//! --------------------------
//! "The Core has basal operational existence that is semantically null,
//! and its cognition is event-driven (activated by input, ended by output)."
//!
//! Maturation happens DURING the event. It is part of the event-driven
//! cognition, not a separate process.
//!
//! --------------------------
//! CHANGE LOG
//! --------------------------
//! 2025-01-02 - Carlos Eduardo Favini - Initial creation (v1.5.0)
//! --------------------------

use std::time::Instant;

// =============================================================================
// MATURATION CONFIG
// =============================================================================

/// Configuration for perceptual maturation.
///
/// Controls how many refinement passes the system performs during a single
/// perceptual cycle and when to stop refining.
///
/// # Design Principles
///
/// 1. **Stateless**: Config is immutable, passed in, not stored
/// 2. **Bounded**: Maximum iterations prevent infinite loops
/// 3. **Convergent**: Threshold determines when refinement stops
/// 4. **Auditable**: All parameters are explicit and traceable
///
/// # What This Is
///
/// - A configuration struct passed to `perceive_mature()`
/// - Defines limits and thresholds for the maturation process
/// - Does NOT store state between calls
///
/// # What This Is NOT
///
/// - NOT a learning rate or training parameter
/// - NOT a memory configuration
/// - NOT a persistent setting that affects future calls
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MaturationConfig {
    /// Maximum number of refinement iterations.
    ///
    /// Guarantees termination. After this many passes, maturation stops
    /// regardless of convergence.
    ///
    /// Default: 5 iterations
    pub max_iterations: usize,

    /// Convergence threshold.
    ///
    /// If the delta between consecutive iterations falls below this value,
    /// maturation is considered converged and stops early.
    ///
    /// Range: (0.0, 1.0)
    /// Default: 0.01 (1% change)
    pub convergence_threshold: f64,

    /// Timeout per iteration in nanoseconds.
    ///
    /// If a single iteration exceeds this time, maturation stops.
    /// Value 0 means no timeout.
    ///
    /// Default: 0 (no timeout)
    pub iteration_timeout_ns: u64,

    /// Minimum iterations before checking convergence.
    ///
    /// Ensures at least N passes happen before early stopping.
    ///
    /// Default: 2 iterations
    pub min_iterations: usize,
}

impl Default for MaturationConfig {
    fn default() -> Self {
        Self {
            max_iterations: 5,
            convergence_threshold: 0.01,
            iteration_timeout_ns: 0,
            min_iterations: 2,
        }
    }
}

impl MaturationConfig {
    /// Creates a new config with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a config for single-pass perception (no maturation).
    pub fn single_pass() -> Self {
        Self {
            max_iterations: 1,
            convergence_threshold: 0.0,
            iteration_timeout_ns: 0,
            min_iterations: 1,
        }
    }

    /// Creates a config for deep maturation (more passes).
    pub fn deep() -> Self {
        Self {
            max_iterations: 10,
            convergence_threshold: 0.001,
            iteration_timeout_ns: 0,
            min_iterations: 3,
        }
    }

    /// Builder: sets maximum iterations.
    pub fn with_max_iterations(mut self, n: usize) -> Self {
        self.max_iterations = n.max(1);
        self
    }

    /// Builder: sets convergence threshold.
    pub fn with_convergence_threshold(mut self, threshold: f64) -> Self {
        self.convergence_threshold = threshold.clamp(0.0, 1.0);
        self
    }

    /// Builder: sets iteration timeout.
    pub fn with_iteration_timeout_ns(mut self, timeout: u64) -> Self {
        self.iteration_timeout_ns = timeout;
        self
    }

    /// Builder: sets minimum iterations.
    pub fn with_min_iterations(mut self, n: usize) -> Self {
        self.min_iterations = n.max(1);
        self
    }
}

// =============================================================================
// MATURATION STATE
// =============================================================================

/// State of the maturation process.
///
/// This struct captures what happened during maturation. It is RETURNED
/// from the perception function, NOT stored anywhere.
///
/// # Lifecycle
///
/// 1. Created when `perceive_mature()` starts
/// 2. Updated during each refinement iteration
/// 3. Returned as part of the output
/// 4. DISCARDED by the caller (or persisted externally)
///
/// The Community Edition does NOT persist this. It exists only during
/// the perceptual cycle and is part of the output.
///
/// # What This Is
///
/// - A record of what happened during maturation
/// - Output data, not internal state
/// - Useful for auditing and replay verification
///
/// # What This Is NOT
///
/// - NOT persistent state
/// - NOT memory between calls
/// - NOT input for future perceptions
#[derive(Debug, Clone, PartialEq)]
pub struct MaturationState {
    /// Number of iterations performed.
    pub iterations_performed: usize,

    /// Whether convergence was achieved.
    pub converged: bool,

    /// Final delta value (difference from last iteration).
    pub final_delta: f64,

    /// Reason for stopping.
    pub stop_reason: StopReason,

    /// History of deltas per iteration.
    ///
    /// Useful for debugging and analysis.
    /// First element is always 1.0 (maximum possible change).
    pub delta_history: Vec<f64>,

    /// Total time spent in maturation (nanoseconds).
    pub total_time_ns: u64,

    /// Time per iteration (nanoseconds).
    pub iteration_times_ns: Vec<u64>,
}

impl MaturationState {
    /// Creates a new maturation state for tracking.
    pub(crate) fn new() -> Self {
        Self {
            iterations_performed: 0,
            converged: false,
            final_delta: 1.0,
            stop_reason: StopReason::NotStarted,
            delta_history: Vec::new(),
            total_time_ns: 0,
            iteration_times_ns: Vec::new(),
        }
    }

    /// Records the completion of an iteration.
    pub(crate) fn record_iteration(&mut self, delta: f64, time_ns: u64) {
        self.iterations_performed += 1;
        self.final_delta = delta;
        self.delta_history.push(delta);
        self.iteration_times_ns.push(time_ns);
        self.total_time_ns += time_ns;
    }

    /// Marks maturation as converged.
    pub(crate) fn mark_converged(&mut self) {
        self.converged = true;
        self.stop_reason = StopReason::Converged;
    }

    /// Marks maturation as stopped due to max iterations.
    pub(crate) fn mark_max_iterations(&mut self) {
        self.stop_reason = StopReason::MaxIterations;
    }

    /// Marks maturation as stopped due to timeout.
    pub(crate) fn mark_timeout(&mut self) {
        self.stop_reason = StopReason::Timeout;
    }

    /// Returns true if maturation achieved convergence.
    pub fn is_converged(&self) -> bool {
        self.converged
    }

    /// Returns the convergence ratio (how close to threshold).
    ///
    /// Returns 1.0 if converged, <1.0 if not converged.
    pub fn convergence_ratio(&self, threshold: f64) -> f64 {
        if threshold <= 0.0 {
            return 0.0;
        }
        (threshold / self.final_delta.max(f64::EPSILON)).min(1.0)
    }
}

impl Default for MaturationState {
    fn default() -> Self {
        Self::new()
    }
}

/// Reason why maturation stopped.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StopReason {
    /// Maturation has not started.
    NotStarted,

    /// Convergence threshold was reached.
    Converged,

    /// Maximum iterations were exhausted.
    MaxIterations,

    /// Iteration timeout was exceeded.
    Timeout,

    /// Input was empty (nothing to mature).
    EmptyInput,
}

impl std::fmt::Display for StopReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StopReason::NotStarted => write!(f, "Not started"),
            StopReason::Converged => write!(f, "Converged"),
            StopReason::MaxIterations => write!(f, "Max iterations reached"),
            StopReason::Timeout => write!(f, "Timeout"),
            StopReason::EmptyInput => write!(f, "Empty input"),
        }
    }
}

// =============================================================================
// REFINEMENT STEP
// =============================================================================

/// A single refinement step during maturation.
///
/// Records what changed during one iteration of refinement.
/// Used for detailed auditing and replay verification.
#[derive(Debug, Clone, PartialEq)]
pub struct RefinementStep {
    /// Iteration number (1-indexed).
    pub iteration: usize,

    /// Delta from previous iteration.
    pub delta: f64,

    /// Time taken for this step (nanoseconds).
    pub time_ns: u64,

    /// Key metrics after this step.
    pub metrics: RefinementMetrics,
}

/// Metrics captured at each refinement step.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RefinementMetrics {
    /// Entropy after refinement.
    pub entropy: f64,

    /// Structure score after refinement.
    pub structure: f64,

    /// Pattern strength after refinement.
    pub pattern: f64,

    /// Proto-agency indicator after refinement.
    pub proto_agency: bool,
}

impl RefinementMetrics {
    /// Creates new metrics from values.
    pub fn new(entropy: f64, structure: f64, pattern: f64, proto_agency: bool) -> Self {
        Self {
            entropy,
            structure,
            pattern,
            proto_agency,
        }
    }

    /// Calculates delta (change) from another metrics instance.
    ///
    /// Returns a value in [0.0, 1.0] representing how much changed.
    pub fn delta_from(&self, other: &RefinementMetrics) -> f64 {
        let entropy_delta = (self.entropy - other.entropy).abs();
        let structure_delta = (self.structure - other.structure).abs();
        let pattern_delta = (self.pattern - other.pattern).abs();
        let agency_delta = if self.proto_agency != other.proto_agency { 1.0 } else { 0.0 };

        // Weighted average of deltas
        // Agency change is binary, so it has high weight
        (entropy_delta + structure_delta + pattern_delta + agency_delta * 0.5) / 3.5
    }
}

impl Default for RefinementMetrics {
    fn default() -> Self {
        Self {
            entropy: 0.0,
            structure: 0.0,
            pattern: 0.0,
            proto_agency: false,
        }
    }
}

// =============================================================================
// MATURATION ENGINE
// =============================================================================

/// Internal engine that performs maturation.
///
/// This is a stateless helper that takes input and config, performs
/// iterative refinement, and returns the result. It does NOT store
/// anything between calls.
///
/// # Lifecycle
///
/// 1. Created fresh for each `perceive_mature()` call
/// 2. Performs iterations
/// 3. Returns results
/// 4. Is dropped (no persistence)
///
/// NOTE: Reserved for future refinement strategies. Currently not used
/// as perceive_mature implements inline logic.
#[allow(dead_code)]
pub(crate) struct MaturationEngine {
    config: MaturationConfig,
    start_time: Instant,
}

#[allow(dead_code)]
impl MaturationEngine {
    /// Creates a new engine with the given config.
    pub fn new(config: MaturationConfig) -> Self {
        Self {
            config,
            start_time: Instant::now(),
        }
    }

    /// Checks if iteration timeout has been exceeded.
    pub fn iteration_timeout_exceeded(&self, iteration_start: Instant) -> bool {
        if self.config.iteration_timeout_ns == 0 {
            return false;
        }
        iteration_start.elapsed().as_nanos() as u64 >= self.config.iteration_timeout_ns
    }

    /// Checks if convergence is achieved.
    pub fn is_converged(&self, delta: f64, iteration: usize) -> bool {
        if iteration < self.config.min_iterations {
            return false;
        }
        delta <= self.config.convergence_threshold
    }

    /// Checks if max iterations reached.
    pub fn max_iterations_reached(&self, iteration: usize) -> bool {
        iteration >= self.config.max_iterations
    }

    /// Returns elapsed time since engine creation.
    pub fn elapsed_ns(&self) -> u64 {
        self.start_time.elapsed().as_nanos() as u64
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = MaturationConfig::default();
        assert_eq!(config.max_iterations, 5);
        assert_eq!(config.convergence_threshold, 0.01);
        assert_eq!(config.min_iterations, 2);
    }

    #[test]
    fn test_single_pass_config() {
        let config = MaturationConfig::single_pass();
        assert_eq!(config.max_iterations, 1);
        assert_eq!(config.min_iterations, 1);
    }

    #[test]
    fn test_deep_config() {
        let config = MaturationConfig::deep();
        assert_eq!(config.max_iterations, 10);
        assert_eq!(config.convergence_threshold, 0.001);
    }

    #[test]
    fn test_builder_pattern() {
        let config = MaturationConfig::new()
            .with_max_iterations(7)
            .with_convergence_threshold(0.05)
            .with_min_iterations(3);

        assert_eq!(config.max_iterations, 7);
        assert_eq!(config.convergence_threshold, 0.05);
        assert_eq!(config.min_iterations, 3);
    }

    #[test]
    fn test_maturation_state_new() {
        let state = MaturationState::new();
        assert_eq!(state.iterations_performed, 0);
        assert!(!state.converged);
        assert_eq!(state.stop_reason, StopReason::NotStarted);
    }

    #[test]
    fn test_maturation_state_record() {
        let mut state = MaturationState::new();
        state.record_iteration(0.5, 1000);
        state.record_iteration(0.2, 800);

        assert_eq!(state.iterations_performed, 2);
        assert_eq!(state.final_delta, 0.2);
        assert_eq!(state.delta_history, vec![0.5, 0.2]);
        assert_eq!(state.total_time_ns, 1800);
    }

    #[test]
    fn test_maturation_state_converged() {
        let mut state = MaturationState::new();
        state.record_iteration(0.5, 1000);
        state.mark_converged();

        assert!(state.is_converged());
        assert_eq!(state.stop_reason, StopReason::Converged);
    }

    #[test]
    fn test_refinement_metrics_delta() {
        let m1 = RefinementMetrics::new(0.5, 0.3, 0.7, false);
        let m2 = RefinementMetrics::new(0.6, 0.4, 0.8, false);

        let delta = m1.delta_from(&m2);
        assert!(delta > 0.0);
        assert!(delta < 1.0);
    }

    #[test]
    fn test_refinement_metrics_delta_with_agency_change() {
        let m1 = RefinementMetrics::new(0.5, 0.3, 0.7, false);
        let m2 = RefinementMetrics::new(0.5, 0.3, 0.7, true);

        // Agency change should increase delta
        let delta = m1.delta_from(&m2);
        assert!(delta > 0.0);
    }

    #[test]
    fn test_refinement_metrics_delta_identical() {
        let m1 = RefinementMetrics::new(0.5, 0.3, 0.7, false);
        let m2 = RefinementMetrics::new(0.5, 0.3, 0.7, false);

        let delta = m1.delta_from(&m2);
        assert_eq!(delta, 0.0);
    }

    #[test]
    fn test_engine_convergence_check() {
        let config = MaturationConfig::new()
            .with_convergence_threshold(0.05)
            .with_min_iterations(2);

        let engine = MaturationEngine::new(config);

        // Below min iterations - not converged
        assert!(!engine.is_converged(0.01, 1));

        // At min iterations with low delta - converged
        assert!(engine.is_converged(0.01, 2));

        // At min iterations with high delta - not converged
        assert!(!engine.is_converged(0.10, 2));
    }

    #[test]
    fn test_engine_max_iterations() {
        let config = MaturationConfig::new().with_max_iterations(5);
        let engine = MaturationEngine::new(config);

        assert!(!engine.max_iterations_reached(4));
        assert!(engine.max_iterations_reached(5));
        assert!(engine.max_iterations_reached(6));
    }

    // =========================================================================
    // AXIOM VERIFICATION TESTS
    // =========================================================================

    #[test]
    fn test_maturation_state_is_output_not_storage() {
        // MaturationState should be cloneable and returnable
        // It's OUTPUT data, not persistent state

        let mut state = MaturationState::new();
        state.record_iteration(0.5, 1000);

        // Can be cloned and passed around
        let cloned = state.clone();
        assert_eq!(cloned.iterations_performed, 1);

        // Original is unaffected
        assert_eq!(state.iterations_performed, 1);
    }

    #[test]
    fn test_config_is_immutable_input() {
        // Config should be immutable once created
        // It's INPUT data, not mutable state

        let config = MaturationConfig::default();

        // Config is Copy - each use gets a fresh copy
        let config2 = config;
        assert_eq!(config.max_iterations, config2.max_iterations);
    }

    #[test]
    fn test_no_persistent_state_in_engine() {
        // Engine is created fresh each time
        // No state carries over between uses

        let config = MaturationConfig::default();

        let engine1 = MaturationEngine::new(config);
        let _elapsed1 = engine1.elapsed_ns();

        // Second engine is completely independent
        let engine2 = MaturationEngine::new(config);
        let elapsed2 = engine2.elapsed_ns();

        // New engine starts fresh
        assert!(elapsed2 < 1_000_000); // Less than 1ms since creation
    }
}
