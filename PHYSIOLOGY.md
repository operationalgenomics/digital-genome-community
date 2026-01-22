# PHYSIOLOGY.md
## Computational Self-Preservation — Neutral Limits

**Date:** 2025-01-02  
**Version:** 1.4.x (Planned)  
**Status:** DESIGN DOCUMENT — REVISED

---

## FUNDAMENTAL PRINCIPLE

> "The system does NOT decide what is observable. It only decides whether it can CONTINUE observing without collapse."

The Community Edition has **computational self-preservation instincts** based exclusively on:
- Time budget
- Memory budget
- Algorithmic complexity
- Numerical stability

**NOT based on:**
- Human sensory limits
- Biological constraints
- Domain assumptions
- Signal nature

---

## WHAT SELF-PRESERVATION IS

| Limit Type | Description | Justification |
|------------|-------------|---------------|
| **Time budget** | Cannot process forever | Fairness, deadlock prevention |
| **Memory budget** | Cannot allocate infinite RAM | OOM prevention |
| **Complexity bound** | Cannot run O(2^n) on large n | Computational tractability |
| **Numerical stability** | Cannot compute with NaN/Inf | IEEE 754 validity |

These are **computational facts**, not sensory assumptions.

---

## WHAT SELF-PRESERVATION IS NOT

| NOT This | Why It's Wrong | Correct Alternative |
|----------|----------------|---------------------|
| "Max 44.1kHz sampling" | Assumes audio | "Max N samples per call" |
| "Human-visible spectrum" | Assumes light | "Max M bytes per call" |
| "Resolution like eyes" | Assumes vision | "Max K operations per call" |
| "Hearing range limits" | Assumes sound | None — no signal assumptions |

**Critical Rule:**  
If a limit mentions a human sense, domain, or signal type, it is **PROHIBITED**.

---

## COMPUTATIONAL BUDGET MODEL

### Allowed Limits

```rust
/// Computational budget for self-preservation.
///
/// These limits are about COMPUTATION, not PERCEPTION.
/// The system does not know what it's processing.
/// It only knows how much resources it can spend.
#[derive(Debug, Clone, Copy)]
pub struct ComputationalBudget {
    /// Maximum bytes to accept in a single call.
    /// Justification: Prevents OOM.
    /// NOT justified as: "too much data for human senses"
    pub max_bytes: usize,

    /// Maximum wall-clock time in nanoseconds.
    /// Justification: Prevents deadlock, ensures fairness.
    /// NOT justified as: "human attention span"
    pub max_time_ns: u64,

    /// Maximum heap allocations in bytes.
    /// Justification: Prevents memory exhaustion.
    /// NOT justified as: "brain memory limits"
    pub max_heap_bytes: usize,

    /// Maximum iterations for convergent algorithms.
    /// Justification: Guarantees termination.
    /// NOT justified as: "human patience"
    pub max_iterations: usize,
}

impl Default for ComputationalBudget {
    fn default() -> Self {
        Self {
            max_bytes: 100 * 1024 * 1024,  // 100 MB — memory constraint
            max_time_ns: 30_000_000_000,    // 30 seconds — fairness
            max_heap_bytes: 500 * 1024 * 1024, // 500 MB — OOM prevention
            max_iterations: 1000,           // Termination guarantee
        }
    }
}
```

### Integrity Check

```rust
/// Result of computational integrity verification.
///
/// This is NOT about whether the data is "valid" in any domain sense.
/// This is about whether the system can process it without collapse.
#[derive(Debug, Clone)]
pub enum IntegrityCheck {
    /// Input can be processed within budget.
    WithinBudget,

    /// Input exceeds memory budget.
    ExceedsMemory {
        requested: usize,
        available: usize,
    },

    /// Processing would exceed time budget.
    ExceedsTime {
        estimated_ns: u64,
        budget_ns: u64,
    },

    /// Input would cause numerical instability.
    NumericalCollapse {
        reason: &'static str,
    },

    /// Input is empty — nothing to process.
    EmptyInput,
}
```

---

## SEPARATION FROM ORCHESTRATION

| Responsibility | Community | External |
|----------------|-----------|----------|
| Declare budget | ✅ | Reads |
| Check if within budget | ✅ | Calls |
| Reject over-budget | ✅ | Handles |
| Decide how to chunk | ❌ | ✅ |
| Implement streaming | ❌ | ✅ |
| Aggregate results | ❌ | ✅ |

**The Community says:** "I cannot process this in one call."  
**The External decides:** "Then I will divide it into 10 calls."

---

## PROHIBITED JUSTIFICATIONS

When documenting limits, the following justifications are **FORBIDDEN**:

| Forbidden | Why |
|-----------|-----|
| "Like human vision" | Assumes visual domain |
| "Like human hearing" | Assumes audio domain |
| "Natural frequency range" | Assumes signal has frequency |
| "Biologically plausible" | Assumes biological reference |
| "Perceptually meaningful" | Assumes perception model |
| "Sensory resolution" | Assumes sensory apparatus |

**Allowed justifications:**

| Allowed | Why |
|---------|-----|
| "Prevents OOM" | Computational fact |
| "Guarantees termination" | Algorithmic fact |
| "Maintains numerical stability" | IEEE 754 fact |
| "Ensures fairness" | Resource scheduling fact |
| "Prevents deadlock" | Concurrency fact |

---

## NUMERICAL STABILITY (Allowed)

Numerical stability checks ARE allowed because they are about **computation**, not **perception**:

```rust
impl SensoryCortex {
    /// Checks for numerical collapse risk.
    ///
    /// This is NOT about signal validity.
    /// This is about computational safety.
    fn check_numerical_stability(values: &[f64]) -> bool {
        for v in values {
            if v.is_nan() || v.is_infinite() {
                return false; // Would collapse computation
            }
        }
        true
    }
}
```

**Why this is allowed:**
- NaN and Inf are IEEE 754 edge cases
- They cause computation to fail
- This has nothing to do with what the signal "means"

---

## EXAMPLE: CORRECT VS INCORRECT

### ❌ INCORRECT (Biological Limit)

```rust
// WRONG: Assumes human hearing
const MAX_FREQUENCY_HZ: f64 = 20000.0; // "Human hearing limit"

fn validate_audio(signal: &[f64], sample_rate: f64) -> bool {
    let nyquist = sample_rate / 2.0;
    nyquist <= MAX_FREQUENCY_HZ // VIOLATION: assumes audio
}
```

### ✅ CORRECT (Computational Limit)

```rust
// CORRECT: About computation, not perception
const MAX_SAMPLES: usize = 1 << 20; // ~1M samples

fn check_budget(input: &[u8], budget: &ComputationalBudget) -> IntegrityCheck {
    if input.len() > budget.max_bytes {
        return IntegrityCheck::ExceedsMemory {
            requested: input.len(),
            available: budget.max_bytes,
        };
    }
    IntegrityCheck::WithinBudget
}
```

---

## ALERT REFERENCE

- **ALERT-013**: Physiology vs Orchestration (REVISED)
- **Insight A.7**: Autopreservação Computacional Neutra

---

## AXIOM

> "The system does not know what it's looking at.  
> It only knows how much it can compute before collapse."

---

*"Limits are about resources, not reality."*
