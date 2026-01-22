//! Universal Notation Language (UNL)
//!
//! Formal specification defining valid cognitive states in GDC.
//! UNL is the "constitution" - GD-QMN implements it.

pub mod spec;
pub mod gd_qmn;
pub mod emulator;

pub use spec::{UnlSpec, UnlRule, UnlInvariant};
pub use gd_qmn::{GdQmn, Profile, Family};
pub use emulator::{GdoEmulator, PerceptualFrame, Observation, GdoResult, AggregatedMotorScores};
