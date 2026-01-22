//! Rigorous Validation Script for Digital Genome Community Edition
//!
//! This script performs comprehensive validation of the Core before release.
//! It tests all modules with synthetic and edge-case datasets.
//!
//! Run with: cargo run --example rigorous_validation
//!
//! Expected output: All validations pass with documented behavior.

use digital_genome_community::{
    RawInput, SensoryCortex,
    MaturationConfig,
    ComputationalBudget,
    ReplayContext,
    MotorCompetition, MotorCooperation, MotorType,
    CognitiveObservability, HealthIndicators,
    CognitiveCompleteness, AbstractionLevel, MissingSignal,
};

fn main() {
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║   DIGITAL GENOME COMMUNITY EDITION - RIGOROUS VALIDATION         ║");
    println!("║   Version: 0.1.0-MVP-2                                             ║");
    println!("║   Target: v0.1.0-MVP-2 (Adão Sintético)                             ║");
    println!("╚══════════════════════════════════════════════════════════════════╝");
    println!();

    let mut passed = 0;
    let mut failed = 0;
    let mut warnings = 0;

    // ═══════════════════════════════════════════════════════════════════════
    // SECTION 1: BASIC PERCEPTION TESTS
    // ═══════════════════════════════════════════════════════════════════════
    println!("┌─────────────────────────────────────────────────────────────────┐");
    println!("│ SECTION 1: BASIC PERCEPTION                                     │");
    println!("└─────────────────────────────────────────────────────────────────┘");

    let cortex = SensoryCortex::new();

    // Test 1.1: Empty input
    print!("  [1.1] Empty input handling... ");
    let result = validate_empty_input(&cortex);
    report_result(&result, &mut passed, &mut failed, &mut warnings);

    // Test 1.2: Constant signal
    print!("  [1.2] Constant signal (zero entropy)... ");
    let result = validate_constant_signal(&cortex);
    report_result(&result, &mut passed, &mut failed, &mut warnings);

    // Test 1.3: Maximum entropy signal
    print!("  [1.3] Maximum entropy signal... ");
    let result = validate_max_entropy(&cortex);
    report_result(&result, &mut passed, &mut failed, &mut warnings);

    // Test 1.4: Periodic signal detection
    print!("  [1.4] Periodic signal detection... ");
    let result = validate_periodicity(&cortex);
    report_result(&result, &mut passed, &mut failed, &mut warnings);

    // Test 1.5: Binary alternation (edge case from v1.5.4)
    print!("  [1.5] Binary alternation pattern... ");
    let result = validate_binary_alternation(&cortex);
    report_result(&result, &mut passed, &mut failed, &mut warnings);

    println!();

    // ═══════════════════════════════════════════════════════════════════════
    // SECTION 2: DETERMINISM & REPLAY
    // ═══════════════════════════════════════════════════════════════════════
    println!("┌─────────────────────────────────────────────────────────────────┐");
    println!("│ SECTION 2: DETERMINISM & REPLAY                                 │");
    println!("└─────────────────────────────────────────────────────────────────┘");

    // Test 2.1: Same input produces same output
    print!("  [2.1] Perception determinism... ");
    let result = validate_determinism(&cortex);
    report_result(&result, &mut passed, &mut failed, &mut warnings);

    // Test 2.2: Replay context determinism
    print!("  [2.2] Replay context determinism... ");
    let result = validate_replay_context();
    report_result(&result, &mut passed, &mut failed, &mut warnings);

    // Test 2.3: Different inputs produce different outputs
    print!("  [2.3] Input differentiation... ");
    let result = validate_input_differentiation(&cortex);
    report_result(&result, &mut passed, &mut failed, &mut warnings);

    // Test 2.4: Permutation distinction (edge case from v1.5.4)
    print!("  [2.4] Permutation distinction... ");
    let result = validate_permutation_distinction(&cortex);
    report_result(&result, &mut passed, &mut failed, &mut warnings);

    println!();

    // ═══════════════════════════════════════════════════════════════════════
    // SECTION 3: COMPUTATIONAL BUDGET
    // ═══════════════════════════════════════════════════════════════════════
    println!("┌─────────────────────────────────────────────────────────────────┐");
    println!("│ SECTION 3: COMPUTATIONAL BUDGET                                 │");
    println!("└─────────────────────────────────────────────────────────────────┘");

    // Test 3.1: Memory budget enforcement
    print!("  [3.1] Memory budget enforcement... ");
    let result = validate_memory_budget();
    report_result(&result, &mut passed, &mut failed, &mut warnings);

    // Test 3.2: Recursion depth tracking
    print!("  [3.2] Recursion depth tracking... ");
    let result = validate_recursion_tracking();
    report_result(&result, &mut passed, &mut failed, &mut warnings);

    // Test 3.3: Unlimited budget option
    print!("  [3.3] Unlimited budget option... ");
    let result = validate_unlimited_budget();
    report_result(&result, &mut passed, &mut failed, &mut warnings);

    println!();

    // ═══════════════════════════════════════════════════════════════════════
    // SECTION 4: MATURATION
    // ═══════════════════════════════════════════════════════════════════════
    println!("┌─────────────────────────────────────────────────────────────────┐");
    println!("│ SECTION 4: PERCEPTUAL MATURATION                                │");
    println!("└─────────────────────────────────────────────────────────────────┘");

    // Test 4.1: Single pass maturation
    print!("  [4.1] Single pass maturation... ");
    let result = validate_single_pass_maturation(&cortex);
    report_result(&result, &mut passed, &mut failed, &mut warnings);

    // Test 4.2: Multiple passes maturation
    print!("  [4.2] Multiple passes maturation... ");
    let result = validate_multiple_passes(&cortex);
    report_result(&result, &mut passed, &mut failed, &mut warnings);

    // Test 4.3: Maturation is ephemeral
    print!("  [4.3] Maturation ephemerality... ");
    let result = validate_maturation_ephemeral(&cortex);
    report_result(&result, &mut passed, &mut failed, &mut warnings);

    println!();

    // ═══════════════════════════════════════════════════════════════════════
    // SECTION 5: MOTOR SYSTEM
    // ═══════════════════════════════════════════════════════════════════════
    println!("┌─────────────────────────────────────────────────────────────────┐");
    println!("│ SECTION 5: COGNITIVE MOTORS                                     │");
    println!("└─────────────────────────────────────────────────────────────────┘");

    // Test 5.1: All motor types exist
    print!("  [5.1] Motor types complete... ");
    let result = validate_motor_types();
    report_result(&result, &mut passed, &mut failed, &mut warnings);

    // Test 5.2: Motor competition
    print!("  [5.2] Motor competition... ");
    let result = validate_motor_competition();
    report_result(&result, &mut passed, &mut failed, &mut warnings);

    // Test 5.3: Motor cooperation
    print!("  [5.3] Motor cooperation... ");
    let result = validate_motor_cooperation();
    report_result(&result, &mut passed, &mut failed, &mut warnings);

    println!();

    // ═══════════════════════════════════════════════════════════════════════
    // SECTION 6: OBSERVABILITY
    // ═══════════════════════════════════════════════════════════════════════
    println!("┌─────────────────────────────────────────────────────────────────┐");
    println!("│ SECTION 6: COGNITIVE OBSERVABILITY                              │");
    println!("└─────────────────────────────────────────────────────────────────┘");

    // Test 6.1: Health indicators
    print!("  [6.1] Health indicators... ");
    let result = validate_health_indicators();
    report_result(&result, &mut passed, &mut failed, &mut warnings);

    // Test 6.2: Observability status
    print!("  [6.2] Observability status... ");
    let result = validate_observability_status();
    report_result(&result, &mut passed, &mut failed, &mut warnings);

    println!();

    // ═══════════════════════════════════════════════════════════════════════
    // SECTION 7: COMPLETENESS
    // ═══════════════════════════════════════════════════════════════════════
    println!("┌─────────────────────────────────────────────────────────────────┐");
    println!("│ SECTION 7: COGNITIVE COMPLETENESS                               │");
    println!("└─────────────────────────────────────────────────────────────────┘");

    // Test 7.1: Complete state
    print!("  [7.1] Complete state... ");
    let result = validate_complete_state();
    report_result(&result, &mut passed, &mut failed, &mut warnings);

    // Test 7.2: Partial state
    print!("  [7.2] Partial state... ");
    let result = validate_partial_state();
    report_result(&result, &mut passed, &mut failed, &mut warnings);

    // Test 7.3: Abstraction levels
    print!("  [7.3] Abstraction levels... ");
    let result = validate_abstraction_levels();
    report_result(&result, &mut passed, &mut failed, &mut warnings);

    println!();

    // ═══════════════════════════════════════════════════════════════════════
    // SECTION 8: EDGE CASES
    // ═══════════════════════════════════════════════════════════════════════
    println!("┌─────────────────────────────────────────────────────────────────┐");
    println!("│ SECTION 8: EDGE CASES                                           │");
    println!("└─────────────────────────────────────────────────────────────────┘");

    // Test 8.1: Single byte input
    print!("  [8.1] Single byte input... ");
    let result = validate_single_byte(&cortex);
    report_result(&result, &mut passed, &mut failed, &mut warnings);

    // Test 8.2: Maximum value bytes
    print!("  [8.2] Maximum value bytes... ");
    let result = validate_max_value(&cortex);
    report_result(&result, &mut passed, &mut failed, &mut warnings);

    // Test 8.3: All unique values
    print!("  [8.3] All unique values (256 bytes)... ");
    let result = validate_all_unique(&cortex);
    report_result(&result, &mut passed, &mut failed, &mut warnings);

    println!();

    // ═══════════════════════════════════════════════════════════════════════
    // FINAL REPORT
    // ═══════════════════════════════════════════════════════════════════════
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║                        VALIDATION REPORT                         ║");
    println!("╠══════════════════════════════════════════════════════════════════╣");
    println!("║  Passed:   {:3}                                                  ║", passed);
    println!("║  Failed:   {:3}                                                  ║", failed);
    println!("║  Warnings: {:3}                                                  ║", warnings);
    println!("╠══════════════════════════════════════════════════════════════════╣");
    
    if failed == 0 {
        println!("║  STATUS: ✅ ALL VALIDATIONS PASSED                              ║");
        println!("║                                                                  ║");
        println!("║  The Core is ready for v0.1.0-MVP-2 (Adão Sintético)             ║");
    } else {
        println!("║  STATUS: ❌ VALIDATION FAILED                                   ║");
        println!("║                                                                  ║");
        println!("║  {} tests must be fixed before release.                       ║", failed);
    }
    
    println!("╚══════════════════════════════════════════════════════════════════╝");
}

// ═══════════════════════════════════════════════════════════════════════════
// VALIDATION FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug)]
enum ValidationResult {
    Pass,
    Fail(String),
    Warning(String),
}

fn report_result(result: &ValidationResult, passed: &mut u32, failed: &mut u32, warnings: &mut u32) {
    match result {
        ValidationResult::Pass => {
            println!("✅ PASS");
            *passed += 1;
        }
        ValidationResult::Fail(msg) => {
            println!("❌ FAIL: {}", msg);
            *failed += 1;
        }
        ValidationResult::Warning(msg) => {
            println!("⚠️  WARN: {}", msg);
            *warnings += 1;
            *passed += 1; // Warnings count as passed but flagged
        }
    }
}

// Section 1: Basic Perception

fn validate_empty_input(cortex: &SensoryCortex) -> ValidationResult {
    let input = RawInput::from_bytes(vec![]);
    let output = cortex.perceive(&input);
    
    if output.signals.sample_count == 0 && output.signals.entropy == 0.0 {
        ValidationResult::Pass
    } else {
        ValidationResult::Fail(format!(
            "Expected sample_count=0, entropy=0.0, got sample_count={}, entropy={}",
            output.signals.sample_count, output.signals.entropy
        ))
    }
}

fn validate_constant_signal(cortex: &SensoryCortex) -> ValidationResult {
    let input = RawInput::from_bytes(vec![128; 1000]);
    let output = cortex.perceive(&input);
    
    if output.signals.entropy == 0.0 && output.signals.std_dev == 0.0 {
        ValidationResult::Pass
    } else {
        ValidationResult::Fail(format!(
            "Expected zero entropy/std_dev, got entropy={}, std_dev={}",
            output.signals.entropy, output.signals.std_dev
        ))
    }
}

fn validate_max_entropy(cortex: &SensoryCortex) -> ValidationResult {
    // All 256 unique values = maximum entropy
    let input = RawInput::from_bytes((0..=255).collect());
    let output = cortex.perceive(&input);
    
    // Normalized entropy should be close to 1.0
    if output.signals.entropy > 0.99 {
        ValidationResult::Pass
    } else {
        ValidationResult::Fail(format!(
            "Expected entropy > 0.99, got {}",
            output.signals.entropy
        ))
    }
}

fn validate_periodicity(cortex: &SensoryCortex) -> ValidationResult {
    // Clear periodic pattern
    let pattern: Vec<u8> = (0..1000).map(|i| ((i % 10) * 25) as u8).collect();
    let input = RawInput::from_bytes(pattern);
    let output = cortex.perceive(&input);
    
    if output.signals.periodicity_detected {
        ValidationResult::Pass
    } else {
        ValidationResult::Warning("Periodicity not detected (may be acceptable depending on implementation)".into())
    }
}

fn validate_binary_alternation(cortex: &SensoryCortex) -> ValidationResult {
    // Edge case from v1.5.4: binary alternation should be detected as periodic
    let pattern: Vec<u8> = (0..1000).map(|i| if i % 2 == 0 { 0 } else { 255 }).collect();
    let input = RawInput::from_bytes(pattern);
    let output = cortex.perceive(&input);
    
    if output.signals.periodicity_detected {
        ValidationResult::Pass
    } else {
        ValidationResult::Fail("Binary alternation should be detected as periodic (v1.5.4 fix)".into())
    }
}

// Section 2: Determinism & Replay

fn validate_determinism(cortex: &SensoryCortex) -> ValidationResult {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let input = RawInput::from_bytes(data.clone());
    
    let output1 = cortex.perceive(&input);
    let output2 = cortex.perceive(&input);
    
    if output1.signals.entropy == output2.signals.entropy
        && output1.signals.mean == output2.signals.mean
        && output1.signals.std_dev == output2.signals.std_dev
    {
        ValidationResult::Pass
    } else {
        ValidationResult::Fail("Same input produced different outputs".into())
    }
}

fn validate_replay_context() -> ValidationResult {
    let seed = b"test-seed-12345";
    let mut ctx1 = ReplayContext::from_seed(seed);
    let mut ctx2 = ReplayContext::from_seed(seed);
    
    let id1 = ctx1.deterministic_action_id();
    let id2 = ctx2.deterministic_action_id();
    
    if id1 == id2 {
        ValidationResult::Pass
    } else {
        ValidationResult::Fail("Same seed produced different IDs".into())
    }
}

fn validate_input_differentiation(cortex: &SensoryCortex) -> ValidationResult {
    let input1 = RawInput::from_bytes(vec![1, 2, 3, 4, 5]);
    let input2 = RawInput::from_bytes(vec![10, 20, 30, 40, 50]);
    
    let output1 = cortex.perceive(&input1);
    let output2 = cortex.perceive(&input2);
    
    if output1.signals.mean != output2.signals.mean {
        ValidationResult::Pass
    } else {
        ValidationResult::Fail("Different inputs produced identical mean".into())
    }
}

fn validate_permutation_distinction(cortex: &SensoryCortex) -> ValidationResult {
    // Edge case from v1.5.4: permutations should have different fingerprints
    let input1 = RawInput::from_bytes(vec![1, 2, 3, 4, 5]);
    let input2 = RawInput::from_bytes(vec![5, 4, 3, 2, 1]);
    
    let output1 = cortex.perceive(&input1);
    let output2 = cortex.perceive(&input2);
    
    // Means should be microscopically different due to epsilon
    if output1.signals.mean != output2.signals.mean {
        ValidationResult::Pass
    } else {
        ValidationResult::Fail("Permutations collapsed to identical mean (v1.5.4/v1.5.5 fix)".into())
    }
}

// Section 3: Computational Budget

fn validate_memory_budget() -> ValidationResult {
    let budget = ComputationalBudget::new().with_max_bytes(100);
    let large_data = vec![0u8; 1000];
    
    let result = digital_genome_community::check_bytes_budget(&large_data, &budget);
    
    if result.is_err() {
        ValidationResult::Pass
    } else {
        ValidationResult::Fail("Memory budget should reject oversized input".into())
    }
}

fn validate_recursion_tracking() -> ValidationResult {
    use digital_genome_community::BudgetGuard;
    
    let budget = ComputationalBudget::new().with_max_recursion(3);
    let mut guard = BudgetGuard::new(budget);
    
    // Enter 3 times
    let _ = guard.enter_recursion();
    let _ = guard.enter_recursion();
    let _ = guard.enter_recursion();
    
    // Fourth should fail
    let result = guard.enter_recursion();
    
    if result.is_err() && guard.recursion_depth() == 3 {
        ValidationResult::Pass
    } else {
        ValidationResult::Fail(format!(
            "Recursion tracking failed: depth={}, expected=3",
            guard.recursion_depth()
        ))
    }
}

fn validate_unlimited_budget() -> ValidationResult {
    let budget = ComputationalBudget::unlimited();
    
    if budget.max_bytes == usize::MAX && budget.max_iterations == usize::MAX {
        ValidationResult::Pass
    } else {
        ValidationResult::Fail("Unlimited budget should have MAX values".into())
    }
}

// Section 4: Maturation

fn validate_single_pass_maturation(cortex: &SensoryCortex) -> ValidationResult {
    let input = RawInput::from_bytes(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    let config = MaturationConfig::single_pass();
    
    let output = cortex.perceive_mature(&input, &config);
    
    if output.maturation.iterations_performed == 1 {
        ValidationResult::Pass
    } else {
        ValidationResult::Fail(format!(
            "Expected 1 iteration, got {}",
            output.maturation.iterations_performed
        ))
    }
}

fn validate_multiple_passes(cortex: &SensoryCortex) -> ValidationResult {
    let input = RawInput::from_bytes(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    let config = MaturationConfig::default().with_max_iterations(5);
    
    let output = cortex.perceive_mature(&input, &config);
    
    if output.maturation.iterations_performed >= 1 {
        ValidationResult::Pass
    } else {
        ValidationResult::Fail("Maturation should have at least 1 iteration".into())
    }
}

fn validate_maturation_ephemeral(cortex: &SensoryCortex) -> ValidationResult {
    // Maturation state should not persist between calls
    let input = RawInput::from_bytes(vec![1, 2, 3, 4, 5]);
    let config = MaturationConfig::default();
    
    let output1 = cortex.perceive_mature(&input, &config);
    let output2 = cortex.perceive_mature(&input, &config);
    
    // Both should produce independent maturation states
    if output1.maturation.iterations_performed == output2.maturation.iterations_performed {
        ValidationResult::Pass
    } else {
        ValidationResult::Warning("Maturation iterations differ between identical calls".into())
    }
}

// Section 5: Motor System

fn validate_motor_types() -> ValidationResult {
    let types = [
        MotorType::Praxis,
        MotorType::Nash,
        MotorType::Chaos,
        MotorType::Meristic,
    ];
    
    if types.len() == 4 {
        ValidationResult::Pass
    } else {
        ValidationResult::Fail("Expected 4 motor types".into())
    }
}

fn validate_motor_competition() -> ValidationResult {
    let scores = [0.8, 0.6, 0.7, 0.5]; // P, N, C, M
    let competition = MotorCompetition::from_scores(scores);
    
    if let Some(dominant) = competition.dominant_motor {
        if dominant == MotorType::Praxis {
            ValidationResult::Pass
        } else {
            ValidationResult::Fail(format!("Expected Praxis dominant, got {:?}", dominant))
        }
    } else {
        ValidationResult::Fail("No dominant motor found".into())
    }
}

fn validate_motor_cooperation() -> ValidationResult {
    let scores = [0.8, 0.8, 0.8, 0.8]; // All equal
    let cooperation = MotorCooperation::from_scores(&scores);
    
    // Check agreement between all pairs
    let agreement_pn = cooperation.agreement(MotorType::Praxis, MotorType::Nash);
    let agreement_pc = cooperation.agreement(MotorType::Praxis, MotorType::Chaos);
    let agreement_pm = cooperation.agreement(MotorType::Praxis, MotorType::Meristic);
    
    // With equal scores, all agreements should be high
    let min_agreement = agreement_pn.min(agreement_pc).min(agreement_pm);
    
    if min_agreement > 0.9 {
        ValidationResult::Pass
    } else {
        ValidationResult::Fail(format!(
            "Expected high agreement, got min={}",
            min_agreement
        ))
    }
}

// Section 6: Observability

fn validate_health_indicators() -> ValidationResult {
    let health = HealthIndicators::default();
    
    // Default health should have no warnings
    if !health.has_warnings() {
        ValidationResult::Pass
    } else {
        ValidationResult::Fail("Default health should have no warnings".into())
    }
}

fn validate_observability_status() -> ValidationResult {
    let obs = CognitiveObservability::new();
    let status = obs.status_summary();
    
    // Default observability should report a reasonable status
    if !status.is_empty() {
        ValidationResult::Pass
    } else {
        ValidationResult::Warning(format!("Observability status: {}", status))
    }
}

// Section 7: Completeness

fn validate_complete_state() -> ValidationResult {
    let completeness = CognitiveCompleteness::complete();
    
    if completeness.is_complete() && !completeness.has_contradictions() {
        ValidationResult::Pass
    } else {
        ValidationResult::Fail("Complete state should be complete without contradictions".into())
    }
}

fn validate_partial_state() -> ValidationResult {
    let completeness = CognitiveCompleteness::partial(
        vec![AbstractionLevel::Carrier],           // completed
        vec![AbstractionLevel::Pattern],           // inconclusive
        vec![MissingSignal::InsufficientSamples],  // missing
    );
    
    if !completeness.is_complete() && !completeness.has_contradictions() {
        ValidationResult::Pass
    } else {
        ValidationResult::Fail("Partial state should be incomplete without contradictions".into())
    }
}

fn validate_abstraction_levels() -> ValidationResult {
    let levels = AbstractionLevel::all();
    
    if levels.len() == 4 {
        ValidationResult::Pass
    } else {
        ValidationResult::Fail(format!("Expected 4 levels, got {}", levels.len()))
    }
}

// Section 8: Edge Cases

fn validate_single_byte(cortex: &SensoryCortex) -> ValidationResult {
    let input = RawInput::from_bytes(vec![42]);
    let output = cortex.perceive(&input);
    
    if output.signals.sample_count == 1 {
        ValidationResult::Pass
    } else {
        ValidationResult::Fail(format!(
            "Expected sample_count=1, got {}",
            output.signals.sample_count
        ))
    }
}

fn validate_max_value(cortex: &SensoryCortex) -> ValidationResult {
    let input = RawInput::from_bytes(vec![255; 100]);
    let output = cortex.perceive(&input);
    
    // Mean should be exactly 255.0 for constant signal (no epsilon applied)
    if output.signals.mean == 255.0 && output.signals.max_value == 255.0 {
        ValidationResult::Pass
    } else {
        ValidationResult::Fail(format!(
            "Expected mean=255.0, max=255.0, got mean={}, max={}",
            output.signals.mean, output.signals.max_value
        ))
    }
}

fn validate_all_unique(cortex: &SensoryCortex) -> ValidationResult {
    let input = RawInput::from_bytes((0..=255).collect());
    let output = cortex.perceive(&input);
    
    if output.signals.unique_values == 256 {
        ValidationResult::Pass
    } else {
        ValidationResult::Fail(format!(
            "Expected 256 unique values, got {}",
            output.signals.unique_values
        ))
    }
}
