use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::core::identifiers::{DnaId, ModelId};

/// Audit record for a CRISPR mutation operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MutationRecord {
    pub mutation_id: Uuid,
    pub original_dna_id: DnaId,
    pub resulting_dna_id: DnaId,
    pub operation_type: String, // e.g., "INSERT", "SWAP"
    pub timestamp: DateTime<Utc>,
    pub justification_model: ModelId,
}

/// Audit record for an evaluation decision (Scoring).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationRecord {
    pub decision_id: Uuid,
    pub subject_id: String, // Can be ActionId or DnaId
    pub model_used: ModelId,
    pub score_assigned: f64,
    pub verdict: String, // "PROMOTED", "REJECTED", "STAGNANT"
    pub timestamp: DateTime<Utc>,
}
