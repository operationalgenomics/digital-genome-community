//! --------------------------
//! INFORMATION
//! --------------------------
//! Title: Core Identifiers
//! Author: Carlos Eduardo Favini
//! Date: 2025-01-02
//! Version: 1.2.0
//! Description: Unique identifier types for Digital Genome entities.
//!              All identifiers are immutable and cryptographically traceable.
//!              Supports both random (v4) and deterministic (v5) UUID generation.
//! Layer: Community
//! Dependencies: uuid, serde, sha2
//! Affected Components: hierarchy, archive, topology, replay
//!
//! --------------------------
//! DETERMINISTIC ID SPECIFICATION (v0.3.0)
//! --------------------------
//! For replay and reproducibility, all ID types support deterministic generation:
//!
//! - `new()` - Random UUID v4 (default, non-deterministic)
//! - `new_deterministic(seed)` - UUID v5 with Digital Genome namespace
//!
//! The deterministic method uses:
//! - Namespace: SHA-1 hash of "digital-genome.community.{TypeName}"
//! - Name: User-provided seed bytes
//! - Algorithm: UUID v5 (RFC 4122)
//!
//! This enables bit-exact replay when the same seeds are provided.
//!
//! --------------------------
//! CHANGE LOG
//! --------------------------
//! 2025-01-02 - Carlos Eduardo Favini - Initial creation
//! 2025-01-02 - Carlos Eduardo Favini - Added deterministic UUID generation (v0.3.0)
//! --------------------------

use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use uuid::Uuid;

/// Digital Genome namespace for deterministic UUID generation.
/// Generated from SHA-1 of "digital-genome.community" (first 16 bytes).
/// Reserved for future use in cross-system UUID coordination.
#[allow(dead_code)]
const _DG_NAMESPACE: Uuid = Uuid::from_bytes([
    0x8b, 0x5a, 0x9c, 0x3d, 0x7e, 0x2f, 0x4a, 0x1b,
    0x9c, 0x8d, 0x6e, 0x5f, 0x4a, 0x3b, 0x2c, 0x1d,
]);

/// Creates a type-specific namespace UUID.
fn type_namespace(type_name: &str) -> Uuid {
    let mut hasher = Sha256::new();
    hasher.update(b"digital-genome.community.");
    hasher.update(type_name.as_bytes());
    let result = hasher.finalize();
    
    // Use first 16 bytes as UUID
    let mut bytes = [0u8; 16];
    bytes.copy_from_slice(&result[..16]);
    
    // Set version 5 (SHA-1) and variant bits
    bytes[6] = (bytes[6] & 0x0f) | 0x50; // Version 5
    bytes[8] = (bytes[8] & 0x3f) | 0x80; // Variant RFC 4122
    
    Uuid::from_bytes(bytes)
}

/// Unique identifier for an observed action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ActionId(pub Uuid);

impl ActionId {
    /// Creates a new random ActionId (non-deterministic).
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Creates a deterministic ActionId from a seed.
    /// Same seed always produces the same ID.
    ///
    /// # Example
    /// ```
    /// use digital_genome_community::ActionId;
    /// let id1 = ActionId::new_deterministic(b"action-001");
    /// let id2 = ActionId::new_deterministic(b"action-001");
    /// assert_eq!(id1, id2);
    /// ```
    pub fn new_deterministic(seed: &[u8]) -> Self {
        let namespace = type_namespace("ActionId");
        Self(Uuid::new_v5(&namespace, seed))
    }

    /// Creates an ActionId from an existing UUID.
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    /// Returns the inner UUID.
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for ActionId {
    fn default() -> Self {
        Self::new()
    }
}

/// Unique identifier for a DNA strand.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DnaId(pub Uuid);

impl DnaId {
    /// Creates a new random DnaId (non-deterministic).
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Creates a deterministic DnaId from a seed.
    pub fn new_deterministic(seed: &[u8]) -> Self {
        let namespace = type_namespace("DnaId");
        Self(Uuid::new_v5(&namespace, seed))
    }

    /// Creates a DnaId from an existing UUID.
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    /// Returns the inner UUID.
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for DnaId {
    fn default() -> Self {
        Self::new()
    }
}

/// Unique identifier for a synapse.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SynapseId(pub Uuid);

impl SynapseId {
    /// Creates a new random SynapseId (non-deterministic).
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Creates a deterministic SynapseId from a seed.
    pub fn new_deterministic(seed: &[u8]) -> Self {
        let namespace = type_namespace("SynapseId");
        Self(Uuid::new_v5(&namespace, seed))
    }

    /// Creates a SynapseId from an existing UUID.
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    /// Returns the inner UUID.
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for SynapseId {
    fn default() -> Self {
        Self::new()
    }
}

/// Unique identifier for a neuron.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NeuronId(pub Uuid);

impl NeuronId {
    /// Creates a new random NeuronId (non-deterministic).
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Creates a deterministic NeuronId from a seed.
    pub fn new_deterministic(seed: &[u8]) -> Self {
        let namespace = type_namespace("NeuronId");
        Self(Uuid::new_v5(&namespace, seed))
    }

    /// Creates a NeuronId from an existing UUID.
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    /// Returns the inner UUID.
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for NeuronId {
    fn default() -> Self {
        Self::new()
    }
}

/// Unique identifier for a brain instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BrainId(pub Uuid);

impl BrainId {
    /// Creates a new random BrainId (non-deterministic).
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Creates a deterministic BrainId from a seed.
    pub fn new_deterministic(seed: &[u8]) -> Self {
        let namespace = type_namespace("BrainId");
        Self(Uuid::new_v5(&namespace, seed))
    }

    /// Creates a BrainId from an existing UUID.
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    /// Returns the inner UUID.
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for BrainId {
    fn default() -> Self {
        Self::new()
    }
}

/// Contextual signature for traceability.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ContextSignature {
    /// The source that generated this context.
    pub source: String,
    /// Cryptographic hash of the context.
    pub hash: String,
}

impl ContextSignature {
    /// Creates a new context signature.
    pub fn new(source: String, hash: String) -> Self {
        Self { source, hash }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deterministic_action_id() {
        let id1 = ActionId::new_deterministic(b"test-seed-123");
        let id2 = ActionId::new_deterministic(b"test-seed-123");
        let id3 = ActionId::new_deterministic(b"different-seed");

        assert_eq!(id1, id2, "Same seed should produce same ID");
        assert_ne!(id1, id3, "Different seeds should produce different IDs");
    }

    #[test]
    fn test_deterministic_dna_id() {
        let id1 = DnaId::new_deterministic(b"dna-seed");
        let id2 = DnaId::new_deterministic(b"dna-seed");

        assert_eq!(id1, id2);
    }

    #[test]
    fn test_different_types_different_namespaces() {
        let seed = b"same-seed";
        
        let action_id = ActionId::new_deterministic(seed);
        let dna_id = DnaId::new_deterministic(seed);
        let synapse_id = SynapseId::new_deterministic(seed);

        // Same seed but different types should produce different UUIDs
        assert_ne!(action_id.0, dna_id.0);
        assert_ne!(action_id.0, synapse_id.0);
        assert_ne!(dna_id.0, synapse_id.0);
    }

    #[test]
    fn test_random_ids_are_different() {
        let id1 = ActionId::new();
        let id2 = ActionId::new();

        assert_ne!(id1, id2, "Random IDs should be different");
    }

    #[test]
    fn test_as_uuid() {
        let id = ActionId::new_deterministic(b"test");
        let uuid = id.as_uuid();
        
        assert_eq!(*uuid, id.0);
    }
}

