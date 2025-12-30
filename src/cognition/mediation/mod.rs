//! --------------------------
//! INFORMATION
//! --------------------------
//! Title: Mediation Motor Module
//! Author: Carlos Eduardo Favini
//! Date: 2025-12-30
//! Version: 1.0.0
//! Description: Exposes the M_N engine (Nash) and types.
//! Layer: Community
//! Dependencies: types, motor
//! Affected Components: reactor
//!
//! --------------------------
//! CHANGE LOG
//! --------------------------
//! 2025-12-30 - Initial creation for Phase 2.4
//! --------------------------

pub mod types;
pub mod motor;

pub use types::{
    OperationalTensionField, 
    TensionDimension, 
    CrossPressure,
    MEDIATION_SCALE
};

pub use motor::{
    MediationMotor,
    NashTuning
};

#[cfg(test)]
mod tests;