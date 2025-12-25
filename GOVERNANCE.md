# Governance — Digital Genome Community Core

## Purpose

This document defines how decisions about the
Digital Genome Community Core are made.

Its primary goal is to prevent concentration of power,
ensure continuity, and protect ethical and architectural invariants.

---

## Scope of Governance

Governance applies to:

- repository direction
- core invariants
- ethical boundaries
- acceptance or rejection of RFCs
- maintenance of long-term intent

Governance does NOT grant authority to execute actions.

---

## Roles

### Founder

The Founder:

- defines the original intent of the system
- acts as final arbiter for core invariants
- may delegate responsibilities
- does not act as an executive authority

The Founder is a steward, not a ruler.

---

### Maintainers

Maintainers:

- review contributions
- merge approved changes
- enforce repository rules
- act as guardians of invariants

Maintainers do not define purpose.
They preserve it.

---

### Community

The Community:

- proposes improvements
- identifies bugs and risks
- participates in RFC discussions
- provides diverse perspectives

The Community advises.
It does not command.

---

## Decision Process

### Routine Changes

Routine changes (documentation, refactors, non-semantic fixes)
may be merged by Maintainers.

---

### Structural or Semantic Changes

Any change affecting:

- invariants
- ethics
- evaluation semantics
- selection logic
- replay guarantees

MUST follow the RFC process.

---

## Benevolent Authority Principle

In exceptional circumstances (e.g. critical vulnerabilities),
Maintainers may act quickly to protect the system.

Such actions must be:

- minimal
- documented
- reversible
- followed by full review

Emergency authority expires immediately after mitigation.

---

## Succession

If the Founder becomes unavailable:

- governance authority transfers to a predefined group of Maintainers
- the group acts collectively
- no single individual inherits unilateral control

Succession must preserve non-coercive design.

---

## Forking and Plurality

Forks are permitted by license.

This repository recognizes as canonical
only changes that follow this governance process.

Plurality is allowed.
Authority is not automatic.

---

## Final Principle

No individual, group, or system
may grant itself authority within this repository.

Authority exists only by explicit, limited, and reviewable mandate.
