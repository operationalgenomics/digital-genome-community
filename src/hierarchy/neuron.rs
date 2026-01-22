//! --------------------------
//! INFORMATION
//! --------------------------
//! Title: Neuron (Level 3)
//! Author: Carlos Eduardo Favini
//! Date: 2025-01-02
//! Version: 1.2.0
//! Description: Neuronal structures that aggregate synapses into
//!              functional cognitive units. A neuron represents a
//!              domain of operational knowledge, emerging from the
//!              patterns of its constituent synapses.
//! Layer: Community
//! Dependencies: core_types, hierarchy/synapse
//! Affected Components: brain, topology
//!
//! --------------------------
//! CHANGE LOG
//! --------------------------
//! 2025-01-02 - Carlos Eduardo Favini - Initial creation
//! --------------------------

use serde::{Deserialize, Serialize};

use crate::core_types::{NeuronId, SynapseId};

/// Level 3: Neuron.
///
/// A neuron aggregates multiple synapses into a functional cognitive
/// unit. It represents an emergent domain of operational knowledge.
///
/// # Emergence
/// The neuron's domain is not declared - it emerges from the patterns
/// of its constituent synapses. The system discovers what the neuron
/// "knows" by observing its activation patterns.
///
/// # Activation
/// Neurons activate when their input synapses reach sufficient
/// combined strength. This is structural observation, not decision.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Neuron {
    /// Unique identifier for this neuron.
    pub id: NeuronId,

    /// Synapses that feed into this neuron (inputs).
    pub input_synapses: Vec<SynapseId>,

    /// Synapses that this neuron feeds (outputs).
    pub output_synapses: Vec<SynapseId>,

    /// Current activation level.
    /// Range: [0.0, 1.0]
    pub activation: f64,

    /// Emergent domain label (inferred, not declared).
    pub emergent_domain: Option<String>,

    /// Aggregate Craft Performance of constituent DNA.
    pub aggregate_cp: f64,
}

impl Neuron {
    /// Creates a new neuron.
    pub fn new() -> Self {
        Self {
            id: NeuronId::new(),
            input_synapses: Vec::new(),
            output_synapses: Vec::new(),
            activation: 0.0,
            emergent_domain: None,
            aggregate_cp: 0.0,
        }
    }

    /// Creates a neuron with initial input synapses.
    pub fn with_inputs(input_synapses: Vec<SynapseId>) -> Self {
        Self {
            id: NeuronId::new(),
            input_synapses,
            output_synapses: Vec::new(),
            activation: 0.0,
            emergent_domain: None,
            aggregate_cp: 0.0,
        }
    }

    /// Adds an input synapse.
    pub fn add_input(&mut self, synapse_id: SynapseId) {
        if !self.input_synapses.contains(&synapse_id) {
            self.input_synapses.push(synapse_id);
        }
    }

    /// Adds an output synapse.
    pub fn add_output(&mut self, synapse_id: SynapseId) {
        if !self.output_synapses.contains(&synapse_id) {
            self.output_synapses.push(synapse_id);
        }
    }

    /// Sets the activation level.
    /// This is structural state, not action.
    pub fn set_activation(&mut self, level: f64) {
        self.activation = level.clamp(0.0, 1.0);
    }

    /// Sets the emergent domain based on structural observation.
    pub fn set_emergent_domain(&mut self, domain: String) {
        self.emergent_domain = Some(domain);
    }

    /// Updates the aggregate Craft Performance.
    pub fn set_aggregate_cp(&mut self, cp: f64) {
        self.aggregate_cp = cp.clamp(0.0, 1.0);
    }

    /// Returns the number of input connections.
    pub fn input_count(&self) -> usize {
        self.input_synapses.len()
    }

    /// Returns the number of output connections.
    pub fn output_count(&self) -> usize {
        self.output_synapses.len()
    }

    /// Checks if this neuron is active (activation > 0).
    pub fn is_active(&self) -> bool {
        self.activation > 0.0
    }
}

impl Default for Neuron {
    fn default() -> Self {
        Self::new()
    }
}

/// Neuronal activation state snapshot.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct NeuronState {
    /// The neuron's ID.
    pub neuron_id: NeuronId,

    /// Activation level at this snapshot.
    pub activation: f64,

    /// Timestamp of this state (nanoseconds).
    pub timestamp_ns: i64,
}

impl NeuronState {
    /// Creates a new neuron state snapshot.
    pub fn new(neuron_id: NeuronId, activation: f64, timestamp_ns: i64) -> Self {
        Self {
            neuron_id,
            activation,
            timestamp_ns,
        }
    }
}
