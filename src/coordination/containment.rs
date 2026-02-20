//! Canon v5.1 | GDC v0.9.0-final
//!
//! Containment - W(Σ) e ⊒
//!
//! Canon References:
//! - Especificação W(Σ) (lines 5026-5053)
//! - Especificação ⊒ (lines 5056-5082)
//! - LEI-COORD-03 (lines 5085-5112)
//!
//! ## Core Principles
//! W(Σ): Trabalho Estrutural = semantic chunks for distribution
//! ⊒: Containment by completeness = S ⊒ W(Σ) when weaving is complete

use serde::{Deserialize, Serialize};
use std::collections::HashSet;

// =============================================================================
// W(Σ) - TRABALHO ESTRUTURAL
// =============================================================================

/// Semantic chunk - autocontained unit of work.
///
/// Canon: §2 - Cada chunk é autocontido, não requer comunicação
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SemanticChunk {
    /// Chunk identifier
    pub id: String,
    
    /// Payload (autocontained)
    pub payload: Vec<u8>,
    
    /// Semantic context (everything Worker needs)
    pub context: ChunkContext,
}

/// Context carried with chunk.
///
/// Canon: §2 - Carrega tudo que Worker necessita para processar
/// em isolamento absoluto (LEI-RSN-01, LEI-RSN-03)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ChunkContext {
    /// Schema hint
    pub schema: String,
    
    /// Semantic boundaries
    pub boundaries: String,
    
    /// Processing instructions
    pub instructions: String,
}

impl SemanticChunk {
    /// Create new semantic chunk.
    pub fn new(id: String, payload: Vec<u8>, context: ChunkContext) -> Self {
        Self { id, payload, context }
    }
    
    /// Check if chunk is self-contained.
    ///
    /// Canon: §2 - Processável em isolamento absoluto
    pub fn is_self_contained(&self) -> bool {
        !self.payload.is_empty() && !self.context.schema.is_empty()
    }
}

/// W(Σ) - Structural Work derived from stimulus.
///
/// Canon: §1 - Conjunto de chunks semânticos para distribuição
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StructuralWork {
    /// Original stimulus reference
    pub stimulus_id: String,
    
    /// Semantic chunks
    pub chunks: Vec<SemanticChunk>,
    
    /// Total chunk count (may vary)
    pub total_count: usize,
}

impl StructuralWork {
    /// Create W(Σ) from stimulus.
    ///
    /// Canon: §1 - W(Σ) não é RawInput bruto, é trabalho estruturado
    pub fn from_stimulus(stimulus_id: String, raw_input: &[u8]) -> Self {
        // Simplified fragmentation for v0.9.0-beta
        // Full implementation would use semantic analysis
        let chunk_size = 1024.max(raw_input.len() / 4);
        
        let mut chunks = Vec::new();
        let mut offset = 0;
        let mut chunk_id = 0;
        
        while offset < raw_input.len() {
            let end = (offset + chunk_size).min(raw_input.len());
            let payload = raw_input[offset..end].to_vec();
            
            let context = ChunkContext {
                schema: "v1".to_string(),
                boundaries: format!("{}-{}", offset, end),
                instructions: "process".to_string(),
            };
            
            chunks.push(SemanticChunk::new(
                format!("{}_{}", stimulus_id, chunk_id),
                payload,
                context,
            ));
            
            offset = end;
            chunk_id += 1;
        }
        
        let total_count = chunks.len();
        
        Self {
            stimulus_id,
            chunks,
            total_count,
        }
    }
    
    /// Check all chunks are self-contained.
    ///
    /// Canon: §2 - ∀ wᵢ ∈ W(Σ): wᵢ é processável em isolamento
    pub fn all_self_contained(&self) -> bool {
        self.chunks.iter().all(|c| c.is_self_contained())
    }
}

// =============================================================================
// ⊒ - CONTENÇÃO POR COMPLETUDE
// =============================================================================

/// Response Set - EDR returns received by Queen.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponseSet {
    /// Chunk IDs that have responses
    pub responded_chunks: HashSet<String>,
    
    /// Total responses received
    pub response_count: usize,
}

impl ResponseSet {
    /// Create new response set.
    pub fn new() -> Self {
        Self {
            responded_chunks: HashSet::new(),
            response_count: 0,
        }
    }
    
    /// Add response for a chunk.
    pub fn add_response(&mut self, chunk_id: String) {
        if self.responded_chunks.insert(chunk_id) {
            self.response_count += 1;
        }
    }
    
    /// Check if chunk has response.
    pub fn has_response(&self, chunk_id: &str) -> bool {
        self.responded_chunks.contains(chunk_id)
    }
}

impl Default for ResponseSet {
    fn default() -> Self {
        Self::new()
    }
}

/// Containment checker - S ⊒ W(Σ).
///
/// Canon: §1 - ⊒ é verificação de completude da tecelagem
pub struct ContainmentChecker;

impl ContainmentChecker {
    /// Check S ⊒ W(Σ) - containment by completeness.
    ///
    /// Canon: §5 - ⊒ é atingido quando estrutura tecida permite emissão
    ///
    /// Implementation: Simplified for v0.9.0-beta
    /// Full implementation would check structural completeness,
    /// not enumerate chunks (LEI-COORD-01: campo não enumera)
    pub fn check_containment(
        responses: &ResponseSet,
        work: &StructuralWork,
    ) -> bool {
        // Simplified: check minimum coverage
        // Full version: structural completeness verification
        
        let coverage_ratio = responses.response_count as f64 / work.total_count as f64;
        
        // Require at least 80% coverage (simplified)
        // Full implementation: DNA can close structurally
        coverage_ratio >= 0.8
    }
    
    /// Check if weaving is structurally complete.
    ///
    /// Canon: §1 - Rainha sabe que falta trabalho porque DNA não fecha
    pub fn is_structurally_complete(_responses: &ResponseSet) -> bool {
        // Placeholder for structural verification
        // Full implementation: check if DNA closes
        true
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_structural_work_creation() {
        let stimulus_id = "stim_001".to_string();
        let raw = vec![0u8; 5000];
        
        let work = StructuralWork::from_stimulus(stimulus_id, &raw);
        
        // Should fragment into chunks
        assert!(work.total_count > 0);
        assert!(work.all_self_contained());
    }
    
    #[test]
    fn test_semantic_chunk_self_contained() {
        let context = ChunkContext {
            schema: "v1".to_string(),
            boundaries: "0-100".to_string(),
            instructions: "process".to_string(),
        };
        
        let chunk = SemanticChunk::new(
            "chunk_1".to_string(),
            vec![1, 2, 3],
            context,
        );
        
        // Canon: §2 - Chunk é autocontido
        assert!(chunk.is_self_contained());
    }
    
    #[test]
    fn test_response_set() {
        let mut responses = ResponseSet::new();
        
        responses.add_response("chunk_1".to_string());
        responses.add_response("chunk_2".to_string());
        
        assert_eq!(responses.response_count, 2);
        assert!(responses.has_response("chunk_1"));
        assert!(!responses.has_response("chunk_3"));
    }
    
    #[test]
    fn test_containment_complete() {
        let work = StructuralWork::from_stimulus(
            "test".to_string(),
            &vec![0u8; 1000],
        );
        
        let mut responses = ResponseSet::new();
        
        // Add responses for all chunks
        for chunk in &work.chunks {
            responses.add_response(chunk.id.clone());
        }
        
        // Canon: §5 - ⊒ atingido quando tecelagem completa
        assert!(ContainmentChecker::check_containment(&responses, &work));
    }
    
    #[test]
    fn test_containment_incomplete() {
        let work = StructuralWork::from_stimulus(
            "test".to_string(),
            &vec![0u8; 1000],
        );
        
        let responses = ResponseSet::new(); // No responses
        
        // Should not be contained
        assert!(!ContainmentChecker::check_containment(&responses, &work));
    }
}
