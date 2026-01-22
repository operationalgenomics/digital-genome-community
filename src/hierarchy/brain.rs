//! --------------------------
//! INFORMATION
//! --------------------------
//! Title: Brain (Level 4)
//! Author: Carlos Eduardo Favini
//! Date: 2025-01-02
//! Version: 1.2.0
//! Description: The highest level of the biological hierarchy.
//!              A Brain aggregates neurons into a complete cognitive
//!              system capable of perception, observation, comprehension,
//!              and emission of DNA with Score.
//! Layer: Community
//! Dependencies: core_types, hierarchy/neuron
//! Affected Components: External consumers (Enterprise)
//!
//! --------------------------
//! CHANGE LOG
//! --------------------------
//! 2025-01-02 - Carlos Eduardo Favini - Initial creation
//! --------------------------

use serde::{Deserialize, Serialize};

use crate::core_types::{BrainId, NeuronId};

/// Level 4: Brain.
///
/// The Brain is the highest level of the cognitive hierarchy.
/// It aggregates neurons into a complete system capable of:
/// - Perception
/// - Observation
/// - Comprehension
/// - Interiorization
/// - Rationalization
/// - Emission of DNA with Score
///
/// # Non-Agency
/// The Brain does NOT act. It does not have a body.
/// It perceives, processes, emits output, and returns to listening.
///
/// # Trans-Kingdom Awareness
/// The Brain can recognize patterns across domains:
/// mineral, vegetal, animal, and potentially others.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Brain {
    /// Unique identifier for this brain instance.
    pub id: BrainId,

    /// Neurons that comprise this brain.
    pub neurons: Vec<NeuronId>,

    /// Current cognitive state.
    pub state: CognitiveState,

    /// Overall coherence metric.
    pub coherence: f64,

    /// Generation in the evolutionary lineage.
    pub generation: u64,
}

/// The cognitive state of the Brain.
///
/// Represents where in the cognitive cycle the brain currently is.
/// The cycle always ends in Listening - the brain does not act.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CognitiveState {
    /// Receiving input from the world.
    Perceiving,

    /// Observing structure in the input.
    Observing,

    /// Understanding patterns and relationships.
    Comprehending,

    /// Internalizing the comprehension.
    Interiorizing,

    /// Processing through cognitive motors.
    Rationalizing,

    /// Preparing DNA with Score for emission.
    Emitting,

    /// Waiting for next input. Default state.
    Listening,
}

impl Default for CognitiveState {
    fn default() -> Self {
        Self::Listening
    }
}

impl Brain {
    /// Creates a new Brain instance.
    pub fn new() -> Self {
        Self {
            id: BrainId::new(),
            neurons: Vec::new(),
            state: CognitiveState::Listening,
            coherence: 0.0,
            generation: 0,
        }
    }

    /// Creates a Brain with initial neurons.
    pub fn with_neurons(neurons: Vec<NeuronId>, generation: u64) -> Self {
        Self {
            id: BrainId::new(),
            neurons,
            state: CognitiveState::Listening,
            coherence: 0.0,
            generation,
        }
    }

    /// Adds a neuron to this brain.
    pub fn add_neuron(&mut self, neuron_id: NeuronId) {
        if !self.neurons.contains(&neuron_id) {
            self.neurons.push(neuron_id);
        }
    }

    /// Returns the number of neurons.
    pub fn neuron_count(&self) -> usize {
        self.neurons.len()
    }

    /// Sets the cognitive state.
    /// This tracks where in the cycle the brain is.
    pub fn set_state(&mut self, state: CognitiveState) {
        self.state = state;
    }

    /// Updates the coherence metric.
    pub fn set_coherence(&mut self, coherence: f64) {
        self.coherence = coherence.clamp(0.0, 1.0);
    }

    /// Checks if the brain is in listening state.
    pub fn is_listening(&self) -> bool {
        self.state == CognitiveState::Listening
    }

    /// Resets the brain to listening state.
    /// This is where every cognitive cycle ends.
    pub fn return_to_listening(&mut self) {
        self.state = CognitiveState::Listening;
    }
}

impl Default for Brain {
    fn default() -> Self {
        Self::new()
    }
}

/// Snapshot of brain state for traceability.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainSnapshot {
    /// Brain identifier.
    pub brain_id: BrainId,

    /// Cognitive state at snapshot.
    pub state: CognitiveState,

    /// Coherence at snapshot.
    pub coherence: f64,

    /// Timestamp (nanoseconds).
    pub timestamp_ns: i64,

    /// Number of active neurons.
    pub active_neuron_count: usize,
}

impl BrainSnapshot {
    /// Creates a new brain snapshot.
    pub fn new(
        brain_id: BrainId,
        state: CognitiveState,
        coherence: f64,
        timestamp_ns: i64,
        active_neuron_count: usize,
    ) -> Self {
        Self {
            brain_id,
            state,
            coherence,
            timestamp_ns,
            active_neuron_count,
        }
    }
}
