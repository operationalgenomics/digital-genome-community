//! Canon v5.1 | GDC v0.9.5
//!
//! Synapse module - Conexões persistentes entre GDCs
//!
//! Canon: AF-11 (Aprendizado Cognitivo Autônomo)

pub mod connection;
pub mod strength;
pub mod pruning;

pub use connection::{Synapse, SynapseId, SynapseNetwork};
pub use strength::{StrengthModulator, StrengthParams};
pub use pruning::PruningResult;
