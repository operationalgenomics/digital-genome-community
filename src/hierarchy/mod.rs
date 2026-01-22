//! --------------------------
//! INFORMATION
//! --------------------------
//! Title: Biological Hierarchy Module
//! Author: Carlos Eduardo Favini
//! Date: 2025-01-02
//! Version: 1.2.0
//! Description: The biological hierarchy of the Digital Genome.
//!              Implements the cognitive structures from Action (Level 0)
//!              through Brain (highest level). Each level builds upon
//!              the previous, forming an emergent cognitive architecture.
//! Layer: Community
//! Dependencies: core_types, serde
//! Affected Components: motors, selection, topology
//!
//! --------------------------
//! CHANGE LOG
//! --------------------------
//! 2025-01-02 - Carlos Eduardo Favini - Initial creation
//! --------------------------

pub mod action;
pub mod dna;
pub mod synapse;
pub mod neuron;
pub mod brain;
pub mod truth;

pub use action::*;
pub use dna::*;
pub use synapse::*;
pub use neuron::*;
pub use brain::*;
pub use truth::*;
