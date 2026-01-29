# Digital Genome — Community Edition

[![Version](https://img.shields.io/badge/version-0.5.1-blue.svg)](CHANGELOG.md)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![Tests](https://img.shields.io/badge/tests-266%20passing-brightgreen.svg)]()

**A synthetic cognitive core for Industry 5.0**

---

## What Is This?

The Digital Genome Community Edition is a **synthetic cognitive brain** that:

- **Perceives** raw phenomena through sensory transduction
- **Evaluates** coherence through four cognitive motors (Praxis, Nash, Chaos, Meristic)
- **Calculates** Craft Performance (CP) with absolute veto
- **Emits** DNA fingerprints as cognitive output
- **Returns to listening** — it does NOT act

This is the **cognitive core** of the Digital Genome ecosystem. Action and persistence belong to Enterprise Edition.

---

## v0.5.1 Status

| Component | Status | Reference |
|-----------|--------|-----------|
| Sensory Cortex | ✅ | E1: Perception |
| Cognitive Cycle | ✅ | E1→E6 Pipeline |
| Four Motors | ✅ | AF-10 (Quadrimotor) |
| Craft Performance | ✅ | AF-10.5 (Multiplicative) |
| DNA Emission | ✅ | SHA-256 fingerprint |
| GDO Framing | ✅ | BOF/EOF Protocol |
| Origin Markers | ✅ | AO-18 (Autorreferência) |
| Determinism | ✅ | AF-6 (1000 replays) |
| Unit Tests | ✅ | 214 passing |
| Integration Tests | ✅ | 35 passing |
| Canonical Tests | ✅ | 17 passing |
| **Total Tests** | ✅ | **266 passing** |

---

## Quick Start

### Installation

```toml
[dependencies]
digital-genome-community = "0.5.1"
```

### Basic Perception

```rust
use digital_genome_community::{RawInput, SensoryCortex};

fn main() {
    let cortex = SensoryCortex::new();
    
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let input = RawInput::from_bytes(data);
    
    let output = cortex.perceive(&input);
    
    println!("Entropy: {:.4}", output.signals.entropy);
    println!("Mean: {:.2}", output.signals.mean);
}
```

### Complete Cognitive Cycle

```rust
use digital_genome_community::{CognitiveCycle, MotorContext};

fn main() {
    let cycle = CognitiveCycle::new();
    let ctx = MotorContext::default();
    
    let data = vec![/* your bytes */];
    let output = cycle.process(&data, &ctx);
    
    println!("Motors: P={:.2} N={:.2} C={:.2} M={:.2}", 
        output.motor_scores.praxis,
        output.motor_scores.nash,
        output.motor_scores.chaos,
        output.motor_scores.meristic
    );
    println!("CP: {:.4}", output.cp_value);
    println!("Vetoed: {}", output.vetoed);
    println!("DNA: {:02x?}", &output.dna_fingerprint[..8]);
}
```

---

## Cognitive Pipeline (E1→E6)

```
Input → E1:Percepção → E2:Codificação → E3:Avaliação → E4:Integração → E5:Deliberação → E6:Emissão → DNA
```

| Stage | Name | Function |
|-------|------|----------|
| E1 | Percepção | Sensory transduction (ORIGIN_EXTERNAL) |
| E2 | Codificação | GDO framing (BOF/EOF) |
| E3 | Avaliação | Quadrimotor evaluation (P→N→C→M) |
| E4 | Integração | CP = M_P × M_N × M_C × M_M |
| E5 | Deliberação | Veto decision (CP=0?) |
| E6 | Emissão | DNA fingerprint output |

---

## The Four Cognitive Motors

| Motor | Name | Function | Reference |
|-------|------|----------|-----------|
| M_P | Praxeological | Evaluates reproducible action cycles | Mises |
| M_N | Nash | Evaluates multi-agent equilibrium (≥2 players) | Nash |
| M_C | Chaotic | Evaluates robustness to perturbations | Lyapunov |
| M_M | Meristic | Proposes structural variations (POSTERIOR) | LEI-AF-10-07 |

---

## Craft Performance Formula

```
CP = M_P × M_N × M_C × M_M

Where:
  M_x ∈ [0, 1]
  CP  ∈ [0, 1]

ABSOLUTE VETO: If any M_x = 0, then CP = 0
```

High scores in three motors **cannot** compensate for failure in one.

---

## Core Principles (B.1-B.7)

| # | Principle | Description |
|---|-----------|-------------|
| B.1 | Estado Basal | GDC is semantically null internally |
| B.2 | Não-Agência | GDC does not act, only emits |
| B.3 | Veto Absoluto | Any M=0 → CP=0 |
| B.4 | Descoberta | Observe first, categorize later |
| B.5 | Determinismo | Same input = same output |
| B.6 | Transparência | All state is auditable |
| B.7 | Fronteira | GDC doesn't persist or execute |

---

## Architecture

```
digital-genome-community/
├── src/
│   ├── cognitive/          # Cognitive cycle (E1→E6)
│   │   ├── cycle.rs        # CognitiveCycle
│   │   └── mod.rs          # TransportCode, ObservationReport
│   ├── sensory/            # Perceptual processing
│   ├── motors/             # Four cognitive motors
│   │   ├── praxis/         # M_P
│   │   ├── nash/           # M_N  
│   │   ├── chaos/          # M_C
│   │   └── meristic/       # M_M
│   ├── unl/                # Universal Notation Language
│   │   └── gd_qmn/         # GD-QMN codes (F1-F6)
│   ├── math/               # CraftPerformance, CpResult
│   └── replay/             # Deterministic replay
├── tests/
│   ├── integration_tests.rs           # 35 tests
│   └── canonical_validation_tests.rs  # 17 tests (1000 replays)
└── examples/
```

---

## Quantum-Ready Architecture

The GD-QMN is **quantum-ready by construction**:

| Gate | Requirement | Status |
|------|-------------|--------|
| QM-01 | Backend Neutrality | ✅ |
| QM-02 | Pure Functions | ✅ |
| QM-03 | Vector Operations | ⚠️ |
| QM-04 | Operator/Executor Separation | ✅ |
| QM-05 | Non-Observable Parallelism | ✅ |

See [GATES_QUANTUM_READY.md](GATES_QUANTUM_READY.md) for details.

---

## Community vs Enterprise

| Community Edition | Enterprise Edition |
|-------------------|-------------------|
| Perception | Execution |
| Evaluation | Governance |
| CP Calculation | DNA Persistence |
| DNA Structure | Blockchain Registration |
| Single Node | Federated Swarm |
| Stateless | Stateful |

---

## Building & Testing

```bash
# Build
cargo build --release

# Run all tests (266 total)
cargo test

# Run canonical validation (1000 replays)
cargo test --test canonical_validation_tests

# Run validation suite
cargo run --example canonical_validation
```

---

## Documentation

| Document | Description |
|----------|-------------|
| [PRINCIPLES.md](PRINCIPLES.md) | Canonical principles & pipeline |
| [GATES_QUANTUM_READY.md](GATES_QUANTUM_READY.md) | Q-Ready conformance |
| [LAWS_UNL_UNIVERSALITY.md](LAWS_UNL_UNIVERSALITY.md) | Semantic universality laws |
| [CHANGELOG.md](CHANGELOG.md) | Version history |

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

*"O GDC não é um programa que calcula. Ele é um cérebro que pensa dentro de leis próprias."*

*The GDC is not a program that calculates. It is a brain that thinks within its own laws.*
