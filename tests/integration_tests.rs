//! --------------------------
//! INFORMATION
//! --------------------------
//! Title: Integration Tests
//! Author: Carlos Eduardo Favini
//! Date: 2025-01-02
//! Version: 1.5.0
//! Description: End-to-end integration tests for the Digital Genome Community Edition.
//!              These tests verify complete workflows from input to output,
//!              including replay determinism and cross-module integration.
//!
//!              Resolves:
//!              - V019: Replay End-to-End Not Tested
//!              - V020: Integration Tests Absent
//!
//! Layer: Community (Test)
//! Dependencies: All modules
//!
//! --------------------------
//! TEST CATEGORIES
//! --------------------------
//! 1. Full Pipeline Tests: Input → SensoryCortex → CommunityOutput
//! 2. Replay Determinism Tests: Same input → Same output
//! 3. Maturation Integration: perceive_mature workflow
//! 4. Budget + Perception Integration: check then perceive
//! 5. Multi-Input Scenarios: Various signal types
//!
//! --------------------------
//! CHANGE LOG
//! --------------------------
//! 2025-01-02 - Carlos Eduardo Favini - Initial creation (v1.5.0)
//! --------------------------

use digital_genome_community::{
    // Core types
    RawInput, SensoryCortex,
    // Budget
    ComputationalBudget, IntegrityCheck,
    // Maturation
    MaturationConfig, StopReason,
    // Replay
    ReplayContext, ReplayVerifier,
    // Motors
    MotorCompetition, MotorCooperation, MotorType,
    // Observability
    CognitiveObservability, HealthIndicators, ProgressTracker,
    // Completeness
    CognitiveCompleteness, AbstractionLevel,
};

// =============================================================================
// FULL PIPELINE TESTS
// =============================================================================

/// Tests the complete perception pipeline from raw bytes to output.
#[test]
fn test_full_pipeline_simple_input() {
    let cortex = SensoryCortex::new();
    let input = RawInput::from_bytes(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    
    let output = cortex.perceive(&input);
    
    // Verify we got meaningful output
    assert_eq!(output.signals.sample_count, 10);
    assert!(output.signals.entropy > 0.0);
    assert!(output.state_history.transitions().len() >= 4);
}

/// Tests pipeline with periodic signal.
#[test]
fn test_full_pipeline_periodic_signal() {
    let cortex = SensoryCortex::new();
    
    // Create a clearly periodic signal
    let signal: Vec<u8> = (0..1000).map(|i| ((i % 10) * 25) as u8).collect();
    let input = RawInput::from_bytes(signal);
    
    let output = cortex.perceive(&input);
    
    // Should detect periodicity
    assert!(output.signals.periodicity_detected);
    assert!(output.signals.max_autocorrelation > 0.5);
}

/// Tests pipeline with high-entropy (random-like) signal.
#[test]
fn test_full_pipeline_high_entropy_signal() {
    let cortex = SensoryCortex::new();
    
    // Pseudo-random signal
    let signal: Vec<u8> = (0..1000)
        .map(|i| ((i * 17 + 31) % 256) as u8)
        .collect();
    let input = RawInput::from_bytes(signal);
    
    let output = cortex.perceive(&input);
    
    // Should have higher entropy
    assert!(output.signals.entropy > 0.5);
    // Lower compressibility
    assert!(output.signals.compressibility < 0.5);
}

/// Tests pipeline with constant signal.
#[test]
fn test_full_pipeline_constant_signal() {
    let cortex = SensoryCortex::new();
    let input = RawInput::from_bytes(vec![128u8; 1000]);
    
    let output = cortex.perceive(&input);
    
    // Constant signal has near-zero entropy
    assert!(output.signals.entropy < 0.1);
    // High compressibility
    assert!(output.signals.compressibility > 0.9);
}

// =============================================================================
// REPLAY DETERMINISM TESTS (V019)
// =============================================================================

/// Tests that identical inputs produce identical outputs.
#[test]
fn test_replay_determinism_basic() {
    let cortex = SensoryCortex::new();
    let input = RawInput::from_bytes(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    
    let output1 = cortex.perceive(&input);
    let output2 = cortex.perceive(&input);
    
    // Core signals must be identical
    assert_eq!(output1.signals.sample_count, output2.signals.sample_count);
    assert_eq!(output1.signals.entropy, output2.signals.entropy);
    assert_eq!(output1.signals.mean, output2.signals.mean);
    assert_eq!(output1.signals.std_dev, output2.signals.std_dev);
    assert_eq!(output1.signals.periodicity_detected, output2.signals.periodicity_detected);
}

/// Tests replay with ReplayContext for deterministic IDs.
#[test]
fn test_replay_with_context() {
    // Use from_seed for deterministic context
    let seed = b"test-seed-42";
    let mut ctx1 = ReplayContext::from_seed(seed);
    let mut ctx2 = ReplayContext::from_seed(seed);
    
    // Same seed should produce same IDs
    let id1 = ctx1.deterministic_action_id();
    let id2 = ctx2.deterministic_action_id();
    
    assert_eq!(id1, id2);
    
    // Sequence should continue deterministically
    let id1_next = ctx1.deterministic_dna_id();
    let id2_next = ctx2.deterministic_dna_id();
    
    assert_eq!(id1_next, id2_next);
}

/// Tests replay session recording and verification.
#[test]
fn test_replay_session_verification() {
    let cortex = SensoryCortex::new();
    
    // Create two identical perceptions
    let input1 = RawInput::from_bytes(vec![10, 20, 30, 40, 50]);
    let input2 = RawInput::from_bytes(vec![10, 20, 30, 40, 50]);
    
    let output1 = cortex.perceive(&input1);
    let output2 = cortex.perceive(&input2);
    
    // Outputs should be identical
    assert_eq!(output1.signals.entropy, output2.signals.entropy);
    assert_eq!(output1.signals.mean, output2.signals.mean);
    
    // Create contexts and export sessions
    let ctx1 = ReplayContext::from_seed(b"session-42");
    let ctx2 = ReplayContext::from_seed(b"session-42");
    
    let session1 = ctx1.export_session();
    let session2 = ctx2.export_session();
    
    // Sessions from same seed should be equivalent
    let comparison = ReplayVerifier::compare(&session1, &session2, 10);
    assert!(comparison.identical);
}

/// Tests that different inputs produce different outputs.
#[test]
fn test_replay_different_inputs_differ() {
    let cortex = SensoryCortex::new();
    
    let input1 = RawInput::from_bytes(vec![1, 2, 3, 4, 5]);
    let input2 = RawInput::from_bytes(vec![5, 4, 3, 2, 1]);
    
    let output1 = cortex.perceive(&input1);
    let output2 = cortex.perceive(&input2);
    
    // Different inputs should produce different mean
    assert_ne!(output1.signals.mean, output2.signals.mean);
}

// =============================================================================
// MATURATION INTEGRATION TESTS
// =============================================================================

/// Tests maturation with default config.
#[test]
fn test_maturation_integration_default() {
    let cortex = SensoryCortex::new();
    let config = MaturationConfig::default();
    let input = RawInput::from_bytes((0..100).collect::<Vec<u8>>());
    
    let output = cortex.perceive_mature(&input, &config);
    
    // Should have performed iterations
    assert!(output.iterations() >= 1);
    // Should have valid perception
    assert!(output.perception.signals.sample_count > 0);
}

/// Tests maturation with deep config.
#[test]
fn test_maturation_integration_deep() {
    let cortex = SensoryCortex::new();
    let config = MaturationConfig::deep();
    let input = RawInput::from_bytes((0..200).collect::<Vec<u8>>());
    
    let output = cortex.perceive_mature(&input, &config);
    
    // Deep config requires at least 3 iterations
    assert!(output.iterations() >= 3 || output.converged());
}

/// Tests maturation produces consistent results.
#[test]
fn test_maturation_determinism() {
    let cortex = SensoryCortex::new();
    let config = MaturationConfig::new().with_max_iterations(3);
    let input = RawInput::from_bytes(vec![50, 100, 150, 200, 250]);
    
    let output1 = cortex.perceive_mature(&input, &config);
    let output2 = cortex.perceive_mature(&input, &config);
    
    // Should produce identical final perceptions
    assert_eq!(output1.perception.signals.entropy, output2.perception.signals.entropy);
    assert_eq!(output1.iterations(), output2.iterations());
}

// =============================================================================
// BUDGET + PERCEPTION INTEGRATION
// =============================================================================

/// Tests budget check followed by perception.
#[test]
fn test_budget_then_perceive_workflow() {
    let cortex = SensoryCortex::new();
    let budget = ComputationalBudget::default();
    let input = RawInput::from_bytes(vec![1, 2, 3, 4, 5, 6, 7, 8]);
    
    // Check budget first
    let check = cortex.check_budget(&input, &budget);
    assert!(matches!(check, IntegrityCheck::WithinBudget));
    
    // Then perceive
    let output = cortex.perceive(&input);
    assert!(output.signals.sample_count > 0);
}

/// Tests perceive_checked complete workflow.
#[test]
fn test_perceive_checked_workflow() {
    let cortex = SensoryCortex::new();
    let budget = ComputationalBudget::default();
    let input = RawInput::from_bytes((0..500).map(|i| (i % 256) as u8).collect());
    
    let result = cortex.perceive_checked(&input, &budget);
    
    assert!(result.is_ok());
    let output = result.unwrap();
    assert_eq!(output.signals.sample_count, 500);
}

/// Tests budget rejection workflow.
#[test]
fn test_budget_rejection_workflow() {
    let cortex = SensoryCortex::new();
    let budget = ComputationalBudget::new().with_max_bytes(10);
    let input = RawInput::from_bytes(vec![0u8; 100]); // 100 bytes > 10 limit
    
    let result = cortex.perceive_checked(&input, &budget);
    
    assert!(result.is_err());
    match result {
        Err(IntegrityCheck::ExceedsMemory { requested, available }) => {
            assert_eq!(requested, 100);
            assert_eq!(available, 10);
        }
        _ => panic!("Expected ExceedsMemory"),
    }
}

// =============================================================================
// MOTOR INTEGRATION TESTS
// =============================================================================

/// Tests motor competition workflow.
#[test]
fn test_motor_competition_integration() {
    // MotorCompetition::from_scores takes [f64; 4] array
    let scores = [0.8, 0.6, 0.7, 0.9]; // [Praxis, Nash, Chaos, Meristic]
    
    let competition = MotorCompetition::from_scores(scores);
    
    // Meristic (index 3) has highest score
    assert_eq!(competition.dominant_motor, Some(MotorType::Meristic));
    assert!((competition.current_scores[3] - 0.9).abs() < 0.001);
}

/// Tests motor cooperation workflow.
#[test]
fn test_motor_cooperation_integration() {
    let scores = [0.8, 0.6, 0.7, 0.9]; // [Praxis, Nash, Chaos, Meristic]
    
    let cooperation = MotorCooperation::from_scores(&scores);
    
    // Pairwise agreement should be computed
    // Agreement between motor i and j = 1 - |score_i - score_j|
    // E.g., Praxis (0.8) and Nash (0.6): agreement = 1 - 0.2 = 0.8
    assert!(cooperation.pairwise_agreement[0][1] > 0.0);
}

// =============================================================================
// OBSERVABILITY INTEGRATION TESTS
// =============================================================================

/// Tests cognitive observability workflow.
#[test]
fn test_observability_integration() {
    let observability = CognitiveObservability::new();
    
    // Check default health
    assert!(!observability.health.has_warnings());
    assert_eq!(observability.health.overall_health, 1.0);
}

/// Tests progress tracker.
#[test]
fn test_progress_tracker_integration() {
    let tracker = ProgressTracker::new();
    
    // Default values
    assert_eq!(tracker.cycles_without_progress, 0);
    assert_eq!(tracker.max_level_reached, 0.0);
}

/// Tests health indicators.
#[test]
fn test_health_indicators_integration() {
    let health = HealthIndicators::healthy();
    
    assert!(!health.possibly_stuck);
    assert!(!health.high_divergence);
    assert!(!health.rapid_oscillation);
    assert!(!health.timeout_risk);
    assert_eq!(health.warning_count(), 0);
}

// =============================================================================
// COMPLETENESS INTEGRATION TESTS
// =============================================================================

/// Tests completeness workflow.
#[test]
fn test_completeness_integration() {
    // Create a complete state
    let completeness = CognitiveCompleteness::complete();
    
    assert!(completeness.is_complete());
    assert!(!completeness.has_contradictions());
}

/// Tests abstraction levels.
#[test]
fn test_abstraction_levels() {
    let level0 = AbstractionLevel::Carrier;
    let level1 = AbstractionLevel::Pattern;
    let level2 = AbstractionLevel::Structure;
    
    // Check progression using level() method
    assert!(level0.level() < level1.level());
    assert!(level1.level() < level2.level());
    
    // Check specific values
    assert_eq!(level0.level(), 0.0);
    assert_eq!(level1.level(), 1.0);
    assert_eq!(level2.level(), 2.0);
}

// =============================================================================
// MULTI-SIGNAL SCENARIO TESTS
// =============================================================================

/// Tests processing multiple different signals sequentially.
#[test]
fn test_multi_signal_sequential() {
    let cortex = SensoryCortex::new();
    
    let signals = vec![
        vec![0u8; 100],                                    // Constant
        (0..100).collect::<Vec<u8>>(),                     // Ramp
        (0..100).map(|i| ((i % 10) * 25) as u8).collect(), // Periodic
    ];
    
    let outputs: Vec<_> = signals
        .into_iter()
        .map(|s| cortex.perceive(&RawInput::from_bytes(s)))
        .collect();
    
    // Each should produce valid output
    for output in &outputs {
        assert!(output.signals.sample_count > 0);
    }
    
    // Constant should have lowest entropy
    assert!(outputs[0].signals.entropy < outputs[1].signals.entropy);
}

/// Tests processing the same signal multiple times.
#[test]
fn test_same_signal_multiple_times() {
    let cortex = SensoryCortex::new();
    let input = RawInput::from_bytes(vec![42, 84, 126, 168, 210]);
    
    let outputs: Vec<_> = (0..10)
        .map(|_| cortex.perceive(&input))
        .collect();
    
    // All outputs should be identical (determinism)
    let first = &outputs[0];
    for output in outputs.iter().skip(1) {
        assert_eq!(output.signals.entropy, first.signals.entropy);
        assert_eq!(output.signals.mean, first.signals.mean);
    }
}

// =============================================================================
// EDGE CASE TESTS
// =============================================================================

/// Tests single byte input.
#[test]
fn test_single_byte_input() {
    let cortex = SensoryCortex::new();
    let input = RawInput::from_bytes(vec![42]);
    
    let output = cortex.perceive(&input);
    
    assert_eq!(output.signals.sample_count, 1);
    // Single byte has defined entropy (0 for single sample)
    assert!(output.signals.entropy >= 0.0);
}

/// Tests maximum value bytes.
#[test]
fn test_max_value_bytes() {
    let cortex = SensoryCortex::new();
    let input = RawInput::from_bytes(vec![255u8; 100]);
    
    let output = cortex.perceive(&input);
    
    assert_eq!(output.signals.sample_count, 100);
    assert_eq!(output.signals.mean, 255.0);
    assert!(output.signals.entropy < 0.1); // Constant
}

/// Tests minimum value bytes.
#[test]
fn test_min_value_bytes() {
    let cortex = SensoryCortex::new();
    let input = RawInput::from_bytes(vec![0u8; 100]);
    
    let output = cortex.perceive(&input);
    
    assert_eq!(output.signals.sample_count, 100);
    assert_eq!(output.signals.mean, 0.0);
    assert!(output.signals.entropy < 0.1); // Constant
}

/// Tests alternating bytes.
#[test]
fn test_alternating_bytes() {
    let cortex = SensoryCortex::new();
    let input = RawInput::from_bytes(
        (0..100).map(|i| if i % 2 == 0 { 0 } else { 255 }).collect()
    );
    
    let output = cortex.perceive(&input);
    
    // Should detect periodicity (period = 2)
    assert!(output.signals.periodicity_detected);
}

/// Tests empty input handling.
#[test]
fn test_empty_input_handling() {
    let cortex = SensoryCortex::new();
    let input = RawInput::from_bytes(vec![]);
    
    let output = cortex.perceive(&input);
    
    assert_eq!(output.signals.sample_count, 0);
}

/// Tests maturation with empty input.
#[test]
fn test_maturation_empty_input() {
    let cortex = SensoryCortex::new();
    let config = MaturationConfig::default();
    let input = RawInput::from_bytes(vec![]);
    
    let output = cortex.perceive_mature(&input, &config);
    
    assert_eq!(output.stop_reason(), StopReason::EmptyInput);
}

// =============================================================================
// COMBINED WORKFLOW TESTS
// =============================================================================

/// Tests complete workflow: budget check → perceive → maturation.
#[test]
fn test_complete_workflow() {
    let cortex = SensoryCortex::new();
    let budget = ComputationalBudget::default();
    let config = MaturationConfig::default();
    let input = RawInput::from_bytes((0..200).map(|i| (i % 256) as u8).collect());
    
    // Step 1: Check budget
    let check = cortex.check_budget(&input, &budget);
    assert!(matches!(check, IntegrityCheck::WithinBudget));
    
    // Step 2: Basic perception
    let basic_output = cortex.perceive(&input);
    assert!(basic_output.signals.sample_count > 0);
    
    // Step 3: Mature perception
    let mature_output = cortex.perceive_mature(&input, &config);
    assert!(mature_output.iterations() >= 1);
    
    // Both should have same sample count
    assert_eq!(
        basic_output.signals.sample_count,
        mature_output.perception.signals.sample_count
    );
}

/// Tests workflow with replay verification.
#[test]
fn test_workflow_with_replay() {
    let cortex = SensoryCortex::new();
    let input_bytes = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
    
    // First run
    let input1 = RawInput::from_bytes(input_bytes.clone());
    let ctx1 = ReplayContext::from_seed(b"workflow-test");
    let output1 = cortex.perceive(&input1);
    
    // Second run (should be identical)
    let input2 = RawInput::from_bytes(input_bytes.clone());
    let ctx2 = ReplayContext::from_seed(b"workflow-test");
    let output2 = cortex.perceive(&input2);
    
    // Export and compare sessions
    let session1 = ctx1.export_session();
    let session2 = ctx2.export_session();
    
    let comparison = ReplayVerifier::compare(&session1, &session2, 10);
    assert!(comparison.identical);
    
    // Verify outputs match
    assert_eq!(output1.signals.entropy, output2.signals.entropy);
    assert_eq!(output1.signals.mean, output2.signals.mean);
    assert_eq!(output1.signals.std_dev, output2.signals.std_dev);
}

// =============================================================================
// MATURATION SPECIFIC TESTS
// =============================================================================

/// Tests single pass maturation.
#[test]
fn test_maturation_single_pass() {
    let cortex = SensoryCortex::new();
    let config = MaturationConfig::single_pass();
    let input = RawInput::from_bytes(vec![1, 2, 3, 4, 5, 6, 7, 8]);

    let output = cortex.perceive_mature(&input, &config);

    assert_eq!(output.iterations(), 1);
    assert_eq!(output.stop_reason(), StopReason::MaxIterations);
}

/// Tests maturation records delta history.
#[test]
fn test_maturation_delta_history() {
    let cortex = SensoryCortex::new();
    let config = MaturationConfig::new()
        .with_max_iterations(3)
        .with_min_iterations(1);
    let input = RawInput::from_bytes(vec![1, 2, 3, 4, 5]);

    let output = cortex.perceive_mature(&input, &config);

    // Should have delta history for each iteration
    assert_eq!(output.maturation.delta_history.len(), output.iterations());
}

/// Tests maturation records time.
#[test]
fn test_maturation_time_tracking() {
    let cortex = SensoryCortex::new();
    let config = MaturationConfig::new().with_max_iterations(3);
    let input = RawInput::from_bytes(vec![1, 2, 3, 4, 5, 6, 7, 8]);

    let output = cortex.perceive_mature(&input, &config);

    // Should have recorded some time
    assert!(output.maturation_time_ns() > 0);
    // Should have time for each iteration
    assert_eq!(output.maturation.iteration_times_ns.len(), output.iterations());
}

/// Tests mature output is stateless.
#[test]
fn test_mature_output_is_stateless() {
    // This test verifies that maturation does NOT create persistent state
    let cortex = SensoryCortex::new();
    let config = MaturationConfig::new().with_max_iterations(5);

    let input1 = RawInput::from_bytes(vec![1, 2, 3, 4, 5]);
    let input2 = RawInput::from_bytes(vec![1, 2, 3, 4, 5]);

    // Two identical inputs should produce identical outputs
    // This verifies no hidden state accumulates
    let output1 = cortex.perceive_mature(&input1, &config);
    let output2 = cortex.perceive_mature(&input2, &config);

    assert_eq!(output1.perception.signals.entropy, output2.perception.signals.entropy);
    assert_eq!(output1.perception.signals.sample_count, output2.perception.signals.sample_count);
}
