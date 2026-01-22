# Digital Genome — Community Edition

[![Version](https://img.shields.io/badge/version-0.1.0--rc1-blue.svg)](CHANGELOG.md)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![Tests](https://img.shields.io/badge/tests-230%20passing-brightgreen.svg)]()

**A synthetic cognitive core for Industry 5.0**

**Codename:** Adão Sintético (Synthetic Adam)

---

## What Is This?

The Digital Genome Community Edition is a **synthetic cognitive core** that:

- **Perceives** structure in observed phenomena (any byte stream)
- **Evaluates** coherence through four cognitive motors
- **Matures** understanding through iterative refinement
- **Emits** cognitive outputs with Craft Performance scores
- **Returns to listening** — it does NOT act

This is the foundational layer for the Digital Genome ecosystem. It provides the cognitive machinery; execution happens elsewhere (Enterprise Edition).

---

## v0.1.0-rc1 Status

| Component | Status |
|-----------|--------|
| Sensory Cortex | ✅ Complete |
| Four Cognitive Motors | ✅ Complete |
| Craft Performance Formula | ✅ Canonical (multiplicative) |
| Perceptual Maturation | ✅ Iterative refinement |
| Replay Harness | ✅ Deterministic |
| Computational Budget | ✅ Resource control |
| Observability | ✅ Health monitoring |
| Unit Tests | ✅ 195 passing |
| Integration Tests | ✅ 35 passing |
| Total Tests | ✅ 230 passing |
| Critical Violations | ✅ Zero |

---

## Quick Start

### Installation

```toml
[dependencies]
digital-genome-community = "0.1.0-rc1"
```

### Basic Usage

```rust
use digital_genome_community::{RawInput, SensoryCortex};

fn main() {
    // Create the cognitive cortex
    let cortex = SensoryCortex::new();
    
    // Feed raw data (any bytes)
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let input = RawInput::from_bytes(data);
    
    // Perceive
    let output = cortex.perceive(&input);
    
    // Analyze results
    println!("Entropy: {:.4}", output.signals.entropy);
    println!("Mean: {:.2}", output.signals.mean);
    println!("Std Dev: {:.2}", output.signals.std_dev);
    println!("Proto-agency: {}", output.proto_agency.detected);
}
```

### With Maturation

```rust
use digital_genome_community::{RawInput, SensoryCortex, MaturationConfig};

fn main() {
    let cortex = SensoryCortex::new();
    let input = RawInput::from_bytes(vec![/* your data */]);
    
    // Configure maturation
    let config = MaturationConfig::default()
        .with_max_iterations(10)
        .with_convergence_threshold(0.001);
    
    // Perceive with iterative maturation
    let output = cortex.perceive_mature(&input, &config);
    
    println!("Converged: {}", output.maturation.converged);
    println!("Iterations: {}", output.maturation.iterations_performed);
    println!("Final delta: {:.6}", output.maturation.final_delta);
}
```

### With Budget Control

```rust
use digital_genome_community::{RawInput, SensoryCortex, ComputationalBudget};

fn main() {
    let cortex = SensoryCortex::new();
    let input = RawInput::from_bytes(large_data);
    
    // Set resource limits
    let budget = ComputationalBudget::new()
        .with_max_bytes(1_000_000)
        .with_max_iterations(1000);
    
    // Perceive with budget enforcement
    match cortex.perceive_checked(&input, &budget) {
        Ok(output) => { /* process */ }
        Err(e) => { /* budget exceeded */ }
    }
}
```

---

## Core Principles

### 1. Non-Agency

The Community Edition **does not act**. It perceives, observes, comprehends, interiorizes, rationalizes, emits output, and returns to listening. All action occurs in external layers.

### 2. Absolute Veto (Non-Compensatory)

The multiplicative formula ensures that any zero score results in total rejection:

```
CP = M_P × M_N × M_C × M_M

If any M_x = 0, then CP = 0 (ABSOLUTE VETO)
```

High scores in three motors cannot compensate for failure in one.

### 3. Determinism

Same input MUST produce same output. Always. Everywhere. Any instance.

```rust
// Deterministic ID generation
let id = ActionId::new_deterministic(b"seed-001");

// Deterministic replay
let ctx = ReplayContext::from_seed(b"experiment-001");
```

### 4. Discovery Before Classification

The system learns by SEEING, not by being told. High entropy represents information, not error. The system discovers patterns in chaos rather than imposing predetermined classifications.

### 5. Axiom 0: "A falta de ação É ação"

Non-action is itself an action. The system never discards latent knowledge ("Arquivo Latente").

---

## Architecture

```
digital-genome-community/
├── src/
│   ├── lib.rs              # Root module with all exports
│   ├── core_types/         # Identifiers and base types
│   ├── sensory/            # Perceptual processing
│   │   ├── cortex.rs       # Main sensory cortex
│   │   ├── carrier.rs      # Carrier signal analysis
│   │   ├── signals.rs      # Signal aggregation
│   │   ├── pattern.rs      # Pattern detection
│   │   ├── structure.rs    # Structural analysis
│   │   ├── proto_agency.rs # Proto-agency detection
│   │   ├── state.rs        # Perceptual state machine
│   │   └── output.rs       # Output structures
│   ├── motors/             # Four cognitive motors
│   │   ├── praxis/         # Praxeological Motor (M_P)
│   │   ├── nash/           # Nash Motor (M_N)
│   │   ├── chaos/          # Chaotic Motor (M_C)
│   │   └── meristic/       # Meristic Motor (M_M)
│   ├── maturation/         # Iterative refinement engine
│   ├── budget/             # Computational budget system
│   ├── replay/             # Deterministic replay
│   ├── competition/        # Motor competition/cooperation
│   ├── completeness/       # Cognitive completeness tracking
│   ├── observability/      # Health and diagnostics
│   ├── hierarchy/          # DNA, Synapse, Neuron, Brain, Truth
│   ├── correlation/        # Co-occurrence tracking
│   ├── topology/           # Graph structures
│   ├── math/               # Mathematical foundations
│   ├── archive/            # Latent archive
│   ├── selection/          # Golden index
│   └── traits/             # Enterprise interfaces
├── tests/
│   └── integration_tests.rs # 35 integration tests
├── examples/
│   ├── from_bytes.rs       # Basic usage
│   ├── from_file.rs        # File processing
│   ├── batch_processing.rs # Batch operations
│   ├── multithread_demo.rs # Thread safety demo
│   ├── generate_datasets.rs # Synthetic data generation
│   └── rigorous_validation.rs # Validation suite
└── validation/
    ├── datasets/           # Generated test datasets
    └── EXPECTATIONS.md     # Validation expectations
```

---

## The Four Cognitive Motors

### M_P — Praxeological Motor

Derived from Ludwig von Mises' praxeology. Recognizes reproducible action cycles through reverse engineering of momentum.

**Formula:** `M_P = φ_comp × φ_coer × φ_adeq`

### M_N — Nash Motor

Based on Nash equilibrium theory. Evaluates collective equilibrium of actions for conscious agents, automata, or hybrid systems.

**Formula:** `M_N = η_eq = 1 / (1 + d̄)`

### M_C — Chaotic Motor

Based on chaos theory and dynamical systems. Maps structural possibilities, instability regions, and sensitivity to initial conditions.

**Formula:** `M_C = ψ_stab × ψ_vol`

### M_M — Meristic Motor

Proposes structural variations to increase Craft Performance. Generates non-binding suggestions.

**Formula:** `M_M = (1-w)×coherence + w×novelty`

---

## What Belongs Here vs Enterprise

| Community Edition | Enterprise Edition |
|-------------------|-------------------|
| Perception | Execution |
| Observation | Decision |
| Evaluation | Incorporation |
| CP Calculation | Governance Enforcement |
| DNA Structure | DNA Persistence |
| Truth Types | Truth Registration (Blockchain) |
| Replay Capture | Replay Storage |
| Nash Canonical | Nash Strategic (Deep Thought) |
| Single Node | Federated Swarm |

---

## Metrics

| Metric | Value |
|--------|-------|
| Lines of Code | 13,367 |
| Source Files | 37 |
| Unit Tests | 195 |
| Integration Tests | 35 |
| Total Tests | 230 |
| Examples | 6 |

---

## Building & Testing

```bash
# Build
cargo build

# Run all tests
cargo test

# Run validation suite
cargo run --example rigorous_validation

# Generate test datasets
cargo run --example generate_datasets
```

Expected: 230 tests passing, 0 warnings.

---

## Documentation

- [RELEASE-NOTES.md](RELEASE-NOTES.md) — Release details
- [CHANGELOG.md](CHANGELOG.md) — Version history
- [CONTRIBUTING.md](CONTRIBUTING.md) — Contribution guidelines
- [PHYSIOLOGY.md](PHYSIOLOGY.md) — Biological analogy explanation
- [THREADING.md](THREADING.md) — Thread safety details
- [KNOWN-VIOLATIONS.md](KNOWN-VIOLATIONS.md) — Known limitations

---

## License

Apache License 2.0

Copyright 2025 Carlos Eduardo Favini / Aitherion Labs

---

## Author

**Carlos Eduardo Favini**

23 years at CENPES/Petrobras, megaproject management (65,000+ people), IFC/BIM and CFIHOS partnerships.

ORCID: [0009-0001-6829-9358](https://orcid.org/0009-0001-6829-9358)

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

The Benevolent Dictator model applies. All contributions are welcome; final decisions rest with the project maintainer.

---

*"O conhecimento É o cérebro, não está armazenado nele."*
*Knowledge IS the brain, not stored in it.*
