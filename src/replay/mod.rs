//! --------------------------
//! INFORMATION
//! --------------------------
//! Title: Replay Harness Module
//! Author: Carlos Eduardo Favini
//! Date: 2025-01-02
//! Version: 1.2.0
//! Description: Deterministic replay infrastructure for bit-exact reproduction
//!              of cognitive evaluations. Captures events, anomalies, and
//!              metadata for scientific validation and debugging.
//! Layer: Community
//! Dependencies: core_types, hierarchy, motors, math
//! Affected Components: All cognitive components
//!
//! --------------------------
//! REPLAY ARCHITECTURE (v0.3.0)
//! --------------------------
//! The replay system consists of:
//!
//! 1. ReplayContext: Deterministic execution context
//!    - Provides deterministic ID generation
//!    - Tracks sequence numbers for ordering
//!    - Captures execution metadata
//!
//! 2. ReplayEvent: Captured evaluation event
//!    - Input snapshot
//!    - Output snapshot
//!    - Anomalies (clamping, warnings)
//!    - Timing information
//!
//! 3. ReplaySession: Complete replay session
//!    - Ordered sequence of events
//!    - Session-level metadata
//!    - Verification methods
//!
//! 4. ReplayVerifier: Comparison tool
//!    - Compares two sessions for equality
//!    - Reports divergences with detail
//!
//! --------------------------
//! CONSTITUTIONAL COMPLIANCE
//! --------------------------
//! The replay system is OBSERVATIONAL:
//! - It captures what happened, not what should happen
//! - It does not modify execution
//! - It does not make decisions
//! - It provides data for external verification
//!
//! --------------------------
//! CHANGE LOG
//! --------------------------
//! 2025-01-02 - Carlos Eduardo Favini - Initial creation (v0.3.0)
//! --------------------------

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::core_types::{ActionId, DnaId};

/// Anomaly types that can be captured during execution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Anomaly {
    /// A value was clamped to valid range.
    ValueClamped {
        /// Name of the clamped field.
        field: String,
        /// Original value before clamping.
        original: f64,
        /// Final value after clamping.
        clamped: f64,
    },

    /// An input was adjusted (e.g., novelty_weight).
    InputAdjusted {
        /// Name of the adjusted field.
        field: String,
        /// Original value.
        original: f64,
        /// Adjusted value.
        adjusted: f64,
    },

    /// A validation warning occurred.
    ValidationWarning {
        /// Warning message.
        message: String,
    },

    /// A numeric edge case was encountered.
    NumericEdgeCase {
        /// Description of the edge case.
        description: String,
    },

    /// Overflow protection was triggered.
    OverflowProtection {
        /// Where overflow was prevented.
        location: String,
    },
}

/// Motor type identifier for replay events.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MotorType {
    /// Praxeological Motor (M_P).
    Praxis,
    /// Nash Motor (M_N).
    Nash,
    /// Chaos Motor (M_C).
    Chaos,
    /// Meristic Motor (M_M).
    Meristic,
    /// Craft Performance calculation.
    CraftPerformance,
}

impl std::fmt::Display for MotorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Praxis => write!(f, "Praxis"),
            Self::Nash => write!(f, "Nash"),
            Self::Chaos => write!(f, "Chaos"),
            Self::Meristic => write!(f, "Meristic"),
            Self::CraftPerformance => write!(f, "CraftPerformance"),
        }
    }
}

/// A single evaluation event captured during replay.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayEvent {
    /// Sequence number within the session.
    pub sequence: u64,

    /// Timestamp (nanoseconds since UNIX epoch).
    pub timestamp_ns: u64,

    /// Motor that produced this event.
    pub motor: MotorType,

    /// Hash of the input (for verification without storing full input).
    pub input_hash: String,

    /// Hash of the output.
    pub output_hash: String,

    /// The computed score.
    pub score: f64,

    /// Anomalies detected during this evaluation.
    pub anomalies: Vec<Anomaly>,

    /// Whether the output was marked as valid.
    pub valid: bool,

    /// Additional metadata.
    pub metadata: BTreeMap<String, String>,
}

impl ReplayEvent {
    /// Creates a new replay event.
    pub fn new(
        sequence: u64,
        motor: MotorType,
        input_hash: String,
        output_hash: String,
        score: f64,
        valid: bool,
    ) -> Self {
        let timestamp_ns = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0);

        Self {
            sequence,
            timestamp_ns,
            motor,
            input_hash,
            output_hash,
            score,
            anomalies: Vec::new(),
            valid,
            metadata: BTreeMap::new(),
        }
    }

    /// Adds an anomaly to this event.
    pub fn add_anomaly(&mut self, anomaly: Anomaly) {
        self.anomalies.push(anomaly);
    }

    /// Adds metadata to this event.
    pub fn add_metadata(&mut self, key: &str, value: &str) {
        self.metadata.insert(key.to_string(), value.to_string());
    }

    /// Returns true if any anomalies were recorded.
    pub fn has_anomalies(&self) -> bool {
        !self.anomalies.is_empty()
    }

    /// Returns the count of anomalies.
    pub fn anomaly_count(&self) -> usize {
        self.anomalies.len()
    }
}

/// Deterministic execution context for replay.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayContext {
    /// Session identifier (deterministic if seeded).
    pub session_id: String,

    /// Base seed for deterministic ID generation.
    pub seed: Vec<u8>,

    /// Current sequence number.
    sequence_counter: u64,

    /// Whether this context is in replay mode (verifying) vs record mode.
    pub replay_mode: bool,

    /// Events captured during this session.
    events: Vec<ReplayEvent>,

    /// Session-level anomaly summary.
    anomaly_counts: BTreeMap<String, u64>,
}

impl ReplayContext {
    /// Creates a new replay context with a random session ID.
    pub fn new() -> Self {
        Self {
            session_id: format!("session-{}", uuid::Uuid::new_v4()),
            seed: Vec::new(),
            sequence_counter: 0,
            replay_mode: false,
            events: Vec::new(),
            anomaly_counts: BTreeMap::new(),
        }
    }

    /// Creates a deterministic replay context from a seed.
    ///
    /// This enables bit-exact replay when the same seed is used.
    pub fn from_seed(seed: &[u8]) -> Self {
        use sha2::{Sha256, Digest};
        
        let mut hasher = Sha256::new();
        hasher.update(b"replay-session-");
        hasher.update(seed);
        let hash = hasher.finalize();
        let session_id = hex::encode(&hash[..16]);

        Self {
            session_id: format!("session-{}", session_id),
            seed: seed.to_vec(),
            sequence_counter: 0,
            replay_mode: false,
            events: Vec::new(),
            anomaly_counts: BTreeMap::new(),
        }
    }

    /// Enters replay/verification mode with a recorded session.
    pub fn enter_replay_mode(recorded: ReplaySession) -> Self {
        Self {
            session_id: recorded.session_id.clone(),
            seed: recorded.seed.clone(),
            sequence_counter: 0,
            replay_mode: true,
            events: recorded.events,
            anomaly_counts: BTreeMap::new(),
        }
    }

    /// Gets the next sequence number.
    pub fn next_sequence(&mut self) -> u64 {
        let seq = self.sequence_counter;
        self.sequence_counter += 1;
        seq
    }

    /// Gets the current sequence number without incrementing.
    pub fn current_sequence(&self) -> u64 {
        self.sequence_counter
    }

    /// Generates a deterministic ActionId for this context.
    pub fn deterministic_action_id(&mut self) -> ActionId {
        let seq = self.next_sequence();
        let mut seed = self.seed.clone();
        seed.extend_from_slice(&seq.to_le_bytes());
        seed.extend_from_slice(b"-action");
        ActionId::new_deterministic(&seed)
    }

    /// Generates a deterministic DnaId for this context.
    pub fn deterministic_dna_id(&mut self) -> DnaId {
        let seq = self.next_sequence();
        let mut seed = self.seed.clone();
        seed.extend_from_slice(&seq.to_le_bytes());
        seed.extend_from_slice(b"-dna");
        DnaId::new_deterministic(&seed)
    }

    /// Records an event in the session.
    pub fn record_event(&mut self, event: ReplayEvent) {
        // Update anomaly counts
        for anomaly in &event.anomalies {
            let key = match anomaly {
                Anomaly::ValueClamped { .. } => "ValueClamped",
                Anomaly::InputAdjusted { .. } => "InputAdjusted",
                Anomaly::ValidationWarning { .. } => "ValidationWarning",
                Anomaly::NumericEdgeCase { .. } => "NumericEdgeCase",
                Anomaly::OverflowProtection { .. } => "OverflowProtection",
            };
            *self.anomaly_counts.entry(key.to_string()).or_insert(0) += 1;
        }

        self.events.push(event);
    }

    /// Gets the expected event at the current sequence (in replay mode).
    pub fn get_expected_event(&self) -> Option<&ReplayEvent> {
        if self.replay_mode {
            self.events.get(self.sequence_counter as usize)
        } else {
            None
        }
    }

    /// Exports the current session for later replay.
    pub fn export_session(&self) -> ReplaySession {
        ReplaySession {
            session_id: self.session_id.clone(),
            seed: self.seed.clone(),
            events: self.events.clone(),
            anomaly_summary: self.anomaly_counts.clone(),
            total_events: self.events.len() as u64,
            total_anomalies: self.anomaly_counts.values().sum(),
        }
    }

    /// Returns the total number of recorded events.
    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    /// Returns the anomaly summary.
    pub fn anomaly_summary(&self) -> &BTreeMap<String, u64> {
        &self.anomaly_counts
    }
}

impl Default for ReplayContext {
    fn default() -> Self {
        Self::new()
    }
}

/// A complete replay session that can be stored and replayed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplaySession {
    /// Session identifier.
    pub session_id: String,

    /// Seed used for deterministic generation.
    pub seed: Vec<u8>,

    /// All events in order.
    pub events: Vec<ReplayEvent>,

    /// Summary of anomalies by type.
    pub anomaly_summary: BTreeMap<String, u64>,

    /// Total event count.
    pub total_events: u64,

    /// Total anomaly count.
    pub total_anomalies: u64,
}

impl ReplaySession {
    /// Creates an empty session.
    pub fn empty() -> Self {
        Self {
            session_id: String::new(),
            seed: Vec::new(),
            events: Vec::new(),
            anomaly_summary: BTreeMap::new(),
            total_events: 0,
            total_anomalies: 0,
        }
    }

    /// Serializes the session to JSON.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Deserializes a session from JSON.
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Gets an event by sequence number.
    pub fn get_event(&self, sequence: u64) -> Option<&ReplayEvent> {
        self.events.get(sequence as usize)
    }

    /// Returns true if the session has any anomalies.
    pub fn has_anomalies(&self) -> bool {
        self.total_anomalies > 0
    }
}

/// Result of comparing two replay sessions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayComparison {
    /// Whether the sessions are identical.
    pub identical: bool,

    /// Number of events compared.
    pub events_compared: u64,

    /// Number of divergences found.
    pub divergence_count: u64,

    /// Detailed divergences (up to a limit).
    pub divergences: Vec<Divergence>,
}

/// A specific divergence between two replay sessions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Divergence {
    /// Sequence number where divergence occurred.
    pub sequence: u64,

    /// Type of divergence.
    pub divergence_type: DivergenceType,

    /// Description of the divergence.
    pub description: String,
}

/// Types of divergence between replay sessions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DivergenceType {
    /// Different motor types at same sequence.
    MotorMismatch,
    /// Different input hashes.
    InputMismatch,
    /// Different output hashes.
    OutputMismatch,
    /// Different scores.
    ScoreMismatch,
    /// Different validity flags.
    ValidityMismatch,
    /// Different anomaly counts.
    AnomalyMismatch,
    /// Missing event in one session.
    MissingEvent,
}

/// Verifier for comparing replay sessions.
pub struct ReplayVerifier;

impl ReplayVerifier {
    /// Compares two replay sessions and returns the comparison result.
    ///
    /// # Arguments
    /// * `expected` - The reference session
    /// * `actual` - The session to verify
    /// * `max_divergences` - Maximum divergences to report in detail
    pub fn compare(
        expected: &ReplaySession,
        actual: &ReplaySession,
        max_divergences: usize,
    ) -> ReplayComparison {
        let mut divergences = Vec::new();
        let mut divergence_count = 0u64;

        let max_events = expected.events.len().max(actual.events.len());

        for seq in 0..max_events {
            let exp_event = expected.events.get(seq);
            let act_event = actual.events.get(seq);

            match (exp_event, act_event) {
                (Some(exp), Some(act)) => {
                    // Compare motor type
                    if exp.motor != act.motor {
                        divergence_count += 1;
                        if divergences.len() < max_divergences {
                            divergences.push(Divergence {
                                sequence: seq as u64,
                                divergence_type: DivergenceType::MotorMismatch,
                                description: format!(
                                    "Expected motor {:?}, got {:?}",
                                    exp.motor, act.motor
                                ),
                            });
                        }
                    }

                    // Compare input hash
                    if exp.input_hash != act.input_hash {
                        divergence_count += 1;
                        if divergences.len() < max_divergences {
                            divergences.push(Divergence {
                                sequence: seq as u64,
                                divergence_type: DivergenceType::InputMismatch,
                                description: "Input hashes differ".to_string(),
                            });
                        }
                    }

                    // Compare output hash
                    if exp.output_hash != act.output_hash {
                        divergence_count += 1;
                        if divergences.len() < max_divergences {
                            divergences.push(Divergence {
                                sequence: seq as u64,
                                divergence_type: DivergenceType::OutputMismatch,
                                description: "Output hashes differ".to_string(),
                            });
                        }
                    }

                    // Compare scores with tolerance
                    if (exp.score - act.score).abs() > 1e-15 {
                        divergence_count += 1;
                        if divergences.len() < max_divergences {
                            divergences.push(Divergence {
                                sequence: seq as u64,
                                divergence_type: DivergenceType::ScoreMismatch,
                                description: format!(
                                    "Score difference: {} vs {}",
                                    exp.score, act.score
                                ),
                            });
                        }
                    }

                    // Compare validity
                    if exp.valid != act.valid {
                        divergence_count += 1;
                        if divergences.len() < max_divergences {
                            divergences.push(Divergence {
                                sequence: seq as u64,
                                divergence_type: DivergenceType::ValidityMismatch,
                                description: format!(
                                    "Validity mismatch: {} vs {}",
                                    exp.valid, act.valid
                                ),
                            });
                        }
                    }

                    // Compare anomaly counts
                    if exp.anomalies.len() != act.anomalies.len() {
                        divergence_count += 1;
                        if divergences.len() < max_divergences {
                            divergences.push(Divergence {
                                sequence: seq as u64,
                                divergence_type: DivergenceType::AnomalyMismatch,
                                description: format!(
                                    "Anomaly count: {} vs {}",
                                    exp.anomalies.len(),
                                    act.anomalies.len()
                                ),
                            });
                        }
                    }
                }
                (Some(_), None) => {
                    divergence_count += 1;
                    if divergences.len() < max_divergences {
                        divergences.push(Divergence {
                            sequence: seq as u64,
                            divergence_type: DivergenceType::MissingEvent,
                            description: "Event missing in actual session".to_string(),
                        });
                    }
                }
                (None, Some(_)) => {
                    divergence_count += 1;
                    if divergences.len() < max_divergences {
                        divergences.push(Divergence {
                            sequence: seq as u64,
                            divergence_type: DivergenceType::MissingEvent,
                            description: "Extra event in actual session".to_string(),
                        });
                    }
                }
                (None, None) => break,
            }
        }

        ReplayComparison {
            identical: divergence_count == 0,
            events_compared: max_events as u64,
            divergence_count,
            divergences,
        }
    }
}

/// Helper trait for creating replay events from motor outputs.
pub trait Replayable {
    /// Creates a replay event from this output.
    fn to_replay_event(&self, sequence: u64, motor: MotorType, input_hash: String) -> ReplayEvent;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deterministic_context() {
        let ctx1 = ReplayContext::from_seed(b"test-seed-123");
        let ctx2 = ReplayContext::from_seed(b"test-seed-123");

        assert_eq!(ctx1.session_id, ctx2.session_id);
    }

    #[test]
    fn test_deterministic_id_generation() {
        let mut ctx1 = ReplayContext::from_seed(b"test-seed");
        let mut ctx2 = ReplayContext::from_seed(b"test-seed");

        let id1a = ctx1.deterministic_action_id();
        let id1b = ctx1.deterministic_action_id();
        let id2a = ctx2.deterministic_action_id();
        let id2b = ctx2.deterministic_action_id();

        // Same seed, same sequence → same ID
        assert_eq!(id1a, id2a);
        assert_eq!(id1b, id2b);

        // Different sequence → different ID
        assert_ne!(id1a, id1b);
    }

    #[test]
    fn test_event_recording() {
        let mut ctx = ReplayContext::new();

        let mut event = ReplayEvent::new(
            ctx.next_sequence(),
            MotorType::Praxis,
            "input-hash-1".to_string(),
            "output-hash-1".to_string(),
            0.85,
            true,
        );
        event.add_anomaly(Anomaly::ValueClamped {
            field: "score".to_string(),
            original: 1.1,
            clamped: 1.0,
        });

        ctx.record_event(event);

        assert_eq!(ctx.event_count(), 1);
        assert_eq!(*ctx.anomaly_summary().get("ValueClamped").unwrap(), 1);
    }

    #[test]
    fn test_session_export_import() {
        let mut ctx = ReplayContext::from_seed(b"export-test");

        let event = ReplayEvent::new(
            ctx.next_sequence(),
            MotorType::Nash,
            "hash1".to_string(),
            "hash2".to_string(),
            0.75,
            true,
        );
        ctx.record_event(event);

        let session = ctx.export_session();
        let json = session.to_json().unwrap();
        let restored = ReplaySession::from_json(&json).unwrap();

        assert_eq!(session.session_id, restored.session_id);
        assert_eq!(session.events.len(), restored.events.len());
    }

    #[test]
    fn test_session_comparison_identical() {
        let mut ctx = ReplayContext::from_seed(b"compare-test");

        let event = ReplayEvent::new(
            ctx.next_sequence(),
            MotorType::Chaos,
            "hash1".to_string(),
            "hash2".to_string(),
            0.5,
            true,
        );
        ctx.record_event(event);

        let session = ctx.export_session();
        let comparison = ReplayVerifier::compare(&session, &session, 10);

        assert!(comparison.identical);
        assert_eq!(comparison.divergence_count, 0);
    }

    #[test]
    fn test_session_comparison_divergent() {
        let event1 = ReplayEvent::new(
            0,
            MotorType::Praxis,
            "hash1".to_string(),
            "hash2".to_string(),
            0.5,
            true,
        );

        let event2 = ReplayEvent::new(
            0,
            MotorType::Praxis,
            "hash1".to_string(),
            "hash2".to_string(),
            0.6, // Different score
            true,
        );

        let session1 = ReplaySession {
            session_id: "s1".to_string(),
            seed: vec![],
            events: vec![event1],
            anomaly_summary: BTreeMap::new(),
            total_events: 1,
            total_anomalies: 0,
        };

        let session2 = ReplaySession {
            session_id: "s2".to_string(),
            seed: vec![],
            events: vec![event2],
            anomaly_summary: BTreeMap::new(),
            total_events: 1,
            total_anomalies: 0,
        };

        let comparison = ReplayVerifier::compare(&session1, &session2, 10);

        assert!(!comparison.identical);
        assert!(comparison.divergence_count > 0);
    }

    #[test]
    fn test_anomaly_types() {
        let anomalies = vec![
            Anomaly::ValueClamped {
                field: "test".to_string(),
                original: 1.5,
                clamped: 1.0,
            },
            Anomaly::InputAdjusted {
                field: "weight".to_string(),
                original: -0.5,
                adjusted: 0.0,
            },
            Anomaly::ValidationWarning {
                message: "Test warning".to_string(),
            },
        ];

        assert_eq!(anomalies.len(), 3);
    }
}
