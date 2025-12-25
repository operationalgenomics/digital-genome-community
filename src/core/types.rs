use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use super::identifiers::{ActionId, DnaId, ContextSignature};

// --- LEVEL 0: RAW EVIDENCE SHAPE ---

/// Observed Action.
/// Represents the shape of raw evidence captured from reality.
/// It is a data container, not a historical record (which belongs in Memory).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservedAction {
    pub id: ActionId,
    pub source_id: String,
    pub timestamp: DateTime<Utc>,
    pub context_vector: HashMap<String, f64>,
    pub payload: serde_json::Value,
    /// Cryptographic hash of the content (Payload + Context).
    pub content_hash: String,
}

impl ObservedAction {
    pub fn new(source_id: String, context: HashMap<String, f64>, payload: serde_json::Value) -> Self {
        // Note: Real hash calculation should be injected via a service, 
        // strictly assigning a placeholder here for the primitive type.
        let id = ActionId(Uuid::new_v4());
        Self {
            id,
            source_id,
            timestamp: Utc::now(),
            context_vector: context,
            payload,
            content_hash: String::new(), 
        }
    }
}

// --- LEVEL 1: PLATONIC FORM (IDEAL) ---

/// Golden DNA.
/// 
/// Represents the "Ideal Form" or recipe.
/// It does NOT contain the raw actions, but the sequence of ActionIds 
/// that constitute the best known method.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoldenDna {
    pub id: DnaId,
    pub version: u32,
    /// The optimized sequence of pointers (not the heavy history).
    pub sequence: Vec<ActionId>,
    pub score: f64,
    pub target_context: ContextSignature,
}
