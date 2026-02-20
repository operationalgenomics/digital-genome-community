//! Canon v5.1 | GDC v0.9.5
//!
//! Neuron Cluster - Agrupamento funcional emergente
//!
//! Canon: AF-11 + AF-15 (Emergência por padrão de ativação)

use super::activation::ActivationPattern;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

// =============================================================================
// NEURON CLUSTER
// =============================================================================

/// Neuron cluster - functional grouping of GDCs.
///
/// Canon: AF-15 - Neurônios emergem de padrões de ressonância
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuronCluster {
    /// Cluster ID
    pub id: String,
    
    /// Member GDC IDs
    pub members: HashSet<String>,
    
    /// Characteristic activation pattern
    pub characteristic_pattern: u64,
    
    /// Cluster cohesion (0.0-1.0)
    pub cohesion: f64,
    
    /// Formation timestamp
    pub formed_at: u64,
}

impl NeuronCluster {
    /// Create new neuron cluster.
    pub fn new(id: String, pattern: u64, timestamp: u64) -> Self {
        Self {
            id,
            members: HashSet::new(),
            characteristic_pattern: pattern,
            cohesion: 0.0,
            formed_at: timestamp,
        }
    }
    
    /// Add member to cluster.
    pub fn add_member(&mut self, gdc_id: String) {
        self.members.insert(gdc_id);
        self.update_cohesion();
    }
    
    /// Remove member from cluster.
    pub fn remove_member(&mut self, gdc_id: &str) -> bool {
        let removed = self.members.remove(gdc_id);
        if removed {
            self.update_cohesion();
        }
        removed
    }
    
    /// Update cluster cohesion.
    fn update_cohesion(&mut self) {
        // Simplified: cohesion based on member count
        self.cohesion = (self.members.len() as f64 / 10.0).min(1.0);
    }
    
    /// Check if cluster matches pattern.
    pub fn matches_pattern(&self, pattern: &ActivationPattern) -> bool {
        pattern.signature == self.characteristic_pattern
    }
}

// =============================================================================
// CLUSTER NETWORK
// =============================================================================

/// Network of neuron clusters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterNetwork {
    /// Active clusters
    clusters: Vec<NeuronCluster>,
    
    /// Next cluster ID
    next_id: usize,
    
    /// Current timestamp
    current_time: u64,
}

impl ClusterNetwork {
    /// Create new cluster network.
    pub fn new() -> Self {
        Self {
            clusters: Vec::new(),
            next_id: 0,
            current_time: 0,
        }
    }
    
    /// Form new cluster from activation pattern.
    ///
    /// Canon: AF-15 - Neurônios emergem de padrões recorrentes
    pub fn form_cluster(&mut self, pattern: &ActivationPattern) -> String {
        let id = format!("cluster_{}", self.next_id);
        self.next_id += 1;
        
        let mut cluster = NeuronCluster::new(id.clone(), pattern.signature, self.current_time);
        cluster.add_member(pattern.gdc_id.clone());
        
        self.clusters.push(cluster);
        self.current_time += 1;
        
        id
    }
    
    /// Find cluster matching pattern.
    pub fn find_matching_cluster(&self, pattern: &ActivationPattern) -> Option<&NeuronCluster> {
        self.clusters.iter().find(|c| c.matches_pattern(pattern))
    }
    
    /// Get cluster count.
    pub fn cluster_count(&self) -> usize {
        self.clusters.len()
    }
}

impl Default for ClusterNetwork {
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
    fn test_neuron_cluster_creation() {
        let cluster = NeuronCluster::new("c1".to_string(), 0xABCD, 100);
        
        assert_eq!(cluster.id, "c1");
        assert_eq!(cluster.characteristic_pattern, 0xABCD);
        assert_eq!(cluster.members.len(), 0);
    }
    
    #[test]
    fn test_cluster_add_remove_members() {
        let mut cluster = NeuronCluster::new("c1".to_string(), 0x100, 0);
        
        cluster.add_member("gdc_1".to_string());
        cluster.add_member("gdc_2".to_string());
        
        assert_eq!(cluster.members.len(), 2);
        assert!(cluster.cohesion > 0.0);
        
        cluster.remove_member("gdc_1");
        assert_eq!(cluster.members.len(), 1);
    }
    
    #[test]
    fn test_neuron_cluster_emerges_from_activation_pattern() {
        // Teste obrigatório v0.9.5
        let mut network = ClusterNetwork::new();
        
        let pattern = ActivationPattern::new("gdc_worker_1".to_string(), 0xDEADBEEF, 0.8, 1);
        
        let cluster_id = network.form_cluster(&pattern);
        
        // Canon: AF-15 - Neurônio emerge de padrão de ativação
        assert_eq!(network.cluster_count(), 1);
        
        let cluster = network.find_matching_cluster(&pattern).unwrap();
        assert_eq!(cluster.id, cluster_id);
        assert!(cluster.members.contains("gdc_worker_1"));
        assert_eq!(cluster.characteristic_pattern, 0xDEADBEEF);
    }
}
