//! --------------------------
//! INFORMATION
//! --------------------------
//! Title: Truth Types (Foucaultian and Platonic)
//! Author: Carlos Eduardo Favini
//! Date: 2025-01-02
//! Version: 1.2.0
//! Description: Two types of truth in the Digital Genome system.
//!              Foucaultian Truth: What WAS - immutable historical record.
//!              Platonic Truth: What SHOULD BE - evolving ideal synthesis.
//! Layer: Community
//! Dependencies: hierarchy/action, hierarchy/dna
//! Affected Components: archive, selection
//!
//! --------------------------
//! CHANGE LOG
//! --------------------------
//! 2025-01-02 - Carlos Eduardo Favini - Initial creation
//! --------------------------

use serde::{Deserialize, Serialize};

use super::action::ObservedAction;
use super::dna::GoldenDna;

/// Foucaultian Truth: What WAS.
///
/// Represents immutable historical facts. Once registered, a
/// Foucaultian truth cannot be changed, even if later proven
/// suboptimal. It is the archaeological record of what occurred.
///
/// # Immutability
/// This truth is registered on blockchain or equivalent immutable
/// store. It serves as evidence for auditing and dispute resolution.
///
/// # Epistemology
/// Named after Michel Foucault's concept of historical truth as
/// constructed through power/knowledge relations, but here used
/// to denote the pure factual record without interpretation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoucaultianTruth {
    /// The raw observed fact.
    pub raw_fact: ObservedAction,

    /// Cryptographic proof of registration.
    pub registration_hash: String,

    /// Timestamp of registration (nanoseconds).
    pub registered_at_ns: i64,

    /// The registering entity.
    pub registrar: String,
}

impl FoucaultianTruth {
    /// Creates a new Foucaultian truth from an observed action.
    pub fn new(
        raw_fact: ObservedAction,
        registration_hash: String,
        registered_at_ns: i64,
        registrar: String,
    ) -> Self {
        Self {
            raw_fact,
            registration_hash,
            registered_at_ns,
            registrar,
        }
    }

    /// Verifies the integrity of this truth.
    ///
    /// Performs complete chain verification:
    /// 1. Verifies raw_fact internal integrity (content_hash)
    /// 2. Verifies registration_hash matches raw_fact.content_hash
    ///
    /// # Returns
    /// `true` if both checks pass, `false` otherwise.
    pub fn verify(&self) -> bool {
        // Step 1: Verify raw fact internal integrity
        if self.raw_fact.verify_integrity().is_err() {
            return false;
        }

        // Step 2: Verify registration hash matches content hash
        // This ensures the registration is for this specific content
        self.registration_hash == self.raw_fact.content_hash
    }

    /// Verifies only the raw fact integrity without checking registration.
    pub fn verify_content_only(&self) -> bool {
        self.raw_fact.verify_integrity().is_ok()
    }
}

/// Platonic Truth: What SHOULD BE.
///
/// Represents the evolving ideal - the best known synthesis of
/// operational knowledge. Unlike Foucaultian truth, Platonic truth
/// can be superseded by better approximations.
///
/// # Evolution
/// Platonic truths are not permanent. They represent the current
/// best understanding, which improves over time through the
/// evolutionary process.
///
/// # Epistemology
/// Named after Plato's concept of ideal forms - perfect patterns
/// that physical reality approximates. Here, Golden DNA represents
/// the ideal operational pattern that actions should approximate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatonicTruth {
    /// The synthesized ideal (Golden DNA).
    pub ideal: GoldenDna,

    /// Version number (increases with each evolution).
    pub version: u64,

    /// Timestamp of synthesis (nanoseconds).
    pub synthesized_at_ns: i64,

    /// Evidence supporting this synthesis (Foucaultian truths).
    pub supporting_evidence: Vec<String>, // Hashes of Foucaultian truths
}

impl PlatonicTruth {
    /// Creates a new Platonic truth.
    pub fn new(
        ideal: GoldenDna,
        version: u64,
        synthesized_at_ns: i64,
        supporting_evidence: Vec<String>,
    ) -> Self {
        Self {
            ideal,
            version,
            synthesized_at_ns,
            supporting_evidence,
        }
    }

    /// Creates the first version of a Platonic truth.
    pub fn initial(ideal: GoldenDna, synthesized_at_ns: i64) -> Self {
        Self {
            ideal,
            version: 1,
            synthesized_at_ns,
            supporting_evidence: Vec::new(),
        }
    }

    /// Returns the Craft Performance of the ideal.
    pub fn craft_performance(&self) -> f64 {
        self.ideal.craft_performance
    }

    /// Checks if this truth has been superseded (CP = 0).
    pub fn is_superseded(&self) -> bool {
        self.ideal.is_vetoed()
    }
}

/// The relationship between truths.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TruthRelation {
    /// The Foucaultian truth supports the Platonic ideal.
    Supports,

    /// The Foucaultian truth contradicts the Platonic ideal.
    Contradicts,

    /// The Foucaultian truth is orthogonal (unrelated).
    Orthogonal,

    /// The relationship is unknown.
    Unknown,
}

/// Result of comparing truths.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TruthComparison {
    /// The Foucaultian truth being compared.
    pub foucaultian_hash: String,

    /// The Platonic truth version being compared against.
    pub platonic_version: u64,

    /// The determined relationship.
    pub relation: TruthRelation,

    /// Confidence in the determination [0.0, 1.0].
    pub confidence: f64,
}

impl TruthComparison {
    /// Creates a new truth comparison.
    pub fn new(
        foucaultian_hash: String,
        platonic_version: u64,
        relation: TruthRelation,
        confidence: f64,
    ) -> Self {
        Self {
            foucaultian_hash,
            platonic_version,
            relation,
            confidence: confidence.clamp(0.0, 1.0),
        }
    }
}
