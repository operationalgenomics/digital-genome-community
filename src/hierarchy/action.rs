//! --------------------------
//! INFORMATION
//! --------------------------
//! Title: Observed Action (Level 0)
//! Author: Carlos Eduardo Favini
//! Date: 2025-01-02
//! Version: 1.2.0
//! Description: The fundamental unit of observation in the Digital Genome.
//!              Represents a raw, immutable event captured from reality.
//!              According to Axiom Zero: "A falta de é ação" - even inaction
//!              is an action. This is the Foucaultian truth - what WAS.
//! Layer: Community
//! Dependencies: core_types, serde, sha2, hex
//! Affected Components: dna, synapse, archive
//!
//! --------------------------
//! CHANGE LOG
//! --------------------------
//! 2025-01-02 - Carlos Eduardo Favini - Initial creation
//! 2025-01-02 - Carlos Eduardo Favini - SHA-256 + deterministic serialization
//! --------------------------

use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use std::collections::BTreeMap;

use crate::core_types::ActionId;

/// Level 0: Observed Action.
///
/// The most fundamental unit in the Digital Genome hierarchy.
/// Represents an immutable observation of reality.
///
/// # Axiom Zero
/// "A falta de é ação" - The lack of being is action.
/// Every state, including non-action, is recorded as an action.
///
/// # Immutability
/// Once created, an ObservedAction cannot be modified.
/// Its content_hash provides cryptographic proof of integrity (SHA-256).
///
/// # Deterministic Serialization
/// Uses BTreeMap (ordered) instead of HashMap to ensure deterministic
/// serialization for hash computation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservedAction {
    /// Unique identifier for this observation.
    pub id: ActionId,

    /// The source that captured this action (sensor, system, human).
    pub source_id: String,

    /// Timestamp of observation (nanoseconds since epoch).
    /// Using i64 for deterministic serialization.
    pub timestamp_ns: i64,

    /// Environmental context as key-value pairs.
    /// Uses BTreeMap for deterministic ordering during serialization.
    pub context_vector: BTreeMap<String, f64>,

    /// The raw payload of the action.
    /// Structure depends on the domain but is always JSON-serializable.
    pub payload: serde_json::Value,

    /// SHA-256 hash of the immutable components.
    /// This is the Foucaultian signature - cryptographic proof of what WAS.
    pub content_hash: String,
}

/// Error type for action creation and validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionError {
    /// Serialization of context failed.
    ContextSerializationFailed(String),
    /// Serialization of payload failed.
    PayloadSerializationFailed(String),
    /// Hash verification failed.
    IntegrityViolation {
        expected: String,
        computed: String,
    },
}

impl std::fmt::Display for ActionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActionError::ContextSerializationFailed(msg) => {
                write!(f, "Context serialization failed: {}", msg)
            }
            ActionError::PayloadSerializationFailed(msg) => {
                write!(f, "Payload serialization failed: {}", msg)
            }
            ActionError::IntegrityViolation { expected, computed } => {
                write!(
                    f,
                    "Integrity violation: expected {}, computed {}",
                    expected, computed
                )
            }
        }
    }
}

impl std::error::Error for ActionError {}

impl ObservedAction {
    /// Creates a new observed action.
    ///
    /// # Arguments
    /// * `source_id` - Identifier of the observing entity
    /// * `timestamp_ns` - Observation timestamp in nanoseconds
    /// * `context` - Environmental context at observation time (BTreeMap for determinism)
    /// * `payload` - The raw action data
    ///
    /// # Returns
    /// Result containing a new ObservedAction with computed SHA-256 hash,
    /// or an ActionError if serialization fails.
    pub fn new(
        source_id: String,
        timestamp_ns: i64,
        context: BTreeMap<String, f64>,
        payload: serde_json::Value,
    ) -> Result<Self, ActionError> {
        let id = ActionId::new();

        let content_hash = Self::compute_sha256(&source_id, timestamp_ns, &context, &payload)?;

        Ok(Self {
            id,
            source_id,
            timestamp_ns,
            context_vector: context,
            payload,
            content_hash,
        })
    }

    /// Computes the SHA-256 hash of the action's immutable components.
    ///
    /// This hash serves as the Foucaultian signature - cryptographic
    /// proof of what was observed at a specific moment.
    ///
    /// # Deterministic Serialization
    /// - BTreeMap ensures key ordering
    /// - Canonical JSON serialization (no whitespace variance)
    /// - Fixed field order in concatenation
    fn compute_sha256(
        source_id: &str,
        timestamp_ns: i64,
        context: &BTreeMap<String, f64>,
        payload: &serde_json::Value,
    ) -> Result<String, ActionError> {
        // Serialize context deterministically (BTreeMap is ordered)
        let context_json = serde_json::to_string(context)
            .map_err(|e| ActionError::ContextSerializationFailed(e.to_string()))?;

        // Serialize payload canonically
        let payload_json = serde_json::to_string(payload)
            .map_err(|e| ActionError::PayloadSerializationFailed(e.to_string()))?;

        // Concatenate in fixed order with delimiters
        let data = format!(
            "source:{}|timestamp:{}|context:{}|payload:{}",
            source_id, timestamp_ns, context_json, payload_json
        );

        // Compute SHA-256
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        let result = hasher.finalize();

        Ok(hex::encode(result))
    }

    /// Verifies the integrity of this action.
    ///
    /// # Returns
    /// `Ok(())` if the content hash matches the computed hash,
    /// `Err(ActionError::IntegrityViolation)` otherwise.
    pub fn verify_integrity(&self) -> Result<(), ActionError> {
        let computed = Self::compute_sha256(
            &self.source_id,
            self.timestamp_ns,
            &self.context_vector,
            &self.payload,
        )?;

        if self.content_hash == computed {
            Ok(())
        } else {
            Err(ActionError::IntegrityViolation {
                expected: self.content_hash.clone(),
                computed,
            })
        }
    }

    /// Returns true if integrity check passes.
    pub fn is_valid(&self) -> bool {
        self.verify_integrity().is_ok()
    }
}

/// A sequence of observed actions forming a temporal chain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionSequence {
    /// The ordered sequence of actions.
    pub actions: Vec<ObservedAction>,
}

impl ActionSequence {
    /// Creates an empty action sequence.
    pub fn new() -> Self {
        Self { actions: Vec::new() }
    }

    /// Creates a sequence from existing actions.
    pub fn from_actions(actions: Vec<ObservedAction>) -> Self {
        Self { actions }
    }

    /// Adds an action to the sequence.
    pub fn push(&mut self, action: ObservedAction) {
        self.actions.push(action);
    }

    /// Returns the number of actions in the sequence.
    pub fn len(&self) -> usize {
        self.actions.len()
    }

    /// Returns true if the sequence is empty.
    pub fn is_empty(&self) -> bool {
        self.actions.is_empty()
    }

    /// Verifies integrity of all actions in the sequence.
    pub fn verify_all(&self) -> Result<(), ActionError> {
        for action in &self.actions {
            action.verify_integrity()?;
        }
        Ok(())
    }
}

impl Default for ActionSequence {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256_determinism() {
        let mut context = BTreeMap::new();
        context.insert("key1".to_string(), 1.0);
        context.insert("key2".to_string(), 2.0);

        let action1 = ObservedAction::new(
            "source".to_string(),
            1000000,
            context.clone(),
            serde_json::json!({"test": true}),
        )
        .unwrap();

        let action2 = ObservedAction::new(
            "source".to_string(),
            1000000,
            context,
            serde_json::json!({"test": true}),
        )
        .unwrap();

        // Same inputs must produce same hash (excluding UUID)
        assert_eq!(action1.content_hash, action2.content_hash);
    }

    #[test]
    fn test_integrity_verification() {
        let context = BTreeMap::new();
        let action = ObservedAction::new(
            "test_source".to_string(),
            1000000,
            context,
            serde_json::json!({"test": true}),
        )
        .unwrap();

        assert!(action.verify_integrity().is_ok());
        assert!(action.is_valid());
    }

    #[test]
    fn test_hash_is_sha256_length() {
        let context = BTreeMap::new();
        let action = ObservedAction::new(
            "source".to_string(),
            0,
            context,
            serde_json::json!(null),
        )
        .unwrap();

        // SHA-256 produces 64 hex characters
        assert_eq!(action.content_hash.len(), 64);
    }

    #[test]
    fn test_different_inputs_different_hashes() {
        let context = BTreeMap::new();

        let action1 = ObservedAction::new(
            "source_a".to_string(),
            1000000,
            context.clone(),
            serde_json::json!({"value": 1}),
        )
        .unwrap();

        let action2 = ObservedAction::new(
            "source_b".to_string(),
            1000000,
            context,
            serde_json::json!({"value": 1}),
        )
        .unwrap();

        assert_ne!(action1.content_hash, action2.content_hash);
    }

    #[test]
    fn test_serialization_invariance() {
        // This test verifies that serialization → deserialization → re-hash
        // produces the same hash. This is the canonical invariant for auditability.
        let mut context = BTreeMap::new();
        context.insert("alpha".to_string(), 1.5);
        context.insert("beta".to_string(), 2.5);
        context.insert("gamma".to_string(), 3.5);

        let original = ObservedAction::new(
            "invariance_test".to_string(),
            1234567890,
            context,
            serde_json::json!({"nested": {"key": "value"}, "array": [1, 2, 3]}),
        )
        .unwrap();

        // Serialize to JSON
        let serialized = serde_json::to_string(&original).unwrap();

        // Deserialize back
        let restored: ObservedAction = serde_json::from_str(&serialized).unwrap();

        // Hashes must be identical
        assert_eq!(original.content_hash, restored.content_hash);

        // Integrity verification must pass on restored
        assert!(restored.verify_integrity().is_ok());
    }

    #[test]
    fn test_btreemap_ordering_determinism() {
        // BTreeMap ordering is the foundation of deterministic serialization.
        // This test verifies that insertion order does not affect the hash.
        
        // Insert in order: a, b, c
        let mut context1 = BTreeMap::new();
        context1.insert("a".to_string(), 1.0);
        context1.insert("b".to_string(), 2.0);
        context1.insert("c".to_string(), 3.0);

        // Insert in reverse order: c, b, a
        let mut context2 = BTreeMap::new();
        context2.insert("c".to_string(), 3.0);
        context2.insert("b".to_string(), 2.0);
        context2.insert("a".to_string(), 1.0);

        let action1 = ObservedAction::new(
            "source".to_string(),
            1000000,
            context1,
            serde_json::json!({"test": true}),
        )
        .unwrap();

        let action2 = ObservedAction::new(
            "source".to_string(),
            1000000,
            context2,
            serde_json::json!({"test": true}),
        )
        .unwrap();

        // Hashes must be identical regardless of insertion order
        assert_eq!(action1.content_hash, action2.content_hash);
    }
}
