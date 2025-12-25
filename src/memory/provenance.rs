use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::core::types::ObservedAction;
use crate::core::invariants::Immutable;

/// Unique identifier for a crystallized Truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TruthId(pub Uuid);

/// Provenance Metadata.
/// 
/// Represents the institutional context of an observation:
/// Who validated it? When? Under what authority?
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Provenance {
    pub captured_at: DateTime<Utc>,
    pub node_identity: String,
    /// Cryptographic signature of the validator node.
    pub validation_signature: Option<String>,
}

/// Foucaultian Truth.
/// 
/// Represents a specific historical fact. Unlike ObservedAction (which is just data),
/// a FoucaultianTruth is an institutional record that "X happened at T".
/// It is immutable and serves as the atomic unit of the Archive.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoucaultianTruth {
    pub id: TruthId,
    /// The raw evidence being memorialized.
    pub raw_fact: ObservedAction,
    /// The metadata guaranteeing origin.
    pub provenance: Provenance,
    /// The hash sealing this record.
    pub integrity_hash: String,
}

impl FoucaultianTruth {
    pub fn new(raw_fact: ObservedAction, node_identity: String) -> Self {
        // In a real scenario, we would sign this data structure.
        let provenance = Provenance {
            captured_at: Utc::now(),
            node_identity,
            validation_signature: None, // Placeholder for crypto signature
        };
        
        // Placeholder hash - strictly for structure compliance
        let integrity_hash = format!("hash_{}", raw_fact.id.0);

        Self {
            id: TruthId(Uuid::new_v4()),
            raw_fact,
            provenance,
            integrity_hash,
        }
    }
}

impl Immutable for FoucaultianTruth {
    fn integrity_hash(&self) -> &str {
        &self.integrity_hash
    }
}
