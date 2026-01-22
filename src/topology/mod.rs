//! --------------------------
//! INFORMATION
//! --------------------------
//! Title: Topological Structures Module
//! Author: Carlos Eduardo Favini
//! Date: 2025-01-02
//! Version: 1.2.0
//! Description: Graph-based structures representing relationships
//!              between cognitive elements. Provides structural
//!              observation of connections, not management of them.
//! Layer: Community
//! Dependencies: core_types, hierarchy
//! Affected Components: External consumers (Enterprise)
//!
//! --------------------------
//! CHANGE LOG
//! --------------------------
//! 2025-01-02 - Carlos Eduardo Favini - Initial creation
//! 2025-01-02 - Carlos Eduardo Favini - Added Result-based API (v0.2.0)
//! 2025-01-02 - Carlos Eduardo Favini - Replaced clamp with validation (v0.2.0)
//! --------------------------

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;

use crate::core_types::{DnaId, SynapseId, NeuronId};

/// Error type for topology operations.
#[derive(Debug, Clone, PartialEq)]
pub enum TopologyError {
    /// Weight is outside valid range [0, 1].
    InvalidWeight { value: f64 },
    /// Weight is NaN or infinite.
    NonFiniteWeight { value: f64 },
    /// Internal consistency error (should not happen).
    InternalError { message: String },
}

impl std::fmt::Display for TopologyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidWeight { value } => {
                write!(f, "Weight {} is outside valid range [0, 1]", value)
            }
            Self::NonFiniteWeight { value } => {
                write!(f, "Weight {} is not finite", value)
            }
            Self::InternalError { message } => {
                write!(f, "Internal error: {}", message)
            }
        }
    }
}

impl std::error::Error for TopologyError {}

/// A graph representing synaptic connections.
///
/// This is a structural observation of relationships.
/// The graph does not manage or modify connections - it
/// reveals the topology that exists.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynapticGraph {
    /// Adjacency list: DNA ID -> connected synapse IDs.
    dna_to_synapses: HashMap<DnaId, HashSet<SynapseId>>,

    /// Reverse index: Synapse ID -> connected DNA IDs.
    synapse_to_dnas: HashMap<SynapseId, HashSet<DnaId>>,

    /// Synapse count.
    synapse_count: usize,

    /// Edge count (DNA-Synapse connections).
    edge_count: usize,
}

impl SynapticGraph {
    /// Creates a new empty synaptic graph.
    pub fn new() -> Self {
        Self {
            dna_to_synapses: HashMap::new(),
            synapse_to_dnas: HashMap::new(),
            synapse_count: 0,
            edge_count: 0,
        }
    }

    /// Records a connection between DNA and synapse.
    ///
    /// This is observation, not creation. The connection exists
    /// externally; this records that it was observed.
    pub fn record_connection(&mut self, dna_id: DnaId, synapse_id: SynapseId) {
        // Use entry API for efficiency
        let dna_synapses = self.dna_to_synapses.entry(dna_id).or_default();
        let is_new_edge = dna_synapses.insert(synapse_id);

        if self.synapse_to_dnas
            .entry(synapse_id)
            .or_default()
            .insert(dna_id)
        {
            // First time seeing this synapse
            if !self.synapse_to_dnas.get(&synapse_id).map_or(false, |s| s.len() > 1) {
                self.synapse_count += 1;
            }
        }

        if is_new_edge {
            self.edge_count += 1;
        }
    }

    /// Gets synapses connected to a DNA.
    pub fn get_synapses(&self, dna_id: &DnaId) -> Option<&HashSet<SynapseId>> {
        self.dna_to_synapses.get(dna_id)
    }

    /// Gets DNAs connected to a synapse.
    pub fn get_dnas(&self, synapse_id: &SynapseId) -> Option<&HashSet<DnaId>> {
        self.synapse_to_dnas.get(synapse_id)
    }

    /// Returns the synapse count.
    pub fn synapse_count(&self) -> usize {
        self.synapse_count
    }

    /// Returns the edge count.
    pub fn edge_count(&self) -> usize {
        self.edge_count
    }
}

impl Default for SynapticGraph {
    fn default() -> Self {
        Self::new()
    }
}

/// A graph representing neuronal connections.
///
/// This is a directed weighted graph where neurons are nodes
/// and connections have weights. Like SynapticGraph, this is
/// observational, not managerial.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuronalGraph {
    /// Adjacency list: Neuron ID -> outgoing connection IDs.
    connections: HashMap<NeuronId, HashSet<NeuronId>>,

    /// Edge weights: (from, to) -> weight.
    weights: HashMap<(NeuronId, NeuronId), f64>,

    /// Node count.
    node_count: usize,

    /// Edge count.
    edge_count: usize,
}

impl NeuronalGraph {
    /// Creates a new empty neuronal graph.
    pub fn new() -> Self {
        Self {
            connections: HashMap::new(),
            weights: HashMap::new(),
            node_count: 0,
            edge_count: 0,
        }
    }

    /// Validates a weight value.
    fn validate_weight(weight: f64) -> Result<f64, TopologyError> {
        if !weight.is_finite() {
            return Err(TopologyError::NonFiniteWeight { value: weight });
        }
        if !(0.0..=1.0).contains(&weight) {
            return Err(TopologyError::InvalidWeight { value: weight });
        }
        Ok(weight)
    }

    /// Records a neuron in the graph.
    fn record_neuron(&mut self, neuron_id: NeuronId) {
        if let Entry::Vacant(e) = self.connections.entry(neuron_id) {
            e.insert(HashSet::new());
            self.node_count += 1;
        }
    }

    /// Records a connection between neurons.
    ///
    /// # Arguments
    /// * `from` - Source neuron ID
    /// * `to` - Target neuron ID
    /// * `weight` - Connection weight, must be in [0, 1]
    ///
    /// # Returns
    /// * `Ok(())` if successful
    /// * `Err(TopologyError)` if weight is invalid
    pub fn record_connection(&mut self, from: NeuronId, to: NeuronId, weight: f64) -> Result<(), TopologyError> {
        // Validate weight first
        let validated_weight = Self::validate_weight(weight)?;

        // Record nodes
        self.record_neuron(from);
        self.record_neuron(to);

        // Record edge using entry API
        if let Some(connections) = self.connections.get_mut(&from) {
            if connections.insert(to) {
                self.edge_count += 1;
            }
        }

        self.weights.insert((from, to), validated_weight);

        Ok(())
    }

    /// Records a connection, clamping weight to [0, 1].
    ///
    /// Use `record_connection` for strict validation.
    /// This method is provided for backward compatibility but
    /// signals when clamping occurred.
    ///
    /// # Returns
    /// * `(bool, f64)` - (was_clamped, effective_weight)
    pub fn record_connection_clamped(&mut self, from: NeuronId, to: NeuronId, weight: f64) -> (bool, f64) {
        let needs_clamping = !weight.is_finite() || !(0.0..=1.0).contains(&weight);
        let effective_weight = if weight.is_nan() {
            0.5 // Default for NaN
        } else {
            weight.clamp(0.0, 1.0)
        };

        // Record nodes
        self.record_neuron(from);
        self.record_neuron(to);

        // Record edge
        if let Some(connections) = self.connections.get_mut(&from) {
            if connections.insert(to) {
                self.edge_count += 1;
            }
        }

        self.weights.insert((from, to), effective_weight);

        (needs_clamping, effective_weight)
    }

    /// Gets the weight of a connection.
    pub fn get_weight(&self, from: &NeuronId, to: &NeuronId) -> Option<f64> {
        self.weights.get(&(*from, *to)).copied()
    }

    /// Gets outgoing connections from a neuron.
    pub fn get_outgoing(&self, neuron_id: &NeuronId) -> Option<&HashSet<NeuronId>> {
        self.connections.get(neuron_id)
    }

    /// Returns the node count.
    pub fn node_count(&self) -> usize {
        self.node_count
    }

    /// Returns the edge count.
    pub fn edge_count(&self) -> usize {
        self.edge_count
    }
}

impl Default for NeuronalGraph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_synaptic_graph_basic() {
        let mut graph = SynapticGraph::new();

        let dna1 = DnaId::new();
        let dna2 = DnaId::new();
        let synapse = SynapseId::new();

        graph.record_connection(dna1, synapse);
        graph.record_connection(dna2, synapse);

        assert!(graph.get_synapses(&dna1).is_some());
        assert!(graph.get_dnas(&synapse).is_some());
        assert_eq!(graph.edge_count(), 2);
    }

    #[test]
    fn test_neuronal_graph_basic() {
        let mut graph = NeuronalGraph::new();

        let n1 = NeuronId::new();
        let n2 = NeuronId::new();

        let result = graph.record_connection(n1, n2, 0.8);

        assert!(result.is_ok());
        assert_eq!(graph.node_count(), 2);
        assert_eq!(graph.edge_count(), 1);
        assert_eq!(graph.get_weight(&n1, &n2), Some(0.8));
    }

    #[test]
    fn test_invalid_weight_rejected() {
        let mut graph = NeuronalGraph::new();

        let n1 = NeuronId::new();
        let n2 = NeuronId::new();

        let result = graph.record_connection(n1, n2, 1.5);

        assert!(matches!(result, Err(TopologyError::InvalidWeight { .. })));
    }

    #[test]
    fn test_nan_weight_rejected() {
        let mut graph = NeuronalGraph::new();

        let n1 = NeuronId::new();
        let n2 = NeuronId::new();

        let result = graph.record_connection(n1, n2, f64::NAN);

        assert!(matches!(result, Err(TopologyError::NonFiniteWeight { .. })));
    }

    #[test]
    fn test_clamped_mode() {
        let mut graph = NeuronalGraph::new();

        let n1 = NeuronId::new();
        let n2 = NeuronId::new();

        let (was_clamped, effective) = graph.record_connection_clamped(n1, n2, 1.5);

        assert!(was_clamped);
        assert!((effective - 1.0).abs() < 1e-10);
        assert_eq!(graph.get_weight(&n1, &n2), Some(1.0));
    }

    #[test]
    fn test_valid_weight_not_clamped() {
        let mut graph = NeuronalGraph::new();

        let n1 = NeuronId::new();
        let n2 = NeuronId::new();

        let (was_clamped, effective) = graph.record_connection_clamped(n1, n2, 0.7);

        assert!(!was_clamped);
        assert!((effective - 0.7).abs() < 1e-10);
    }
}
