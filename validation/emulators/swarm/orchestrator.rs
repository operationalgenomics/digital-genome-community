//! Canon v5.1 | GDC v1.0.0δ
//!
//! SwarmOrchestrator - INSTRUMENTAÇÃO EXTERNA
//!
//! IMPORTANTE: Este componente NÃO pertence ao Core canônico.
//! É EXTERNO ao GDC, usado apenas para teste e validação.
//!
//! Canon v5.1, linha 6491:
//! "As camadas superiores (GDO, GDE, e quaisquer outras) não pertencem ao Canon do GDC."
//!
//! ## Responsabilidade
//!
//! Instrumentar N ≥ 10 GDCs para validar comportamento de enxame.
//! NÃO possui autoridade estrutural. NÃO altera CF(G).

use digital_genome_community::coordination::vibration::*;
use digital_genome_community::coordination::manifestation_delta::*;
use digital_genome_community::coordination::fragmentation::*;
use digital_genome_community::coordination::weaving::*;
use digital_genome_community::coordination::budget_delta::BudgetDelta;
use std::collections::{HashMap, HashSet};

// =============================================================================
// SWARM ORCHESTRATOR (EXTERNO)
// =============================================================================

/// Orquestrador de Enxame - EXTERNO ao Core.
///
/// Status: Teste/Instrumentação
/// Autoridade: NENHUMA
/// Afeta CF(G): NÃO
pub struct SwarmOrchestrator {
    /// GDCs instrumentados
    gdcs: Vec<GdcNode>,
    
    /// Métricas de observação
    metrics: SwarmMetrics,
}

impl SwarmOrchestrator {
    /// Criar orquestrador de teste.
    pub fn new() -> Self {
        SwarmOrchestrator {
            gdcs: Vec::new(),
            metrics: SwarmMetrics::default(),
        }
    }
    
    /// Adicionar GDC ao ambiente de teste.
    pub fn add_gdc(&mut self, gdc: GdcNode) {
        self.gdcs.push(gdc);
    }
    
    /// Observar ciclo de enxame (não controlar).
    pub fn observe_swarm_cycle(&mut self, stimulus: Vec<u8>) -> SwarmObservation {
        let n_gdcs = self.gdcs.len();
        
        if n_gdcs < 10 {
            return SwarmObservation {
                success: false,
                error: Some("N < 10 GDCs".to_string()),
                queen_id: None,
                worker_count: 0,
                dna_emitted: false,
            };
        }
        
        // 1. Simular: primeiro GDC vira Queen
        let queen_id = self.gdcs[0].id;
        let work_id = WorkId::new();
        
        // 2. Queen emite vibração
        let vibration = Vibration::emit(
            work_id,
            StructuralRequirements::basic(),
            BudgetDelta::default(),
        );
        
        self.metrics.vibrations_emitted += 1;
        
        // 3. Workers manifestam (se compatíveis)
        let mut manifestations = Vec::new();
        for gdc in &self.gdcs[1..] {
            if let Some(manifest) = ManifestationDelta::manifest_if_compatible(
                gdc.id,
                &vibration,
                gdc.available_budget.clone(),
                gdc.capabilities.clone(),
            ) {
                manifestations.push(manifest);
            }
        }
        
        self.metrics.manifestations_received += manifestations.len();
        
        // 4. Queen fragmenta trabalho
        let mut worker_budgets = HashMap::new();
        for m in &manifestations {
            worker_budgets.insert(WorkerId::new(), m.available_budget.clone());
        }
        
        let fragmentation = WorkFragmentation::fragment(
            work_id,
            &stimulus,
            manifestations.len(),
            &worker_budgets,
        );
        
        // 5. Simular processamento + tecelagem
        let expected_chunks: HashSet<ChunkId> = fragmentation.chunks
            .iter()
            .map(|c| c.id)
            .collect();
        
        let _weaving = WeavingEngine::new(expected_chunks);
        
        // Simular recebimento de todos EDRs
        // (EDR real requer muito código, usamos mock aqui)
        let all_received = fragmentation.chunks.len() == manifestations.len();
        
        SwarmObservation {
            success: true,
            error: None,
            queen_id: Some(queen_id),
            worker_count: manifestations.len(),
            dna_emitted: all_received,
        }
    }
    
    /// Obter métricas.
    pub fn metrics(&self) -> &SwarmMetrics {
        &self.metrics
    }
}

impl Default for SwarmOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// GDC NODE (Representação Externa)
// =============================================================================

/// Representação externa de um GDC.
pub struct GdcNode {
    pub id: GdcId,
    pub available_budget: BudgetDelta,
    pub capabilities: HashSet<Capability>,
}

impl GdcNode {
    pub fn new(id: GdcId) -> Self {
        let mut capabilities = HashSet::new();
        capabilities.insert(Capability::ProcessUnl);
        capabilities.insert(Capability::ComputeMotors);
        
        GdcNode {
            id,
            available_budget: BudgetDelta::default(),
            capabilities,
        }
    }
}

// =============================================================================
// MÉTRICAS
// =============================================================================

/// Métricas de observação do enxame.
#[derive(Debug, Default)]
pub struct SwarmMetrics {
    pub vibrations_emitted: usize,
    pub manifestations_received: usize,
    pub cycles_observed: usize,
}

/// Observação de um ciclo.
#[derive(Debug)]
pub struct SwarmObservation {
    pub success: bool,
    pub error: Option<String>,
    pub queen_id: Option<GdcId>,
    pub worker_count: usize,
    pub dna_emitted: bool,
}

// =============================================================================
// TESTES
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_swarm_orchestrator_creation() {
        let orchestrator = SwarmOrchestrator::new();
        assert_eq!(orchestrator.gdcs.len(), 0);
    }
    
    #[test]
    fn test_add_gdcs() {
        let mut orchestrator = SwarmOrchestrator::new();
        
        for _i in 0..10 {
            let gdc = GdcNode::new(GdcId::new());
            orchestrator.add_gdc(gdc);
        }
        
        assert_eq!(orchestrator.gdcs.len(), 10);
    }
    
    #[test]
    fn test_observe_cycle_requires_10_gdcs() {
        let mut orchestrator = SwarmOrchestrator::new();
        
        // Apenas 5 GDCs
        for _ in 0..5 {
            orchestrator.add_gdc(GdcNode::new(GdcId::new()));
        }
        
        let observation = orchestrator.observe_swarm_cycle(vec![1, 2, 3, 4]);
        
        assert!(!observation.success);
        assert!(observation.error.is_some());
    }
    
    #[test]
    fn test_observe_cycle_with_10_gdcs() {
        let mut orchestrator = SwarmOrchestrator::new();
        
        // 10 GDCs
        for _ in 0..10 {
            orchestrator.add_gdc(GdcNode::new(GdcId::new()));
        }
        
        let observation = orchestrator.observe_swarm_cycle(vec![1, 2, 3, 4]);
        
        assert!(observation.success);
        assert!(observation.queen_id.is_some());
        assert!(observation.worker_count > 0);
    }
}
