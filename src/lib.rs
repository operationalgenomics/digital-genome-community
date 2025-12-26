//! # Digital Genome: Community Edition
//! 
//! The open-source cognitive core of the Digital Genome project.
//! 
//! ## Architecture (Canonical)
//! 
//! The system is divided into four neutral zones:
//! 
//! * **Core**: Primitive types, identifiers, and invariant traits.
//! * **Memory**: Immutable historical records (Foucaultian Truth) and Journals.
//! * **Engine**: Pure logic for evaluation, mutation (CRISPR), and selection.
//! * **Topology**: The structural organization of knowledge (Graph/Network).

// 1. Core Types (The Vocabulary)
pub mod core;

// 2. Archive & Provenance (The Memory)
pub mod memory;

// 3. Evaluation & Logic (The CPU)
pub mod engine;

// 4. Graph Structure (The Connectome)
pub mod topology;

// Re-exports for easier access to root concepts
pub use uuid::Uuid;
