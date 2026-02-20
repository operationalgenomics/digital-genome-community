//! Canon v5.1 | GDC v0.9.5
//!
//! Synaptic Pruning - Remoção de conexões fracas
//!
//! Canon: AF-11 (Otimização por remoção)

use super::connection::{Synapse, SynapseId, SynapseNetwork};
use super::strength::StrengthModulator;

// =============================================================================
// PRUNING
// =============================================================================

/// Pruning result.
#[derive(Debug, Clone)]
pub struct PruningResult {
    /// Number of synapses pruned
    pub pruned_count: usize,
    
    /// IDs of pruned synapses
    pub pruned_ids: Vec<SynapseId>,
}

impl SynapseNetwork {
    /// Prune weak synapses.
    ///
    /// Canon: AF-11 - Remove conexões não produtivas
    pub fn prune_weak_synapses(&mut self, modulator: &StrengthModulator) -> PruningResult {
        let to_prune: Vec<SynapseId> = self
            .synapses
            .iter()
            .filter(|(_, synapse)| modulator.should_prune(synapse))
            .map(|(id, _): (&SynapseId, &Synapse)| id.clone())
            .collect();
        
        let pruned_count = to_prune.len();
        
        for id in &to_prune {
            self.synapses.remove(id);
        }
        
        PruningResult {
            pruned_count,
            pruned_ids: to_prune,
        }
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_prune_removes_weak_synapses() {
        let mut network = SynapseNetwork::new();
        let modulator = StrengthModulator::default();
        
        // Create weak synapse
        let weak_id = network.form_synapse("A".to_string(), "B".to_string(), 0.01);
        // Create strong synapse
        let strong_id = network.form_synapse("C".to_string(), "D".to_string(), 0.8);
        
        let result = network.prune_weak_synapses(&modulator);
        
        assert_eq!(result.pruned_count, 1);
        assert!(!network.has_synapse(&weak_id));
        assert!(network.has_synapse(&strong_id));
    }
}
