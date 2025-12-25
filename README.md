# Digital Genome — Community Core

## Purpose

The Digital Genome Community Core defines the **cognitive foundations**
of the Digital Genome.

It provides deterministic, replayable, and auditable primitives for:

- memory
- evaluation
- selection
- topology
- replay

This repository **does not execute actions** and **does not control systems**.

---

## What This Repository Is

This repository is:

- a cognitive core
- a knowledge and decision modeling layer
- a source of truth for evaluation and selection logic
- an ethical and architectural boundary

It defines **how the system thinks**, not **how it acts**.

---

## What This Repository Is NOT

This repository MUST NOT:

- execute commands
- actuate physical or digital systems
- control humans or non-humans
- contain runtime loops or schedulers
- manage infrastructure, networking, or deployment
- hold authority to enforce decisions

Any code that causes side effects belongs elsewhere.

---

## Determinism and Replay

All logic defined here must be:

- deterministic
- replayable offline
- auditable after the fact

If a decision cannot be replayed, it does not belong here.

---

## Ethics and Authority

This repository defines **ethical boundaries and limits**,
but it does not enforce authority.

Authority, mandates, execution, and control
must be implemented outside of the Community Core.

The Community Core can guide.
It must never command.

---

## Governance

Changes to core concepts, invariants, or semantics
are governed by explicit processes defined in this repository.

See:
- `GOVERNANCE.md`
- `ETHICS.md`
- `RFC_PROCESS.md`

---

## Long-Term Intent

This repository is designed to remain safe, understandable,
and auditable even in the absence of its original authors.

It must remain non-coercive by design.

---

## License

See `LICENSE`.
