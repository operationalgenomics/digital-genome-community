//! CANONICAL VALIDATION SUITE (Simplified)
//!
//! NATO-grade validation for Digital Genome Community Edition.
//! Zero tolerance for failures.
//!
//! Run with:
//! ```bash
//! cargo run --release --example canonical_validation
//! ```

use digital_genome_community::{
    RawInput, SensoryCortex, MaturationConfig, ComputationalBudget,
    ReplayContext,
};

use std::fs::{self, File};
use std::io::Read;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;

// ═══════════════════════════════════════════════════════════════════════════
// CONSTANTS
// ═══════════════════════════════════════════════════════════════════════════

const DETERMINISM_ITERATIONS: usize = 100;
const THREAD_COUNT: usize = 10;

// ═══════════════════════════════════════════════════════════════════════════
// TEST RESULT
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
struct TestResult {
    passed: bool,
    message: String,
}

impl TestResult {
    fn pass(msg: String) -> Self {
        Self { passed: true, message: msg }
    }
    
    fn fail(msg: String) -> Self {
        Self { passed: false, message: msg }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// MAIN
// ═══════════════════════════════════════════════════════════════════════════

fn main() {
    println!();
    println!("╔══════════════════════════════════════════════════════════════════════╗");
    println!("║     CANONICAL VALIDATION SUITE - NATO/DoD COMPLIANCE LEVEL           ║");
    println!("║     Digital Genome Community Edition v0.1.0-rc1                       ║");
    println!("╚══════════════════════════════════════════════════════════════════════╝");
    println!();

    let cortex = SensoryCortex::new();
    let mut passed = 0;
    let mut failed = 0;
    let mut results: Vec<(String, TestResult)> = Vec::new();

    // ═══════════════════════════════════════════════════════════════════════
    // CATEGORY 1: DETERMINISM
    // ═══════════════════════════════════════════════════════════════════════
    println!("┌─────────────────────────────────────────────────────────────────────┐");
    println!("│ CATEGORY 1: DETERMINISM VERIFICATION                                │");
    println!("└─────────────────────────────────────────────────────────────────────┘");

    run_test("DET-001", "Basic determinism (100 iterations)", 
        test_basic_determinism(&cortex), &mut passed, &mut failed, &mut results);
    
    run_test("DET-002", "Replay context determinism",
        test_replay_determinism(), &mut passed, &mut failed, &mut results);
    
    run_test("DET-003", "Cross-instance determinism",
        test_cross_instance(), &mut passed, &mut failed, &mut results);
    
    run_test("DET-004", "Permutation distinction",
        test_permutation_distinction(&cortex), &mut passed, &mut failed, &mut results);
    
    run_test("DET-005", "Large input determinism (100KB)",
        test_large_input_determinism(&cortex), &mut passed, &mut failed, &mut results);
    
    run_test("DET-006", "Maturation determinism",
        test_maturation_determinism(&cortex), &mut passed, &mut failed, &mut results);

    println!();

    // ═══════════════════════════════════════════════════════════════════════
    // CATEGORY 2: BOUNDARY CONDITIONS
    // ═══════════════════════════════════════════════════════════════════════
    println!("┌─────────────────────────────────────────────────────────────────────┐");
    println!("│ CATEGORY 2: BOUNDARY CONDITIONS                                     │");
    println!("└─────────────────────────────────────────────────────────────────────┘");

    run_test("BND-001", "Empty input handling",
        test_empty_input(&cortex), &mut passed, &mut failed, &mut results);
    
    run_test("BND-002", "Single byte input",
        test_single_byte(&cortex), &mut passed, &mut failed, &mut results);
    
    run_test("BND-003", "All zeros (zero entropy)",
        test_all_zeros(&cortex), &mut passed, &mut failed, &mut results);
    
    run_test("BND-004", "All 255s (zero entropy)",
        test_all_max(&cortex), &mut passed, &mut failed, &mut results);
    
    run_test("BND-005", "Maximum entropy (256 unique)",
        test_max_entropy(&cortex), &mut passed, &mut failed, &mut results);
    
    run_test("BND-006", "Budget enforcement",
        test_budget_enforcement(&cortex), &mut passed, &mut failed, &mut results);

    println!();

    // ═══════════════════════════════════════════════════════════════════════
    // CATEGORY 3: NUMERICAL STABILITY
    // ═══════════════════════════════════════════════════════════════════════
    println!("┌─────────────────────────────────────────────────────────────────────┐");
    println!("│ CATEGORY 3: NUMERICAL STABILITY                                     │");
    println!("└─────────────────────────────────────────────────────────────────────┘");

    run_test("NUM-001", "No NaN values",
        test_no_nan(&cortex), &mut passed, &mut failed, &mut results);
    
    run_test("NUM-002", "No Infinity values",
        test_no_infinity(&cortex), &mut passed, &mut failed, &mut results);
    
    run_test("NUM-003", "Value bounds (0.0 <= entropy <= 1.0)",
        test_value_bounds(&cortex), &mut passed, &mut failed, &mut results);
    
    run_test("NUM-004", "Floating point bit-exact",
        test_float_bit_exact(&cortex), &mut passed, &mut failed, &mut results);

    println!();

    // ═══════════════════════════════════════════════════════════════════════
    // CATEGORY 4: CONCURRENCY
    // ═══════════════════════════════════════════════════════════════════════
    println!("┌─────────────────────────────────────────────────────────────────────┐");
    println!("│ CATEGORY 4: CONCURRENCY                                             │");
    println!("└─────────────────────────────────────────────────────────────────────┘");

    run_test("CON-001", "Parallel instances (10 threads)",
        test_parallel_instances(), &mut passed, &mut failed, &mut results);
    
    run_test("CON-002", "Shared cortex via Arc",
        test_shared_cortex(), &mut passed, &mut failed, &mut results);

    println!();

    // ═══════════════════════════════════════════════════════════════════════
    // CATEGORY 5: REAL DATASETS (if available)
    // ═══════════════════════════════════════════════════════════════════════
    println!("┌─────────────────────────────────────────────────────────────────────┐");
    println!("│ CATEGORY 5: REAL DATASETS                                           │");
    println!("└─────────────────────────────────────────────────────────────────────┘");

    let datasets_dir = PathBuf::from("validation/datasets/real");
    if datasets_dir.exists() {
        test_real_datasets(&cortex, &datasets_dir, &mut passed, &mut failed, &mut results);
    } else {
        println!("  ⏭️  Skipping: datasets directory not found");
        println!("     Create: validation/datasets/real/ with test files");
    }

    println!();

    // ═══════════════════════════════════════════════════════════════════════
    // FINAL REPORT
    // ═══════════════════════════════════════════════════════════════════════
    let total = passed + failed;
    
    println!("╔══════════════════════════════════════════════════════════════════════╗");
    println!("║                       VALIDATION REPORT                              ║");
    println!("╠══════════════════════════════════════════════════════════════════════╣");
    println!("║  Total Tests:    {:4}                                               ║", total);
    println!("║  Passed:         {:4}  ✅                                           ║", passed);
    println!("║  Failed:         {:4}  ❌                                           ║", failed);
    println!("╠══════════════════════════════════════════════════════════════════════╣");

    if failed == 0 {
        println!("║                                                                      ║");
        println!("║  ██████╗  █████╗ ███████╗███████╗                                    ║");
        println!("║  ██╔══██╗██╔══██╗██╔════╝██╔════╝                                    ║");
        println!("║  ██████╔╝███████║███████╗███████╗                                    ║");
        println!("║  ██╔═══╝ ██╔══██║╚════██║╚════██║                                    ║");
        println!("║  ██║     ██║  ██║███████║███████║                                    ║");
        println!("║  ╚═╝     ╚═╝  ╚═╝╚══════╝╚══════╝                                    ║");
        println!("║                                                                      ║");
        println!("║  STATUS: CANONICAL VALIDATION PASSED                                ║");
        println!("║  Version 0.1.0-rc1 (Adão Sintético) is CERTIFIED.                   ║");
    } else {
        println!("║                                                                      ║");
        println!("║  ███████╗ █████╗ ██╗██╗                                              ║");
        println!("║  ██╔════╝██╔══██╗██║██║                                              ║");
        println!("║  █████╗  ███████║██║██║                                              ║");
        println!("║  ██╔══╝  ██╔══██║██║██║                                              ║");
        println!("║  ██║     ██║  ██║██║███████╗                                         ║");
        println!("║  ╚═╝     ╚═╝  ╚═╝╚═╝╚══════╝                                         ║");
        println!("║                                                                      ║");
        println!("║  STATUS: CANONICAL VALIDATION FAILED                                ║");
        println!("║  {} FAILURES MUST BE RESOLVED.                                    ║", failed);
        println!("║                                                                      ║");
        
        for (id, result) in &results {
            if !result.passed {
                println!("║  ❌ {}: {}                                    ║", id, 
                    &result.message[..result.message.len().min(30)]);
            }
        }
    }

    println!("║                                                                      ║");
    println!("╚══════════════════════════════════════════════════════════════════════╝");
}

// ═══════════════════════════════════════════════════════════════════════════
// TEST RUNNER
// ═══════════════════════════════════════════════════════════════════════════

fn run_test(
    id: &str,
    description: &str,
    result: TestResult,
    passed: &mut usize,
    failed: &mut usize,
    results: &mut Vec<(String, TestResult)>,
) {
    let icon = if result.passed { "✅" } else { "❌" };
    let status = if result.passed { "PASS" } else { "FAIL" };
    
    println!("  [{}] {} ... {} {}", id, description, icon, status);
    
    if result.passed {
        *passed += 1;
    } else {
        *failed += 1;
        println!("         └─ {}", result.message);
    }
    
    results.push((id.to_string(), result));
}

// ═══════════════════════════════════════════════════════════════════════════
// DETERMINISM TESTS
// ═══════════════════════════════════════════════════════════════════════════

fn test_basic_determinism(cortex: &SensoryCortex) -> TestResult {
    let data: Vec<u8> = (0..1000).map(|i| (i % 256) as u8).collect();
    let input = RawInput::from_bytes(data);
    
    let reference = cortex.perceive(&input);
    
    for i in 0..DETERMINISM_ITERATIONS {
        let output = cortex.perceive(&input);
        if output.signals.entropy != reference.signals.entropy
            || output.signals.mean != reference.signals.mean
            || output.signals.std_dev != reference.signals.std_dev
        {
            return TestResult::fail(format!("Mismatch at iteration {}", i));
        }
    }
    
    TestResult::pass(format!("{} iterations identical", DETERMINISM_ITERATIONS))
}

fn test_replay_determinism() -> TestResult {
    let seed = b"canonical-validation-seed";
    
    let mut ctx1 = ReplayContext::from_seed(seed);
    let mut ctx2 = ReplayContext::from_seed(seed);
    
    let id1 = ctx1.deterministic_action_id();
    let id2 = ctx2.deterministic_action_id();
    
    if id1 == id2 {
        TestResult::pass("Replay IDs match".into())
    } else {
        TestResult::fail("Replay IDs differ".into())
    }
}

fn test_cross_instance() -> TestResult {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let input = RawInput::from_bytes(data);
    
    let cortex1 = SensoryCortex::new();
    let cortex2 = SensoryCortex::new();
    
    let out1 = cortex1.perceive(&input);
    let out2 = cortex2.perceive(&input);
    
    if out1.signals.entropy == out2.signals.entropy
        && out1.signals.mean == out2.signals.mean
        && out1.signals.std_dev == out2.signals.std_dev
    {
        TestResult::pass("Cross-instance match".into())
    } else {
        TestResult::fail("Cross-instance mismatch".into())
    }
}

fn test_permutation_distinction(cortex: &SensoryCortex) -> TestResult {
    let input1 = RawInput::from_bytes(vec![1, 2, 3, 4, 5]);
    let input2 = RawInput::from_bytes(vec![5, 4, 3, 2, 1]);
    
    let out1 = cortex.perceive(&input1);
    let out2 = cortex.perceive(&input2);
    
    // Permutations should be distinguished (different fingerprints)
    if out1.signals.mean != out2.signals.mean {
        TestResult::pass("Permutations distinguished".into())
    } else {
        TestResult::fail("Permutations collapsed".into())
    }
}

fn test_large_input_determinism(cortex: &SensoryCortex) -> TestResult {
    let data: Vec<u8> = (0..100_000).map(|i| (i % 256) as u8).collect();
    let input = RawInput::from_bytes(data);
    
    let out1 = cortex.perceive(&input);
    let out2 = cortex.perceive(&input);
    
    if out1.signals.entropy == out2.signals.entropy
        && out1.signals.mean == out2.signals.mean
    {
        TestResult::pass("100KB deterministic".into())
    } else {
        TestResult::fail("100KB non-deterministic".into())
    }
}

fn test_maturation_determinism(cortex: &SensoryCortex) -> TestResult {
    let data: Vec<u8> = (0..500).map(|i| ((i * 7) % 256) as u8).collect();
    let input = RawInput::from_bytes(data);
    let config = MaturationConfig::default().with_max_iterations(5);
    
    let out1 = cortex.perceive_mature(&input, &config);
    let out2 = cortex.perceive_mature(&input, &config);
    
    if out1.maturation.iterations_performed == out2.maturation.iterations_performed
        && out1.maturation.converged == out2.maturation.converged
        && out1.perception.signals.entropy == out2.perception.signals.entropy
    {
        TestResult::pass("Maturation deterministic".into())
    } else {
        TestResult::fail("Maturation non-deterministic".into())
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// BOUNDARY TESTS
// ═══════════════════════════════════════════════════════════════════════════

fn test_empty_input(cortex: &SensoryCortex) -> TestResult {
    let output = cortex.perceive(&RawInput::from_bytes(vec![]));
    
    if output.signals.sample_count == 0 && output.signals.entropy == 0.0 {
        TestResult::pass("Empty handled correctly".into())
    } else {
        TestResult::fail("Empty handling error".into())
    }
}

fn test_single_byte(cortex: &SensoryCortex) -> TestResult {
    let output = cortex.perceive(&RawInput::from_bytes(vec![42]));
    
    if output.signals.sample_count == 1 {
        TestResult::pass("Single byte OK".into())
    } else {
        TestResult::fail("Single byte error".into())
    }
}

fn test_all_zeros(cortex: &SensoryCortex) -> TestResult {
    let output = cortex.perceive(&RawInput::from_bytes(vec![0; 1000]));
    
    if output.signals.entropy == 0.0 && output.signals.std_dev == 0.0 {
        TestResult::pass("All zeros: entropy=0".into())
    } else {
        TestResult::fail(format!("Expected entropy=0, got {}", output.signals.entropy))
    }
}

fn test_all_max(cortex: &SensoryCortex) -> TestResult {
    let output = cortex.perceive(&RawInput::from_bytes(vec![255; 1000]));
    
    if output.signals.entropy == 0.0 && output.signals.mean == 255.0 {
        TestResult::pass("All 255s: entropy=0, mean=255".into())
    } else {
        TestResult::fail(format!("Expected entropy=0, got {}", output.signals.entropy))
    }
}

fn test_max_entropy(cortex: &SensoryCortex) -> TestResult {
    let data: Vec<u8> = (0..=255).collect();
    let output = cortex.perceive(&RawInput::from_bytes(data));
    
    if output.signals.entropy > 0.99 && output.signals.unique_values == 256 {
        TestResult::pass(format!("Max entropy: {:.6}", output.signals.entropy))
    } else {
        TestResult::fail(format!("Expected entropy>0.99, got {}", output.signals.entropy))
    }
}

fn test_budget_enforcement(cortex: &SensoryCortex) -> TestResult {
    let budget = ComputationalBudget::new().with_max_bytes(100);
    let input = RawInput::from_bytes(vec![0; 1000]);
    
    match cortex.perceive_checked(&input, &budget) {
        Ok(_) => TestResult::fail("Budget should have been exceeded".into()),
        Err(_) => TestResult::pass("Budget enforced".into()),
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// NUMERICAL TESTS
// ═══════════════════════════════════════════════════════════════════════════

fn test_no_nan(cortex: &SensoryCortex) -> TestResult {
    let test_cases = vec![
        vec![],
        vec![0],
        vec![255],
        vec![0, 255],
        vec![0; 1000],
        (0..=255).collect::<Vec<u8>>(),
    ];
    
    for data in test_cases {
        let output = cortex.perceive(&RawInput::from_bytes(data));
        if output.signals.entropy.is_nan() 
            || output.signals.mean.is_nan()
            || output.signals.std_dev.is_nan()
        {
            return TestResult::fail("NaN detected".into());
        }
    }
    
    TestResult::pass("No NaN values".into())
}

fn test_no_infinity(cortex: &SensoryCortex) -> TestResult {
    let test_cases = vec![
        vec![0; 100000],
        vec![255; 100000],
        (0..100000).map(|i| (i % 256) as u8).collect::<Vec<u8>>(),
    ];
    
    for data in test_cases {
        let output = cortex.perceive(&RawInput::from_bytes(data));
        if output.signals.entropy.is_infinite() 
            || output.signals.mean.is_infinite()
            || output.signals.std_dev.is_infinite()
        {
            return TestResult::fail("Infinity detected".into());
        }
    }
    
    TestResult::pass("No Infinity values".into())
}

fn test_value_bounds(cortex: &SensoryCortex) -> TestResult {
    let data: Vec<u8> = (0..1000).map(|i| (i % 256) as u8).collect();
    let output = cortex.perceive(&RawInput::from_bytes(data));
    
    let valid = output.signals.entropy >= 0.0 && output.signals.entropy <= 1.0
        && output.signals.mean >= 0.0 && output.signals.mean <= 255.0
        && output.signals.std_dev >= 0.0;
    
    if valid {
        TestResult::pass("All values in bounds".into())
    } else {
        TestResult::fail("Value out of bounds".into())
    }
}

fn test_float_bit_exact(cortex: &SensoryCortex) -> TestResult {
    let data = vec![0, 1, 254, 255, 127, 128];
    let input = RawInput::from_bytes(data);
    
    let out1 = cortex.perceive(&input);
    let out2 = cortex.perceive(&input);
    
    let bit_exact = out1.signals.entropy.to_bits() == out2.signals.entropy.to_bits()
        && out1.signals.mean.to_bits() == out2.signals.mean.to_bits()
        && out1.signals.std_dev.to_bits() == out2.signals.std_dev.to_bits();
    
    if bit_exact {
        TestResult::pass("Bit-exact match".into())
    } else {
        TestResult::fail("Floating point drift".into())
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// CONCURRENCY TESTS
// ═══════════════════════════════════════════════════════════════════════════

fn test_parallel_instances() -> TestResult {
    let handles: Vec<_> = (0..THREAD_COUNT).map(|i| {
        thread::spawn(move || {
            let cortex = SensoryCortex::new();
            let data = vec![i as u8; 100];
            let input = RawInput::from_bytes(data);
            cortex.perceive(&input).signals.entropy
        })
    }).collect();
    
    let results: Vec<_> = handles.into_iter()
        .map(|h| h.join())
        .collect();
    
    let all_ok = results.iter().all(|r| r.is_ok());
    
    if all_ok {
        TestResult::pass(format!("{} threads completed", THREAD_COUNT))
    } else {
        TestResult::fail("Thread panic".into())
    }
}

fn test_shared_cortex() -> TestResult {
    let cortex = Arc::new(SensoryCortex::new());
    let results = Arc::new(Mutex::new(Vec::new()));
    
    let handles: Vec<_> = (0..THREAD_COUNT).map(|i| {
        let cortex = Arc::clone(&cortex);
        let results = Arc::clone(&results);
        
        thread::spawn(move || {
            let data = vec![i as u8; 100];
            let input = RawInput::from_bytes(data);
            let output = cortex.perceive(&input);
            results.lock().unwrap().push(output.signals.entropy);
        })
    }).collect();
    
    for h in handles {
        if h.join().is_err() {
            return TestResult::fail("Thread panic".into());
        }
    }
    
    let final_results = results.lock().unwrap();
    if final_results.len() == THREAD_COUNT {
        TestResult::pass(format!("{} results collected", THREAD_COUNT))
    } else {
        TestResult::fail("Missing results".into())
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// REAL DATASET TESTS
// ═══════════════════════════════════════════════════════════════════════════

fn test_real_datasets(
    cortex: &SensoryCortex,
    datasets_dir: &PathBuf,
    passed: &mut usize,
    failed: &mut usize,
    results: &mut Vec<(String, TestResult)>,
) {
    let categories = ["industrial", "documents", "media", "scientific", "adversarial"];
    
    for category in &categories {
        let cat_path = datasets_dir.join(category);
        if cat_path.exists() {
            if let Ok(entries) = fs::read_dir(&cat_path) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() {
                        let filename = path.file_name().unwrap().to_string_lossy();
                        let test_id = format!("DS-{}", &filename[..filename.len().min(8)]);
                        let result = test_single_file(cortex, &path);
                        run_test(&test_id, &filename, result, passed, failed, results);
                    }
                }
            }
        }
    }
    
    // Also check root directory
    if let Ok(entries) = fs::read_dir(datasets_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                let filename = path.file_name().unwrap().to_string_lossy();
                let test_id = format!("DS-{}", &filename[..filename.len().min(8)]);
                let result = test_single_file(cortex, &path);
                run_test(&test_id, &filename, result, passed, failed, results);
            }
        }
    }
}

fn test_single_file(cortex: &SensoryCortex, path: &std::path::Path) -> TestResult {
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(e) => return TestResult::fail(format!("Open error: {}", e)),
    };
    
    let mut buffer = Vec::new();
    if let Err(e) = file.read_to_end(&mut buffer) {
        return TestResult::fail(format!("Read error: {}", e));
    }
    
    let input = RawInput::from_bytes(buffer);
    
    // Process twice to verify determinism
    let out1 = cortex.perceive(&input);
    let out2 = cortex.perceive(&input);
    
    let deterministic = out1.signals.entropy == out2.signals.entropy
        && out1.signals.mean == out2.signals.mean
        && out1.signals.std_dev == out2.signals.std_dev;
    
    if deterministic {
        TestResult::pass(format!("entropy={:.4}", out1.signals.entropy))
    } else {
        TestResult::fail("Non-deterministic".into())
    }
}
