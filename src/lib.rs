//! --------------------------
//! INFORMATION
//! --------------------------
//! Title: Digital Genome Community Edition - Core Library
//! Author: Carlos Eduardo Favini
//! Date: 2025-01-02
//! Version: 0.1.0-rc1 (Adão Sintético)
//! Description: Root module for the Digital Genome Community Edition.
//! This is a synthetic cognitive core that perceives, observes,
//! comprehends, and emits DNA with Score. It does not act.
//! Thread-safe: All public types are Send + Sync.
//! Epistemologically neutral: No domain knowledge, no ontologies.
//!
//! Foundational Axiom B.1: "The Core has basal operational existence
//! that is semantically null, and its cognition is event-driven
//! (activated by input, ended by output)."
//!
//! Layer: Community
//! Dependencies: All internal modules
//! Affected Components: External consumers of the library
//!
//! --------------------------
//! CHANGE LOG
//! --------------------------
//! 2025-01-02 - Carlos Eduardo Favini - Initial creation (Marco Zero)
//! 2025-01-02 - Carlos Eduardo Favini - Added replay module (v0.3.0)
//! 2025-01-02 - Carlos Eduardo Favini - Added sensory cortex (v1.1.0)
//! 2025-01-02 - Carlos Eduardo Favini - Cognitive Depth (v1.2.0)
//! 2025-01-02 - Carlos Eduardo Favini - Threading & Epistemological Neutrality (v1.3.0)
//! 2025-01-02 - Carlos Eduardo Favini - Computational Self-Preservation (v1.4.0)
//! 2025-01-02 - Carlos Eduardo Favini - Perceptual Maturation (v1.5.0)
//! --------------------------

// =============================================================================
// CORE MODULES - COMMUNITY EDITION
// =============================================================================

/// Core type definitions (identifiers, base types)
pub mod core_types;

/// Biological hierarchy (Action, DNA, Synapse, Neuron, Brain, Truth)
pub mod hierarchy;

/// Cognitive motors (Praxeological, Nash, Chaotic, Meristic)
pub mod motors;

/// Mathematical foundations (Craft Performance formula)
pub mod math;

/// Latent Archive (Foucaultian memory)
pub mod archive;

/// Selection criteria (Golden Index)
pub mod selection;

/// Topological structures (synaptic connections)
pub mod topology;

/// Traits for Enterprise integration
pub mod traits;

/// Replay harness for deterministic execution and verification (v0.3.0)
pub mod replay;

/// Sensory cortex for abstraction hierarchy (v1.1.0)
pub mod sensory;

// =============================================================================
// COGNITIVE DEPTH MODULES - v1.2.0
// =============================================================================

/// Inference by correlation (Insight #3)
pub mod correlation;

/// Motor competition and cooperation (Insight #6)
pub mod competition;

/// Cognitive observability / metacognition (Insight #7)
pub mod observability;

/// Cognitive completeness states (Insight #10)
pub mod completeness;

// =============================================================================
// COMPUTATIONAL SELF-PRESERVATION - v1.4.0
// =============================================================================

/// Computational budget and integrity verification (Insight A.7)
///
/// This module implements self-preservation instincts based EXCLUSIVELY
/// on computational constraints (memory, time, complexity, numerical stability).
///
/// FORBIDDEN: Any limit based on human senses or biological analogies.
/// ALLOWED: Only limits justified by computational facts.
pub mod budget;

// =============================================================================
// PERCEPTUAL MATURATION - v1.5.0
// =============================================================================

/// Perceptual maturation (Insight A.5)
///
/// Implements multiple internal refinement passes during a single perceptual
/// cycle, allowing the system to "mature" before emitting output.
///
/// CRITICAL CONSTRAINTS:
/// - Maturation is NOT learning (no persistent changes)
/// - Maturation is NOT memory (no recall of previous inputs)
/// - Maturation IS confined to the perceptual cycle
/// - Maturation IS discarded entirely at the end
pub mod maturation;

// =============================================================================
// RE-EXPORTS
// =============================================================================

pub use core_types::*;
pub use math::craft::CraftPerformance;
pub use replay::{ReplayContext, ReplaySession, ReplayEvent, ReplayVerifier};
pub use sensory::{
    CommunityOutput, CortexOutput, MatureOutput, 
    RawInput, SensoryCortex, SensorySignals,
    PerceptualState, StateHistory, StateTransition,
};

// v1.2.0 exports
pub use correlation::{CorrelationMatrix, CooccurrenceTracker, TransformationTracker};
pub use competition::{MotorCompetition, MotorCooperation, MotorDynamics, MotorType};
pub use observability::{CognitiveObservability, HealthIndicators, ProgressTracker};
pub use completeness::{CognitiveCompleteness, AbstractionLevel, ConflictType, MissingSignal};

// v1.4.0 exports
pub use budget::{
    ComputationalBudget, IntegrityCheck, NumericalIssue,
    ComplexityClass, BudgetGuard,
    check_bytes_budget, check_numerical_stability, check_time_budget,
};

// v1.5.0 exports
pub use maturation::{
    MaturationConfig, MaturationState, StopReason,
    RefinementStep, RefinementMetrics,
};

// =============================================================================
// THREAD-SAFETY VERIFICATION
// =============================================================================
// These tests verify at compile-time that all public types are Send + Sync.
// If any type fails to be Send + Sync, the build will fail.
// See THREADING.md for the complete threading policy.

#[cfg(test)]
mod thread_safety_tests {
    use super::*;

    fn assert_send_sync<T: Send + Sync>() {}

    #[test]
    fn test_sensory_types_are_send_sync() {
        assert_send_sync::<SensoryCortex>();
        assert_send_sync::<RawInput>();
        assert_send_sync::<SensorySignals>();
        assert_send_sync::<CommunityOutput>();
    }

    #[test]
    fn test_correlation_types_are_send_sync() {
        assert_send_sync::<CorrelationMatrix>();
        assert_send_sync::<CooccurrenceTracker>();
        assert_send_sync::<TransformationTracker>();
    }

    #[test]
    fn test_competition_types_are_send_sync() {
        assert_send_sync::<MotorCompetition>();
        assert_send_sync::<MotorCooperation>();
        assert_send_sync::<MotorDynamics>();
        assert_send_sync::<MotorType>();
    }

    #[test]
    fn test_observability_types_are_send_sync() {
        assert_send_sync::<CognitiveObservability>();
        assert_send_sync::<HealthIndicators>();
        assert_send_sync::<ProgressTracker>();
    }

    #[test]
    fn test_completeness_types_are_send_sync() {
        assert_send_sync::<CognitiveCompleteness>();
        assert_send_sync::<AbstractionLevel>();
        assert_send_sync::<ConflictType>();
    }

    #[test]
    fn test_math_types_are_send_sync() {
        assert_send_sync::<CraftPerformance>();
    }

    #[test]
    fn test_replay_types_are_send_sync() {
        assert_send_sync::<ReplayContext>();
        assert_send_sync::<ReplaySession>();
        assert_send_sync::<ReplayEvent>();
    }

    #[test]
    fn test_budget_types_are_send_sync() {
        assert_send_sync::<ComputationalBudget>();
        assert_send_sync::<IntegrityCheck>();
        assert_send_sync::<NumericalIssue>();
        assert_send_sync::<ComplexityClass>();
        assert_send_sync::<BudgetGuard>();
    }

    #[test]
    fn test_maturation_types_are_send_sync() {
        assert_send_sync::<MaturationConfig>();
        assert_send_sync::<MaturationState>();
        assert_send_sync::<StopReason>();
        assert_send_sync::<RefinementStep>();
        assert_send_sync::<RefinementMetrics>();
    }
}
