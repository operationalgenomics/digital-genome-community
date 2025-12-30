//! # Perception Module
//!
//! Defines the ontological primitives for sensing and observing phenomena.
//!
//! **Constitutional Principle**: The Community perceives structure through reverse engineering.
//! It defines the *shape* of perception, but does not execute the sensing (IO).
//!
//! This module contains:
//! * `types`: Canonical data structures for raw, unclassified phenomena (Deterministic, No Floats, No Wall-Clock).
//! * `traits`: Pure synchronous contracts for signal definition.

pub mod types;
pub mod traits;