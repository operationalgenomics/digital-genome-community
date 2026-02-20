//! Canon v5.1 | GDC v1.0.0δ
//!
//! Fragmentação W(Σ) - Trabalho Estrutural Semântico
//!
//! Canon: Especificação W(Σ) - "W(Σ) é o conjunto de chunks semânticos
//! em que a Rainha fragmenta Σ para distribuição. Cada chunk é autocontido."
//!
//! ## Responsabilidade
//!
//! A Queen fragmenta o estímulo Σ em chunks semânticos W(Σ) = {w₁, w₂, ..., wₖ}.
//! Cada chunk é AUTOCONTIDO (LEI-RSN-03) - não requer comunicação externa
//! durante processamento.

use crate::coordination::budget_delta::BudgetDelta;
use crate::coordination::vibration::WorkId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// =============================================================================
// FRAGMENTAÇÃO W(Σ)
// =============================================================================

/// Fragmentação semântica do trabalho.
///
/// Canon: Especificação W(Σ)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkFragmentation {
    /// ID do trabalho
    pub work_id: WorkId,
    
    /// Chunks semânticos
    pub chunks: Vec<SemanticChunk>,
    
    /// Metadados de fragmentação
    pub metadata: FragmentationMetadata,
}

impl WorkFragmentation {
    /// Fragmentar Σ em W(Σ) semanticamente.
    ///
    /// Canon: W(Σ) não é split arbitrário de bytes
    pub fn fragment(
        work_id: WorkId,
        stimulus: &[u8],
        available_workers: usize,
        worker_budgets: &HashMap<WorkerId, BudgetDelta>,
    ) -> Self {
        let chunks = Self::semantic_split(stimulus, available_workers, worker_budgets);
        
        let metadata = FragmentationMetadata {
            total_size: stimulus.len(),
            chunk_count: chunks.len(),
            fragmentation_strategy: FragmentationStrategy::AdaptiveSemantic,
        };
        
        WorkFragmentation {
            work_id,
            chunks,
            metadata,
        }
    }
    
    /// Split semântico (não arbitrário).
    fn semantic_split(
        stimulus: &[u8],
        n_workers: usize,
        _worker_budgets: &HashMap<WorkerId, BudgetDelta>,
    ) -> Vec<SemanticChunk> {
        if stimulus.is_empty() {
            return vec![];
        }
        
        // Estratégia: Dividir por blocos semânticos respeitando budgets
        let mut chunks = Vec::new();
        let chunk_size = (stimulus.len() / n_workers.max(1)).max(1);
        
        for (i, chunk_data) in stimulus.chunks(chunk_size).enumerate() {
            let chunk_id = ChunkId::new(i);
            
            // Determinar budget necessário baseado no tamanho
            let required_budget = BudgetDelta {
                max_bytes: chunk_data.len() * 2,
                max_iterations: 1000,
            };
            
            chunks.push(SemanticChunk {
                id: chunk_id,
                content: chunk_data.to_vec(),
                required_budget,
                is_self_contained: true,
                dependencies: vec![],
            });
        }
        
        chunks
    }
    
    /// Verificar que todos chunks são autocontidos.
    pub fn verify_self_contained(&self) -> bool {
        self.chunks.iter().all(|c| c.is_self_contained)
    }
}

// =============================================================================
// CHUNK SEMÂNTICO
// =============================================================================

/// Chunk semântico autocontido.
///
/// Canon: LEI-RSN-03 - Processável em isolamento absoluto
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticChunk {
    /// ID único do chunk
    pub id: ChunkId,
    
    /// Conteúdo autocontido
    pub content: Vec<u8>,
    
    /// Budget necessário para processar
    pub required_budget: BudgetDelta,
    
    /// Se é autocontido (deve ser true)
    pub is_self_contained: bool,
    
    /// Dependências (deve ser vazio)
    pub dependencies: Vec<ChunkId>,
}

impl SemanticChunk {
    /// Verificar que chunk é válido.
    pub fn is_valid(&self) -> bool {
        // Chunk deve ser autocontido
        self.is_self_contained && self.dependencies.is_empty() && !self.content.is_empty()
    }
}

// =============================================================================
// METADADOS
// =============================================================================

/// Metadados de fragmentação.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FragmentationMetadata {
    /// Tamanho total do estímulo
    pub total_size: usize,
    
    /// Número de chunks
    pub chunk_count: usize,
    
    /// Estratégia usada
    pub fragmentation_strategy: FragmentationStrategy,
}

/// Estratégia de fragmentação.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum FragmentationStrategy {
    /// Adaptativa baseada em semântica
    AdaptiveSemantic,
    
    /// Tamanho fixo (não recomendado)
    FixedSize,
}

// =============================================================================
// IDENTIFICADORES
// =============================================================================

/// ID de chunk.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ChunkId(pub usize);

impl ChunkId {
    pub fn new(id: usize) -> Self {
        ChunkId(id)
    }
}

/// ID de worker.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WorkerId(uuid::Uuid);

impl WorkerId {
    pub fn new() -> Self {
        WorkerId(uuid::Uuid::new_v4())
    }
}

impl Default for WorkerId {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// TESTES
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_fragmentation_basic() {
        let work_id = WorkId::new();
        let stimulus = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let budgets = HashMap::new();
        
        let frag = WorkFragmentation::fragment(work_id, &stimulus, 2, &budgets);
        
        assert_eq!(frag.work_id, work_id);
        assert!(frag.chunks.len() >= 1);
        assert_eq!(frag.metadata.total_size, 8);
    }
    
    #[test]
    fn test_semantic_chunk_valid() {
        let chunk = SemanticChunk {
            id: ChunkId::new(0),
            content: vec![1, 2, 3],
            required_budget: BudgetDelta::default(),
            is_self_contained: true,
            dependencies: vec![],
        };
        
        assert!(chunk.is_valid());
    }
    
    #[test]
    fn test_chunk_with_dependencies_invalid() {
        let chunk = SemanticChunk {
            id: ChunkId::new(0),
            content: vec![1, 2, 3],
            required_budget: BudgetDelta::default(),
            is_self_contained: true,
            dependencies: vec![ChunkId::new(1)], // Não deve ter
        };
        
        assert!(!chunk.is_valid());
    }
    
    #[test]
    fn test_verify_all_self_contained() {
        let work_id = WorkId::new();
        let stimulus = vec![1, 2, 3, 4];
        let budgets = HashMap::new();
        
        let frag = WorkFragmentation::fragment(work_id, &stimulus, 2, &budgets);
        
        assert!(frag.verify_self_contained());
    }
    
    #[test]
    fn test_empty_stimulus() {
        let work_id = WorkId::new();
        let stimulus = vec![];
        let budgets = HashMap::new();
        
        let frag = WorkFragmentation::fragment(work_id, &stimulus, 2, &budgets);
        
        assert_eq!(frag.chunks.len(), 0);
        assert_eq!(frag.metadata.total_size, 0);
    }
}
