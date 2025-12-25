use serde::{Serialize, Deserialize};
use crate::core::identifiers::DnaId;

/// Synapse.
/// 
/// Represents a learned pathway between a stimulus (Intent) and a solution (Golden DNA).
/// It constitutes the edges of the cognitive graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Synapse {
    /// The intent or stimulus name (e.g., "arc_stability").
    pub intent: String,
    
    /// Pointer to the Best Known Method (Platonic Form) for this intent.
    pub target_dna: DnaId,
    
    /// The strength of this connection (Plasticity).
    pub weight: f64,
}

impl Synapse {
    pub fn new(intent: String, target_dna: DnaId) -> Self {
        Self {
            intent,
            target_dna,
            weight: 0.1, // Starts weak
        }
    }

    /// Reinforces the connection based on successful application.
    pub fn reinforce(&mut self, factor: f64) {
        self.weight = (self.weight + factor).min(1.0);
    }
}
