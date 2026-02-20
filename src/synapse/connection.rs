//! Canon v5.1 | GDC v0.9.5
//!
//! Synapse - Conexão persistente entre par de GDCs
//!
//! Canon: AF-11 (Aprendizado Cognitivo Autônomo)
//!
//! ## Core Principle
//! Sinapses são conexões emergentes entre GDCs que fortalecem
//! ou enfraquecem baseado em padrões de cooperação.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// =============================================================================
// SYNAPSE - CONEXÃO PERSISTENTE
// =============================================================================

/// Synapse ID - par ordenado de GDC IDs.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SynapseId {
    /// Source GDC (typically Queen)
    pub source: String,
    
    /// Target GDC (typically Worker)
    pub target: String,
}

impl SynapseId {
    /// Create new synapse ID.
    pub fn new(source: String, target: String) -> Self {
        Self { source, target }
    }
    
    /// Create canonical ID string.
    pub fn canonical_id(&self) -> String {
        format!("{}→{}", self.source, self.target)
    }
}

/// Synapse - persistent connection between GDC pair.
///
/// Canon: AF-11 - Sinapses emergem de padrões de cooperação
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Synapse {
    /// Synapse identifier
    pub id: SynapseId,
    
    /// Synaptic strength (0.0 = weak, 1.0 = strong)
    pub strength: f64,
    
    /// Number of successful interactions
    pub success_count: u64,
    
    /// Number of failed interactions
    pub failure_count: u64,
    
    /// Last interaction timestamp (for pruning)
    pub last_used: u64,
    
    /// Formation timestamp
    pub formed_at: u64,
}

impl Synapse {
    /// Create new synapse with initial strength.
    ///
    /// Canon: AF-11 - Sinapses iniciam com força base
    pub fn new(id: SynapseId, initial_strength: f64, timestamp: u64) -> Self {
        Self {
            id,
            strength: initial_strength.clamp(0.0, 1.0),
            success_count: 0,
            failure_count: 0,
            last_used: timestamp,
            formed_at: timestamp,
        }
    }
    
    /// Check if synapse is active (strength above threshold).
    pub fn is_active(&self) -> bool {
        self.strength > 0.1 // Minimum threshold
    }
    
    /// Get success rate.
    pub fn success_rate(&self) -> f64 {
        let total = self.success_count + self.failure_count;
        if total == 0 {
            0.0
        } else {
            self.success_count as f64 / total as f64
        }
    }
}

// =============================================================================
// SYNAPSE NETWORK
// =============================================================================

/// Synapse network - manages all synapses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynapseNetwork {
    /// Active synapses
    pub(crate) synapses: HashMap<SynapseId, Synapse>,
    
    /// Current timestamp
    pub(crate) current_time: u64,
}

impl SynapseNetwork {
    /// Create new synapse network.
    pub fn new() -> Self {
        Self {
            synapses: HashMap::new(),
            current_time: 0,
        }
    }
    
    /// Form new synapse between GDCs.
    ///
    /// Canon: AF-11 - Sinapses emergem de cooperação
    pub fn form_synapse(
        &mut self,
        source: String,
        target: String,
        initial_strength: f64,
    ) -> SynapseId {
        let id = SynapseId::new(source, target);
        let synapse = Synapse::new(id.clone(), initial_strength, self.current_time);
        self.synapses.insert(id.clone(), synapse);
        self.current_time += 1;
        id
    }
    
    /// Get synapse by ID.
    pub fn get_synapse(&self, id: &SynapseId) -> Option<&Synapse> {
        self.synapses.get(id)
    }
    
    /// Get mutable synapse by ID.
    pub fn get_synapse_mut(&mut self, id: &SynapseId) -> Option<&mut Synapse> {
        self.synapses.get_mut(id)
    }
    
    /// Check if synapse exists.
    pub fn has_synapse(&self, id: &SynapseId) -> bool {
        self.synapses.contains_key(id)
    }
    
    /// Get all active synapses.
    pub fn active_synapses(&self) -> Vec<&Synapse> {
        self.synapses
            .values()
            .filter(|s| s.is_active())
            .collect()
    }
    
    /// Get synapse count.
    pub fn synapse_count(&self) -> usize {
        self.synapses.len()
    }
    
    /// Advance time.
    pub fn tick(&mut self) {
        self.current_time += 1;
    }
}

impl Default for SynapseNetwork {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_synapse_creation() {
        let id = SynapseId::new("queen_1".to_string(), "worker_1".to_string());
        let synapse = Synapse::new(id.clone(), 0.5, 100);
        
        assert_eq!(synapse.strength, 0.5);
        assert_eq!(synapse.formed_at, 100);
        assert!(synapse.is_active());
    }
    
    #[test]
    fn test_synapse_id_canonical() {
        let id = SynapseId::new("A".to_string(), "B".to_string());
        assert_eq!(id.canonical_id(), "A→B");
    }
    
    #[test]
    fn test_synapse_network_formation() {
        let mut network = SynapseNetwork::new();
        
        let id = network.form_synapse(
            "queen".to_string(),
            "worker".to_string(),
            0.3,
        );
        
        assert!(network.has_synapse(&id));
        assert_eq!(network.synapse_count(), 1);
        
        let synapse = network.get_synapse(&id).unwrap();
        assert_eq!(synapse.strength, 0.3);
    }
    
    #[test]
    fn test_synapse_forms_between_cooperating_gdcs() {
        // Test obrigatório v0.9.5
        let mut network = SynapseNetwork::new();
        
        let id = network.form_synapse(
            "gdc_queen".to_string(),
            "gdc_worker_1".to_string(),
            0.5,
        );
        
        // Canon: AF-11 - Sinapse emerge de cooperação
        assert!(network.has_synapse(&id));
        let synapse = network.get_synapse(&id).unwrap();
        assert!(synapse.is_active());
        assert_eq!(synapse.id.source, "gdc_queen");
        assert_eq!(synapse.id.target, "gdc_worker_1");
    }
}
