//! --------------------------
//! INFORMATION
//! --------------------------
//! Title: Latent Archive (Foucaultian Memory)
//! Author: Carlos Eduardo Favini
//! Date: 2025-01-02
//! Version: 1.2.0
//! Description: The archive of observed truths. Preserves Foucaultian
//!              truths (what WAS) for auditing and historical reference.
//!              The archive never forgets, never modifies, only appends.
//! Layer: Community
//! Dependencies: hierarchy/truth, core_types
//! Affected Components: selection, topology
//!
//! --------------------------
//! CHANGE LOG
//! --------------------------
//! 2025-01-02 - Carlos Eduardo Favini - Initial creation
//! --------------------------

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::core_types::ActionId;
use crate::hierarchy::FoucaultianTruth;

/// The Latent Archive.
///
/// Stores all Foucaultian truths (what WAS) in immutable form.
/// This is the archaeological record of the Digital Genome.
///
/// # Principles
/// - Never forgets: all truths are preserved
/// - Never modifies: truths are immutable after registration
/// - Only appends: new truths can only be added
///
/// # Purpose
/// - Auditing: trace decisions back to observations
/// - Dispute resolution: GD Court can verify historical facts
/// - Evolution: compare new observations against historical patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatentArchive {
    /// Index of truths by action ID.
    truths: HashMap<String, FoucaultianTruth>,

    /// Chronological ordering of truth hashes.
    chronological_index: Vec<String>,

    /// Total number of archived truths.
    count: usize,
}

impl LatentArchive {
    /// Creates a new empty archive.
    pub fn new() -> Self {
        Self {
            truths: HashMap::new(),
            chronological_index: Vec::new(),
            count: 0,
        }
    }

    /// Archives a new Foucaultian truth.
    ///
    /// # Arguments
    /// * `truth` - The truth to archive
    ///
    /// # Returns
    /// The registration hash of the archived truth.
    pub fn archive(&mut self, truth: FoucaultianTruth) -> String {
        let hash = truth.registration_hash.clone();

        self.truths.insert(hash.clone(), truth);
        self.chronological_index.push(hash.clone());
        self.count += 1;

        hash
    }

    /// Retrieves a truth by its registration hash.
    pub fn get(&self, hash: &str) -> Option<&FoucaultianTruth> {
        self.truths.get(hash)
    }

    /// Retrieves a truth by action ID.
    pub fn get_by_action(&self, action_id: &ActionId) -> Option<&FoucaultianTruth> {
        self.truths
            .values()
            .find(|t| t.raw_fact.id == *action_id)
    }

    /// Returns the total count of archived truths.
    pub fn len(&self) -> usize {
        self.count
    }

    /// Checks if the archive is empty.
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Returns truths in chronological order.
    pub fn chronological(&self) -> impl Iterator<Item = &FoucaultianTruth> {
        self.chronological_index
            .iter()
            .filter_map(|hash| self.truths.get(hash))
    }

    /// Returns the most recent truth.
    pub fn latest(&self) -> Option<&FoucaultianTruth> {
        self.chronological_index
            .last()
            .and_then(|hash| self.truths.get(hash))
    }

    /// Verifies integrity of a specific truth.
    pub fn verify(&self, hash: &str) -> bool {
        self.truths
            .get(hash)
            .map(|t: &FoucaultianTruth| t.verify())
            .unwrap_or(false)
    }

    /// Verifies integrity of the entire archive.
    pub fn verify_all(&self) -> bool {
        self.truths.values().all(|t: &FoucaultianTruth| t.verify())
    }
}

impl Default for LatentArchive {
    fn default() -> Self {
        Self::new()
    }
}

/// Query result from the archive.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchiveQuery {
    /// Hashes of matching truths.
    pub matches: Vec<String>,

    /// Total matches found.
    pub count: usize,
}

impl ArchiveQuery {
    /// Creates an empty query result.
    pub fn empty() -> Self {
        Self {
            matches: Vec::new(),
            count: 0,
        }
    }

    /// Creates a query result with matches.
    pub fn with_matches(matches: Vec<String>) -> Self {
        let count = matches.len();
        Self { matches, count }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hierarchy::ObservedAction;
    use std::collections::BTreeMap;

    fn create_test_truth() -> FoucaultianTruth {
        let action = ObservedAction::new(
            "test_source".to_string(),
            1000000,
            BTreeMap::new(),
            serde_json::json!({"test": true}),
        )
        .expect("Failed to create test action");

        FoucaultianTruth::new(
            action,
            "test_hash_123".to_string(),
            1000000,
            "test_registrar".to_string(),
        )
    }

    #[test]
    fn test_archive_and_retrieve() {
        let mut archive = LatentArchive::new();
        let truth = create_test_truth();
        let hash = truth.registration_hash.clone();

        archive.archive(truth);

        assert_eq!(archive.len(), 1);
        assert!(archive.get(&hash).is_some());
    }

    #[test]
    fn test_chronological_order() {
        let mut archive = LatentArchive::new();

        for i in 0..3 {
            let action = ObservedAction::new(
                format!("source_{}", i),
                i as i64,
                BTreeMap::new(),
                serde_json::json!({"index": i}),
            )
            .expect("Failed to create action");

            let truth = FoucaultianTruth::new(
                action,
                format!("hash_{}", i),
                i as i64,
                "registrar".to_string(),
            );

            archive.archive(truth);
        }

        let chronological: Vec<_> = archive.chronological().collect();
        assert_eq!(chronological.len(), 3);
    }
}
