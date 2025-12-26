use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::collections::HashMap;
use super::nodes::Neuron;

/// Lifecycle State of the Network.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OntogenesisState {
    /// Fetal state: Learning allowed, critical action blocked.
    Fetus,
    /// Active state: Fully operational.
    Viable,
}

/// The Cognitive Graph (Brain).
/// 
/// The root entity of the topology. It manages the collection of Neurons
/// and monitors the systemic maturity (Ontogenesis).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveNetwork {
    pub id: Uuid,
    pub identity: String,
    pub cortex: HashMap<String, Neuron>,
    pub plasticity_index: f64,
    pub state: OntogenesisState,
}

impl CognitiveNetwork {
    pub fn new(identity: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            identity,
            cortex: HashMap::new(),
            plasticity_index: 1.0,
            state: OntogenesisState::Fetus,
        }
    }

    pub fn grow_neuron(&mut self, domain: String) -> &mut Neuron {
        self.cortex
            .entry(domain.clone())
            .or_insert_with(|| Neuron::new(domain))
    }

    /// Evaluates Ontogenesis: Checks if the system is mature enough to be "Viable".
    pub fn check_vital_signs(&mut self) {
        let neuron_count = self.cortex.len();
        let total_weight: f64 = self.cortex.values().map(|n| n.activation_level).sum();
        
        // Recalculate global plasticity
        if neuron_count > 0 {
            self.plasticity_index = total_weight / (neuron_count as f64);
        }

        // Ontogenesis Thresholds: At least 1 neuron and some experience.
        if neuron_count >= 1 && self.plasticity_index > 0.1 {
            self.state = OntogenesisState::Viable;
        } else {
            self.state = OntogenesisState::Fetus;
        }
    }
}
