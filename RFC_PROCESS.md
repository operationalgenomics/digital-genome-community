# RFC Process — Digital Genome Community Core

## Purpose

The RFC (Request for Comments) process governs
all significant or foundational changes to the
Digital Genome Community Core.

Its purpose is to protect:

- core invariants
- ethical boundaries
- determinism and replayability
- long-term architectural intent

---

## When an RFC Is Required

An RFC is mandatory for any change that affects:

- core invariants
- evaluation semantics
- selection logic
- replay guarantees
- ethical boundaries
- authority or mandate-related concepts
- public interfaces or contracts

If there is doubt, an RFC is required.

---

## RFC Lifecycle

### 1. Draft

- The author creates a new RFC in the `/rfc/` directory
- File name format: `NNNN-short-title.md`
- RFCs must be written in clear technical English

---

### 2. Discussion

- The RFC is submitted via pull request
- Maintainers and the community provide feedback
- Revisions are allowed during this phase

---

### 3. Decision

- The Founder or delegated governance body
  evaluates the RFC
- Possible outcomes:
  - Accept
  - Request changes
  - Reject

---

### 4. Acceptance

- Accepted RFCs are immutable
- Acceptance does not mandate immediate implementation
- Implementation must strictly follow the accepted RFC

---

## RFC Structure

Each RFC MUST include:

1. Summary
2. Motivation
3. Scope
4. Detailed Design
5. Impact on Invariants
6. Ethical Considerations
7. Determinism and Replay Analysis
8. Alternatives Considered
9. Backward Compatibility

Incomplete RFCs will not be accepted.

---

## Implementation Rules

- No implementation may deviate from an accepted RFC
- Implementation PRs must reference the RFC
- Experimental code must not enter the Community Core

---

## Emergency Changes

Emergency fixes may bypass the RFC process
only for critical bugs or security issues.

Such fixes must:

- be minimal
- preserve semantics
- be documented
- be followed by an RFC if structural impact exists

---

## Rejection and Forking

Rejected RFCs may be revised and resubmitted.

Forks are permitted by license,
but this repository recognizes as canonical
only changes that follow the RFC process.

---

## Final Principle

The system evolves by understanding,
not by momentum.

The RFC process exists to make
change explicit, reviewable, and intentional.
