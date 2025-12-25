# Request for Comments (RFC)

## Purpose

The RFC directory contains all accepted and proposed
Requests for Comments that define structural, semantic,
and ethical evolution of the Digital Genome Community Core.

RFCs represent **explicit, deliberate change**.

Nothing in this directory executes code.

---

## What an RFC Is

An RFC is a formal proposal that:

- explains *why* a change is needed
- describes *what* will change
- analyzes *how* it affects invariants
- documents ethical and long-term impact

RFCs exist to make change visible and reviewable.

---

## When an RFC Is Required

An RFC is required for any change that affects:

- core invariants
- evaluation semantics
- selection logic
- replay guarantees
- ethical boundaries
- authority or mandate-related concepts
- public interfaces or contracts

If in doubt, write an RFC.

---

## RFC Lifecycle

1. Drafted as a Markdown file in this directory
2. Discussed via pull request
3. Accepted, revised, or rejected through governance
4. Preserved as immutable record if accepted

Accepted RFCs MUST NOT be modified.

---

## RFC Naming Convention

RFC files must follow this format:

```
NNNN-short-title.md
```

Where:
- `NNNN` is a zero-padded sequential number
- `short-title` is descriptive and lowercase

Example:
```
0001-golden-selection-semantics.md
```

---

## What an RFC Is NOT

An RFC is not:

- a feature request
- an implementation detail
- a patch
- an experiment
- a discussion thread

Implementation happens elsewhere.

---

## Long-Term Role

RFCs form the institutional memory of the project.

They exist to ensure that future contributors
understand not only *what* changed,
but *why* it changed.

---

## Final Note

The goal of the RFC process is not speed.

It is clarity.
