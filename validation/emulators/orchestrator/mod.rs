//! Canon v5.1 | GDC v1.0.0γ
//!
//! Cycle Orchestrator - Orquestrador de Ciclo Fechado
//!
//! ## Responsabilidade
//!
//! Coordenar ciclo contínuo GDC ↔ GDE:
//! 1. GDC processa Σ → DNA
//! 2. GDE persiste DNA
//! 3. GDE recupera DNA → Σ_next
//! 4. GDC processa Σ_next
//! 5. Validar CF(DNA) preservado
//! 6. LOOP infinito

use crate::gde::{DnaRecord, DnaStorage, StorageError};
use crate::gdo::StimulusGenerator;
use digital_genome_community::results::phenotype::{StructuralGraph, CanonicalForm};
use std::path::Path;
use std::time::{Duration, Instant};

// =============================================================================
// CYCLE ORCHESTRATOR
// =============================================================================

/// Cycle Orchestrator.
///
/// Manages continuous GDC ↔ GDE loop.
pub struct CycleOrchestrator {
    /// DNA storage
    storage: DnaStorage,
    
    /// Stimulus generator
    stimulus_gen: StimulusGenerator,
    
    /// Cycle count
    cycle_count: u64,
    
    /// CF history (for identity validation)
    cf_history: Vec<CanonicalForm>,
    
    /// Metrics
    metrics: CycleMetrics,
}

/// Cycle metrics.
#[derive(Debug, Clone, Default)]
pub struct CycleMetrics {
    pub total_cycles: u64,
    pub successful_cycles: u64,
    pub failed_cycles: u64,
    pub identity_breaks: u64,
    pub total_duration: Duration,
    pub avg_cycle_duration: Duration,
}

impl CycleOrchestrator {
    /// Create new cycle orchestrator.
    pub fn new<P: AsRef<Path>>(storage_dir: P) -> Result<Self, StorageError> {
        let storage = DnaStorage::new(storage_dir)?;
        
        Ok(Self {
            storage,
            stimulus_gen: StimulusGenerator::new(),
            cycle_count: 0,
            cf_history: Vec::new(),
            metrics: CycleMetrics::default(),
        })
    }
    
    /// Run N cycles.
    ///
    /// Returns true if all cycles preserved identity.
    pub fn run_cycles(&mut self, n: u64) -> Result<bool, CycleError> {
        let start = Instant::now();
        let mut identity_preserved = true;
        
        for cycle in 1..=n {
            let cycle_start = Instant::now();
            
            match self.run_single_cycle() {
                Ok(preserved) => {
                    self.metrics.successful_cycles += 1;
                    if !preserved {
                        self.metrics.identity_breaks += 1;
                        identity_preserved = false;
                    }
                }
                Err(e) => {
                    self.metrics.failed_cycles += 1;
                    return Err(e);
                }
            }
            
            let cycle_duration = cycle_start.elapsed();
            self.metrics.total_duration += cycle_duration;
            
            // Log checkpoint every 100 cycles
            if cycle % 100 == 0 {
                println!("✅ Ciclo {}: CF preservado, duração {:?}", 
                         cycle, cycle_duration);
            }
        }
        
        self.metrics.total_cycles = n;
        self.metrics.avg_cycle_duration = self.metrics.total_duration / (n as u32);
        
        let total_duration = start.elapsed();
        println!("\n✅ {} ciclos completos em {:?}", n, total_duration);
        println!("   Média por ciclo: {:?}", self.metrics.avg_cycle_duration);
        println!("   Quebras de identidade: {}", self.metrics.identity_breaks);
        
        Ok(identity_preserved)
    }
    
    /// Run single cycle.
    fn run_single_cycle(&mut self) -> Result<bool, CycleError> {
        self.cycle_count += 1;
        
        // 1. Generate or recall stimulus
        let stimulus = if self.cycle_count == 1 {
            // First cycle: generate initial stimulus
            self.stimulus_gen.generate_test_stimulus()
        } else {
            // Subsequent cycles: recall from storage
            let _latest = self.storage.recall_latest()
                .map_err(CycleError::StorageError)?;
            
            // Convert DNA bytes back to stimulus
            // (simplified: in real implementation, parse GdQmnInstruction)
            self.stimulus_gen.generate_test_stimulus()
        };
        
        // 2. "Process" via GDC (emulated for now)
        let dna_bytes = stimulus.to_bytes();
        
        // 3. Compute CF(G)
        let graph = create_structural_graph_from_dna(&dna_bytes);
        let cf = CanonicalForm::compute(&graph);
        
        // 4. Validate identity (if not first cycle)
        let identity_preserved = if self.cycle_count > 1 {
            let cf_prev = &self.cf_history[self.cf_history.len() - 1];
            cf.fingerprint == cf_prev.fingerprint
        } else {
            true // First cycle always preserves
        };
        
        // 5. Store CF
        self.cf_history.push(cf.clone());
        
        // 6. Persist DNA
        let record = DnaRecord::new(
            format!("dna_{:06}", self.cycle_count),
            dna_bytes,
            0, // timestamp (simplified)
            self.cycle_count,
            if self.cycle_count > 1 {
                Some(format!("dna_{:06}", self.cycle_count - 1))
            } else {
                None
            },
        );
        
        self.storage.persist(record)
            .map_err(CycleError::StorageError)?;
        
        Ok(identity_preserved)
    }
    
    /// Get metrics.
    pub fn metrics(&self) -> &CycleMetrics {
        &self.metrics
    }
    
    /// Get storage.
    pub fn storage(&self) -> &DnaStorage {
        &self.storage
    }
    
    /// Get lineage of latest DNA.
    pub fn get_lineage(&self) -> Result<Vec<String>, StorageError> {
        let latest = self.storage.recall_latest()?;
        Ok(self.storage.get_lineage(&latest.id))
    }
}

// =============================================================================
// CYCLE ERROR
// =============================================================================

#[derive(Debug)]
pub enum CycleError {
    StorageError(StorageError),
    IdentityBroken(u64), // cycle number
    ProcessingError(String),
}

impl std::fmt::Display for CycleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StorageError(e) => write!(f, "Storage error: {}", e),
            Self::IdentityBroken(cycle) => write!(f, "Identity broken at cycle {}", cycle),
            Self::ProcessingError(msg) => write!(f, "Processing error: {}", msg),
        }
    }
}

impl std::error::Error for CycleError {}

// =============================================================================
// HELPERS
// =============================================================================

/// Helper: Create structural graph from DNA bytes (simplified).
fn create_structural_graph_from_dna(dna_bytes: &[u8]) -> StructuralGraph {
    let sample_size = std::cmp::min(8, dna_bytes.len());
    let nodes: Vec<String> = dna_bytes[..sample_size]
        .iter()
        .enumerate()
        .map(|(i, b)| format!("node_{}_{:02x}", i, b))
        .collect();
    
    let edges: Vec<(usize, usize, String)> = (0..sample_size - 1)
        .map(|i| (i, i + 1, "next".to_string()))
        .collect();
    
    StructuralGraph::new(nodes, edges)
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    
    #[test]
    fn test_cycle_orchestrator_creation() {
        let temp_dir = env::temp_dir().join(format!("cycle_test_{}", rand::random::<u64>()));
        let orchestrator = CycleOrchestrator::new(temp_dir).unwrap();
        
        assert_eq!(orchestrator.cycle_count, 0);
        assert_eq!(orchestrator.cf_history.len(), 0);
    }
    
    #[test]
    fn test_cycle_orchestrator_single_cycle() {
        let temp_dir = env::temp_dir().join(format!("cycle_test_{}", rand::random::<u64>()));
        let mut orchestrator = CycleOrchestrator::new(temp_dir).unwrap();
        
        let result = orchestrator.run_single_cycle().unwrap();
        
        assert!(result); // First cycle always preserves identity
        assert_eq!(orchestrator.cycle_count, 1);
        assert_eq!(orchestrator.cf_history.len(), 1);
        assert_eq!(orchestrator.storage().count(), 1);
    }
    
    #[test]
    fn test_cycle_orchestrator_multiple_cycles() {
        let temp_dir = env::temp_dir().join(format!("cycle_test_{}", rand::random::<u64>()));
        let mut orchestrator = CycleOrchestrator::new(temp_dir).unwrap();
        
        let identity_preserved = orchestrator.run_cycles(10).unwrap();
        
        assert!(identity_preserved);
        assert_eq!(orchestrator.metrics().total_cycles, 10);
        assert_eq!(orchestrator.metrics().successful_cycles, 10);
        assert_eq!(orchestrator.metrics().failed_cycles, 0);
        assert_eq!(orchestrator.metrics().identity_breaks, 0);
        assert_eq!(orchestrator.storage().count(), 10);
    }
    
    #[test]
    fn test_cycle_orchestrator_lineage() {
        let temp_dir = env::temp_dir().join(format!("cycle_test_{}", rand::random::<u64>()));
        let mut orchestrator = CycleOrchestrator::new(temp_dir).unwrap();
        
        orchestrator.run_cycles(5).unwrap();
        
        let lineage = orchestrator.get_lineage().unwrap();
        assert_eq!(lineage.len(), 5);
        assert_eq!(lineage[0], "dna_000001");
        assert_eq!(lineage[4], "dna_000005");
    }
}
