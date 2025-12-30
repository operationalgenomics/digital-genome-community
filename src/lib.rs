//! --------------------------
//! INFORMATION
//! --------------------------
//! Title: Digital Genome Community Library
//! Author: Carlos Eduardo Favini
//! Date: 2025-12-30
//! Version: 1.0.0
//! Description: The open-source cognitive core of the Digital Genome project.
//!              Defines the immutable laws of perception and cognition.
//! Layer: Community Root
//! --------------------------

//! ## Architecture (Canonical)
//! 
//! The system is divided into neutral zones:
//! 
//! * **Core**: Primitive types, identifiers, and invariant traits.
//! * **Memory**: Immutable historical records (Foucaultian Truth).
//! * **Perception**: Ontological primitives for sensing (Phase 1).
//! * **Engine**: Pure logic for evaluation.
//! * **Topology**: The structural organization of knowledge.

// 1. Core Types (The Vocabulary)
pub mod core;

// 2. Archive & Provenance (The Memory)
pub mod memory;

// 3. Perception (The Senses - Ontological Foundation)
pub mod perception;

// 4. Evaluation & Logic (The CPU)
pub mod engine;

// 5. Graph Structure (The Connectome)
pub mod topology;

// Re-exports for easier access to root concepts
pub use uuid::Uuid;