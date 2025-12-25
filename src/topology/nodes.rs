use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::collections::HashMap;
use super::connections::Synapse;

/// Neuron.
/// 
/// Represents a cluster of knowledge within a specific semantic domain.
/// It acts as a node in the cognitive graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Neuron {
    pub id: Uuid,
    
    /// The domain this neuron governs (e.g., "maintenance.hydraulic").
    pub domain: String,
    
    /// The skills (synapses) owned by this neuron.
    pub synapses: HashMap<String, Synapse>,
    
    /// Aggregate activation level of the node.
    pub activation_level: f64,
}

impl Neuron {
    pub fn new(domain: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            domain,
            synapses: HashMap::new(),
            activation_level: 0.0,
        }
    }

    pub fn attach_synapse(&mut self, synapse: Synapse) {
        self.activation_level += synapse.weight;
        self.synapses.insert(synapse.intent.clone(), synapse);
    }
}
