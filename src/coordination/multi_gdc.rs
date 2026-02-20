//! Canon v5.1 | GDC v0.9.5
//!
//! Multi-GDC Orchestration - Rainha + N Workers
//!
//! Canon: AO-21 (Orquestração Rainha/Worker)

use super::multi_field::MultiField;
use crate::coordination::containment::{StructuralWork, ResponseSet, ContainmentChecker};
use crate::results::result::CognitiveResult;

/// Multi-GDC orchestrator.
#[derive(Debug, Clone)]
pub struct MultiGdcOrchestrator {
    /// Worker IDs
    pub workers: Vec<String>,
    
    /// Active field
    pub field: MultiField,
    
    /// Response tracking
    pub responses: ResponseSet,
}

impl MultiGdcOrchestrator {
    pub fn new(workers: Vec<String>) -> Self {
        Self {
            workers,
            field: MultiField::new(),
            responses: ResponseSet::new(),
        }
    }
    
    /// Distribute work to N workers.
    pub fn distribute_work(&self, work: &StructuralWork) -> Vec<(String, Vec<u8>)> {
        // Simplified distribution
        self.workers
            .iter()
            .zip(work.chunks.iter())
            .map(|(w, chunk)| (w.clone(), chunk.payload.clone()))
            .collect()
    }
    
    /// Collect response from worker.
    pub fn collect_response(&mut self, worker_id: String, result: CognitiveResult) {
        self.responses.add_response(worker_id);
        self.field.add_manifestation(result);
    }
    
    /// Check if orchestration is complete.
    pub fn is_complete(&self, work: &StructuralWork) -> bool {
        ContainmentChecker::check_containment(&self.responses, work)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::results::phenotype::{StructuralGraph, CanonicalForm};
    use std::collections::BTreeMap;
    
    #[test]
    fn test_determinism_preserved_at_scale() {
        // Teste obrigatório v0.9.5
        let workers = vec!["w1".to_string(), "w2".to_string(), "w3".to_string()];
        let mut orch1 = MultiGdcOrchestrator::new(workers.clone());
        let mut orch2 = MultiGdcOrchestrator::new(workers);
        
        // Same input
        let graph = StructuralGraph::new(vec!["A".to_string()], vec![]);
        let cf = CanonicalForm::compute(&graph);
        let result = CognitiveResult::new(cf.clone(), BTreeMap::new(), 1, true);
        
        // Same processing
        orch1.collect_response("w1".to_string(), result.clone());
        orch2.collect_response("w1".to_string(), result);
        
        // Canon: AF-6 - Determinismo preservado em escala
        assert_eq!(orch1.field.count(), orch2.field.count());
        assert_eq!(orch1.field.manifestations[0], orch2.field.manifestations[0]);
    }
}
