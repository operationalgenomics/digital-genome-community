//! Canon v5.1 | GDC v0.9.0-final
//!
//! Results module - R(Σ), FCE(R), CF(G)
//!
//! ## Status v0.9.0-final
//! - ✅ phenotype.rs - CF(G) COMPLETE
//! - ✅ fce.rs - FCE(R) COMPLETE
//! - ✅ result.rs - R(Σ) COMPLETE
//!
//! Canon References:
//! - Especificação R(Σ) e FCE(R)
//! - Especificação CF(G)/Fenótipo
//! - Especificação DE/DD

pub mod phenotype;
pub mod fce;
pub mod result;

pub use phenotype::{StructuralGraph, CanonicalForm, phenotype_equivalent};
pub use fce::{FCE, NormalizedStructure};
pub use result::{CognitiveResult, ResultType};
