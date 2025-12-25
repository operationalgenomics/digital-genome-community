use uuid::Uuid;
use serde::{Serialize, Deserialize};

/// Unique identifier for a specific Action event (Raw Evidence).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ActionId(pub Uuid);

/// Unique identifier for an Evaluation Model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ModelId(pub Uuid);

/// Cryptographic signature representing a specific Context/Intent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ContextSignature(pub u64);

/// Unique identifier for a Golden DNA Sequence (Platonic Form).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DnaId(pub Uuid);
