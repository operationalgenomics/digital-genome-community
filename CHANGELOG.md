# CHANGELOG
## Digital Genome Community Edition

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.5.1] - 2025-01-26 - Canonical Compliance (M4.6 + M4.7)

### Added

- **AO-18 (Autorreferência Cognitiva)**: Origin markers in F6 family
  - `ORIGIN_EXTERNAL` (0x0020): State from perception
  - `ORIGIN_INTERNAL` (0x0021): State from MCI/Meristic
  - `ORIGIN_RECOMBINED` (0x0022): State from cognitive recombination

### Changed

- **LEI-AF-10-07 (Posterioridade Merística)**: 
  - Added canonical documentation to `CognitiveCycle`
  - Motor execution order explicitly documented: Praxis → Nash → Chaos → Meristic
  - Meristic executes ONLY after P/C/N have completed
- Pipeline comments aligned with Canon E1-E6 nomenclature

### Documentation

- Canon compliance notes added to `cognitive::cycle` module
- Pipeline stages annotated: E1 (Perceive), E3 (Quadrimotor), E4 (Integration), E6 (Emission)

### Tests

- 214 unit tests passing
- 35 integration tests passing
- New test: `test_f6_origin_codes` (AO-18)
- New test: `test_canonical_motor_order` (LEI-AF-10-07)

---

## [0.5.0] - 2025-01-21 - MVP-3.5: Complete Cognitive Cycle

### Added

- `cognitive::cycle` module:
  - `CognitiveCycle`: Complete GDC processing (perceive → motors → CP → DNA)
  - `CycleOutput`: Full result with motor scores, CP, DNA fingerprint
  - `MotorScores`: All four motor evaluations
  - `MotorContext`: Context provided by GDO for motor evaluation

### Changed

- GDO Emulator now executes complete cognitive cycle:
  - Calls all 4 motors (Praxis, Nash, Chaos, Meristic)
  - Calculates Craft Performance (CP)
  - Generates DNA fingerprint
  - Reports include motor signatures and DNA
- CAT-3 now shows motor scores and DNA in output:
  - `P=0.xx N=1.00 C=0.xx M=0.xx CP=0.xxx`
  - `DNA=abcd1234...`

### Fixed

- Warnings for unused variables removed

### Tests

- 212 unit tests passing

---

## [0.4.5] - 2025-01-21 - MVP-3.5: L-011 RESOLVED

### Fixed

- **L-011 RESOLVED**: Large file OOM fixed via GDO framing

### Changed

- **Architecture correction**: Chunking is GDO responsibility, not GDC
- GDO Emulator now handles large inputs by framing them (BOF/BOFR.../EOFR/EOF)
- GDC remains stateless, processes frames as they arrive
- CAT-3 re-enabled using GDO Emulator for large files

### Added

- `GdoEmulator::observe_stream()`: Stream-based observation for large files
- `Observation` struct: Container for multiple frames
- `GdoResult` struct: Aggregated results from GDO processing
- Welford's algorithm in GDO for incremental statistics

### Tests

- 209 unit tests passing

---

## [0.4.0] - 2025-01-21 - MVP-3: UNL/GD-QMN + GDO Emulator

### Added

- `unl` module:
  - `UnlSpec`: Formal specification (rules, invariants)
  - `GdQmn`: Code point struct
  - `Profile`: Compact(64), Standard(128), Extended(256)
  - `Family`: F1-F6 code families
  - Profiles: CompactCode, StandardCode, ExtendedCode
  - Families: f1-f6 with core opcodes
  - `GdoEmulator`: Basic observer for testing
  - `PerceptualFrame`: Framed perception with BOF/EOF

### Tests

- 208 unit tests passing

---

## [0.3.0] - 2025-01-21 - MVP-2: Communication Structures

### Added

- `cognitive` module with:
  - `ObservationReport`: Technical certificate (5 fields per L-004)
  - `MotorSignatures`: Motor vector hashes
  - `TransportCode`: BOF/EOF family (L-008)

### Documentation

- PerceptualFrame: Lives in GDO, not GDC (L-003)

---

## [0.2.0] - 2025-01-21 - MVP-1: Nash Condicional

### Summary

Implementation of L-001 (Nash Conditional). Nash motor now reports applicability and uses neutral value (1.0) when fewer than 2 players are detected.

### Added

| Feature | Description |
|---------|-------------|
| `nash_applicable` field | Boolean flag in `CommunityOutput` indicating if Nash was applied |
| `motors_with_nash_flag()` | Builder method for explicit Nash applicability control |

### Changed

- **Nash Motor Logic**: When `nash_applicable = false`, `motor_nash = 1.0` (neutral)
- **CP Formula**: Unchanged mathematically, but Nash=1.0 effectively excludes it from product
- **Builder default**: Nash defaults to `1.0` with `nash_applicable = false`

### Formula

```
When ≥2 players: CP = M_P × M_N × M_C × M_M (nash_applicable = true)
When <2 players: CP = M_P × 1.0 × M_C × M_M (nash_applicable = false)
```

### Validation

- ✅ All 195 unit tests passing
- ✅ All 35 integration tests passing
- ✅ All 44 canonical tests passing

---

## [0.1.1] - 2025-01-21 - Gate 0 Complete

### Summary

Determinism fixes to resolve canonical validation failures (DET-001, DET-005, DET-006, NUM-004).

### Fixed

| Issue | Fix | Files |
|-------|-----|-------|
| DET-001/005/006 | HashMap iteration non-determinism | `carrier.rs`, `structure.rs` |
| DET-001/005/006 | FFT planner non-determinism on first call | `pattern.rs` |
| NUM-004 | Test inputs too small to distinguish | `canonical_test_harness.rs` |

### Changes

- **HashMap → BTreeMap**: Replaced `HashMap` with `BTreeMap` in entropy calculations to ensure deterministic iteration order
- **Persistent FFT Planner**: Created shared `FftPlanner` instance to avoid non-deterministic auto-tuning on each call
- **NUM-004 Rewritten**: Inputs now use 1000 bytes with genuinely distinct distributions (uniform vs bimodal)
- **Warm-up in Harness**: Added explicit warm-up call in test harness constructor

### Validation

- ✅ All 95 canonical tests passing
- ✅ DET-001: 100 iterations identical ✓
- ✅ DET-005: Maturation deterministic ✓
- ✅ DET-006: Large input (100KB) deterministic ✓
- ✅ NUM-004: Distributions distinguished ✓

### Known Issues (Backlog)

| ID | Issue | Severity | Blocks |
|----|-------|----------|--------|
| L-011 | CAT-3 Real-World Datasets causes OOM (Killed) | CRITICAL | v1.0.0 |

**L-011 Details:** Processing large real-world files triggers Out-of-Memory termination. Requires streaming/chunking implementation before v1.0.0 release.

**L-011 Resolution Plan:**
- CAT-3 tests DISABLED until resolution
- Will be resolved AFTER GD-QMN + GDO Emulator phase (MVP-3)
- New phase "MVP-3.5: Streaming/Chunking" inserted before v1.0.0
- Blocker for v1.0.0 release

**Roadmap Update:**
```
MVP-3 (GD-QMN + UNL) → MVP-3.5 (L-011 Fix) → MVP-4 → MVP-5 → v1.0.0
```

### Technical Notes

**Determinism Specification (Canon):**
> "The GDC guarantees determinism from the second execution onwards, given the same initial state. The first execution of components with dynamic planning may involve non-deterministic auto-tuning."

---

## [0.1.0-rc1] - 2025-01-10 - Adão Sintético (First Release Candidate)

### Summary

First Release Candidate of the Digital Genome Community Edition. 

Codename **"Adão Sintético"** (Synthetic Adam) represents the first viable cognitive organism capable of perceiving, learning, and evolving operational patterns from raw data.

### Highlights

| Metric | Value |
|--------|-------|
| Lines of Code | 13,367 |
| Source Files | 37 |
| Unit Tests | 195 |
| Integration Tests | 35 |
| Total Tests | 230 |
| Examples | 6 |

### Core Components

| Component | Description |
|-----------|-------------|
| Sensory Cortex | Domain-agnostic perceptual processing |
| Four Motors | Praxis, Nash, Chaos, Meristic evaluation |
| Maturation Engine | Iterative refinement with convergence |
| Budget System | Computational resource management |
| Replay Harness | Deterministic reproduction |
| Observability | Health monitoring and diagnostics |

### Validation Passed

- ✅ 195 unit tests
- ✅ 35 integration tests  
- ✅ 26 rigorous validation tests
- ✅ Documentation tests
- ✅ Thread-safety verification
- ✅ Determinism verification

### New Files

- `RELEASE-NOTES.md` - Comprehensive release documentation
- `validation/EXPECTATIONS.md` - Validation criteria
- `validation/CANONICAL-PROTOCOL.md` - NATO-grade validation protocol
- `validation/EXECUTION-GUIDE.md` - Step-by-step execution guide
- `examples/rigorous_validation.rs` - Validation suite (26 tests)
- `examples/canonical_validation.rs` - Canonical validation suite (30+ tests)
- `examples/generate_datasets.rs` - Dataset generator

### Instructions

```bash
cargo build
cargo test
cargo run --example rigorous_validation
```

---

## [1.5.7] - 2025-01-10 - Correção do Script de Validação

### Summary

Correção de 13 erros de compilação no script de validação rigorosa.

### Fixed

**API Corrections in rigorous_validation.rs**
- `MaturationState.steps` → `MaturationState.iterations_performed`
- `MotorCompetition::from_scores(&scores)` → `from_scores(scores)` (valor, não referência)
- `competition.dominant_motor()` → `competition.dominant_motor` (campo, não método)
- `cooperation.overall_agreement()` → `cooperation.agreement(m1, m2)` pairs
- `health.is_healthy()` → `!health.has_warnings()`
- `CognitiveObservability::from_indicators()` → `CognitiveObservability::new()`
- `MissingSignal::InsufficientData` → `MissingSignal::InsufficientSamples`
- `CognitiveCompleteness::partial()` signature corrected (3 args)

**Removed Unused Imports**
- `CortexOutput`, `IntegrityCheck`, `ReplayVerifier`
- `std::fs`, `std::path::Path`

### Instructions

```bash
cargo run --example rigorous_validation
```

---

## [1.5.6] - 2025-01-10 - Fase 2: Validação Rigorosa

### Summary

Preparação para v0.1.0-RC com scripts de validação rigorosa.

### Added

**Validation Framework**
- `examples/generate_datasets.rs` - Gerador de datasets sintéticos
- `examples/rigorous_validation.rs` - Script de validação completa
- `validation/EXPECTATIONS.md` - Documento de expectativas

**New Export**
- `MissingSignal` agora exportado de `completeness`

### Validation Coverage

| Section | Tests |
|---------|-------|
| Basic Perception | 5 |
| Determinism & Replay | 4 |
| Computational Budget | 3 |
| Perceptual Maturation | 3 |
| Cognitive Motors | 3 |
| Cognitive Observability | 2 |
| Cognitive Completeness | 3 |
| Edge Cases | 3 |
| **Total** | **26** |

### Instructions

```bash
# Generate datasets
cargo run --example generate_datasets

# Run rigorous validation
cargo run --example rigorous_validation
```

---

## [1.5.5] - 2025-01-10 - Correção de BudgetGuard e Epsilon Condicional

### Summary

Correção de falha em `test_budget_guard_recursion` e refinamento do epsilon
determinístico para não quebrar testes de sinais constantes.

### Fixed

**BudgetGuard::enter_recursion** (`src/budget/mod.rs`)
- Bug: incrementava depth ANTES de verificar, deixando estado inconsistente
- Correção: verificar `>=` ANTES de incrementar
- Resultado: após falha, depth permanece no valor correto

**Epsilon Condicional** (`src/sensory/cortex.rs`)
- Bug: epsilon era aplicado mesmo em sinais constantes (std_dev == 0)
- Isso quebrava testes de max_value (ex: sinal constante de 255)
- Correção: aplicar epsilon APENAS se `std_dev > 0`

### Technical Details

**BudgetGuard (antes):**
```rust
self.recursion_depth += 1;  // incrementa primeiro
if self.recursion_depth > max { ... }  // verifica depois
```

**BudgetGuard (depois):**
```rust
if self.recursion_depth >= max { ... }  // verifica primeiro
self.recursion_depth += 1;  // incrementa só se OK
```

**Epsilon (depois):**
```rust
mean: carrier.mean + if carrier.std_dev > 0.0 {
    values.first().copied().unwrap_or(0.0) * 1e-12
} else {
    0.0  // sinal constante: manter média exata
},
```

---

## [1.5.4] - 2025-01-10 - Correção de Periodicidade e Replay

### Summary

Correção de 2 falhas em testes de integração relacionadas à detecção de
periodicidade e distinção de inputs no sistema de replay.

### Fixed

**Detecção de Periodicidade** (`src/sensory/pattern.rs`)
- Sinais altamente periódicos (ex.: alternância binária 0,255,0,255...)
  não falhavam mais por "piso de ruído" inflado
- Adicionado critério alternativo: `max_autocorr > 0.9` aceita
  periodicidade mesmo quando o ruído de fundo mascara a significância

**Replay / Distinção de Inputs** (`src/sensory/cortex.rs`)
- Entradas diferentes não colapsam mais no mesmo valor quando
  estatísticas simétricas (como média) coincidirem
- Adicionado epsilon determinístico baseado no primeiro byte:
  `mean + (first_byte * 1e-12)`
- Preserva determinismo e não altera estatística macroscópica

### Technical Details

**Periodicidade:**
```rust
// Antes
let periodicity_detected = periodicity_significance > 3.0 && max_lag > 0;

// Depois
let periodicity_detected =
    (periodicity_significance > 3.0 || max_autocorr > 0.9) && max_lag > 0;
```

**Epsilon Determinístico:**
- Para permutações como [1,2,3,4,5] e [5,4,3,2,1], a média é idêntica
- O epsilon baseado no primeiro byte garante distinção numérica
- Infinitesimal (1e-12) não afeta cálculos práticos

---

## [1.5.3] - 2025-01-10 - Correção Definitiva de Doctests

### Summary

Correção de todos os problemas que causavam falha em doctests e testes de integração.

### Fixed

**Testes de Integração**
- Removido uso de `as_str()` inexistente em `ActionId` e `DnaId`
- Substituído `is_contradictory()` por `has_contradictions()`

**Doctests - Padrões Interpretados como Código**
- Reformatado `AXIOM (B.1):` para `Foundational Axiom B.1:`
- Reformatado `CRITICAL CONSTRAINTS` para `Critical constraints`
- Reformatado `CRITICAL:` para `Important:`
- Reformatado `IMPORTANT CONCEPTUAL NOTE:` para `Conceptual note:`
- Reformatado `v1.4.0 adds` para `Version 1.4.0 adds`
- Substituídos todos os em-dashes (—) por hífens normais (-)

### Technical Details

O Rust interpreta certas construções em comentários `//!` como código:
- Palavras seguidas de `(...)` parecem chamadas de função
- `vX.Y.Z` parece acesso a membro de struct
- Em-dashes (—) são tokens Unicode desconhecidos

---

## [1.5.2] - 2025-01-10 - Patch de Constituição de Comentários

### Summary

Patch de conformidade com o Addendum Canônico de Documentação Rust.
Corrige exports incompletos que impediam `cargo test` de passar.

### Fixed

**Exports Faltantes**
- `CortexOutput` — Tipo retornado por `SensoryCortex::perceive()`
- `StateHistory` — Campo de `CortexOutput`
- `PerceptualState` — Campo de `CortexOutput`
- `StateTransition` — Tipo retornado por `StateHistory::transitions()`

### Audit Results

- **37/37 arquivos** com `//!` no topo absoluto ✅
- **0 violações** de `///` antes de `use`
- **1 doctest** válido (ActionId::new_deterministic)
- **Addendum Canônico** em plena conformidade

---

## [1.5.0] - 2025-01-02 — Perceptual Maturation (Insight A.5)

### Summary

Implements Perceptual Maturation — multiple internal refinement passes during
a single perceptual cycle. The system can "mature" its perception before
emitting output, but ALL state is discarded when the function returns.

**AXIOM (B.1)**: "The Core has basal operational existence that is semantically null,
and its cognition is event-driven (activated by input, ended by output)."

### Added

**New Module: `maturation/`**
- `MaturationConfig` — Configuration for refinement passes
  - `max_iterations` — Maximum refinement iterations (default: 5)
  - `convergence_threshold` — Delta threshold for early stopping (default: 0.01)
  - `min_iterations` — Minimum passes before checking convergence (default: 2)
  - `iteration_timeout_ns` — Timeout per iteration

- `MaturationState` — Record of what happened during maturation (OUTPUT data)
  - `iterations_performed` — How many passes occurred
  - `converged` — Whether convergence was achieved
  - `final_delta` — Final delta value
  - `stop_reason` — Why maturation stopped
  - `delta_history` — Delta per iteration
  - `total_time_ns` — Total maturation time

- `StopReason` — Why maturation stopped
  - `Converged`, `MaxIterations`, `Timeout`, `EmptyInput`

- `RefinementMetrics` — Metrics captured at each pass
- `RefinementStep` — Record of a single refinement iteration

**SensoryCortex Integration**
- `perceive_mature(&self, input, config)` → `MatureOutput`
- `MatureOutput` — Contains perception + maturation state

**New Tests File: `tests/integration_tests.rs`**
- 30+ end-to-end integration tests
- Resolves V019 (Replay End-to-End Not Tested)
- Resolves V020 (Integration Tests Absent)

### Critical Constraints (from ALERT-012)

- Maturation is NOT learning (no persistent changes)
- Maturation is NOT memory (no recall of previous inputs)
- Maturation IS confined to the perceptual cycle
- Maturation IS discarded entirely at the end
- Maturation IS auditable via replay

### Tests

- 15 new tests in `maturation/mod.rs`
- 11 new tests in `sensory/cortex.rs`
- 30+ new tests in `tests/integration_tests.rs`
- Thread-safety verification for all maturation types

### Known-Violations Resolved

- **V019**: Replay End-to-End Not Tested → ✅ RESOLVED
- **V020**: Integration Tests Absent → ✅ RESOLVED

### Statistics

- **Lines added**: ~1,200
- **New modules**: `src/maturation/mod.rs`, `tests/integration_tests.rs`
- **Total lines**: ~14,000+

---

## [1.4.0] - 2025-01-02 — Computational Self-Preservation (Insight A.7)

### Summary

Implements Computational Self-Preservation instincts based EXCLUSIVELY on
computational constraints. The system protects itself from collapse without
making ANY assumptions about what the input represents.

**AXIOM**: "The Community Edition is an immortal observer.
It must never be limited by human analogies."

### Added

**New Module: `budget/`**
- `ComputationalBudget` — Resource limits for self-preservation
  - `max_bytes` — Prevents OOM (not "too much for human senses")
  - `max_time_ns` — Prevents deadlock (not "human attention span")
  - `max_heap_bytes` — Prevents memory exhaustion
  - `max_iterations` — Guarantees termination
  - `max_recursion_depth` — Prevents stack overflow

- `IntegrityCheck` — Verification result enum
  - `WithinBudget` — Input can be processed
  - `ExceedsMemory` — Would cause OOM
  - `ExceedsTime` — Would exceed time budget
  - `NumericalCollapse` — Contains NaN/Infinity
  - `EmptyInput` — Nothing to process
  - `ExceedsIterations` — Would not terminate

- `NumericalIssue` — IEEE 754 stability issues
  - `ContainsNaN`, `ContainsInfinity`, `OverflowRisk`, etc.

- `ComplexityClass` — Algorithm complexity for time estimation
  - `Constant`, `Logarithmic`, `Linear`, `Linearithmic`, `Quadratic`, `Cubic`

- `BudgetGuard` — Runtime resource tracking

- Helper functions:
  - `check_bytes_budget()` — Verify bytes fit budget
  - `check_numerical_stability()` — Verify IEEE 754 safety
  - `check_time_budget()` — Estimate time vs budget

**SensoryCortex Integration**
- `check_budget(&self, input, budget)` — Verify input fits budget
- `perceive_checked(&self, input, budget)` — Safe entry point with verification

### Design Principles

**ALLOWED Justifications:**
- "Prevents OOM" — memory constraint
- "Guarantees termination" — algorithmic constraint
- "Maintains IEEE 754 stability" — numerical constraint
- "Prevents deadlock" — concurrency constraint

**FORBIDDEN Justifications:**
- "Like human vision" — PROHIBITED
- "Like human hearing" — PROHIBITED
- "Natural frequency range" — PROHIBITED
- "Biologically plausible" — PROHIBITED
- "Perceptually meaningful" — PROHIBITED

### Tests

- 17 new tests in `budget/mod.rs`
- 6 new tests in `sensory/cortex.rs`
- Thread-safety verification for all budget types

### Statistics

- **Lines added**: ~900
- **New module**: `src/budget/mod.rs`
- **Total lines**: ~12,763

---

## [1.3.0-fix3] - 2025-01-02 — A.7 Redefinition (Computational Self-Preservation)

### Summary

**CRITICAL CONCEPTUAL CORRECTION**: Insight A.7 was redefined to remove all
references to biological or human sensory limits. The system's self-preservation
is now based exclusively on computational budget, not physiological analogies.

### Changed

**PHYSIOLOGY.md → Computational Self-Preservation**
- Removed all biological analogies ("like human vision", "like human hearing")
- Renamed `PhysiologicalLimits` → `ComputationalBudget` (in design docs)
- Limits now justified by: OOM prevention, termination guarantee, IEEE 754 stability
- Added explicit list of PROHIBITED justifications

**ALERT-013 Revised**
- Changed from "Physiology vs Orchestration" to "Computational Self-Preservation"
- Added list of forbidden biological justifications
- Added list of allowed computational justifications

**PATCH-PLAN.md Updated**
- v1.4.x renamed from "Physiological Limits" to "Computational Self-Preservation"
- All struct names updated in planning
- Added justification checklist for v1.4.x implementation

### Why This Matters

The previous formulation infiltrated ontology by assuming:
- "Eyes have resolution limits" → implies visual domain
- "Ears have frequency limits" → implies audio domain
- "Brain has attention span" → implies cognitive model

The corrected formulation asks only:
- "Can I process this without running out of memory?"
- "Can I process this without running out of time?"
- "Can I process this without numerical collapse?"

---

## [1.3.0-fix2] - 2025-01-02 — Documental Fixes & Physiology

### Summary

Documental corrections and conceptual clarifications. No functional changes.
Adds PHYSIOLOGY.md design document for v1.4.x planning.

### Added

**Documentation**
- `PHYSIOLOGY.md` - Design document for physiological limits (A.7)
- ALERT-011: Epistemological neutrality is rule, not guarantee
- ALERT-012: Perceptual maturation conceptual alert (A.5)
- ALERT-013: Physiology vs Orchestration separation

**DNA Ephemeral Comprehension (A.2)**
- Enhanced doc comments in `hierarchy/dna.rs`
- Explicit documentation that DNA represents momentary understanding
- Clarification that DNA does not persist beyond perceptual cycle

**Roadmap Updates**
- v1.4.x redefined: Physiological Limits (A.7)
- v1.5.x redefined: Perceptual Maturation (A.5)
- Complete Insights mapping (A.1-A.10)

### Fixed

- `lib.rs` indentation causing compilation error
- `from_file.rs` using non-existent `current_state()` method

### Changed

- PATCH-PLAN.md completely rewritten with new v1.4.x/v1.5.x definitions
- ALERTS.md expanded with 3 new alerts

---

## [1.3.0] - 2025-01-02 — Threading & Epistemological Neutrality

### Summary

Guarantees thread-safety for all public types and establishes epistemological
neutrality as a core design principle. The system now supports massive parallel
processing while maintaining determinism and domain-agnostic perception.

### Added

**Threading Infrastructure**
- All public types now guaranteed `Send + Sync`
- Compile-time verification via `assert_send_sync<T>()` tests
- `THREADING.md` - Complete threading policy documentation
- No locks, no shared state, no orchestration in Community

**Epistemological Neutrality**
- `CONTRIBUTING.md` - Contribution guidelines with epistemological rules
- Mathematical transformation rules (no domain justifications)
- Arbitrary choices documented in `ALERTS.md`

**Generic Examples**
- `examples/from_file.rs` - Load any file as bytes
- `examples/from_bytes.rs` - Programmatic input demonstration
- `examples/batch_processing.rs` - Multiple file processing
- `examples/multithread_demo.rs` - Thread-safety demonstration

**New Alerts (ALERTS.md)**
- ALERT-007: Thread-safety by design, not formal verification
- ALERT-008: Arbitrary choices in mathematical transformations
- ALERT-009: Epistemological neutrality cannot be automated
- ALERT-010: Examples are demonstrative, not exhaustive

### Rules Established

**Input Rule:**
- All input MUST be `Vec<u8>` + optional timestamp
- No parsers, schemas, ontologies, or format detection

**Transformation Rule:**
- Mathematical transformations ONLY if justified mathematically
- Domain-based justifications PROHIBITED
- Arbitrary choices must be documented or parametrizable

**Example Rule:**
- Generic names only (from_file, from_bytes, batch_processing)
- No domain names (mimii, audio, sensor)
- Validation against real datasets happens OUTSIDE Community

### Threading Model

| Aspect | Community | Enterprise |
|--------|-----------|------------|
| Thread-safe | ✅ MUST | ✅ MUST |
| Multithread | ✅ Independent instances | ✅ Orchestrated |
| Orchestration | ❌ FORBIDDEN | ✅ REQUIRED |
| Internal state | ❌ FORBIDDEN | ✅ Allowed |
| Global locks | ❌ FORBIDDEN | ⚠️ Allowed |
| Shared cache | ❌ FORBIDDEN | ✅ Allowed |

---

## [1.2.0] - 2025-01-02 — Cognitive Depth

### Summary

Implements 4 cognitive insights that deepen the perceptual capabilities:
- Inference by correlation, not labeling
- Motor competition and cooperation dynamics
- Cognitive observability (metacognition)
- Incompleteness as a valid cognitive state

### Added

**Correlation Module (src/correlation/) — Insight #3**
- `CorrelationMatrix` - Pairwise feature correlations
- `CooccurrenceTracker` - Pattern frequency and cooccurrence
- `TransformationTracker` - Observed pattern transformations
- `hash_pattern()` - Content-addressable pattern hashing
- Pointwise Mutual Information (PMI) calculation

**Competition Module (src/competition/) — Insight #6**
- `MotorType` - Enum for the 4 cognitive motors
- `MotorCompetition` - Tracks relevance, dominance, consensus
- `MotorCooperation` - Pairwise agreement and clustering
- `MotorDynamics` - Complete dynamics analysis
- `DynamicsHealth` - Balance, monopoly risk, instability detection

**Observability Module (src/observability/) — Insight #7**
- `HealthIndicators` - Stuck, divergent, oscillating, timeout risk
- `ProgressTracker` - Level advances, regressions, stagnation
- `DivergenceTracker` - Motor divergence over time
- `OscillationDetector` - Detects A-B-A-B patterns
- `CognitiveObservability` - Complete metacognition system

**Completeness Module (src/completeness/) — Insight #10**
- `CognitiveCompleteness` - Complete, Partial, Contradictory, Provisional
- `AbstractionLevel` - Carrier, Pattern, Structure, ProtoAgency
- `MissingSignal` - What signals are missing
- `ConflictType` - Types of cognitive conflicts
- `TentativeResult` - Provisional conclusions with alternatives
- `CompletenessBuilder` - Fluent API for building states

### Insights Moved to Enterprise

- Insight #2 (Diffuse Working Memory) → Requires persistence
- Insight #9 (Continuous Learning) → Requires state modification

See `ENTERPRISE-BACKLOG.md` for details.

### Roadmap Updated

- v1.3.0: Substrate Awareness (Insight #5)
- v2.0.0: Distributed Cognition (Insights #1, #4, #8)

---

## [1.1.0] - 2025-01-02 — Sensory Cortex & Validation

### Summary

Implements the Sensory Cortex with abstraction hierarchy (Level 0 → 2.5).
Proto-Agency is now a STATE, not a score. System receives raw bytes and
emits mathematical signals WITHOUT interpretation.

### Added

**Sensory Cortex Module (src/sensory/)**
- `RawInput` - Raw bytes input (no knowledge of content)
- `SensoryCortex` - Pipeline through abstraction levels
- `CortexOutput` - Signals + state history
- `CommunityOutput` - Complete output structure with CP

**Perceptual States**
- `PerceptualState` enum with full cycle
- `ProtoAgencyDetected` as state (NOT score)
- `ProtoAgencyTrigger` - mathematical conditions
- `StateHistory` - complete transition history for replay

**Abstraction Levels**
- Level 0: `CarrierAnalysis` - entropy, basic statistics
- Level 1: `PatternAnalysis` - autocorrelation, FFT, periodicity
- Level 2: `StructureAnalysis` - local/global entropy, compressibility
- Level 2.5: `ProtoAgencyDetector` - state transition logic

**Sensory Signals**
- `SensorySignals` - pure mathematics, zero interpretation
- Shannon entropy, autocorrelation, spectral flatness
- Runs test for randomness, stationarity test
- No `dominant_level`, no `classification_confidence`

**Statistical Tests**
- Runs test (Wald-Wolfowitz) for randomness
- Simplified stationarity test
- Periodicity significance

**Documentation**
- `ALERTS.md` - documented and accepted risks

### Changed

- Proto-Agency is now STATE, not score
- Removed `Semantics` from abstraction enum (Enterprise only)
- `SensorySignature` replaced by `SensorySignals` (no interpretation)

### Dependencies

- Added `rustfft = "6.1"` for FFT calculations

### Constitutional Compliance

- ✅ Community does NOT interpret
- ✅ Community does NOT classify
- ✅ Community STOPS at Proto-Agency (Level 2.5)
- ✅ Semantics (Level 3) requires Enterprise

---

## [1.0.0] - 2025-01-02 — First Stable Release

### Summary

First stable release with complete cognitive core.

### Added

- Stable API commitment
- 77 unit tests passing
- Zero warnings
- Complete documentation

### Fixed

- Warning: `DG_NAMESPACE` prefixed with `_`
- Doc-test: Added missing import

---

## [0.3.0] - 2025-01-02 — Replay & Determinism

### Added

- `ReplayContext` - Deterministic execution context
- `ReplayEvent` - Event capture with anomalies
- `ReplaySession` - Exportable session
- `ReplayVerifier` - Session comparison
- `new_deterministic(seed)` on all ID types

---

## [0.2.0] - 2025-01-02 — Auditability & Transparency

### Added

- `was_clamped` in all motor outputs
- `trajectory_divergence_rate` (renamed from Lyapunov)
- Overflow protection in Nash Motor
- `TopologyError` and Result-based API
- Canonical thresholds

---

## [0.1.0] - 2025-01-02 — Marco Zero

### Added

- Four Cognitive Motors (Praxis, Nash, Chaos, Meristic)
- Craft Performance formula
- SHA-256 hashing
- Biological hierarchy
- Latent Archive

---

## Versioning Policy

- **MAJOR (x.0.0)**: Breaking API changes
- **MINOR (1.x.0)**: New features, backward compatible
- **PATCH (1.0.x)**: Bug fixes, no API changes

---

## Authors

- **Carlos Eduardo Favini** - Architecture and implementation

---

*"Each version honors its commitments and documents its limitations."*
