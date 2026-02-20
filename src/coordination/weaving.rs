//! Canon v5.1 | GDC v1.0.0δ
//!
//! Tecelagem ⨆ - Integração Progressiva de Retornos
//!
//! Canon: Especificação ⊒ - "A Rainha tece resultados à medida que chegam,
//! via operação ⨆. Verificação é por completude estrutural, não enumeração."
//!
//! ## Responsabilidade
//!
//! Queen tece EDRs progressivamente até atingir S ⊒ W(Σ) (completude).
//! Quando DNA fecha estruturalmente, emissão é possível.

use crate::coordination::edr::Edr;
use crate::coordination::fragmentation::ChunkId;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

// =============================================================================
// MOTOR DE TECELAGEM
// =============================================================================

/// Motor de tecelagem progressiva.
///
/// Canon: Operação ⨆
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeavingEngine {
    /// EDRs recebidos
    received_edrs: Vec<EdWrapped>,
    
    /// Estado tecido atual
    woven_state: WovenState,
    
    /// Chunks esperados
    expected_chunks: HashSet<ChunkId>,
    
    /// Chunks recebidos
    received_chunks: HashSet<ChunkId>,
}

impl WeavingEngine {
    /// Criar motor de tecelagem.
    pub fn new(expected_chunks: HashSet<ChunkId>) -> Self {
        WeavingEngine {
            received_edrs: Vec::new(),
            woven_state: WovenState::Empty,
            expected_chunks,
            received_chunks: HashSet::new(),
        }
    }
    
    /// Tecer novo EDR no estado (⨆).
    ///
    /// Canon: Tecelagem progressiva
    pub fn weave(&mut self, edr: Edr, chunk_id: ChunkId) {
        // Adicionar EDR
        self.received_edrs.push(EdWrapped { edr, chunk_id });
        self.received_chunks.insert(chunk_id);
        
        // Recomputar estado tecido
        self.woven_state = self.compute_progressive_weave();
    }
    
    /// Verificar completude: S ⊒ W(Σ)?
    ///
    /// Canon: Especificação ⊒ - Completude estrutural
    pub fn is_complete(&self) -> bool {
        match &self.woven_state {
            WovenState::Complete { .. } => true,
            _ => false,
        }
    }
    
    /// Computar tecelagem progressiva.
    fn compute_progressive_weave(&self) -> WovenState {
        if self.received_edrs.is_empty() {
            return WovenState::Empty;
        }
        
        // Verificar se temos todos os chunks esperados
        let all_chunks_received = self.expected_chunks
            .iter()
            .all(|chunk| self.received_chunks.contains(chunk));
        
        if all_chunks_received {
            // DNA fecha estruturalmente
            WovenState::Complete {
                edr_count: self.received_edrs.len(),
                can_emit_dna: true,
            }
        } else {
            // Tecelagem incompleta
            WovenState::Partial {
                progress: self.compute_progress(),
            }
        }
    }
    
    /// Computar progresso da tecelagem.
    fn compute_progress(&self) -> f64 {
        if self.expected_chunks.is_empty() {
            return 0.0;
        }
        
        let received = self.received_chunks.len() as f64;
        let expected = self.expected_chunks.len() as f64;
        
        received / expected
    }
    
    /// Identificar chunks faltantes.
    pub fn missing_chunks(&self) -> Vec<ChunkId> {
        self.expected_chunks
            .difference(&self.received_chunks)
            .copied()
            .collect()
    }
}

// =============================================================================
// ESTADO TECIDO
// =============================================================================

/// Estado da tecelagem.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WovenState {
    /// Vazio (nenhum EDR)
    Empty,
    
    /// Parcial (em progresso)
    Partial {
        progress: f64,
    },
    
    /// Completo (DNA pode ser emitido)
    Complete {
        edr_count: usize,
        can_emit_dna: bool,
    },
}

impl WovenState {
    /// Verificar se DNA pode ser emitido.
    pub fn can_emit_dna(&self) -> bool {
        match self {
            WovenState::Complete { can_emit_dna, .. } => *can_emit_dna,
            _ => false,
        }
    }
}

// =============================================================================
// EDR ENCAPSULADO
// =============================================================================

/// EDR com contexto de chunk.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct EdWrapped {
    edr: Edr,
    chunk_id: ChunkId,
}

// =============================================================================
// TESTES
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_weaving_engine_creation() {
        let mut expected = HashSet::new();
        expected.insert(ChunkId::new(0));
        expected.insert(ChunkId::new(1));
        
        let engine = WeavingEngine::new(expected);
        
        assert!(!engine.is_complete());
        assert_eq!(engine.received_edrs.len(), 0);
    }
    
    #[test]
    fn test_empty_state() {
        let expected = HashSet::new();
        let engine = WeavingEngine::new(expected);
        
        match engine.woven_state {
            WovenState::Empty => {}, // ✅
            _ => panic!("Expected Empty state"),
        }
    }
    
    #[test]
    fn test_missing_chunks_identification() {
        let mut expected = HashSet::new();
        expected.insert(ChunkId::new(0));
        expected.insert(ChunkId::new(1));
        expected.insert(ChunkId::new(2));
        
        let engine = WeavingEngine::new(expected);
        
        let missing = engine.missing_chunks();
        assert_eq!(missing.len(), 3);
    }
    
    #[test]
    fn test_progress_computation() {
        let mut expected = HashSet::new();
        expected.insert(ChunkId::new(0));
        expected.insert(ChunkId::new(1));
        
        let engine = WeavingEngine::new(expected);
        
        // Progress com 0/2 chunks
        let progress = engine.compute_progress();
        assert_eq!(progress, 0.0);
    }
}
