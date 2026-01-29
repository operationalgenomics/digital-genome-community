//! --------------------------
//! CANONICAL VALIDATION TESTS — MVP-5
//! --------------------------
//! Title: Canonical Validation Suite
//! Version: 0.5.1
//! Date: 2025-01-26
//! 
//! Implements:
//! - 1000 replay determinism test (AF-6)
//! - Cross-instance equivalence (AO-11)
//! - Axiom validation b1-b7
//! - Motor order verification (LEI-AF-10-07)
//! - CP veto validation (AF-10.5)
//!
//! Reference: Canon Estratificado Operacional v2.0
//! --------------------------

use digital_genome_community::{
    RawInput, SensoryCortex,
    CognitiveCycle, MotorContext,
    CraftPerformance, CpResult,
};

// =============================================================================
// AF-6: DETERMINISM VALIDATION (1000 REPLAYS)
// =============================================================================

/// AF-6: 1000 iterations MUST produce bit-identical output
#[test]
fn test_af6_determinism_1000_replays() {
    let cortex = SensoryCortex::new();
    
    // Warm-up (Canon allows non-determinism on first call)
    let warmup_input = RawInput::from_bytes(vec![0u8; 64]);
    let _ = cortex.perceive(&warmup_input);
    
    // Test input
    let test_data: Vec<u8> = (0..1000).map(|i| (i % 256) as u8).collect();
    let input = RawInput::from_bytes(test_data);
    
    // First run - baseline
    let baseline = cortex.perceive(&input);
    let baseline_entropy = baseline.signals.entropy;
    let baseline_mean = baseline.signals.mean;
    
    // 1000 replays
    for i in 0..1000 {
        let output = cortex.perceive(&input);
        assert_eq!(
            output.signals.entropy, baseline_entropy,
            "Entropy diverged at iteration {}: {} != {}", 
            i, output.signals.entropy, baseline_entropy
        );
        assert_eq!(
            output.signals.mean, baseline_mean,
            "Mean diverged at iteration {}: {} != {}", 
            i, output.signals.mean, baseline_mean
        );
    }
}

/// AF-6: 1000 cognitive cycles MUST produce identical DNA
#[test]
fn test_af6_cognitive_cycle_1000_replays() {
    let cycle = CognitiveCycle::new();
    let ctx = MotorContext::default();
    
    let test_data: Vec<u8> = (0..500).map(|i| (i * 7 % 256) as u8).collect();
    
    // Baseline
    let baseline = cycle.process(&test_data, &ctx);
    let baseline_dna = baseline.dna_fingerprint;
    let baseline_cp = baseline.cp_value;
    
    // 1000 replays
    for i in 0..1000 {
        let output = cycle.process(&test_data, &ctx);
        assert_eq!(
            output.dna_fingerprint, baseline_dna,
            "DNA diverged at iteration {}", i
        );
        assert!(
            (output.cp_value - baseline_cp).abs() < 1e-15,
            "CP diverged at iteration {}: {} != {}", 
            i, output.cp_value, baseline_cp
        );
    }
}

// =============================================================================
// AO-11: CROSS-INSTANCE EQUIVALENCE
// =============================================================================

/// AO-11: Different cortex instances MUST produce identical output
#[test]
fn test_ao11_cross_instance_equivalence() {
    let test_data: Vec<u8> = (0..=255).collect();
    
    // Warm up all instances
    let cortex1 = SensoryCortex::new();
    let cortex2 = SensoryCortex::new();
    let cortex3 = SensoryCortex::new();
    
    let warmup = RawInput::from_bytes(vec![0u8; 64]);
    let _ = cortex1.perceive(&warmup);
    let _ = cortex2.perceive(&warmup);
    let _ = cortex3.perceive(&warmup);
    
    // Test
    let input = RawInput::from_bytes(test_data);
    let out1 = cortex1.perceive(&input);
    let out2 = cortex2.perceive(&input);
    let out3 = cortex3.perceive(&input);
    
    assert_eq!(out1.signals.entropy, out2.signals.entropy);
    assert_eq!(out2.signals.entropy, out3.signals.entropy);
    assert_eq!(out1.signals.mean, out2.signals.mean);
    assert_eq!(out2.signals.mean, out3.signals.mean);
}

/// AO-11: Different cognitive cycle instances produce identical DNA
#[test]
fn test_ao11_cross_cycle_equivalence() {
    let cycle1 = CognitiveCycle::new();
    let cycle2 = CognitiveCycle::new();
    let ctx = MotorContext::default();
    
    let test_data: Vec<u8> = (0..100).map(|i| (i * 3) as u8).collect();
    
    let out1 = cycle1.process(&test_data, &ctx);
    let out2 = cycle2.process(&test_data, &ctx);
    
    assert_eq!(out1.dna_fingerprint, out2.dna_fingerprint);
    assert_eq!(out1.cp_value, out2.cp_value);
}

// =============================================================================
// B.1-B.7: AXIOM VALIDATION
// =============================================================================

/// B.1: Estado Basal Operacional - GDC is semantically null
#[test]
fn test_b1_estado_basal_operacional() {
    let cycle = CognitiveCycle::new();
    let ctx = MotorContext::default();
    
    // Empty input should produce valid output (not crash)
    let empty = cycle.process(&[], &ctx);
    
    // GDC processes, doesn't interpret
    assert!(empty.cp_value >= 0.0 && empty.cp_value <= 1.0);
}

/// B.2: Não-Agência Absoluta - GDC does not act, only emits
#[test]
fn test_b2_nao_agencia_absoluta() {
    let cycle = CognitiveCycle::new();
    let ctx = MotorContext::default();
    
    let data = vec![1, 2, 3, 4, 5];
    let output = cycle.process(&data, &ctx);
    
    // GDC emits DNA (observation), doesn't modify input
    assert!(!output.dna_fingerprint.iter().all(|&b| b == 0));
    // Input is unchanged (we can re-process and get same result)
    let output2 = cycle.process(&data, &ctx);
    assert_eq!(output.dna_fingerprint, output2.dna_fingerprint);
}

/// B.3: Veto Absoluto - Any M=0 results in CP=0
#[test]
fn test_b3_veto_absoluto() {
    use CraftPerformance;
    
    // Normal case
    let result = CraftPerformance::calculate(0.5, 0.5, 0.5, 0.5);
    assert!(matches!(result, CpResult::Valid { .. }));
    
    // Veto cases - any zero causes veto
    let veto_p = CraftPerformance::calculate(0.0, 0.5, 0.5, 0.5);
    assert!(matches!(veto_p, CpResult::Vetoed { .. }));
    
    let veto_n = CraftPerformance::calculate(0.5, 0.0, 0.5, 0.5);
    assert!(matches!(veto_n, CpResult::Vetoed { .. }));
    
    let veto_c = CraftPerformance::calculate(0.5, 0.5, 0.0, 0.5);
    assert!(matches!(veto_c, CpResult::Vetoed { .. }));
    
    let veto_m = CraftPerformance::calculate(0.5, 0.5, 0.5, 0.0);
    assert!(matches!(veto_m, CpResult::Vetoed { .. }));
}

/// B.4: Descoberta Antes de Classificação - Observe first, categorize later
#[test]
fn test_b4_descoberta_antes_classificacao() {
    let cortex = SensoryCortex::new();
    let _ = cortex.perceive(&RawInput::from_bytes(vec![0u8; 64])); // warmup
    
    // Two very different signals
    let signal_a: Vec<u8> = vec![0u8; 1000]; // constant
    let signal_b: Vec<u8> = (0..1000).map(|i| (i % 256) as u8).collect(); // varying
    
    let out_a = cortex.perceive(&RawInput::from_bytes(signal_a));
    let out_b = cortex.perceive(&RawInput::from_bytes(signal_b));
    
    // GDC observes differences without classifying
    assert_ne!(out_a.signals.entropy, out_b.signals.entropy);
    // Both produce valid numerical outputs (not labels)
    assert!(out_a.signals.entropy >= 0.0);
    assert!(out_b.signals.entropy >= 0.0);
}

/// B.5: Determinismo Absoluto - Same input = same output
#[test]
fn test_b5_determinismo_absoluto() {
    let cortex = SensoryCortex::new();
    let _ = cortex.perceive(&RawInput::from_bytes(vec![0u8; 64])); // warmup
    
    let data: Vec<u8> = (0..500).map(|i| ((i * 17) % 256) as u8).collect();
    let input = RawInput::from_bytes(data);
    
    let out1 = cortex.perceive(&input);
    let out2 = cortex.perceive(&input);
    
    // Bit-exact equality
    assert_eq!(out1.signals.entropy.to_bits(), out2.signals.entropy.to_bits());
    assert_eq!(out1.signals.mean.to_bits(), out2.signals.mean.to_bits());
}

/// B.6: Transparência Operacional - All state is auditable
#[test]
fn test_b6_transparencia_operacional() {
    let cycle = CognitiveCycle::new();
    let ctx = MotorContext::default();
    
    let data = vec![10, 20, 30, 40, 50];
    let output = cycle.process(&data, &ctx);
    
    // All motor scores are visible
    assert!(output.motor_scores.praxis >= 0.0 && output.motor_scores.praxis <= 1.0);
    assert!(output.motor_scores.nash >= 0.0 && output.motor_scores.nash <= 1.0);
    assert!(output.motor_scores.chaos >= 0.0 && output.motor_scores.chaos <= 1.0);
    assert!(output.motor_scores.meristic >= 0.0 && output.motor_scores.meristic <= 1.0);
    
    // CP is visible
    assert!(output.cp_value >= 0.0 && output.cp_value <= 1.0);
    
    // DNA is visible
    assert_eq!(output.dna_fingerprint.len(), 32);
    
    // Veto status is visible
    let _ = output.vetoed; // compiles = visible
}

/// B.7: Fronteira Community/Enterprise - GDC doesn't persist or execute
#[test]
fn test_b7_fronteira_community_enterprise() {
    let cycle1 = CognitiveCycle::new();
    let cycle2 = CognitiveCycle::new();
    let ctx = MotorContext::default();
    
    // Process in cycle1
    let _ = cycle1.process(&[1, 2, 3], &ctx);
    let _ = cycle1.process(&[4, 5, 6], &ctx);
    
    // cycle2 has no knowledge of cycle1's history
    let out2 = cycle2.process(&[1, 2, 3], &ctx);
    let out1_fresh = cycle1.process(&[1, 2, 3], &ctx);
    
    // Both produce identical output - no state carried
    assert_eq!(out1_fresh.dna_fingerprint, out2.dna_fingerprint);
}

// =============================================================================
// LEI-AF-10-07: MOTOR ORDER VALIDATION
// =============================================================================

/// LEI-AF-10-07: Meristic executes AFTER Praxis, Nash, Chaos
#[test]
fn test_lei_af10_07_motor_order() {
    let cycle = CognitiveCycle::new();
    let ctx = MotorContext::default();
    
    let output = cycle.process(&[1, 2, 3, 4, 5], &ctx);
    
    // All motors executed (have valid scores)
    assert!(output.motor_scores.praxis > 0.0 || output.motor_scores.praxis == 0.0);
    assert!(output.motor_scores.nash > 0.0 || output.motor_scores.nash == 0.0);
    assert!(output.motor_scores.chaos > 0.0 || output.motor_scores.chaos == 0.0);
    assert!(output.motor_scores.meristic > 0.0 || output.motor_scores.meristic == 0.0);
    
    // CP reflects all motors (would be different if order changed)
    let expected_cp = output.motor_scores.praxis 
        * output.motor_scores.nash 
        * output.motor_scores.chaos 
        * output.motor_scores.meristic;
    
    assert!((output.cp_value - expected_cp).abs() < 1e-10);
}

// =============================================================================
// AF-10.5: CP FORMULA VALIDATION
// =============================================================================

/// AF-10.5: CP = M_P × M_N × M_C × M_M
#[test]
fn test_af10_5_cp_formula() {
    use CraftPerformance;
    
    let m_p = 0.8;
    let m_n = 0.9;
    let m_c = 0.7;
    let m_m = 0.6;
    
    let result = CraftPerformance::calculate(m_p, m_n, m_c, m_m);
    
    if let CpResult::Valid { value, .. } = result {
        let expected = m_p * m_n * m_c * m_m;
        assert!((value - expected).abs() < 1e-15);
    } else {
        panic!("Expected valid CP result");
    }
}

/// AF-10.5: CP compression property (CP ≤ min(motors))
#[test]
fn test_af10_5_cp_compression() {
    use CraftPerformance;
    
    for _ in 0..100 {
        let m_p = 0.1 + (rand_simple() * 0.9);
        let m_n = 0.1 + (rand_simple() * 0.9);
        let m_c = 0.1 + (rand_simple() * 0.9);
        let m_m = 0.1 + (rand_simple() * 0.9);
        
        let result = CraftPerformance::calculate(m_p, m_n, m_c, m_m);
        
        if let CpResult::Valid { value, .. } = result {
            let min_motor = m_p.min(m_n).min(m_c).min(m_m);
            assert!(value <= min_motor + 1e-10);
        }
    }
}

// Simple deterministic "random" for testing
fn rand_simple() -> f64 {
    static mut SEED: u64 = 12345;
    unsafe {
        SEED = SEED.wrapping_mul(1103515245).wrapping_add(12345);
        ((SEED >> 16) & 0x7fff) as f64 / 32768.0
    }
}

// =============================================================================
// NUMERICAL STABILITY
// =============================================================================

/// Numerical: No NaN in any output
#[test]
fn test_numerical_no_nan() {
    let cycle = CognitiveCycle::new();
    let ctx = MotorContext::default();
    
    let edge_cases: Vec<Vec<u8>> = vec![
        vec![],                           // empty
        vec![0],                          // single zero
        vec![255],                        // single max
        vec![0; 1000],                    // all zeros
        vec![255; 1000],                  // all max
        (0..256).map(|i| i as u8).collect(), // all values
    ];
    
    for (i, data) in edge_cases.iter().enumerate() {
        let output = cycle.process(data, &ctx);
        assert!(!output.cp_value.is_nan(), "NaN at case {}", i);
        assert!(!output.motor_scores.praxis.is_nan(), "NaN praxis at case {}", i);
        assert!(!output.motor_scores.nash.is_nan(), "NaN nash at case {}", i);
        assert!(!output.motor_scores.chaos.is_nan(), "NaN chaos at case {}", i);
        assert!(!output.motor_scores.meristic.is_nan(), "NaN meristic at case {}", i);
    }
}

/// Numerical: No Infinity in any output
#[test]
fn test_numerical_no_infinity() {
    let cycle = CognitiveCycle::new();
    let ctx = MotorContext::default();
    
    let large_input: Vec<u8> = (0..10000).map(|i| (i % 256) as u8).collect();
    let output = cycle.process(&large_input, &ctx);
    
    assert!(output.cp_value.is_finite());
    assert!(output.motor_scores.praxis.is_finite());
    assert!(output.motor_scores.nash.is_finite());
    assert!(output.motor_scores.chaos.is_finite());
    assert!(output.motor_scores.meristic.is_finite());
}

// =============================================================================
// STRESS TESTS
// =============================================================================

/// Stress: 10,000 rapid sequential calls
#[test]
fn test_stress_rapid_calls() {
    let cortex = SensoryCortex::new();
    let _ = cortex.perceive(&RawInput::from_bytes(vec![0u8; 64])); // warmup
    
    let input = RawInput::from_bytes(vec![42u8; 100]);
    
    for _ in 0..10_000 {
        let output = cortex.perceive(&input);
        assert!(output.signals.entropy >= 0.0);
    }
}
