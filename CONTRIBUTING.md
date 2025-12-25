# Contributing — Digital Genome Community Core

Thank you for your interest in contributing to the
Digital Genome Community Core.

This repository is intentionally constrained.
Not all contributions are appropriate here.

---

## What You Can Contribute

Contributions are welcome in the following areas:

- documentation improvements
- clarification of concepts and terminology
- bug fixes that preserve semantics
- performance improvements without semantic change
- tests and verification tools
- deterministic replay tooling
- analysis of edge cases and failure modes

---

## What Requires an RFC

Any change that affects:

- core invariants
- evaluation semantics
- selection logic
- ethical boundaries
- authority-related concepts
- replay guarantees

MUST follow the RFC process before implementation.

See `RFC_PROCESS.md`.

---

## What Is Not Accepted

The following contributions will not be accepted:

- execution logic
- runtime engines or schedulers
- actuation or control code
- authority enforcement mechanisms
- autonomous decision loops
- networking or infrastructure code
- features that bypass ethical boundaries

Such code does not belong in the Community Core.

---

## Contribution Process

1. Fork the repository
2. Create a focused branch
3. Make minimal, well-documented changes
4. Submit a pull request
5. Participate in review and discussion

All contributions are reviewed for:
- technical correctness
- alignment with invariants
- ethical compliance
- long-term impact

---

## Determinism Requirement

All contributions must preserve determinism.

If a change introduces non-determinism,
it will be rejected.

If a decision cannot be replayed,
it does not belong here.

---

## Tests and Verification

Contributors are strongly encouraged to include:

- unit tests
- property-based tests
- replay validation tests

Tests must not depend on external systems.

---

## Communication

Be precise.
Be respectful.
Be patient.

This is a long-term project.
Stability matters more than speed.

---

## Final Note

This repository values understanding over novelty.

If you are unsure whether a contribution belongs here,
open a discussion before writing code.
