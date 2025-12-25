use uuid::Uuid;
use crate::core::identifiers::{ActionId, DnaId, TruthId};
use crate::core::types::{ObservedAction, GoldenDna};
// Note: We use fully qualified paths to avoid circular deps or ambiguity

/// Interface for reading historical data without coupling to storage implementation.
pub trait MemoryReader {
    /// Retrieves a raw action by its ID.
    fn get_action(&self, id: &ActionId) -> Option<ObservedAction>;
    
    /// Retrieves a Golden DNA sequence by its ID.
    fn get_dna(&self, id: &DnaId) -> Option<GoldenDna>;
    
    /// Checks if a Truth exists for a given Action.
    fn exists_truth(&self, id: &TruthId) -> bool;
}
