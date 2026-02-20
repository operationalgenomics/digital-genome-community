//! Canon v5.1 | GDC v0.9.5
//!
//! Multi-Field - R(Σ) com N manifestações
//!
//! Canon: LEI-COORD-03 (Absorção Estrutural)

use crate::results::result::CognitiveResult;
use serde::{Deserialize, Serialize};

/// Multi-manifestation field - integrates N GDC responses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiField {
    /// Integrated results from N GDCs
    pub manifestations: Vec<CognitiveResult>,
    
    /// Integration complete
    pub complete: bool,
}

impl MultiField {
    pub fn new() -> Self {
        Self {
            manifestations: Vec::new(),
            complete: false,
        }
    }
    
    /// Add manifestation.
    pub fn add_manifestation(&mut self, result: CognitiveResult) {
        self.manifestations.push(result);
    }
    
    /// Mark as complete.
    pub fn mark_complete(&mut self) {
        self.complete = true;
    }
    
    /// Get manifestation count.
    pub fn count(&self) -> usize {
        self.manifestations.len()
    }
}

impl Default for MultiField {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::results::phenotype::{StructuralGraph, CanonicalForm};
    use std::collections::BTreeMap;
    
    #[test]
    fn test_multi_gdc_field_integrates_n_manifestations() {
        // Teste obrigatório v0.9.5
        let mut field = MultiField::new();
        
        // Simulate N GDC responses
        for i in 0..5 {
            let graph = StructuralGraph::new(
                vec![format!("node_{}", i)],
                vec![],
            );
            let cf = CanonicalForm::compute(&graph);
            let result = CognitiveResult::new(cf, BTreeMap::new(), 1, true);
            field.add_manifestation(result);
        }
        
        field.mark_complete();
        
        // Canon: LEI-COORD-03 - Campo integra N manifestações
        assert_eq!(field.count(), 5);
        assert!(field.complete);
    }
}
