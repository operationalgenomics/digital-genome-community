//! --------------------------
//! INFORMATION
//! --------------------------
//! Title: Core Types Module
//! Author: Carlos Eduardo Favini
//! Date: 2025-01-02
//! Version: 1.2.0
//! Description: Fundamental type definitions for the Digital Genome.
//!              Contains identifiers, basic structures, and type aliases
//!              used throughout the Community Edition.
//! Layer: Community
//! Dependencies: uuid, serde
//! Affected Components: All modules depend on these types
//!
//! --------------------------
//! CHANGE LOG
//! --------------------------
//! 2025-01-02 - Carlos Eduardo Favini - Initial creation
//! 2026-02-19 - Carlos Eduardo Favini - Added logical_time (Canon v5.1 compliance - Fase 1)
//! 2026-02-21 - Carlos Eduardo Favini - Added canonical_ids (Canon v5.1 compliance - Fase 2)
//! --------------------------

mod identifiers;
mod logical_time;
mod canonical_ids;

pub use identifiers::*;
pub use logical_time::*;
pub use canonical_ids::*;
