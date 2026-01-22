//! --------------------------
//! INFORMATION
//! --------------------------
//! Title: Synapse (Level 2)
//! Author: Carlos Eduardo Favini
//! Date: 2025-01-02
//! Version: 1.2.0
//! Description: Synaptic structures that connect DNA strands.
//!              A synapse represents a functional relationship between
//!              multiple DNA strands, forming the basis of cognitive
//!              pathways. Synapses enable pattern recognition across
//!              related operational knowledge.
//! Layer: Community
//! Dependencies: core_types, hierarchy/dna
//! Affected Components: neuron, topology
//!
//! --------------------------
//! CHANGE LOG
//! --------------------------
//! 2025-01-02 - Carlos Eduardo Favini - Initial creation
//! --------------------------

use serde::{Deserialize, Serialize};

use crate::core_types::{DnaId, SynapseId};

/// Level 2: Synapse.
///
/// A synapse connects multiple DNA strands that share functional
/// relationships. It represents emergent associations discovered
/// through cognitive processing, not predefined categories.
///
/// # Biological Analogy
/// Like biological synapses connect neurons, these synapses connect
/// operational knowledge (DNA) into functional pathways.
///
/// # Emergent Semantics
/// The domain of a synapse is INFERRED from the DNA it contains,
/// never declared. The system discovers categories, it does not
/// receive them.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Synapse {
    /// Unique identifier for this synapse.
    pub id: SynapseId,

    /// The DNA strands connected by this synapse.
    pub connected_dna: Vec<DnaId>,

    /// Connection strength based on functional similarity.
    /// Range: [0.0, 1.0]
    pub strength: f64,

    /// Inferred domain (emergent, not declared).
    /// This is a structural observation, not a classification.
    pub inferred_domain: Option<String>,

    /// Activation count (how often this pathway is used).
    pub activation_count: u64,
}

impl Synapse {
    /// Creates a new synapse connecting DNA strands.
    ///
    /// # Arguments
    /// * `connected_dna` - IDs of DNA strands to connect
    /// * `initial_strength` - Initial connection strength
    ///
    /// # Returns
    /// A new Synapse with no inferred domain.
    pub fn new(connected_dna: Vec<DnaId>, initial_strength: f64) -> Self {
        Self {
            id: SynapseId::new(),
            connected_dna,
            strength: initial_strength.clamp(0.0, 1.0),
            inferred_domain: None,
            activation_count: 0,
        }
    }

    /// Creates a synapse between two DNA strands.
    pub fn between(dna_a: DnaId, dna_b: DnaId, strength: f64) -> Self {
        Self::new(vec![dna_a, dna_b], strength)
    }

    /// Returns the number of connected DNA strands.
    pub fn connection_count(&self) -> usize {
        self.connected_dna.len()
    }

    /// Checks if this synapse connects a specific DNA.
    pub fn connects(&self, dna_id: &DnaId) -> bool {
        self.connected_dna.contains(dna_id)
    }

    /// Records an activation of this synaptic pathway.
    /// Note: This is observation, not action. The count reflects usage.
    pub fn record_activation(&mut self) {
        self.activation_count = self.activation_count.saturating_add(1);
    }

    /// Sets the inferred domain based on structural observation.
    /// This is emergent semantics, not classification.
    pub fn set_inferred_domain(&mut self, domain: String) {
        self.inferred_domain = Some(domain);
    }
}

/// Synaptic weight representing connection significance.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SynapticWeight {
    /// The base weight of the connection.
    pub base: f64,

    /// Temporal decay factor.
    pub decay: f64,

    /// Reinforcement from repeated activation.
    pub reinforcement: f64,
}

impl SynapticWeight {
    /// Creates a new synaptic weight with explicit base value.
    ///
    /// # Arguments
    /// * `base` - The base weight value, must be in [0.0, 1.0]
    ///
    /// # Returns
    /// Some(SynapticWeight) if base is valid, None otherwise.
    pub fn new(base: f64) -> Option<Self> {
        if base.is_nan() || base.is_infinite() || base < 0.0 || base > 1.0 {
            return None;
        }

        Some(Self {
            base,
            decay: 0.0,
            reinforcement: 0.0,
        })
    }

    /// Creates a new synaptic weight, clamping invalid values.
    ///
    /// Use `new()` for strict validation.
    pub fn new_clamped(base: f64) -> Self {
        Self {
            base: base.clamp(0.0, 1.0),
            decay: 0.0,
            reinforcement: 0.0,
        }
    }

    /// Calculates the effective weight.
    pub fn effective(&self) -> f64 {
        ((self.base - self.decay) + self.reinforcement).clamp(0.0, 1.0)
    }
}

// Note: Default intentionally NOT implemented.
// SynapticWeight requires explicit initialization to avoid
// implicit behavior in the cognitive core.
