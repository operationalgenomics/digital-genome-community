# Security Policy — Digital Genome Community Core

## Purpose

This document defines how security, safety, and integrity
issues must be reported and handled in the
Digital Genome Community Core.

The goal is rapid mitigation without compromising
ethical boundaries or architectural invariants.

---

## Scope

This policy applies to issues that may cause:

- data corruption or loss
- violation of core invariants
- loss of determinism or replayability
- ethical boundary violations
- unintended authority escalation
- unsafe or undefined behavior

---

## Responsible Disclosure

Security issues MUST NOT be reported via public issues.

Instead, contact the maintainers privately.

Contact details must be published
in the repository settings or documentation.

---

## Required Information

A security report should include:

- description of the issue
- affected components
- steps to reproduce (if available)
- potential impact
- suggested mitigation (optional)

Incomplete reports may delay response.

---

## Response Commitment

Maintainers commit to:

- acknowledge reports within a reasonable timeframe
- assess severity promptly
- prioritize critical issues

Critical issues may be addressed immediately.

---

## Emergency Mitigation

In exceptional cases, maintainers may apply
minimal emergency fixes to prevent harm.

Such fixes must:

- be narrowly scoped
- preserve determinism and replayability
- avoid semantic expansion
- be documented after mitigation

Emergency authority expires immediately
after the issue is contained.

---

## Disclosure and Transparency

After resolution:

- a public summary may be provided
- sensitive details may remain undisclosed
- lessons learned should be documented when appropriate

Transparency must not increase risk.

---

## Prohibited Practices

Under no circumstances may a security fix:

- introduce execution or control logic
- bypass ethical boundaries
- expand authority
- reduce auditability
- alter historical data silently

Security is not a justification for power.

---

## Final Note

Security is a shared responsibility.

Speed matters.
Integrity matters more.
