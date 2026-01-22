//! --------------------------
//! INFORMATION
//! --------------------------
//! Title: Traits Module (Enterprise Interfaces)
//! Author: Carlos Eduardo Favini
//! Date: 2025-01-02
//! Version: 1.2.0
//! Description: Trait definitions that the Enterprise layer must implement.
//!              These traits define the contract between Community (cognition)
//!              and Enterprise (execution). Community defines WHAT must be done;
//!              Enterprise defines HOW to do it.
//! Layer: Community
//! Dependencies: hierarchy, core_types, replay
//! Affected Components: Enterprise Edition
//!
//! --------------------------
//! CHANGE LOG
//! --------------------------
//! 2025-01-02 - Carlos Eduardo Favini - Initial creation
//! 2025-01-02 - Carlos Eduardo Favini - Added replay traits (v0.3.0)
//! --------------------------

use crate::core_types::DnaId;
use crate::hierarchy::{GoldenDna, FoucaultianTruth, PlatonicTruth};
use crate::math::craft::CraftPerformanceResult;
use crate::replay::{ReplaySession, ReplayContext};

/// Trait for DNA persistence.
///
/// The Community defines DNA structure; the Enterprise persists it.
/// This trait must be implemented by the Enterprise layer.
pub trait DnaPersistence {
    /// Error type for persistence operations.
    type Error;

    /// Saves a Golden DNA.
    fn save(&mut self, dna: &GoldenDna) -> Result<DnaId, Self::Error>;

    /// Loads a Golden DNA by ID.
    fn load(&self, id: &DnaId) -> Result<Option<GoldenDna>, Self::Error>;

    /// Checks if a DNA exists.
    fn exists(&self, id: &DnaId) -> Result<bool, Self::Error>;

    /// Deletes a DNA (if allowed by governance).
    fn delete(&mut self, id: &DnaId) -> Result<bool, Self::Error>;
}

/// Trait for truth registration.
///
/// The Community defines truth types; the Enterprise registers them
/// on immutable storage (blockchain, etc).
pub trait TruthRegistry {
    /// Error type for registry operations.
    type Error;

    /// Registers a Foucaultian truth (immutable).
    fn register_foucaultian(&mut self, truth: FoucaultianTruth) -> Result<String, Self::Error>;

    /// Registers a Platonic truth (evolvable).
    fn register_platonic(&mut self, truth: PlatonicTruth) -> Result<String, Self::Error>;

    /// Retrieves a Foucaultian truth by hash.
    fn get_foucaultian(&self, hash: &str) -> Result<Option<FoucaultianTruth>, Self::Error>;

    /// Retrieves the current Platonic truth for a domain.
    fn get_platonic(&self, domain: &str) -> Result<Option<PlatonicTruth>, Self::Error>;
}

/// Trait for evolution engine.
///
/// The Community provides evaluation; the Enterprise decides evolution.
pub trait EvolutionEngine {
    /// Error type for evolution operations.
    type Error;

    /// Evaluates a candidate against current state.
    /// Returns CP result without making decisions.
    fn evaluate(&self, candidate: &GoldenDna, current: Option<&GoldenDna>) -> CraftPerformanceResult;

    /// Decides whether to evolve (Enterprise responsibility).
    /// Community provides this trait definition; Enterprise implements the decision.
    fn decide_evolution(
        &mut self,
        candidate: &GoldenDna,
        evaluation: &CraftPerformanceResult,
    ) -> Result<EvolutionDecision, Self::Error>;
}

/// Evolution decision made by Enterprise.
#[derive(Debug, Clone)]
pub enum EvolutionDecision {
    /// Accept the candidate as new best.
    Evolve {
        /// ID of the new DNA.
        new_id: DnaId,
        /// Improvement factor.
        improvement: f64,
    },

    /// Reject the candidate.
    Reject {
        /// Reason for rejection.
        reason: String,
    },

    /// Defer decision (needs human input).
    Defer {
        /// Reason for deferral.
        reason: String,
    },
}

/// Trait for meristic proposal handling.
///
/// The Community generates proposals; the Enterprise decides incorporation.
pub trait MeristicHandler {
    /// Error type.
    type Error;

    /// Receives a meristic proposal.
    /// This is non-binding suggestion from the Community.
    fn receive_proposal(&mut self, proposal: MeristicProposal) -> Result<(), Self::Error>;

    /// Decides whether to incorporate a proposal.
    /// This is Enterprise responsibility - Community only proposes.
    fn decide_incorporation(&mut self, proposal_id: &str) -> Result<bool, Self::Error>;
}

/// A meristic proposal (non-binding suggestion).
#[derive(Debug, Clone)]
pub struct MeristicProposal {
    /// Unique proposal ID.
    pub id: String,

    /// The proposed DNA variation.
    pub proposed_dna: GoldenDna,

    /// Expected improvement in CP.
    pub expected_improvement: f64,

    /// Confidence in the proposal.
    pub confidence: f64,

    /// Description of what the proposal changes.
    pub description: String,
}

impl MeristicProposal {
    /// Creates a new meristic proposal.
    pub fn new(
        id: String,
        proposed_dna: GoldenDna,
        expected_improvement: f64,
        confidence: f64,
        description: String,
    ) -> Self {
        Self {
            id,
            proposed_dna,
            expected_improvement,
            confidence: confidence.clamp(0.0, 1.0),
            description,
        }
    }
}

/// Trait for governance enforcement.
///
/// The Community defines the VETO rule; the Enterprise enforces governance.
pub trait GovernanceEnforcer {
    /// Error type.
    type Error;

    /// Checks if a DNA passes governance rules.
    fn validate(&self, dna: &GoldenDna) -> Result<GovernanceResult, Self::Error>;

    /// Enforces a veto (Enterprise action).
    fn enforce_veto(&mut self, dna_id: &DnaId, reason: &str) -> Result<(), Self::Error>;
}

/// Result of governance validation.
#[derive(Debug, Clone)]
pub struct GovernanceResult {
    /// Whether the DNA passed validation.
    pub passed: bool,

    /// Violations found (if any).
    pub violations: Vec<String>,

    /// Recommendations.
    pub recommendations: Vec<String>,
}

impl GovernanceResult {
    /// Creates a passing result.
    pub fn pass() -> Self {
        Self {
            passed: true,
            violations: Vec::new(),
            recommendations: Vec::new(),
        }
    }

    /// Creates a failing result with violations.
    pub fn fail(violations: Vec<String>) -> Self {
        Self {
            passed: false,
            violations,
            recommendations: Vec::new(),
        }
    }
}

// =============================================================================
// REPLAY TRAITS (v0.3.0)
// =============================================================================

/// Trait for replay session storage.
///
/// The Community captures replay data; the Enterprise persists it.
pub trait ReplayStorage {
    /// Error type for storage operations.
    type Error;

    /// Saves a replay session.
    fn save_session(&mut self, session: &ReplaySession) -> Result<String, Self::Error>;

    /// Loads a replay session by ID.
    fn load_session(&self, session_id: &str) -> Result<Option<ReplaySession>, Self::Error>;

    /// Lists available session IDs.
    fn list_sessions(&self) -> Result<Vec<String>, Self::Error>;

    /// Deletes a session.
    fn delete_session(&mut self, session_id: &str) -> Result<bool, Self::Error>;
}

/// Trait for replay-aware execution.
///
/// Components that support replay implement this trait.
pub trait ReplayAware {
    /// Executes with replay context, capturing events.
    fn execute_with_replay(&self, context: &mut ReplayContext);
}

/// Trait for replay verification callback.
///
/// Enterprise can implement this to receive verification results.
pub trait ReplayVerificationCallback {
    /// Called when verification completes.
    fn on_verification_complete(&mut self, result: ReplayVerificationResult);
}

/// Result of replay verification.
#[derive(Debug, Clone)]
pub struct ReplayVerificationResult {
    /// Whether the replay was successful (bit-exact).
    pub success: bool,

    /// Number of events verified.
    pub events_verified: u64,

    /// Number of divergences found.
    pub divergences: u64,

    /// Session ID that was verified.
    pub session_id: String,

    /// Error message if verification failed.
    pub error: Option<String>,
}
