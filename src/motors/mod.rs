//! --------------------------
//! INFORMATION
//! --------------------------
//! Title: Cognitive Motors Module
//! Author: Carlos Eduardo Favini
//! Date: 2025-01-02
//! Version: 1.2.0
//! Description: The four cognitive motors of the Digital Genome.
//! Each motor evaluates structure from a different perspective.
//! Motors are deterministic, auditable, and semantically silent.
//! They reveal structure - they do not decide or act.
//! Layer: Community
//! Dependencies: Consagrated mathematics and physics
//! Affected Components: math/craft, hierarchy/dna
//!
//! --------------------------
//! CHANGE LOG
//! --------------------------
//! 2025-01-02 - Carlos Eduardo Favini - Initial creation
//! 2025-01-02 - Carlos Eduardo Favini - Added Meristic motor
//! --------------------------

pub mod praxis;
pub mod nash;
pub mod chaos;
pub mod meristic;

// Re-exports
pub use praxis::{PraxisMotor, PraxisInput, PraxisOutput};
pub use nash::{NashMotor, NashInput, NashOutput};
pub use chaos::{ChaosMotor, ChaosInput, ChaosOutput};
pub use meristic::{MeristicMotor, MeristicInput, MeristicOutput, MeristicProposal};

/// Trait that all cognitive motors must implement.
///
/// Motors are instincts - they do not learn domain knowledge.
/// They evaluate structure and produce continuous scores.
/// They do NOT apply thresholds or make decisions.
pub trait CognitiveMotor {
    /// The input type for this motor.
    type Input;

    /// The output type for this motor.
    type Output;

    /// Evaluates the input and produces a continuous score.
    ///
    /// # Returns
    /// Output containing the motor score and supporting metrics.
    /// The score is always in range [0.0, 1.0].
    ///
    /// # Determinism
    /// Same input MUST produce same output. Always.
    fn evaluate(&self, input: &Self::Input) -> Self::Output;
}

/// Motor evaluation result with score and metadata.
#[derive(Debug, Clone, Copy)]
pub struct MotorResult {
    /// The motor's score [0.0, 1.0].
    pub score: f64,

    /// Whether the evaluation completed successfully.
    pub valid: bool,
}

impl MotorResult {
    /// Creates a valid result with the given score.
    pub fn valid(score: f64) -> Self {
        Self {
            score: score.clamp(0.0, 1.0),
            valid: true,
        }
    }

    /// Creates an invalid result (score = 0).
    pub fn invalid() -> Self {
        Self {
            score: 0.0,
            valid: false,
        }
    }
}
