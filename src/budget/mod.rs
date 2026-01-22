//! --------------------------
//! INFORMATION
//! --------------------------
//! Title: Computational Self-Preservation (Insight A.7)
//! Author: Carlos Eduardo Favini
//! Date: 2025-01-02
//! Version: 1.4.0
//! Description: Computational budget and integrity verification for the
//! synthetic cognitive core. This module implements self-preservation
//! instincts based EXCLUSIVELY on computational constraints.
//!
//! Important: This module contains ZERO biological analogies.
//! All limits are justified by computational facts:
//! - OOM prevention
//! - Termination guarantee
//! - Numerical stability (IEEE 754)
//! - Deadlock prevention
//!
//! The system does NOT know what it is processing.
//! It only knows how much it can compute before collapse.
//!
//! Layer: Community
//! Dependencies: None (pure computation)
//! Affected Components: SensoryCortex
//!
//! --------------------------
//! Foundational Axiom
//! --------------------------
//! "The Community Edition is an immortal observer.
//! It must never be limited by human analogies."
//!
//! --------------------------
//! CHANGE LOG
//! --------------------------
//! 2025-01-02 - Carlos Eduardo Favini - Initial creation (v1.4.0)
//! --------------------------

use std::time::{Duration, Instant};

// =============================================================================
// COMPUTATIONAL BUDGET
// =============================================================================

/// Computational budget for self-preservation.
///
/// These limits are about COMPUTATION, not PERCEPTION.
/// The system does not know what it's processing.
/// It only knows how much RESOURCE it can spend before collapse.
///
/// # Design Principles
///
/// 1. **No biological analogies**: Limits are NOT "like human vision" or "like human hearing"
/// 2. **No domain assumptions**: Limits are NOT "audio should be 44.1kHz"
/// 3. **Pure computation**: Limits are about memory, time, complexity, stability
///
/// # Allowed Justifications
///
/// - "Prevents OOM" - memory constraint
/// - "Guarantees termination" - algorithmic constraint
/// - "Maintains IEEE 754 stability" - numerical constraint
/// - "Prevents deadlock" - concurrency constraint
/// - "Ensures fairness" - resource scheduling constraint
///
/// # Prohibited Justifications
///
/// - "Like human vision" - FORBIDDEN
/// - "Like human hearing" - FORBIDDEN
/// - "Natural frequency range" - FORBIDDEN
/// - "Biologically plausible" - FORBIDDEN
/// - "Perceptually meaningful" - FORBIDDEN
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ComputationalBudget {
    /// Maximum bytes to accept in a single call.
    ///
    /// **Justification**: Prevents Out-Of-Memory (OOM) crash.
    /// **NOT justified as**: "Too much data for human senses"
    ///
    /// Default: 100 MB (104_857_600 bytes)
    pub max_bytes: usize,

    /// Maximum wall-clock time in nanoseconds.
    ///
    /// **Justification**: Prevents deadlock, ensures resource fairness.
    /// **NOT justified as**: "Human attention span"
    ///
    /// Default: 30 seconds (30_000_000_000 ns)
    /// Value 0 means no timeout.
    pub max_time_ns: u64,

    /// Maximum heap allocations allowed during processing.
    ///
    /// **Justification**: Prevents memory exhaustion during computation.
    /// **NOT justified as**: "Brain memory limits"
    ///
    /// Default: 500 MB (524_288_000 bytes)
    pub max_heap_bytes: usize,

    /// Maximum iterations for convergent algorithms.
    ///
    /// **Justification**: Guarantees termination of iterative processes.
    /// **NOT justified as**: "Human patience"
    ///
    /// Default: 10_000 iterations
    pub max_iterations: usize,

    /// Maximum recursion depth.
    ///
    /// **Justification**: Prevents stack overflow.
    /// **NOT justified as**: "Human cognitive depth"
    ///
    /// Default: 1000 levels
    pub max_recursion_depth: usize,
}

impl Default for ComputationalBudget {
    fn default() -> Self {
        Self {
            max_bytes: 100 * 1024 * 1024,       // 100 MB - OOM prevention
            max_time_ns: 30_000_000_000,        // 30 seconds - fairness
            max_heap_bytes: 500 * 1024 * 1024,  // 500 MB - memory exhaustion prevention
            max_iterations: 10_000,              // Termination guarantee
            max_recursion_depth: 1000,           // Stack overflow prevention
        }
    }
}

impl ComputationalBudget {
    /// Creates a new budget with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an unlimited budget (no constraints).
    ///
    /// **WARNING**: Use only for testing or when external systems
    /// guarantee resource availability.
    pub fn unlimited() -> Self {
        Self {
            max_bytes: usize::MAX,
            max_time_ns: 0, // 0 means no timeout
            max_heap_bytes: usize::MAX,
            max_iterations: usize::MAX,
            max_recursion_depth: usize::MAX,
        }
    }

    /// Creates a minimal budget for resource-constrained environments.
    pub fn minimal() -> Self {
        Self {
            max_bytes: 1024 * 1024,             // 1 MB
            max_time_ns: 1_000_000_000,         // 1 second
            max_heap_bytes: 10 * 1024 * 1024,   // 10 MB
            max_iterations: 100,
            max_recursion_depth: 100,
        }
    }

    /// Builder: sets maximum input bytes.
    pub fn with_max_bytes(mut self, bytes: usize) -> Self {
        self.max_bytes = bytes;
        self
    }

    /// Builder: sets maximum processing time.
    pub fn with_max_time(mut self, duration: Duration) -> Self {
        self.max_time_ns = duration.as_nanos() as u64;
        self
    }

    /// Builder: sets maximum heap allocation.
    pub fn with_max_heap(mut self, bytes: usize) -> Self {
        self.max_heap_bytes = bytes;
        self
    }

    /// Builder: sets maximum iterations.
    pub fn with_max_iterations(mut self, iterations: usize) -> Self {
        self.max_iterations = iterations;
        self
    }

    /// Builder: sets maximum recursion depth.
    pub fn with_max_recursion(mut self, depth: usize) -> Self {
        self.max_recursion_depth = depth;
        self
    }
}

// =============================================================================
// INTEGRITY CHECK
// =============================================================================

/// Result of computational integrity verification.
///
/// This is NOT about whether the data is "valid" in any domain sense.
/// This is about whether the system can PROCESS it without COLLAPSE.
///
/// The system does not judge the input's meaning.
/// It only asks: "Can I compute this without breaking?"
#[derive(Debug, Clone, PartialEq)]
pub enum IntegrityCheck {
    /// Input can be processed within the computational budget.
    WithinBudget,

    /// Input exceeds memory budget.
    ///
    /// The system cannot allocate enough memory to process this input.
    ExceedsMemory {
        /// Bytes requested
        requested: usize,
        /// Bytes available in budget
        available: usize,
    },

    /// Processing would exceed time budget.
    ///
    /// Based on complexity estimation, processing would take too long.
    ExceedsTime {
        /// Estimated processing time in nanoseconds
        estimated_ns: u64,
        /// Time budget in nanoseconds
        budget_ns: u64,
    },

    /// Input would cause numerical instability.
    ///
    /// Contains NaN, Infinity, or values that would cause IEEE 754 issues.
    NumericalCollapse {
        /// Description of the instability
        reason: NumericalIssue,
    },

    /// Input is empty - nothing to compute.
    ///
    /// This is not rejection; there's simply nothing to process.
    EmptyInput,

    /// Estimated iterations would exceed budget.
    ///
    /// Convergent algorithms would not terminate within budget.
    ExceedsIterations {
        /// Estimated iterations needed
        estimated: usize,
        /// Maximum allowed
        budget: usize,
    },
}

impl IntegrityCheck {
    /// Returns true if the input can be processed.
    pub fn is_ok(&self) -> bool {
        matches!(self, IntegrityCheck::WithinBudget)
    }

    /// Returns true if the input cannot be processed.
    pub fn is_err(&self) -> bool {
        !self.is_ok()
    }

    /// Returns true if input is empty (not an error, just nothing to do).
    pub fn is_empty(&self) -> bool {
        matches!(self, IntegrityCheck::EmptyInput)
    }
}

/// Types of numerical instability.
///
/// These are IEEE 754 facts, not domain assumptions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumericalIssue {
    /// Input contains NaN (Not a Number)
    ContainsNaN,

    /// Input contains positive or negative infinity
    ContainsInfinity,

    /// Values would cause overflow in computation
    OverflowRisk,

    /// Values would cause underflow (loss of precision)
    UnderflowRisk,

    /// Division by zero would occur
    DivisionByZero,

    /// Logarithm of non-positive number
    LogOfNonPositive,

    /// Square root of negative number
    SqrtOfNegative,
}

impl std::fmt::Display for NumericalIssue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NumericalIssue::ContainsNaN => write!(f, "Input contains NaN"),
            NumericalIssue::ContainsInfinity => write!(f, "Input contains Infinity"),
            NumericalIssue::OverflowRisk => write!(f, "Computation would overflow"),
            NumericalIssue::UnderflowRisk => write!(f, "Computation would underflow"),
            NumericalIssue::DivisionByZero => write!(f, "Division by zero would occur"),
            NumericalIssue::LogOfNonPositive => write!(f, "Logarithm of non-positive number"),
            NumericalIssue::SqrtOfNegative => write!(f, "Square root of negative number"),
        }
    }
}

// =============================================================================
// BUDGET CHECKER
// =============================================================================

/// Checks if raw bytes fit within a computational budget.
///
/// This function examines the input from a purely computational perspective.
/// It does NOT know what the bytes represent.
/// It only asks: "Can I process this many bytes without collapse?"
pub fn check_bytes_budget(bytes: &[u8], budget: &ComputationalBudget) -> IntegrityCheck {
    // Empty input is not an error, just nothing to do
    if bytes.is_empty() {
        return IntegrityCheck::EmptyInput;
    }

    // Check memory budget
    if bytes.len() > budget.max_bytes {
        return IntegrityCheck::ExceedsMemory {
            requested: bytes.len(),
            available: budget.max_bytes,
        };
    }

    // Estimate heap usage (bytes + processing overhead)
    // Overhead estimate: ~3x input size for intermediate buffers
    let estimated_heap = bytes.len() * 3;
    if estimated_heap > budget.max_heap_bytes {
        return IntegrityCheck::ExceedsMemory {
            requested: estimated_heap,
            available: budget.max_heap_bytes,
        };
    }

    IntegrityCheck::WithinBudget
}

/// Checks if f64 values are numerically stable.
///
/// This is about IEEE 754 validity, not signal interpretation.
/// The system does not know what these numbers represent.
/// It only asks: "Will computation with these numbers succeed?"
pub fn check_numerical_stability(values: &[f64]) -> IntegrityCheck {
    if values.is_empty() {
        return IntegrityCheck::EmptyInput;
    }

    for &v in values {
        if v.is_nan() {
            return IntegrityCheck::NumericalCollapse {
                reason: NumericalIssue::ContainsNaN,
            };
        }
        if v.is_infinite() {
            return IntegrityCheck::NumericalCollapse {
                reason: NumericalIssue::ContainsInfinity,
            };
        }
    }

    IntegrityCheck::WithinBudget
}

/// Estimates if computation would exceed time budget.
///
/// This is a heuristic based on input size and algorithm complexity.
/// It does NOT know what the input represents.
///
/// # Arguments
/// * `input_size` - Size of input in elements
/// * `complexity` - Algorithm complexity class
/// * `budget` - Computational budget
pub fn check_time_budget(
    input_size: usize,
    complexity: ComplexityClass,
    budget: &ComputationalBudget,
) -> IntegrityCheck {
    if budget.max_time_ns == 0 {
        // No timeout configured
        return IntegrityCheck::WithinBudget;
    }

    // Estimate operations based on complexity
    let estimated_ops = complexity.estimate_operations(input_size);

    // Assume ~1 nanosecond per operation (conservative estimate)
    let estimated_ns = estimated_ops as u64;

    if estimated_ns > budget.max_time_ns {
        return IntegrityCheck::ExceedsTime {
            estimated_ns,
            budget_ns: budget.max_time_ns,
        };
    }

    IntegrityCheck::WithinBudget
}

/// Algorithm complexity classes.
///
/// Used for time estimation. These are mathematical facts about algorithms.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComplexityClass {
    /// O(1) - Constant time
    Constant,
    /// O(log n) - Logarithmic
    Logarithmic,
    /// O(n) - Linear
    Linear,
    /// O(n log n) - Linearithmic (e.g., FFT, sorting)
    Linearithmic,
    /// O(n²) - Quadratic
    Quadratic,
    /// O(n³) - Cubic
    Cubic,
}

impl ComplexityClass {
    /// Estimates number of operations for a given input size.
    pub fn estimate_operations(&self, n: usize) -> usize {
        match self {
            ComplexityClass::Constant => 1,
            ComplexityClass::Logarithmic => {
                if n == 0 { 1 } else { (n as f64).log2().ceil() as usize }
            }
            ComplexityClass::Linear => n,
            ComplexityClass::Linearithmic => {
                if n == 0 { 1 } else { n * (n as f64).log2().ceil() as usize }
            }
            ComplexityClass::Quadratic => n.saturating_mul(n),
            ComplexityClass::Cubic => n.saturating_mul(n).saturating_mul(n),
        }
    }
}

// =============================================================================
// BUDGET GUARD
// =============================================================================

/// A guard that tracks resource usage during computation.
///
/// This is for internal use to monitor whether computation stays within budget.
/// It does NOT know what is being computed - only resource consumption.
#[derive(Debug)]
pub struct BudgetGuard {
    budget: ComputationalBudget,
    start_time: Instant,
    iterations_used: usize,
    recursion_depth: usize,
}

impl BudgetGuard {
    /// Creates a new budget guard.
    pub fn new(budget: ComputationalBudget) -> Self {
        Self {
            budget,
            start_time: Instant::now(),
            iterations_used: 0,
            recursion_depth: 0,
        }
    }

    /// Checks if time budget is exhausted.
    pub fn time_exhausted(&self) -> bool {
        if self.budget.max_time_ns == 0 {
            return false; // No timeout
        }
        let elapsed = self.start_time.elapsed().as_nanos() as u64;
        elapsed >= self.budget.max_time_ns
    }

    /// Records an iteration and checks budget.
    pub fn record_iteration(&mut self) -> Result<(), IntegrityCheck> {
        self.iterations_used += 1;
        if self.iterations_used > self.budget.max_iterations {
            return Err(IntegrityCheck::ExceedsIterations {
                estimated: self.iterations_used,
                budget: self.budget.max_iterations,
            });
        }
        Ok(())
    }

    /// Enters a recursion level and checks budget.
    pub fn enter_recursion(&mut self) -> Result<(), IntegrityCheck> {
        // Check BEFORE incrementing to avoid leaving depth in inconsistent state
        if self.recursion_depth >= self.budget.max_recursion_depth {
            return Err(IntegrityCheck::ExceedsIterations {
                estimated: self.recursion_depth + 1,
                budget: self.budget.max_recursion_depth,
            });
        }
        self.recursion_depth += 1;
        Ok(())
    }

    /// Exits a recursion level.
    pub fn exit_recursion(&mut self) {
        self.recursion_depth = self.recursion_depth.saturating_sub(1);
    }

    /// Returns elapsed time in nanoseconds.
    pub fn elapsed_ns(&self) -> u64 {
        self.start_time.elapsed().as_nanos() as u64
    }

    /// Returns iterations used.
    pub fn iterations(&self) -> usize {
        self.iterations_used
    }

    /// Returns current recursion depth.
    pub fn recursion_depth(&self) -> usize {
        self.recursion_depth
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_budget() {
        let budget = ComputationalBudget::default();
        assert_eq!(budget.max_bytes, 100 * 1024 * 1024);
        assert_eq!(budget.max_time_ns, 30_000_000_000);
        assert_eq!(budget.max_iterations, 10_000);
    }

    #[test]
    fn test_unlimited_budget() {
        let budget = ComputationalBudget::unlimited();
        assert_eq!(budget.max_bytes, usize::MAX);
        assert_eq!(budget.max_time_ns, 0);
    }

    #[test]
    fn test_minimal_budget() {
        let budget = ComputationalBudget::minimal();
        assert_eq!(budget.max_bytes, 1024 * 1024);
        assert_eq!(budget.max_time_ns, 1_000_000_000);
    }

    #[test]
    fn test_builder_pattern() {
        let budget = ComputationalBudget::new()
            .with_max_bytes(1024)
            .with_max_iterations(100);
        assert_eq!(budget.max_bytes, 1024);
        assert_eq!(budget.max_iterations, 100);
    }

    #[test]
    fn test_empty_input() {
        let budget = ComputationalBudget::default();
        let result = check_bytes_budget(&[], &budget);
        assert!(matches!(result, IntegrityCheck::EmptyInput));
        assert!(result.is_empty());
    }

    #[test]
    fn test_within_budget() {
        let budget = ComputationalBudget::default();
        let data = vec![0u8; 1000];
        let result = check_bytes_budget(&data, &budget);
        assert!(result.is_ok());
    }

    #[test]
    fn test_exceeds_memory() {
        let budget = ComputationalBudget::new().with_max_bytes(100);
        let data = vec![0u8; 200];
        let result = check_bytes_budget(&data, &budget);
        assert!(matches!(result, IntegrityCheck::ExceedsMemory { .. }));
        assert!(result.is_err());
    }

    #[test]
    fn test_numerical_stability_valid() {
        let values = vec![1.0, 2.0, 3.0, -1.5, 0.0];
        let result = check_numerical_stability(&values);
        assert!(result.is_ok());
    }

    #[test]
    fn test_numerical_stability_nan() {
        let values = vec![1.0, f64::NAN, 3.0];
        let result = check_numerical_stability(&values);
        assert!(matches!(
            result,
            IntegrityCheck::NumericalCollapse { reason: NumericalIssue::ContainsNaN }
        ));
    }

    #[test]
    fn test_numerical_stability_infinity() {
        let values = vec![1.0, f64::INFINITY, 3.0];
        let result = check_numerical_stability(&values);
        assert!(matches!(
            result,
            IntegrityCheck::NumericalCollapse { reason: NumericalIssue::ContainsInfinity }
        ));
    }

    #[test]
    fn test_complexity_estimation() {
        assert_eq!(ComplexityClass::Constant.estimate_operations(1000), 1);
        assert_eq!(ComplexityClass::Linear.estimate_operations(1000), 1000);
        assert_eq!(ComplexityClass::Quadratic.estimate_operations(100), 10000);
    }

    #[test]
    fn test_budget_guard_iterations() {
        let budget = ComputationalBudget::new().with_max_iterations(5);
        let mut guard = BudgetGuard::new(budget);

        for _ in 0..5 {
            assert!(guard.record_iteration().is_ok());
        }
        assert!(guard.record_iteration().is_err());
    }

    #[test]
    fn test_budget_guard_recursion() {
        let budget = ComputationalBudget::new().with_max_recursion(3);
        let mut guard = BudgetGuard::new(budget);

        assert!(guard.enter_recursion().is_ok());
        assert!(guard.enter_recursion().is_ok());
        assert!(guard.enter_recursion().is_ok());
        assert!(guard.enter_recursion().is_err());

        guard.exit_recursion();
        assert_eq!(guard.recursion_depth(), 2);
    }

    // =========================================================================
    // AXIOM VERIFICATION TESTS
    // =========================================================================

    #[test]
    fn test_no_domain_assumptions() {
        // This test verifies that the budget system makes NO assumptions
        // about what the data represents.

        let budget = ComputationalBudget::default();

        // Random bytes - could be anything
        let mystery_data = vec![0xDE, 0xAD, 0xBE, 0xEF];
        let result = check_bytes_budget(&mystery_data, &budget);

        // The system ONLY checks size, not content meaning
        assert!(result.is_ok());
    }

    #[test]
    fn test_no_biological_limits() {
        // Verify that limits are NOT based on human perception

        let budget = ComputationalBudget::default();

        // Create data that would be "imperceptible to humans" but valid for computation
        // A single byte - "too small for human perception" is NOT a valid rejection
        let tiny_data = vec![0x42];
        let result = check_bytes_budget(&tiny_data, &budget);
        assert!(result.is_ok()); // Accepted because it's computationally valid

        // The system does NOT reject based on "meaningful perception"
    }
}
