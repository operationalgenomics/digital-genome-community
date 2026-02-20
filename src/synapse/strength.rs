//! Canon v5.1 | GDC v0.9.5
//!
//! Synaptic Strength - Fortalecimento/Enfraquecimento
//!
//! Canon: AF-11 (Aprendizado por Incorporação Replayável)
//!
//! ## Core Principle
//! Força sináptica aumenta com uso bem-sucedido,
//! diminui com falhas ou desuso.

use super::connection::{Synapse, SynapseId, SynapseNetwork};

// =============================================================================
// STRENGTH MODULATION
// =============================================================================

/// Strength modulation parameters.
#[derive(Debug, Clone, Copy)]
pub struct StrengthParams {
    /// Strengthening rate per success
    pub strengthen_rate: f64,
    
    /// Weakening rate per failure
    pub weaken_rate: f64,
    
    /// Natural decay rate (per time unit)
    pub decay_rate: f64,
    
    /// Maximum strength
    pub max_strength: f64,
    
    /// Minimum strength before pruning
    pub min_strength: f64,
}

impl Default for StrengthParams {
    fn default() -> Self {
        Self {
            strengthen_rate: 0.1,
            weaken_rate: 0.05,
            decay_rate: 0.01,
            max_strength: 1.0,
            min_strength: 0.05,
        }
    }
}

/// Synaptic strength modulator.
///
/// Canon: AF-11 - LEI-AF-11-02 (Melhoria Estrita)
pub struct StrengthModulator {
    params: StrengthParams,
}

impl StrengthModulator {
    /// Create new strength modulator.
    pub fn new(params: StrengthParams) -> Self {
        Self { params }
    }
    
    /// Strengthen synapse after successful interaction.
    ///
    /// Canon: AF-11 - Incorporação por melhoria estrita
    pub fn strengthen(&self, synapse: &mut Synapse, timestamp: u64) {
        synapse.success_count += 1;
        synapse.last_used = timestamp;
        
        // Strengthen proportionally to current weakness
        let delta = self.params.strengthen_rate * (self.params.max_strength - synapse.strength);
        synapse.strength = (synapse.strength + delta).min(self.params.max_strength);
    }
    
    /// Weaken synapse after failed interaction.
    pub fn weaken(&self, synapse: &mut Synapse, timestamp: u64) {
        synapse.failure_count += 1;
        synapse.last_used = timestamp;
        
        synapse.strength = (synapse.strength - self.params.weaken_rate).max(0.0);
    }
    
    /// Apply natural decay (disuse weakening).
    ///
    /// Canon: AF-11 - Sinapses não utilizadas enfraquecem
    pub fn apply_decay(&self, synapse: &mut Synapse, time_elapsed: u64) {
        let decay = self.params.decay_rate * time_elapsed as f64;
        synapse.strength = (synapse.strength - decay).max(0.0);
    }
    
    /// Check if synapse should be pruned.
    pub fn should_prune(&self, synapse: &Synapse) -> bool {
        synapse.strength < self.params.min_strength
    }
}

impl Default for StrengthModulator {
    fn default() -> Self {
        Self::new(StrengthParams::default())
    }
}

// =============================================================================
// NETWORK EXTENSION
// =============================================================================

impl SynapseNetwork {
    /// Record successful interaction.
    pub fn record_success(&mut self, id: &SynapseId, modulator: &StrengthModulator) {
        let timestamp = self.current_time;
        if let Some(synapse) = self.get_synapse_mut(id) {
            modulator.strengthen(synapse, timestamp);
        }
    }
    
    /// Record failed interaction.
    pub fn record_failure(&mut self, id: &SynapseId, modulator: &StrengthModulator) {
        let timestamp = self.current_time;
        if let Some(synapse) = self.get_synapse_mut(id) {
            modulator.weaken(synapse, timestamp);
        }
    }
    
    /// Apply decay to all synapses.
    pub fn apply_decay(&mut self, modulator: &StrengthModulator, time_elapsed: u64) {
        for synapse in self.synapses.values_mut() {
            modulator.apply_decay(synapse, time_elapsed);
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
    fn test_synapse_strengthens_with_repeated_use() {
        // Teste obrigatório v0.9.5
        let mut network = SynapseNetwork::new();
        let modulator = StrengthModulator::default();
        
        let id = network.form_synapse("A".to_string(), "B".to_string(), 0.3);
        
        // Multiple successful uses
        for _ in 0..5 {
            network.record_success(&id, &modulator);
            network.tick();
        }
        
        let synapse = network.get_synapse(&id).unwrap();
        
        // Canon: AF-11 - Uso bem-sucedido fortalece
        assert!(synapse.strength > 0.3);
        assert_eq!(synapse.success_count, 5);
    }
    
    #[test]
    fn test_synapse_weakens_without_use() {
        // Teste obrigatório v0.9.5
        let mut network = SynapseNetwork::new();
        let modulator = StrengthModulator::default();
        
        let id = network.form_synapse("A".to_string(), "B".to_string(), 0.5);
        let initial_strength = network.get_synapse(&id).unwrap().strength;
        
        // Time passes without use
        network.apply_decay(&modulator, 10);
        
        let synapse = network.get_synapse(&id).unwrap();
        
        // Canon: AF-11 - Desuso enfraquece
        assert!(synapse.strength < initial_strength);
    }
    
    #[test]
    fn test_strengthen_increases_strength() {
        let modulator = StrengthModulator::default();
        let id = SynapseId::new("X".to_string(), "Y".to_string());
        let mut synapse = Synapse::new(id, 0.2, 0);
        
        modulator.strengthen(&mut synapse, 1);
        
        assert!(synapse.strength > 0.2);
        assert_eq!(synapse.success_count, 1);
    }
    
    #[test]
    fn test_weaken_decreases_strength() {
        let modulator = StrengthModulator::default();
        let id = SynapseId::new("X".to_string(), "Y".to_string());
        let mut synapse = Synapse::new(id, 0.5, 0);
        
        modulator.weaken(&mut synapse, 1);
        
        assert!(synapse.strength < 0.5);
        assert_eq!(synapse.failure_count, 1);
    }
}
