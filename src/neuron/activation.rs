//! Canon v5.1 | GDC v0.9.5
//!
//! Activation Pattern - Padrão de ativação baseado em ressonância
//!
//! Canon: AF-15 (Ressonância Estrutural)

use serde::{Deserialize, Serialize};

// =============================================================================
// ACTIVATION PATTERN
// =============================================================================

/// Activation pattern - signature of GDC firing.
///
/// Canon: AF-15 - Ressonância como padrão de ativação
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActivationPattern {
    /// GDC identifier
    pub gdc_id: String,
    
    /// Activation signature (simplified hash)
    pub signature: u64,
    
    /// Activation strength (0.0-1.0)
    pub strength: f64,
    
    /// Timestamp
    pub timestamp: u64,
}

impl ActivationPattern {
    /// Create new activation pattern.
    pub fn new(gdc_id: String, signature: u64, strength: f64, timestamp: u64) -> Self {
        Self {
            gdc_id,
            signature,
            strength: strength.clamp(0.0, 1.0),
            timestamp,
        }
    }
    
    /// Check if patterns are similar (within threshold).
    pub fn is_similar(&self, other: &Self, threshold: f64) -> bool {
        // Simplified: check if signatures match
        // Real implementation would use structural similarity
        self.signature == other.signature && (self.strength - other.strength).abs() < threshold
    }
}

// =============================================================================
// ACTIVATION TRACKER
// =============================================================================

/// Tracks activation patterns across GDCs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivationTracker {
    /// Recent activations
    activations: Vec<ActivationPattern>,
    
    /// Maximum history size
    max_history: usize,
}

impl ActivationTracker {
    /// Create new activation tracker.
    pub fn new(max_history: usize) -> Self {
        Self {
            activations: Vec::new(),
            max_history,
        }
    }
    
    /// Record new activation.
    pub fn record(&mut self, pattern: ActivationPattern) {
        self.activations.push(pattern);
        
        // Keep only recent history
        if self.activations.len() > self.max_history {
            self.activations.remove(0);
        }
    }
    
    /// Find similar patterns.
    pub fn find_similar(&self, pattern: &ActivationPattern, threshold: f64) -> Vec<&ActivationPattern> {
        self.activations
            .iter()
            .filter(|p| p.is_similar(pattern, threshold))
            .collect()
    }
    
    /// Get activation count for GDC.
    pub fn activation_count(&self, gdc_id: &str) -> usize {
        self.activations.iter().filter(|p| p.gdc_id == gdc_id).count()
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_activation_pattern_creation() {
        let pattern = ActivationPattern::new("gdc_1".to_string(), 0x1234, 0.7, 100);
        
        assert_eq!(pattern.gdc_id, "gdc_1");
        assert_eq!(pattern.signature, 0x1234);
        assert_eq!(pattern.strength, 0.7);
    }
    
    #[test]
    fn test_similar_patterns() {
        let p1 = ActivationPattern::new("A".to_string(), 0xABCD, 0.5, 1);
        let p2 = ActivationPattern::new("A".to_string(), 0xABCD, 0.52, 2);
        let p3 = ActivationPattern::new("A".to_string(), 0x1234, 0.5, 3);
        
        assert!(p1.is_similar(&p2, 0.1));
        assert!(!p1.is_similar(&p3, 0.1));
    }
    
    #[test]
    fn test_activation_tracker() {
        let mut tracker = ActivationTracker::new(10);
        
        let p1 = ActivationPattern::new("gdc_1".to_string(), 0x100, 0.6, 1);
        let p2 = ActivationPattern::new("gdc_1".to_string(), 0x100, 0.61, 2);
        
        tracker.record(p1.clone());
        tracker.record(p2);
        
        assert_eq!(tracker.activation_count("gdc_1"), 2);
        
        let similar = tracker.find_similar(&p1, 0.1);
        assert_eq!(similar.len(), 2);
    }
}
