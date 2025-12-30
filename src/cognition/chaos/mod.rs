//! --------------------------
//! INFORMATION
//! --------------------------
//! Title: Chaotic Motor Module
//! Author: Carlos Eduardo Favini
//! Date: 2025-12-30
//! Version: 1.2.0
//! Description: Exposes the M_C engine and types (Deterministic).
//! Layer: Community
//! Dependencies: types, motor
//! Affected Components: reactor
//!
//! --------------------------
//! CHANGE LOG
//! --------------------------
//! 2025-12-30 - Refactored for Community (Deterministic).
//! 2025-12-27 - Initial creation
//! --------------------------

pub mod types;
pub mod motor;

pub use types::{
    ChaoticAssessment, 
    ContractualProximity, 
    PerturbationPath,
    ChaoticTensionField,
    ChaoticTensionDimension,
    ChaoticCrossPressure,
    CHAOS_SCALE
};

pub use motor::ChaoticMotor;

#[cfg(test)]
mod tests;