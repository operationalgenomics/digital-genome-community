# GDC Emulators - External to Canon

**Location:** `validation/emulators/`  
**Status:** External validation tools  
**Governed by:** Community rules (NOT Canon)

---

## ⚠️ CRITICAL NOTICE

**These components are NOT part of the GDC Core.**

From Canon v5.1, lines 6491-6500:

> "As camadas superiores (GDO, GDE, e quaisquer outras) **não pertencem ao Canon do GDC**. 
> Essas camadas existem exclusivamente como emuladores, geradores de estímulo e simuladores 
> para fins de teste, estresse e validação do GDC."

### Prohibitions:
- ❌ Insert external layer rules into Canon
- ❌ Mix external contracts with GDC invariants  
- ❌ **Allow structural contamination between layers**

---

## Components

### GDO - Genoma Digital Orchestrator
**Purpose:** Orchestration emulator for testing  
**Location:** `validation/emulators/gdo/`  
**Used in:** v1.0.0α validation

### GDE - Genoma Digital Educator  
**Purpose:** Translation layer emulator (UNL ↔ Human)  
**Location:** `validation/emulators/gde/`  
**Used in:** v1.0.0α, v1.0.0γ validation

### Adapter - Trans-Kingdom Adapters
**Purpose:** Community Edition adapters (Industrial, Financial)  
**Location:** `validation/emulators/adapter/`  
**Used in:** v1.0.0β validation

### Orchestrator - Closed-Loop Cycle
**Purpose:** Continuous cycle validation  
**Location:** `validation/emulators/orchestrator/`  
**Used in:** v1.0.0γ validation

---

## Usage

```rust
use gdc_emulators::gdo::GdoOrchestrator;
use gdc_emulators::gde::GdeEducator;
use gdc_emulators::adapter::{IndustrialAdapter, FinancialAdapter};
use gdc_emulators::orchestrator::CycleOrchestrator;

fn main() {
    // These are EXTERNAL tools, not Core
    let orchestrator = GdoOrchestrator::new();
    let educator = GdeEducator::new();
    // ...
}
```

---

## Canonical Compliance

| Aspect | Status |
|--------|--------|
| **Location** | ✅ Outside Core (validation/) |
| **Contamination** | ✅ No contamination of src/ |
| **Governance** | ✅ Community rules (not Canon) |
| **Technology** | ✅ Can use JSON, Protobuf, etc. |
| **Purpose** | ✅ Validation/testing only |

---

## Building

```bash
cd validation/emulators
cargo build
cargo test
```

---

## Relation to Core

```
digital-genome-community/
├── src/                    ← CANONICAL CORE ONLY
│   ├── cognitive/          ✅ AF-1..AF-17
│   ├── motors/             ✅ AO-1..AO-24
│   └── ...                 ✅ Canon-governed
│
└── validation/emulators/   ← EXTERNAL EMULATORS
    ├── gdo/                🔧 Testing tool
    ├── gde/                🔧 Testing tool
    ├── adapter/            🔧 Community Edition
    └── orchestrator/       🔧 Validation tool
```

**Core depends on:** Nothing external  
**Emulators depend on:** Core (for testing)

---

## Version History

| Version | Component | Status |
|---------|-----------|--------|
| v1.0.0α | GDO + GDE | Validation tools |
| v1.0.0β | Adapters | Community Edition |
| v1.0.0γ | Orchestrator | Cycle validation |

---

**Canonical Status:** ✅ Compliant (external to Core)  
**Last Updated:** February 18, 2026
