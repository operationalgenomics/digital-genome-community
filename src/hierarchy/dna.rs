//! --------------------------
//! INFORMATION
//! --------------------------
//! Title: Golden DNA (Level 1)
//! Author: Carlos Eduardo Favini
//! Date: 2025-01-02
//! Version: 1.3.0
//! Description: The DNA structure of the Digital Genome.
//! Represents a validated pattern of actions with its
//! Craft Performance score. DNA is the unit of heredity
//! in the cognitive system - it carries operational knowledge.
//!
//! Conceptual note: DNA in the Community Edition represents EPHEMERAL
//! COMPREHENSION. It is valid ONLY during the perceptual cycle that
//! created it. The Community does not persist DNA - persistence is the
//! responsibility of external systems (Enterprise Edition).
//!
//! Analogy: DNA is like a thought - it exists while being thought,
//! but disappears when the thinking ends. Memory is a separate system.
//!
//! Layer: Community
//! Dependencies: core_types, hierarchy/action
//! Affected Components: synapse, selection, archive
//!
//! --------------------------
//! CHANGE LOG
//! --------------------------
//! 2025-01-02 - Carlos Eduardo Favini - Initial creation
//! 2025-01-02 - Carlos Eduardo Favini - Added ephemeral comprehension documentation (v1.3.0)
//! --------------------------

use serde::{Deserialize, Serialize};

use crate::core_types::DnaId;
use super::action::ActionSequence;

/// Level 1: Golden DNA.
///
/// A DNA strand represents a validated pattern of actions.
/// The "Golden" qualifier indicates it has been evaluated by all
/// cognitive motors and received a non-zero Craft Performance score.
///
/// # CRITICAL CONCEPTUAL NOTE: EPHEMERAL COMPREHENSION
///
/// **DNA represents momentary understanding, NOT persistent memory.**
///
/// In the Community Edition:
/// - DNA is created during perception
/// - DNA exists only while the perceptual cycle is active
/// - DNA is NOT automatically persisted
/// - DNA does NOT survive beyond the function call that created it
///
/// This is by design. The Community Edition models the **perceptual cortex**,
/// not the **hippocampus**. Comprehension and memory are separate systems.
///
/// If you need to persist DNA:
/// - The Enterprise Edition provides persistence mechanisms
/// - External systems can serialize and store DNA
/// - But the Community itself "forgets" everything when perception ends
///
/// # Structure
/// - Contains a sequence of observed actions (the "genes")
/// - Carries the Craft Performance score (CP)
/// - Maintains lineage information for evolutionary tracing
///
/// # Immutability in Community
/// In the Community Edition, DNA is immutable once created.
/// Modifications (CRISPR operations) belong to the Enterprise layer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoldenDna {
    /// Unique identifier for this DNA strand.
    pub id: DnaId,

    /// The sequence of actions that form this DNA.
    pub action_sequence: ActionSequence,

    /// Craft Performance score: CP = M_P × M_C × M_N × M_M
    /// Range: [0.0, 1.0]
    /// CP = 0 triggers absolute veto.
    pub craft_performance: f64,

    /// Individual motor scores for transparency.
    pub motor_scores: MotorScores,

    /// Generation number in the evolutionary lineage.
    pub generation: u64,

    /// Parent DNA IDs (empty for primordial DNA).
    pub lineage: Vec<DnaId>,
}

/// Individual scores from each cognitive motor.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MotorScores {
    /// Praxeological Motor score (M_P).
    pub praxeological: f64,

    /// Nash Motor score (M_N).
    pub nash: f64,

    /// Chaotic Motor score (M_C).
    pub chaotic: f64,

    /// Meristic Motor score (M_M).
    /// In Community Edition, this reflects structural creativity potential.
    pub meristic: f64,
}

impl MotorScores {
    /// Creates a new MotorScores instance.
    pub fn new(praxeological: f64, nash: f64, chaotic: f64, meristic: f64) -> Self {
        Self {
            praxeological,
            nash,
            chaotic,
            meristic,
        }
    }

    /// Calculates Craft Performance using the non-compensatory formula.
    /// CP = M_P × M_N × M_C × M_M
    ///
    /// If any motor is below threshold, CP is zero (absolute veto).
    pub fn calculate_cp(&self) -> f64 {
        use crate::math::VETO_THRESHOLD;
        
        // Check for veto before calculation
        if self.has_veto() {
            return 0.0;
        }
        
        let cp = self.praxeological * self.nash * self.chaotic * self.meristic;
        
        // Final CP below threshold also triggers veto
        if cp < VETO_THRESHOLD {
            return 0.0;
        }
        
        cp.clamp(0.0, 1.0)
    }

    /// Checks if any motor triggered a veto (score below threshold).
    pub fn has_veto(&self) -> bool {
        use crate::math::VETO_THRESHOLD;
        
        self.praxeological < VETO_THRESHOLD
            || self.nash < VETO_THRESHOLD
            || self.chaotic < VETO_THRESHOLD
            || self.meristic < VETO_THRESHOLD
    }
}

impl Default for MotorScores {
    fn default() -> Self {
        Self {
            praxeological: 0.0,
            nash: 0.0,
            chaotic: 0.0,
            meristic: 0.0,
        }
    }
}

impl GoldenDna {
    /// Creates a new Golden DNA from an action sequence and motor scores.
    ///
    /// # Arguments
    /// * `action_sequence` - The sequence of observed actions
    /// * `motor_scores` - Scores from each cognitive motor
    /// * `generation` - Generation number (0 for primordial)
    /// * `lineage` - Parent DNA IDs
    ///
    /// # Returns
    /// A new GoldenDna with computed Craft Performance.
    pub fn new(
        action_sequence: ActionSequence,
        motor_scores: MotorScores,
        generation: u64,
        lineage: Vec<DnaId>,
    ) -> Self {
        let craft_performance = motor_scores.calculate_cp();

        Self {
            id: DnaId::new(),
            action_sequence,
            craft_performance,
            motor_scores,
            generation,
            lineage,
        }
    }

    /// Creates primordial DNA (generation 0, no lineage).
    pub fn primordial(action_sequence: ActionSequence, motor_scores: MotorScores) -> Self {
        Self::new(action_sequence, motor_scores, 0, Vec::new())
    }

    /// Checks if this DNA was vetoed (CP = 0).
    pub fn is_vetoed(&self) -> bool {
        self.craft_performance == 0.0
    }

    /// Returns the number of actions in this DNA.
    pub fn action_count(&self) -> usize {
        self.action_sequence.len()
    }
}

/// A codon represents a minimal functional unit within DNA.
/// Used by the Meristic motor for hypothesis generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Codon {
    /// Unique identifier.
    pub id: String,

    /// Embedding vector for similarity calculations.
    pub embedding: Vec<f64>,

    /// Classification tags (emergent, not predefined).
    pub tags: Vec<String>,
}

impl Codon {
    /// Creates a new codon.
    pub fn new(id: String, embedding: Vec<f64>, tags: Vec<String>) -> Self {
        Self { id, embedding, tags }
    }
}
