//! --------------------------
//! INFORMATION
//! --------------------------
//! Title: Perceptual States
//! Author: Carlos Eduardo Favini
//! Date: 2025-01-02
//! Version: 1.2.0
//! Description: Perceptual states of the sensory cortex.
//! Proto-Agency is a STATE, not a score.
//! States determine how perception continues, not what to decide.
//! Layer: Community
//! Dependencies: serde
//! Affected Components: sensory, replay
//!
//! --------------------------
//! ARCHITECTURAL NOTES (v1.1.0)
//! --------------------------
//! Proto-Agency (Level 2.5) is NOT:
//! - A classification
//! - A decision
//! - A semantic interpretation
//!
//! Proto-Agency IS:
//! - A perceptual state
//! - A transition that alters how perception continues
//! - A signal that "suspected intentionality" was detected
//!
//! The Community Edition STOPS at Proto-Agency.
//! Semantics (Level 3) requires Enterprise.
//!
//! --------------------------
//! CHANGE LOG
//! --------------------------
//! 2025-01-02 - Carlos Eduardo Favini - Initial creation (v1.1.0)
//! --------------------------

use serde::{Deserialize, Serialize};

/// Perceptual state of the sensory cortex.
/// This is NOT a score - it is a cognitive state that determines
/// how perception continues.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PerceptualState {
    /// Initial/final state: waiting for input
    Listening,

    /// Processing carrier signal (Level 0)
    /// Raw signal received, calculating basic statistics
    PerceivingCarrier,

    /// Pattern detected, processing (Level 1)
    /// Repetition/rhythm found in signal
    PerceivingPattern,

    /// Structure detected, processing (Level 2)
    /// Low entropy organization found
    PerceivingStructure,

    /// CRITICAL STATE: Suspected intentionality (Level 2.5)
    /// This state ALTERS how perception continues.
    /// It does NOT decide. It does NOT act.
    /// But it signals that the system detected something that
    /// "appears to have intention".
    ProtoAgencyDetected {
        /// What mathematically triggered this transition
        trigger: ProtoAgencyTrigger,
    },

    /// Emitting DNA with CP (output state)
    Emitting,
}

impl PerceptualState {
    /// Returns the abstraction level (0, 1, 2, 2.5)
    pub fn level(&self) -> f64 {
        match self {
            Self::Listening => 0.0,
            Self::PerceivingCarrier => 0.0,
            Self::PerceivingPattern => 1.0,
            Self::PerceivingStructure => 2.0,
            Self::ProtoAgencyDetected { .. } => 2.5,
            Self::Emitting => 2.5, // Maximum Community level
        }
    }

    /// Returns true if this is the Proto-Agency state
    pub fn is_proto_agency(&self) -> bool {
        matches!(self, Self::ProtoAgencyDetected { .. })
    }

    /// Returns the name of the state
    pub fn name(&self) -> &'static str {
        match self {
            Self::Listening => "Listening",
            Self::PerceivingCarrier => "PerceivingCarrier",
            Self::PerceivingPattern => "PerceivingPattern",
            Self::PerceivingStructure => "PerceivingStructure",
            Self::ProtoAgencyDetected { .. } => "ProtoAgencyDetected",
            Self::Emitting => "Emitting",
        }
    }
}

impl Default for PerceptualState {
    fn default() -> Self {
        Self::Listening
    }
}

/// What mathematically triggered Proto-Agency detection.
/// These are NOT interpretations - they are mathematical conditions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProtoAgencyTrigger {
    /// Predictability exceeds what random would produce
    /// (autocorrelation significantly above noise floor)
    pub predictability_exceeds_random: bool,

    /// Non-randomness confirmed by statistical test
    /// (runs test, p < significance level)
    pub non_randomness_confirmed: bool,

    /// Temporal coherence detected
    /// (local entropy significantly lower than global)
    pub temporal_coherence_detected: bool,
}

impl ProtoAgencyTrigger {
    /// Creates a new trigger with all conditions false
    pub fn new() -> Self {
        Self {
            predictability_exceeds_random: false,
            non_randomness_confirmed: false,
            temporal_coherence_detected: false,
        }
    }

    /// Returns true if ANY trigger condition is met
    pub fn any_triggered(&self) -> bool {
        self.predictability_exceeds_random
            || self.non_randomness_confirmed
            || self.temporal_coherence_detected
    }

    /// Returns true if ALL trigger conditions are met
    pub fn all_triggered(&self) -> bool {
        self.predictability_exceeds_random
            && self.non_randomness_confirmed
            && self.temporal_coherence_detected
    }

    /// Returns the count of triggered conditions
    pub fn trigger_count(&self) -> usize {
        let mut count = 0;
        if self.predictability_exceeds_random {
            count += 1;
        }
        if self.non_randomness_confirmed {
            count += 1;
        }
        if self.temporal_coherence_detected {
            count += 1;
        }
        count
    }
}

impl Default for ProtoAgencyTrigger {
    fn default() -> Self {
        Self::new()
    }
}

/// A transition between perceptual states.
/// Used for replay and auditing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateTransition {
    /// State before transition
    pub from: PerceptualState,

    /// State after transition
    pub to: PerceptualState,

    /// Timestamp in nanoseconds since start of perception
    pub timestamp_ns: u64,

    /// Sequence number of this transition
    pub sequence: u64,
}

impl StateTransition {
    /// Creates a new state transition
    pub fn new(from: PerceptualState, to: PerceptualState, timestamp_ns: u64, sequence: u64) -> Self {
        Self {
            from,
            to,
            timestamp_ns,
            sequence,
        }
    }
}

/// Tracks the complete state history of a perception cycle.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StateHistory {
    /// All transitions in order
    transitions: Vec<StateTransition>,

    /// Current state
    current: PerceptualState,

    /// Sequence counter
    sequence: u64,

    /// Start timestamp
    start_ns: u64,
}

impl StateHistory {
    /// Creates a new state history starting in Listening state
    pub fn new(start_ns: u64) -> Self {
        Self {
            transitions: Vec::new(),
            current: PerceptualState::Listening,
            sequence: 0,
            start_ns,
        }
    }

    /// Transitions to a new state
    pub fn transition_to(&mut self, new_state: PerceptualState, timestamp_ns: u64) {
        let transition = StateTransition::new(
            self.current.clone(),
            new_state.clone(),
            timestamp_ns - self.start_ns,
            self.sequence,
        );
        self.transitions.push(transition);
        self.current = new_state;
        self.sequence += 1;
    }

    /// Returns the current state
    pub fn current(&self) -> &PerceptualState {
        &self.current
    }

    /// Returns all transitions
    pub fn transitions(&self) -> &[StateTransition] {
        &self.transitions
    }

    /// Returns true if Proto-Agency was ever detected
    pub fn proto_agency_detected(&self) -> bool {
        self.transitions
            .iter()
            .any(|t| t.to.is_proto_agency())
    }

    /// Returns the maximum level reached
    pub fn max_level(&self) -> f64 {
        self.transitions
            .iter()
            .map(|t| t.to.level())
            .fold(0.0_f64, f64::max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_levels() {
        assert_eq!(PerceptualState::Listening.level(), 0.0);
        assert_eq!(PerceptualState::PerceivingCarrier.level(), 0.0);
        assert_eq!(PerceptualState::PerceivingPattern.level(), 1.0);
        assert_eq!(PerceptualState::PerceivingStructure.level(), 2.0);
        assert_eq!(
            PerceptualState::ProtoAgencyDetected {
                trigger: ProtoAgencyTrigger::new()
            }
            .level(),
            2.5
        );
    }

    #[test]
    fn test_trigger_count() {
        let mut trigger = ProtoAgencyTrigger::new();
        assert_eq!(trigger.trigger_count(), 0);
        assert!(!trigger.any_triggered());

        trigger.predictability_exceeds_random = true;
        assert_eq!(trigger.trigger_count(), 1);
        assert!(trigger.any_triggered());
        assert!(!trigger.all_triggered());

        trigger.non_randomness_confirmed = true;
        trigger.temporal_coherence_detected = true;
        assert_eq!(trigger.trigger_count(), 3);
        assert!(trigger.all_triggered());
    }

    #[test]
    fn test_state_history() {
        let mut history = StateHistory::new(0);
        assert_eq!(history.current().name(), "Listening");

        history.transition_to(PerceptualState::PerceivingCarrier, 1000);
        history.transition_to(PerceptualState::PerceivingPattern, 2000);
        history.transition_to(PerceptualState::PerceivingStructure, 3000);

        assert_eq!(history.transitions().len(), 3);
        assert_eq!(history.max_level(), 2.0);
        assert!(!history.proto_agency_detected());

        history.transition_to(
            PerceptualState::ProtoAgencyDetected {
                trigger: ProtoAgencyTrigger {
                    predictability_exceeds_random: true,
                    non_randomness_confirmed: true,
                    temporal_coherence_detected: false,
                },
            },
            4000,
        );

        assert!(history.proto_agency_detected());
        assert_eq!(history.max_level(), 2.5);
    }
}
