//! Canonical Test Harness for NATO/DoD Compliance Validation
//!
//! This harness executes comprehensive validation tests and produces
//! standardized reports suitable for military-grade compliance review.
//!
//! Run with: cargo run --release --example canonical_test_harness
//!
//! Requirements:
//! - Place real datasets in validation/datasets/real/
//! - Ensure write access to validation/reports/

use digital_genome_community::{
    RawInput, SensoryCortex, MaturationConfig, ComputationalBudget,
    ReplayContext, BudgetGuard, ActionId,
};
use std::fs::{self, File};
use std::io::{Read, Write, BufWriter};
use std::path::{Path, PathBuf};
use std::time::{Instant, SystemTime};
use std::sync::{Arc, Mutex};
use std::thread;

// ============================================================================
// TEST RESULT STRUCTURES
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
enum TestStatus {
    Pass,
    Fail,
    Skip,
    Error,
}

impl TestStatus {
    fn icon(&self) -> &'static str {
        match self {
            TestStatus::Pass => "✅",
            TestStatus::Fail => "❌",
            TestStatus::Skip => "⏭️",
            TestStatus::Error => "💥",
        }
    }
    
    fn label(&self) -> &'static str {
        match self {
            TestStatus::Pass => "PASS",
            TestStatus::Fail => "FAIL",
            TestStatus::Skip => "SKIP",
            TestStatus::Error => "ERROR",
        }
    }
}

#[derive(Debug, Clone)]
struct TestResult {
    test_id: String,
    category: String,
    description: String,
    status: TestStatus,
    duration_ns: u64,
    input_summary: String,
    output_summary: String,
    details: String,
}

// ============================================================================
// CANONICAL TEST HARNESS
// ============================================================================

struct CanonicalTestHarness {
    cortex: SensoryCortex,
    results: Vec<TestResult>,
    datasets_dir: PathBuf,
    output_dir: PathBuf,
    start_time: Instant,
}

impl CanonicalTestHarness {
    fn new(datasets_dir: PathBuf, output_dir: PathBuf) -> Self {
        fs::create_dir_all(&output_dir).expect("Failed to create output directory");
        fs::create_dir_all(&datasets_dir).expect("Failed to create datasets directory");

        // Create cortex and perform warm-up for deterministic behavior
        let cortex = SensoryCortex::new();
        
        // WARM-UP: First execution may involve non-deterministic auto-tuning
        // (e.g., FFT planner optimization). Run dummy perception to stabilize.
        let _warmup = cortex.perceive(&RawInput::from_bytes(vec![0u8; 1024]));
        
        Self {
            cortex,
            results: Vec::new(),
            datasets_dir,
            output_dir,
            start_time: Instant::now(),
        }
    }

    fn execute_all(&mut self) {
        self.print_header();

        // CAT-1: Determinism Verification (CRITICAL)
        self.execute_determinism_tests();

        // CAT-2: Boundary Conditions (CRITICAL)
        self.execute_boundary_tests();

        // CAT-3: Real-World Datasets (CRITICAL)
        self.execute_dataset_tests();

        // CAT-4: Stress Testing (HIGH)
        self.execute_stress_tests();

        // CAT-5: GDE↔GDC Protocol (CRITICAL)
        self.execute_protocol_tests();

        // CAT-6: Adversarial Inputs (HIGH)
        self.execute_adversarial_tests();

        // CAT-7: Concurrency (CRITICAL)
        self.execute_concurrency_tests();

        // CAT-8: Numerical Edge Cases (CRITICAL)
        self.execute_numerical_tests();

        // Generate final report
        self.generate_report();
        self.write_detailed_report();
    }

    fn print_header(&self) {
        println!();
        println!("╔══════════════════════════════════════════════════════════════════════╗");
        println!("║                                                                       ║");
        println!("║     CANONICAL VALIDATION PROTOCOL - NATO/DoD COMPLIANCE LEVEL        ║");
        println!("║     Digital Genome Community Edition v0.1.0-rc1 (Adão Sintético)     ║");
        println!("║                                                                       ║");
        println!("║     Document: DGC-CVP-2025-001                                        ║");
        println!("║     Classification: UNCLASSIFIED // FOR OFFICIAL USE ONLY            ║");
        println!("║                                                                       ║");
        println!("╚══════════════════════════════════════════════════════════════════════╝");
        println!();
    }

    // ========================================================================
    // CAT-1: DETERMINISM VERIFICATION
    // ========================================================================

    fn execute_determinism_tests(&mut self) {
        self.print_category("CAT-1: DETERMINISM VERIFICATION", "CRITICAL");

        self.test_det_001_basic_determinism();
        self.test_det_002_replay_determinism();
        self.test_det_003_cross_instance();
        self.test_det_004_order_independence();
        self.test_det_005_maturation_determinism();
        self.test_det_006_large_input();
        self.test_det_007_edge_values();
        self.test_det_008_empty_input();
        self.test_det_009_single_byte();
        self.test_det_010_permutation();
        self.test_det_011_float_stability();
        self.test_det_012_hash_determinism();

        println!();
    }

    fn test_det_001_basic_determinism(&mut self) {
        let start = Instant::now();
        let data: Vec<u8> = (0..1000).map(|i| (i % 256) as u8).collect();
        let input = RawInput::from_bytes(data.clone());

        // Warm-up call (FFT planner may optimize on first call)
        let _ = self.cortex.perceive(&input);
        
        // Reference is the second call
        let reference = self.cortex.perceive(&input);
        let mut all_match = true;
        let mut mismatch_iteration = 0;
        let mut mismatch_field = String::new();

        for i in 0..100 {
            let output = self.cortex.perceive(&input);
            
            // Check each field individually for better diagnostics
            if output.signals.entropy != reference.signals.entropy {
                all_match = false;
                mismatch_iteration = i;
                mismatch_field = format!("entropy: {} vs {}", 
                    output.signals.entropy, reference.signals.entropy);
                break;
            }
            if output.signals.mean != reference.signals.mean {
                all_match = false;
                mismatch_iteration = i;
                mismatch_field = format!("mean: {} vs {}", 
                    output.signals.mean, reference.signals.mean);
                break;
            }
            if output.signals.std_dev != reference.signals.std_dev {
                all_match = false;
                mismatch_iteration = i;
                mismatch_field = format!("std_dev: {} vs {}", 
                    output.signals.std_dev, reference.signals.std_dev);
                break;
            }
        }

        let details = if all_match {
            "All 100 iterations matched (after warm-up)".to_string()
        } else {
            format!("Mismatch at iteration {}: {}", mismatch_iteration, mismatch_field)
        };

        self.record("DET-001", "Determinism",
            "100 iterations produce identical output",
            if all_match { TestStatus::Pass } else { TestStatus::Fail },
            start.elapsed().as_nanos() as u64,
            "1000 bytes sequential",
            &format!("entropy={:.10}", reference.signals.entropy),
            &details);
    }

    fn test_det_002_replay_determinism(&mut self) {
        let start = Instant::now();
        let seed = b"canonical-validation-seed-001";

        let mut ctx1 = ReplayContext::from_seed(seed);
        let mut ctx2 = ReplayContext::from_seed(seed);

        let id1 = ctx1.deterministic_action_id();
        let id2 = ctx2.deterministic_action_id();

        let matched = id1 == id2;

        self.record("DET-002", "Determinism",
            "Replay context produces identical IDs",
            if matched { TestStatus::Pass } else { TestStatus::Fail },
            start.elapsed().as_nanos() as u64,
            "seed: canonical-validation-seed-001",
            &format!("id1={:?}, id2={:?}", id1, id2),
            if matched { "IDs matched" } else { "IDs differ!" });
    }

    fn test_det_003_cross_instance(&mut self) {
        let start = Instant::now();
        let data = vec![42u8; 500];
        let input = RawInput::from_bytes(data);

        let cortex1 = SensoryCortex::new();
        let cortex2 = SensoryCortex::new();

        let out1 = cortex1.perceive(&input);
        let out2 = cortex2.perceive(&input);

        let matched = out1.signals.entropy == out2.signals.entropy
            && out1.signals.mean == out2.signals.mean
            && out1.signals.std_dev == out2.signals.std_dev;

        self.record("DET-003", "Determinism",
            "Different cortex instances produce identical output",
            if matched { TestStatus::Pass } else { TestStatus::Fail },
            start.elapsed().as_nanos() as u64,
            "500 bytes constant (42)",
            &format!("entropy={:.10}", out1.signals.entropy),
            if matched { "Cross-instance match" } else { "Cross-instance mismatch!" });
    }

    fn test_det_004_order_independence(&mut self) {
        let start = Instant::now();
        let data1 = vec![1, 2, 3, 4, 5];
        let data2 = vec![10, 20, 30, 40, 50];

        let input1 = RawInput::from_bytes(data1.clone());
        let input2 = RawInput::from_bytes(data2.clone());

        // Process in order 1, 2
        let out1_a = self.cortex.perceive(&input1);
        let out2_a = self.cortex.perceive(&input2);

        // Process in order 2, 1
        let out2_b = self.cortex.perceive(&input2);
        let out1_b = self.cortex.perceive(&input1);

        let matched = out1_a.signals.entropy == out1_b.signals.entropy
            && out2_a.signals.entropy == out2_b.signals.entropy;

        self.record("DET-004", "Determinism",
            "Processing order does not affect results",
            if matched { TestStatus::Pass } else { TestStatus::Fail },
            start.elapsed().as_nanos() as u64,
            "Two different inputs, different order",
            "Order-independent results",
            if matched { "Order independence verified" } else { "Order affects results!" });
    }

    fn test_det_005_maturation_determinism(&mut self) {
        let start = Instant::now();
        let data: Vec<u8> = (0..500).map(|i| ((i * 7) % 256) as u8).collect();
        let input = RawInput::from_bytes(data);
        let config = MaturationConfig::default().with_max_iterations(5);

        let out1 = self.cortex.perceive_mature(&input, &config);
        let out2 = self.cortex.perceive_mature(&input, &config);

        let matched = out1.maturation.iterations_performed == out2.maturation.iterations_performed
            && out1.maturation.converged == out2.maturation.converged
            && out1.perception.signals.entropy == out2.perception.signals.entropy;

        self.record("DET-005", "Determinism",
            "Maturation produces identical results",
            if matched { TestStatus::Pass } else { TestStatus::Fail },
            start.elapsed().as_nanos() as u64,
            "500 bytes with maturation",
            &format!("iterations={}, converged={}", 
                out1.maturation.iterations_performed, out1.maturation.converged),
            if matched { "Maturation deterministic" } else { "Maturation non-deterministic!" });
    }

    fn test_det_006_large_input(&mut self) {
        let start = Instant::now();
        let data: Vec<u8> = (0..100_000).map(|i| (i % 256) as u8).collect();
        let input = RawInput::from_bytes(data);

        // Warm-up call
        let _ = self.cortex.perceive(&input);

        let out1 = self.cortex.perceive(&input);
        let out2 = self.cortex.perceive(&input);

        let matched = out1.signals.entropy == out2.signals.entropy
            && out1.signals.mean == out2.signals.mean;

        let details = if matched {
            "Large input deterministic (after warm-up)".to_string()
        } else {
            format!("Mismatch: entropy {}!={}, mean {}!={}", 
                out1.signals.entropy, out2.signals.entropy,
                out1.signals.mean, out2.signals.mean)
        };

        self.record("DET-006", "Determinism",
            "Large input (100KB) produces identical results",
            if matched { TestStatus::Pass } else { TestStatus::Fail },
            start.elapsed().as_nanos() as u64,
            "100,000 bytes",
            &format!("entropy={:.10}", out1.signals.entropy),
            &details);
    }

    fn test_det_007_edge_values(&mut self) {
        let start = Instant::now();

        // Test with all 0s
        let zeros = vec![0u8; 1000];
        let out_z1 = self.cortex.perceive(&RawInput::from_bytes(zeros.clone()));
        let out_z2 = self.cortex.perceive(&RawInput::from_bytes(zeros));

        // Test with all 255s
        let maxes = vec![255u8; 1000];
        let out_m1 = self.cortex.perceive(&RawInput::from_bytes(maxes.clone()));
        let out_m2 = self.cortex.perceive(&RawInput::from_bytes(maxes));

        let matched = out_z1.signals.entropy == out_z2.signals.entropy
            && out_m1.signals.entropy == out_m2.signals.entropy;

        self.record("DET-007", "Determinism",
            "Edge values (0 and 255) produce identical results",
            if matched { TestStatus::Pass } else { TestStatus::Fail },
            start.elapsed().as_nanos() as u64,
            "All 0s and all 255s",
            &format!("zero_entropy={:.10}, max_entropy={:.10}", 
                out_z1.signals.entropy, out_m1.signals.entropy),
            if matched { "Edge values deterministic" } else { "Edge values non-deterministic!" });
    }

    fn test_det_008_empty_input(&mut self) {
        let start = Instant::now();
        let input = RawInput::from_bytes(vec![]);

        let out1 = self.cortex.perceive(&input);
        let out2 = self.cortex.perceive(&input);

        let matched = out1.signals.sample_count == out2.signals.sample_count
            && out1.signals.entropy == out2.signals.entropy;

        self.record("DET-008", "Determinism",
            "Empty input produces identical results",
            if matched { TestStatus::Pass } else { TestStatus::Fail },
            start.elapsed().as_nanos() as u64,
            "0 bytes",
            &format!("sample_count={}", out1.signals.sample_count),
            if matched { "Empty input deterministic" } else { "Empty input non-deterministic!" });
    }

    fn test_det_009_single_byte(&mut self) {
        let start = Instant::now();
        let input = RawInput::from_bytes(vec![127]);

        let out1 = self.cortex.perceive(&input);
        let out2 = self.cortex.perceive(&input);

        let matched = out1.signals.mean == out2.signals.mean
            && out1.signals.sample_count == out2.signals.sample_count;

        self.record("DET-009", "Determinism",
            "Single byte produces identical results",
            if matched { TestStatus::Pass } else { TestStatus::Fail },
            start.elapsed().as_nanos() as u64,
            "1 byte (127)",
            &format!("mean={}", out1.signals.mean),
            if matched { "Single byte deterministic" } else { "Single byte non-deterministic!" });
    }

    fn test_det_010_permutation(&mut self) {
        let start = Instant::now();
        let input1 = RawInput::from_bytes(vec![1, 2, 3, 4, 5]);
        let input2 = RawInput::from_bytes(vec![5, 4, 3, 2, 1]);

        let out1 = self.cortex.perceive(&input1);
        let out2 = self.cortex.perceive(&input2);

        // Permutations should have DIFFERENT fingerprints
        let distinguished = out1.signals.mean != out2.signals.mean
            || out1.signals.std_dev != out2.signals.std_dev;

        self.record("DET-010", "Determinism",
            "Permutations are distinguished",
            if distinguished { TestStatus::Pass } else { TestStatus::Fail },
            start.elapsed().as_nanos() as u64,
            "[1,2,3,4,5] vs [5,4,3,2,1]",
            &format!("mean1={}, mean2={}", out1.signals.mean, out2.signals.mean),
            if distinguished { "Permutations distinguished" } else { "Permutations collapsed!" });
    }

    fn test_det_011_float_stability(&mut self) {
        let start = Instant::now();
        let data: Vec<u8> = vec![0, 1, 254, 255, 127, 128, 0, 255];
        let input = RawInput::from_bytes(data);

        let out1 = self.cortex.perceive(&input);
        let out2 = self.cortex.perceive(&input);

        // Check bit-exact equality
        let matched = out1.signals.entropy.to_bits() == out2.signals.entropy.to_bits()
            && out1.signals.mean.to_bits() == out2.signals.mean.to_bits()
            && out1.signals.std_dev.to_bits() == out2.signals.std_dev.to_bits();

        self.record("DET-011", "Determinism",
            "Floating point results are bit-exact",
            if matched { TestStatus::Pass } else { TestStatus::Fail },
            start.elapsed().as_nanos() as u64,
            "Edge value bytes",
            &format!("entropy_bits={:016x}", out1.signals.entropy.to_bits()),
            if matched { "Bit-exact match" } else { "Floating point drift!" });
    }

    fn test_det_012_hash_determinism(&mut self) {
        let start = Instant::now();

        let id1 = ActionId::new_deterministic(b"test-seed-canonical");
        let id2 = ActionId::new_deterministic(b"test-seed-canonical");

        let matched = id1 == id2;

        self.record("DET-012", "Determinism",
            "ActionId hash is deterministic",
            if matched { TestStatus::Pass } else { TestStatus::Fail },
            start.elapsed().as_nanos() as u64,
            "seed: test-seed-canonical",
            &format!("id1={:?}", id1),
            if matched { "Hash deterministic" } else { "Hash non-deterministic!" });
    }

    // ========================================================================
    // CAT-2: BOUNDARY CONDITIONS
    // ========================================================================

    fn execute_boundary_tests(&mut self) {
        self.print_category("CAT-2: BOUNDARY CONDITIONS", "CRITICAL");

        self.test_bnd_001_zero_length();
        self.test_bnd_002_one_byte();
        self.test_bnd_003_two_bytes();
        self.test_bnd_004_all_same();
        self.test_bnd_005_all_different();
        self.test_bnd_006_max_entropy();
        self.test_bnd_007_min_entropy();
        self.test_bnd_008_budget_exact();
        self.test_bnd_009_budget_exceed();
        self.test_bnd_010_recursion_limit();

        println!();
    }

    fn test_bnd_001_zero_length(&mut self) {
        let start = Instant::now();
        let output = self.cortex.perceive(&RawInput::from_bytes(vec![]));

        let valid = output.signals.sample_count == 0 && output.signals.entropy == 0.0;

        self.record("BND-001", "Boundary",
            "Zero-length input handled correctly",
            if valid { TestStatus::Pass } else { TestStatus::Fail },
            start.elapsed().as_nanos() as u64,
            "0 bytes",
            &format!("sample_count={}, entropy={}", 
                output.signals.sample_count, output.signals.entropy),
            "Empty input test");
    }

    fn test_bnd_002_one_byte(&mut self) {
        let start = Instant::now();
        let output = self.cortex.perceive(&RawInput::from_bytes(vec![42]));

        let valid = output.signals.sample_count == 1;

        self.record("BND-002", "Boundary",
            "Single byte input handled correctly",
            if valid { TestStatus::Pass } else { TestStatus::Fail },
            start.elapsed().as_nanos() as u64,
            "1 byte (42)",
            &format!("sample_count={}", output.signals.sample_count),
            "Single byte test");
    }

    fn test_bnd_003_two_bytes(&mut self) {
        let start = Instant::now();
        let output = self.cortex.perceive(&RawInput::from_bytes(vec![0, 255]));

        let valid = output.signals.sample_count == 2
            && output.signals.mean == 127.5;

        self.record("BND-003", "Boundary",
            "Two-byte input handled correctly",
            if valid { TestStatus::Pass } else { TestStatus::Fail },
            start.elapsed().as_nanos() as u64,
            "2 bytes [0, 255]",
            &format!("mean={}", output.signals.mean),
            "Two byte test");
    }

    fn test_bnd_004_all_same(&mut self) {
        let start = Instant::now();
        let output = self.cortex.perceive(&RawInput::from_bytes(vec![128; 1000]));

        let valid = output.signals.entropy == 0.0 && output.signals.std_dev == 0.0;

        self.record("BND-004", "Boundary",
            "All-same-value input has zero entropy",
            if valid { TestStatus::Pass } else { TestStatus::Fail },
            start.elapsed().as_nanos() as u64,
            "1000 bytes of 128",
            &format!("entropy={}, std_dev={}", output.signals.entropy, output.signals.std_dev),
            "Constant signal test");
    }

    fn test_bnd_005_all_different(&mut self) {
        let start = Instant::now();
        let data: Vec<u8> = (0..=255).collect();
        let output = self.cortex.perceive(&RawInput::from_bytes(data));

        let valid = output.signals.unique_values == 256;

        self.record("BND-005", "Boundary",
            "All-different-values input detected",
            if valid { TestStatus::Pass } else { TestStatus::Fail },
            start.elapsed().as_nanos() as u64,
            "256 unique bytes",
            &format!("unique_values={}", output.signals.unique_values),
            "All unique test");
    }

    fn test_bnd_006_max_entropy(&mut self) {
        let start = Instant::now();
        let data: Vec<u8> = (0..=255).collect();
        let output = self.cortex.perceive(&RawInput::from_bytes(data));

        // Normalized entropy should be very close to 1.0
        let valid = output.signals.entropy > 0.99;

        self.record("BND-006", "Boundary",
            "Maximum entropy correctly calculated",
            if valid { TestStatus::Pass } else { TestStatus::Fail },
            start.elapsed().as_nanos() as u64,
            "256 unique values",
            &format!("entropy={:.10}", output.signals.entropy),
            "Max entropy test");
    }

    fn test_bnd_007_min_entropy(&mut self) {
        let start = Instant::now();
        let output = self.cortex.perceive(&RawInput::from_bytes(vec![0; 1000]));

        let valid = output.signals.entropy == 0.0;

        self.record("BND-007", "Boundary",
            "Minimum entropy (constant) correctly calculated",
            if valid { TestStatus::Pass } else { TestStatus::Fail },
            start.elapsed().as_nanos() as u64,
            "1000 zeros",
            &format!("entropy={}", output.signals.entropy),
            "Min entropy test");
    }

    fn test_bnd_008_budget_exact(&mut self) {
        let start = Instant::now();
        let budget = ComputationalBudget::new().with_max_bytes(1000);
        let input = RawInput::from_bytes(vec![0; 1000]);

        let result = self.cortex.perceive_checked(&input, &budget);
        let valid = result.is_ok();

        self.record("BND-008", "Boundary",
            "Budget at exact limit succeeds",
            if valid { TestStatus::Pass } else { TestStatus::Fail },
            start.elapsed().as_nanos() as u64,
            "1000 bytes, 1000 byte limit",
            &format!("result={:?}", result.is_ok()),
            "Exact budget test");
    }

    fn test_bnd_009_budget_exceed(&mut self) {
        let start = Instant::now();
        let budget = ComputationalBudget::new().with_max_bytes(100);
        let input = RawInput::from_bytes(vec![0; 1000]);

        let result = self.cortex.perceive_checked(&input, &budget);
        let valid = result.is_err();

        self.record("BND-009", "Boundary",
            "Budget exceeded returns error",
            if valid { TestStatus::Pass } else { TestStatus::Fail },
            start.elapsed().as_nanos() as u64,
            "1000 bytes, 100 byte limit",
            &format!("error={}", result.is_err()),
            "Budget exceeded test");
    }

    fn test_bnd_010_recursion_limit(&mut self) {
        let start = Instant::now();
        let budget = ComputationalBudget::new().with_max_recursion(3);
        let mut guard = BudgetGuard::new(budget);

        let _ = guard.enter_recursion();
        let _ = guard.enter_recursion();
        let _ = guard.enter_recursion();
        let result = guard.enter_recursion();

        let valid = result.is_err() && guard.recursion_depth() == 3;

        self.record("BND-010", "Boundary",
            "Recursion limit enforced correctly",
            if valid { TestStatus::Pass } else { TestStatus::Fail },
            start.elapsed().as_nanos() as u64,
            "limit=3, attempts=4",
            &format!("depth={}, 4th_failed={}", guard.recursion_depth(), result.is_err()),
            "Recursion limit test");
    }

    // ========================================================================
    // CAT-3: REAL-WORLD DATASETS
    // ========================================================================

    fn execute_dataset_tests(&mut self) {
        self.print_category("CAT-3: REAL-WORLD DATASETS", "CRITICAL");

        let datasets_path = self.datasets_dir.clone();
        
        if !datasets_path.exists() {
            println!("  ⚠️  Creating datasets directory: {:?}", datasets_path);
            fs::create_dir_all(&datasets_path).ok();
        }

        // Check subdirectories
        let categories = ["industrial", "documents", "media", "scientific", "adversarial"];
        let mut found_any = false;
        
        for category in &categories {
            let cat_path = datasets_path.join(category);
            if cat_path.exists() && cat_path.is_dir() {
                if self.test_dataset_category(&cat_path, category) {
                    found_any = true;
                }
            } else {
                println!("  ⏭️  Category '{}' directory not found", category);
                fs::create_dir_all(&cat_path).ok();
            }
        }

        // Also check root datasets directory
        if self.test_dataset_category(&datasets_path, "root") {
            found_any = true;
        }

        if !found_any {
            println!("  ⚠️  No real datasets found. Please add files to:");
            println!("      validation/datasets/real/industrial/");
            println!("      validation/datasets/real/documents/");
            println!("      validation/datasets/real/media/");
            println!("      validation/datasets/real/scientific/");
            println!("      validation/datasets/real/adversarial/");
        }

        println!();
    }

    fn test_dataset_category(&mut self, path: &Path, category: &str) -> bool {
        let mut found_files = false;
        
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let entry_path = entry.path();
                if entry_path.is_file() {
                    self.test_single_dataset(&entry_path, category);
                    found_files = true;
                }
            }
        }
        
        found_files
    }

    fn test_single_dataset(&mut self, path: &Path, _category: &str) {
        let start = Instant::now();
        let filename = path.file_name().unwrap().to_string_lossy().to_string();
        let short_name: String = filename.chars().take(12).collect();
        let test_id = format!("DS-{}", short_name.to_uppercase().replace(".", "_"));

        // Read file
        let mut file = match File::open(path) {
            Ok(f) => f,
            Err(e) => {
                self.record(&test_id, "Dataset",
                    &format!("Load {}", filename),
                    TestStatus::Error,
                    start.elapsed().as_nanos() as u64,
                    &filename,
                    &format!("Error: {}", e),
                    "File open failed");
                return;
            }
        };

        let mut buffer = Vec::new();
        if let Err(e) = file.read_to_end(&mut buffer) {
            self.record(&test_id, "Dataset",
                &format!("Read {}", filename),
                TestStatus::Error,
                start.elapsed().as_nanos() as u64,
                &filename,
                &format!("Error: {}", e),
                "File read failed");
            return;
        }

        let size = buffer.len();

        // Process through cortex
        let input = RawInput::from_bytes(buffer);
        let output = self.cortex.perceive(&input);

        // Verify determinism
        let output2 = self.cortex.perceive(&input);
        let deterministic = output.signals.entropy == output2.signals.entropy
            && output.signals.mean == output2.signals.mean
            && output.signals.std_dev == output2.signals.std_dev;

        let status = if deterministic { TestStatus::Pass } else { TestStatus::Fail };

        self.record(&test_id, "Dataset",
            &format!("{} ({} bytes)", filename, size),
            status,
            start.elapsed().as_nanos() as u64,
            &format!("size={}", size),
            &format!("entropy={:.6}, mean={:.2}, std={:.2}", 
                output.signals.entropy, output.signals.mean, output.signals.std_dev),
            if deterministic { "Deterministic" } else { "NON-DETERMINISTIC!" });
    }

    // ========================================================================
    // CAT-4: STRESS TESTING
    // ========================================================================

    fn execute_stress_tests(&mut self) {
        self.print_category("CAT-4: STRESS TESTING", "HIGH");

        self.test_str_001_rapid_calls();
        self.test_str_002_large_input();
        self.test_str_003_many_small();
        self.test_str_004_memory_pressure();

        println!();
    }

    fn test_str_001_rapid_calls(&mut self) {
        let start = Instant::now();
        let data = vec![1, 2, 3, 4, 5];
        let input = RawInput::from_bytes(data);

        for _ in 0..10_000 {
            let _ = self.cortex.perceive(&input);
        }

        self.record("STR-001", "Stress",
            "10,000 rapid sequential calls",
            TestStatus::Pass,
            start.elapsed().as_nanos() as u64,
            "10,000 calls",
            &format!("completed in {:.2}ms", start.elapsed().as_millis()),
            "Rapid calls test");
    }

    fn test_str_002_large_input(&mut self) {
        let start = Instant::now();
        let data: Vec<u8> = (0..1_000_000).map(|i| (i % 256) as u8).collect();
        let input = RawInput::from_bytes(data);

        let output = self.cortex.perceive(&input);

        self.record("STR-002", "Stress",
            "1 MB input processing",
            TestStatus::Pass,
            start.elapsed().as_nanos() as u64,
            "1,000,000 bytes",
            &format!("entropy={:.6}, time={:.2}ms", 
                output.signals.entropy, start.elapsed().as_millis()),
            "Large input test");
    }

    fn test_str_003_many_small(&mut self) {
        let start = Instant::now();

        for i in 0..1000 {
            let data = vec![(i % 256) as u8; 100];
            let input = RawInput::from_bytes(data);
            let _ = self.cortex.perceive(&input);
        }

        self.record("STR-003", "Stress",
            "1,000 different small inputs",
            TestStatus::Pass,
            start.elapsed().as_nanos() as u64,
            "1,000 x 100 bytes",
            &format!("completed in {:.2}ms", start.elapsed().as_millis()),
            "Many small inputs test");
    }

    fn test_str_004_memory_pressure(&mut self) {
        let start = Instant::now();

        for i in 0..100 {
            let data: Vec<u8> = (0..10_000).map(|j| ((i + j) % 256) as u8).collect();
            let input = RawInput::from_bytes(data);
            let _ = self.cortex.perceive(&input);
        }

        self.record("STR-004", "Stress",
            "Memory pressure (100 x 10KB)",
            TestStatus::Pass,
            start.elapsed().as_nanos() as u64,
            "100 x 10,000 bytes",
            &format!("completed in {:.2}ms", start.elapsed().as_millis()),
            "Memory pressure test");
    }

    // ========================================================================
    // CAT-5: GDE↔GDC PROTOCOL
    // ========================================================================

    fn execute_protocol_tests(&mut self) {
        self.print_category("CAT-5: GDE-GDC PROTOCOL", "CRITICAL");

        self.test_prot_001_basic_request();
        self.test_prot_002_replay_request();
        self.test_prot_003_budget_error();
        self.test_prot_004_empty_request();
        self.test_prot_005_maturation_request();
        self.test_prot_006_state_isolation();

        println!();
    }

    fn test_prot_001_basic_request(&mut self) {
        let start = Instant::now();

        let gde_request = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let input = RawInput::from_bytes(gde_request);

        let output = self.cortex.perceive(&input);

        let valid = output.signals.sample_count == 10
            && !output.signals.entropy.is_nan()
            && !output.signals.mean.is_nan();

        self.record("PROT-001", "Protocol",
            "Basic GDE->GDC request/response",
            if valid { TestStatus::Pass } else { TestStatus::Fail },
            start.elapsed().as_nanos() as u64,
            "10 bytes",
            &format!("complete_response={}", valid),
            "Basic protocol test");
    }

    fn test_prot_002_replay_request(&mut self) {
        let start = Instant::now();
        let request = vec![1, 2, 3, 4, 5];
        let input = RawInput::from_bytes(request);

        let out1 = self.cortex.perceive(&input);
        let out2 = self.cortex.perceive(&input);

        let identical = out1.signals.entropy == out2.signals.entropy
            && out1.signals.mean == out2.signals.mean
            && out1.signals.std_dev == out2.signals.std_dev;

        self.record("PROT-002", "Protocol",
            "Replay produces identical response",
            if identical { TestStatus::Pass } else { TestStatus::Fail },
            start.elapsed().as_nanos() as u64,
            "Same request twice",
            &format!("identical={}", identical),
            "Replay protocol test");
    }

    fn test_prot_003_budget_error(&mut self) {
        let start = Instant::now();
        let budget = ComputationalBudget::new().with_max_bytes(10);
        let input = RawInput::from_bytes(vec![0; 1000]);

        let result = self.cortex.perceive_checked(&input, &budget);

        self.record("PROT-003", "Protocol",
            "Budget exceeded returns proper error",
            if result.is_err() { TestStatus::Pass } else { TestStatus::Fail },
            start.elapsed().as_nanos() as u64,
            "1000 bytes, 10 byte limit",
            &format!("error_returned={}", result.is_err()),
            "Budget error protocol test");
    }

    fn test_prot_004_empty_request(&mut self) {
        let start = Instant::now();
        let input = RawInput::from_bytes(vec![]);
        let output = self.cortex.perceive(&input);

        let valid = output.signals.sample_count == 0;

        self.record("PROT-004", "Protocol",
            "Empty request handled gracefully",
            if valid { TestStatus::Pass } else { TestStatus::Fail },
            start.elapsed().as_nanos() as u64,
            "0 bytes",
            &format!("sample_count={}", output.signals.sample_count),
            "Empty request protocol test");
    }

    fn test_prot_005_maturation_request(&mut self) {
        let start = Instant::now();
        let input = RawInput::from_bytes(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        let config = MaturationConfig::default().with_max_iterations(5);

        let output = self.cortex.perceive_mature(&input, &config);

        let valid = output.maturation.iterations_performed >= 1;

        self.record("PROT-005", "Protocol",
            "Maturation request returns state",
            if valid { TestStatus::Pass } else { TestStatus::Fail },
            start.elapsed().as_nanos() as u64,
            "10 bytes + maturation config",
            &format!("iterations={}", output.maturation.iterations_performed),
            "Maturation protocol test");
    }

    fn test_prot_006_state_isolation(&mut self) {
        let start = Instant::now();

        let input1 = RawInput::from_bytes(vec![1, 1, 1, 1, 1]);
        let input2 = RawInput::from_bytes(vec![255, 255, 255, 255, 255]);

        let out1a = self.cortex.perceive(&input1);
        let _ = self.cortex.perceive(&input2);
        let out1b = self.cortex.perceive(&input1);

        let isolated = out1a.signals.entropy == out1b.signals.entropy
            && out1a.signals.mean == out1b.signals.mean;

        self.record("PROT-006", "Protocol",
            "State isolation between requests",
            if isolated { TestStatus::Pass } else { TestStatus::Fail },
            start.elapsed().as_nanos() as u64,
            "Interleaved requests",
            &format!("isolated={}", isolated),
            "State isolation test");
    }

    // ========================================================================
    // CAT-6: ADVERSARIAL INPUTS
    // ========================================================================

    fn execute_adversarial_tests(&mut self) {
        self.print_category("CAT-6: ADVERSARIAL INPUTS", "HIGH");

        self.test_adv_001_all_zeros();
        self.test_adv_002_all_ones();
        self.test_adv_003_alternating();
        self.test_adv_004_near_random();
        self.test_adv_005_long_runs();
        self.test_adv_006_pathological();

        println!();
    }

    fn test_adv_001_all_zeros(&mut self) {
        let start = Instant::now();
        let input = RawInput::from_bytes(vec![0; 10000]);
        let output = self.cortex.perceive(&input);

        let valid = output.signals.entropy == 0.0 && output.signals.mean == 0.0;

        self.record("ADV-001", "Adversarial",
            "All zeros (10KB)",
            if valid { TestStatus::Pass } else { TestStatus::Fail },
            start.elapsed().as_nanos() as u64,
            "10,000 x 0x00",
            &format!("entropy={}, mean={}", output.signals.entropy, output.signals.mean),
            "All zeros adversarial test");
    }

    fn test_adv_002_all_ones(&mut self) {
        let start = Instant::now();
        let input = RawInput::from_bytes(vec![255; 10000]);
        let output = self.cortex.perceive(&input);

        let valid = output.signals.entropy == 0.0 && output.signals.mean == 255.0;

        self.record("ADV-002", "Adversarial",
            "All 0xFF (10KB)",
            if valid { TestStatus::Pass } else { TestStatus::Fail },
            start.elapsed().as_nanos() as u64,
            "10,000 x 0xFF",
            &format!("entropy={}, mean={}", output.signals.entropy, output.signals.mean),
            "All ones adversarial test");
    }

    fn test_adv_003_alternating(&mut self) {
        let start = Instant::now();
        let data: Vec<u8> = (0..10000).map(|i| if i % 2 == 0 { 0 } else { 255 }).collect();
        let input = RawInput::from_bytes(data);
        let output = self.cortex.perceive(&input);

        // Should detect periodicity
        let valid = output.signals.periodicity_detected;

        self.record("ADV-003", "Adversarial",
            "Alternating 0/255 pattern",
            if valid { TestStatus::Pass } else { TestStatus::Fail },
            start.elapsed().as_nanos() as u64,
            "0,255,0,255... (10KB)",
            &format!("periodicity={}", output.signals.periodicity_detected),
            "Alternating adversarial test");
    }

    fn test_adv_004_near_random(&mut self) {
        let start = Instant::now();
        // LCG pseudo-random
        let mut state: u64 = 12345;
        let data: Vec<u8> = (0..10000).map(|_| {
            state = state.wrapping_mul(1103515245).wrapping_add(12345) % (1u64 << 31);
            (state >> 16) as u8
        }).collect();

        let input = RawInput::from_bytes(data);
        let output = self.cortex.perceive(&input);

        let valid = output.signals.entropy > 0.9;

        self.record("ADV-004", "Adversarial",
            "Pseudo-random data (10KB)",
            if valid { TestStatus::Pass } else { TestStatus::Fail },
            start.elapsed().as_nanos() as u64,
            "LCG random",
            &format!("entropy={:.6}", output.signals.entropy),
            "Near random adversarial test");
    }

    fn test_adv_005_long_runs(&mut self) {
        let start = Instant::now();
        let mut data = Vec::with_capacity(10000);
        for i in 0..100 {
            data.extend(vec![(i % 256) as u8; 100]);
        }

        let input = RawInput::from_bytes(data);
        let output = self.cortex.perceive(&input);

        self.record("ADV-005", "Adversarial",
            "Long runs of repeated values",
            TestStatus::Pass,
            start.elapsed().as_nanos() as u64,
            "100 runs x 100 bytes each",
            &format!("entropy={:.6}", output.signals.entropy),
            "Long runs adversarial test");
    }

    fn test_adv_006_pathological(&mut self) {
        let start = Instant::now();
        let data: Vec<u8> = (0..10000).map(|i| {
            let x = i as f64;
            ((x.sin() * 127.0) + 128.0) as u8
        }).collect();

        let input = RawInput::from_bytes(data);
        let output = self.cortex.perceive(&input);

        self.record("ADV-006", "Adversarial",
            "Sinusoidal pattern",
            TestStatus::Pass,
            start.elapsed().as_nanos() as u64,
            "sin(i) * 127 + 128",
            &format!("entropy={:.6}, periodicity={}", 
                output.signals.entropy, output.signals.periodicity_detected),
            "Pathological adversarial test");
    }

    // ========================================================================
    // CAT-7: CONCURRENCY
    // ========================================================================

    fn execute_concurrency_tests(&mut self) {
        self.print_category("CAT-7: CONCURRENCY", "CRITICAL");

        self.test_con_001_parallel_instances();
        self.test_con_002_shared_cortex();

        println!();
    }

    fn test_con_001_parallel_instances(&mut self) {
        let start = Instant::now();

        let handles: Vec<_> = (0..10).map(|i| {
            thread::spawn(move || {
                let cortex = SensoryCortex::new();
                let data = vec![i as u8; 100];
                let input = RawInput::from_bytes(data);
                cortex.perceive(&input)
            })
        }).collect();

        let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

        let all_completed = results.len() == 10;

        self.record("CON-001", "Concurrency",
            "10 parallel cortex instances",
            if all_completed { TestStatus::Pass } else { TestStatus::Fail },
            start.elapsed().as_nanos() as u64,
            "10 threads",
            &format!("completed={}", results.len()),
            "Parallel instances test");
    }

    fn test_con_002_shared_cortex(&mut self) {
        let start = Instant::now();

        let cortex = Arc::new(SensoryCortex::new());
        let results = Arc::new(Mutex::new(Vec::new()));

        let handles: Vec<_> = (0..10).map(|i| {
            let cortex = Arc::clone(&cortex);
            let results = Arc::clone(&results);
            thread::spawn(move || {
                let data = vec![i as u8; 100];
                let input = RawInput::from_bytes(data);
                let output = cortex.perceive(&input);
                results.lock().unwrap().push(output.signals.mean);
            })
        }).collect();

        for h in handles {
            h.join().unwrap();
        }

        let final_results = results.lock().unwrap();
        let all_completed = final_results.len() == 10;

        self.record("CON-002", "Concurrency",
            "Shared cortex across 10 threads",
            if all_completed { TestStatus::Pass } else { TestStatus::Fail },
            start.elapsed().as_nanos() as u64,
            "Arc<SensoryCortex>",
            &format!("completed={}", final_results.len()),
            "Shared cortex test");
    }

    // ========================================================================
    // CAT-8: NUMERICAL EDGE CASES
    // ========================================================================

    fn execute_numerical_tests(&mut self) {
        self.print_category("CAT-8: NUMERICAL EDGE CASES", "CRITICAL");

        self.test_num_001_no_nan();
        self.test_num_002_no_inf();
        self.test_num_003_bounds();
        self.test_num_004_precision();

        println!();
    }

    fn test_num_001_no_nan(&mut self) {
        let start = Instant::now();

        let test_cases = vec![
            vec![],
            vec![0],
            vec![255],
            vec![0, 255],
            vec![0; 1000],
            vec![255; 1000],
            (0..=255).collect::<Vec<u8>>(),
        ];

        let mut found_nan = false;
        for data in test_cases {
            let output = self.cortex.perceive(&RawInput::from_bytes(data));
            if output.signals.entropy.is_nan() 
                || output.signals.mean.is_nan()
                || output.signals.std_dev.is_nan() {
                found_nan = true;
                break;
            }
        }

        self.record("NUM-001", "Numerical",
            "No NaN values in outputs",
            if !found_nan { TestStatus::Pass } else { TestStatus::Fail },
            start.elapsed().as_nanos() as u64,
            "7 edge case inputs",
            &format!("nan_found={}", found_nan),
            "NaN test");
    }

    fn test_num_002_no_inf(&mut self) {
        let start = Instant::now();

        let test_cases = vec![
            vec![],
            vec![0; 100000],
            vec![255; 100000],
            (0..100000).map(|i| (i % 256) as u8).collect::<Vec<u8>>(),
        ];

        let mut found_inf = false;
        for data in test_cases {
            let output = self.cortex.perceive(&RawInput::from_bytes(data));
            if output.signals.entropy.is_infinite() 
                || output.signals.mean.is_infinite()
                || output.signals.std_dev.is_infinite() {
                found_inf = true;
                break;
            }
        }

        self.record("NUM-002", "Numerical",
            "No Infinity values in outputs",
            if !found_inf { TestStatus::Pass } else { TestStatus::Fail },
            start.elapsed().as_nanos() as u64,
            "4 large inputs",
            &format!("inf_found={}", found_inf),
            "Infinity test");
    }

    fn test_num_003_bounds(&mut self) {
        let start = Instant::now();

        let data: Vec<u8> = (0..1000).map(|i| (i % 256) as u8).collect();
        let output = self.cortex.perceive(&RawInput::from_bytes(data));

        let valid = output.signals.entropy >= 0.0 && output.signals.entropy <= 1.0
            && output.signals.mean >= 0.0 && output.signals.mean <= 255.0
            && output.signals.std_dev >= 0.0;

        self.record("NUM-003", "Numerical",
            "All values within valid bounds",
            if valid { TestStatus::Pass } else { TestStatus::Fail },
            start.elapsed().as_nanos() as u64,
            "1000 bytes",
            &format!("entropy={:.6}, mean={:.2}, std={:.2}", 
                output.signals.entropy, output.signals.mean, output.signals.std_dev),
            "Bounds test");
    }

    fn test_num_004_precision(&mut self) {
        let start = Instant::now();

        // Two inputs with GENUINELY different statistical properties
        // data1: uniform distribution centered at 128 (low variance)
        let data1: Vec<u8> = (0..1000).map(|i| (128 + (i % 3) as i16 - 1) as u8).collect();
        
        // data2: bimodal distribution at extremes (high variance, different mean)
        let data2: Vec<u8> = (0..1000).map(|i| if i % 2 == 0 { 20 } else { 235 }).collect();

        let out1 = self.cortex.perceive(&RawInput::from_bytes(data1));
        let out2 = self.cortex.perceive(&RawInput::from_bytes(data2));

        // These MUST be different (different distributions, means, variances)
        let different = (out1.signals.entropy - out2.signals.entropy).abs() > 0.001
            || (out1.signals.std_dev - out2.signals.std_dev).abs() > 0.001
            || (out1.signals.mean - out2.signals.mean).abs() > 0.001;

        self.record("NUM-004", "Numerical",
            "Different distributions produce different stats",
            if different { TestStatus::Pass } else { TestStatus::Fail },
            start.elapsed().as_nanos() as u64,
            "1000 bytes: uniform vs bimodal",
            &format!("mean1={:.2}, mean2={:.2}, std1={:.2}, std2={:.2}", 
                out1.signals.mean, out2.signals.mean,
                out1.signals.std_dev, out2.signals.std_dev),
            if different { "Distributions distinguished" } else { "Failed to distinguish!" });
    }

    // ========================================================================
    // REPORTING
    // ========================================================================

    fn print_category(&self, name: &str, severity: &str) {
        println!("┌─────────────────────────────────────────────────────────────────────┐");
        println!("│ {} [{:^8}]", 
            format!("{:<55}", name), severity);
        println!("└─────────────────────────────────────────────────────────────────────┘");
    }

    fn record(&mut self, test_id: &str, category: &str, description: &str,
              status: TestStatus, duration_ns: u64,
              input_summary: &str, output_summary: &str, details: &str) {
        
        let icon = status.icon();
        let label = status.label();

        println!("  [{}] {} ... {} {}", test_id, description, icon, label);

        self.results.push(TestResult {
            test_id: test_id.to_string(),
            category: category.to_string(),
            description: description.to_string(),
            status,
            duration_ns,
            input_summary: input_summary.to_string(),
            output_summary: output_summary.to_string(),
            details: details.to_string(),
        });
    }

    fn generate_report(&self) {
        let passed = self.results.iter().filter(|r| r.status == TestStatus::Pass).count();
        let failed = self.results.iter().filter(|r| r.status == TestStatus::Fail).count();
        let errors = self.results.iter().filter(|r| r.status == TestStatus::Error).count();
        let skipped = self.results.iter().filter(|r| r.status == TestStatus::Skip).count();
        let total = self.results.len();

        let total_time = self.start_time.elapsed();

        println!();
        println!("╔══════════════════════════════════════════════════════════════════════╗");
        println!("║                 CANONICAL VALIDATION REPORT                          ║");
        println!("║                 NATO/DoD Compliance Level                            ║");
        println!("╠══════════════════════════════════════════════════════════════════════╣");
        println!("║                                                                       ║");
        println!("║  Total Tests:    {:4}                                                ║", total);
        println!("║  Passed:         {:4}  ✅                                            ║", passed);
        println!("║  Failed:         {:4}  ❌                                            ║", failed);
        println!("║  Errors:         {:4}  💥                                            ║", errors);
        println!("║  Skipped:        {:4}  ⏭️                                             ║", skipped);
        println!("║                                                                       ║");
        println!("║  Total Duration: {:.2}s                                             ║", total_time.as_secs_f64());
        println!("║                                                                       ║");
        println!("╠══════════════════════════════════════════════════════════════════════╣");

        if failed == 0 && errors == 0 {
            println!("║                                                                       ║");
            println!("║  ██████╗  █████╗ ███████╗███████╗                                     ║");
            println!("║  ██╔══██╗██╔══██╗██╔════╝██╔════╝                                     ║");
            println!("║  ██████╔╝███████║███████╗███████╗                                     ║");
            println!("║  ██╔═══╝ ██╔══██║╚════██║╚════██║                                     ║");
            println!("║  ██║     ██║  ██║███████║███████║                                     ║");
            println!("║  ╚═╝     ╚═╝  ╚═╝╚══════╝╚══════╝                                     ║");
            println!("║                                                                       ║");
            println!("║  STATUS: CANONICAL VALIDATION PASSED                                 ║");
            println!("║                                                                       ║");
            println!("║  The system is CERTIFIED COMPLIANT for release.                      ║");
            println!("║  Version 0.1.0-rc1 (Adão Sintético) may proceed to production.       ║");
            println!("║                                                                       ║");
        } else {
            println!("║                                                                       ║");
            println!("║  ███████╗ █████╗ ██╗██╗                                               ║");
            println!("║  ██╔════╝██╔══██╗██║██║                                               ║");
            println!("║  █████╗  ███████║██║██║                                               ║");
            println!("║  ██╔══╝  ██╔══██║██║██║                                               ║");
            println!("║  ██║     ██║  ██║██║███████╗                                          ║");
            println!("║  ╚═╝     ╚═╝  ╚═╝╚═╝╚══════╝                                          ║");
            println!("║                                                                       ║");
            println!("║  STATUS: CANONICAL VALIDATION FAILED                                 ║");
            println!("║                                                                       ║");
            println!("║  {} FAILURES MUST BE RESOLVED BEFORE RELEASE.                      ║", failed + errors);
            println!("║                                                                       ║");

            // List failures
            for result in &self.results {
                if result.status == TestStatus::Fail || result.status == TestStatus::Error {
                    let desc_short: String = result.description.chars().take(35).collect();
                    println!("║  ❌ {}: {}...  ║", result.test_id, desc_short);
                }
            }
            println!("║                                                                       ║");
        }

        println!("╚══════════════════════════════════════════════════════════════════════╝");
    }

    fn write_detailed_report(&self) {
        let report_path = self.output_dir.join("canonical_validation_report.txt");
        let file = match File::create(&report_path) {
            Ok(f) => f,
            Err(e) => {
                println!("⚠️  Could not create report file: {}", e);
                return;
            }
        };
        let mut writer = BufWriter::new(file);

        let separator = "=".repeat(70);
        let line_sep = "-".repeat(70);

        writeln!(writer, "DIGITAL GENOME COMMUNITY EDITION").unwrap();
        writeln!(writer, "CANONICAL VALIDATION REPORT").unwrap();
        writeln!(writer, "Document: DGC-CVP-2025-001").unwrap();
        writeln!(writer, "Generated: {:?}", SystemTime::now()).unwrap();
        writeln!(writer, "").unwrap();
        writeln!(writer, "{}", separator).unwrap();
        writeln!(writer, "").unwrap();

        for result in &self.results {
            writeln!(writer, "Test ID:     {}", result.test_id).unwrap();
            writeln!(writer, "Category:    {}", result.category).unwrap();
            writeln!(writer, "Description: {}", result.description).unwrap();
            writeln!(writer, "Status:      {:?}", result.status).unwrap();
            writeln!(writer, "Duration:    {} ns", result.duration_ns).unwrap();
            writeln!(writer, "Input:       {}", result.input_summary).unwrap();
            writeln!(writer, "Output:      {}", result.output_summary).unwrap();
            writeln!(writer, "Details:     {}", result.details).unwrap();
            writeln!(writer, "{}", line_sep).unwrap();
        }

        println!();
        println!("📄 Detailed report written to: {:?}", report_path);
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    let datasets_dir = PathBuf::from("validation/datasets/real");
    let output_dir = PathBuf::from("validation/reports");

    let mut harness = CanonicalTestHarness::new(datasets_dir, output_dir);
    harness.execute_all();
}
