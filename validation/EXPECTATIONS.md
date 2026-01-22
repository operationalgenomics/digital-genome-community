# VALIDATION EXPECTATIONS

## Digital Genome Community Edition v1.5.5
## Target: v0.1.0-RC (Adão Sintético)

---

## SECTION 1: BASIC PERCEPTION

| Test | Input | Expected Output | Justification |
|------|-------|-----------------|---------------|
| 1.1 | Empty vec | sample_count=0, entropy=0.0 | Empty input should produce null signals |
| 1.2 | 1000x 128 | entropy=0.0, std_dev=0.0 | Constant signal has zero entropy |
| 1.3 | 0..=255 | entropy > 0.99 | All unique values = max entropy |
| 1.4 | Period-10 pattern | periodicity_detected=true | Clear periodic structure |
| 1.5 | 0,255,0,255... | periodicity_detected=true | Binary alternation (v1.5.4 fix) |

---

## SECTION 2: DETERMINISM & REPLAY

| Test | Input | Expected Output | Justification |
|------|-------|-----------------|---------------|
| 2.1 | Same bytes twice | Identical signals | Perception must be deterministic |
| 2.2 | Same seed twice | Identical IDs | Replay context must be deterministic |
| 2.3 | Different bytes | Different means | Different inputs must produce different outputs |
| 2.4 | [1,2,3,4,5] vs [5,4,3,2,1] | Different means | Permutations must be distinguishable (v1.5.5 fix) |

---

## SECTION 3: COMPUTATIONAL BUDGET

| Test | Input | Expected Output | Justification |
|------|-------|-----------------|---------------|
| 3.1 | 1000 bytes with 100-byte limit | Error | Budget must enforce memory limits |
| 3.2 | 3 recursions with limit=3 | depth=3, 4th fails | Recursion tracking must be accurate |
| 3.3 | Unlimited budget | max_bytes=MAX, max_iterations=MAX | Unlimited option must exist |

---

## SECTION 4: PERCEPTUAL MATURATION

| Test | Input | Expected Output | Justification |
|------|-------|-----------------|---------------|
| 4.1 | Single-pass config | steps.len()=1 | Single pass produces exactly 1 step |
| 4.2 | Multi-pass config | steps.len()>=1 | Multiple passes should execute |
| 4.3 | Two identical calls | Independent states | Maturation is ephemeral, no persistence |

---

## SECTION 5: COGNITIVE MOTORS

| Test | Input | Expected Output | Justification |
|------|-------|-----------------|---------------|
| 5.1 | MotorType enum | 4 types | All 4 motors must exist |
| 5.2 | Scores [0.8, 0.6, 0.7, 0.5] | Praxis dominant | Highest score wins competition |
| 5.3 | Scores [0.8, 0.8, 0.8, 0.8] | agreement > 0.9 | Equal scores = high cooperation |

---

## SECTION 6: COGNITIVE OBSERVABILITY

| Test | Input | Expected Output | Justification |
|------|-------|-----------------|---------------|
| 6.1 | Default health | is_healthy=true | Default state should be healthy |
| 6.2 | Default indicators | status="Healthy" | Observability should report healthy |

---

## SECTION 7: COGNITIVE COMPLETENESS

| Test | Input | Expected Output | Justification |
|------|-------|-----------------|---------------|
| 7.1 | Complete state | is_complete=true, has_contradictions=false | Complete means all levels done |
| 7.2 | Partial state | is_complete=false | Partial means some levels incomplete |
| 7.3 | AbstractionLevel::all() | 4 levels | All abstraction levels exist |

---

## SECTION 8: EDGE CASES

| Test | Input | Expected Output | Justification |
|------|-------|-----------------|---------------|
| 8.1 | Single byte [42] | sample_count=1 | Must handle minimal input |
| 8.2 | 100x 255 | mean=255.0, max=255.0 | Constant 255 must not overflow |
| 8.3 | 0..=255 | unique_values=256 | Must count all unique values |

---

## ACCEPTANCE CRITERIA

### Gate 2 Requirements

- [ ] All 27 tests pass
- [ ] Zero failures
- [ ] Warnings are documented and acceptable
- [ ] Edge cases from v1.5.4 and v1.5.5 are validated
- [ ] Determinism is verified
- [ ] Budget enforcement is verified
- [ ] Maturation is ephemeral (no persistence)

### Human Approval Required

After running `cargo run --example rigorous_validation`, the human must:

1. Verify all tests pass
2. Review any warnings
3. Confirm the output matches this document
4. Approve advancement to Phase 3

---

## KNOWN LIMITATIONS (Acceptable)

1. **Periodicity detection**: May not detect very short or weak patterns
2. **Proto-agency**: Is a state transition, not a classification
3. **Maturation convergence**: Depends on input characteristics

---

## RESIDUAL RISKS

| Risk | Severity | Mitigation |
|------|----------|------------|
| Thread-safety in edge cases | Low | Verified by compile-time checks |
| Numerical precision | Low | IEEE 754 compliance verified |
| Performance on large inputs | Medium | Budget system prevents DoS |

---

## COMMANDS TO EXECUTE

```bash
# 1. Generate datasets
cargo run --example generate_datasets

# 2. Run rigorous validation
cargo run --example rigorous_validation

# 3. Run all tests
cargo test

# 4. Run doctests
cargo test --doc
```

---

**Document Version:** 1.0
**Date:** 2025-01-10
**Author:** Digital Genome Community
