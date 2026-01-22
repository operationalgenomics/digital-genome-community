//! --------------------------
//! INFORMATION
//! --------------------------
//! Title: Community Output
//! Author: Carlos Eduardo Favini
//! Date: 2025-01-02
//! Version: 1.2.0
//! Description: Complete output structure of the Community Edition.
//! Contains CP, motor scores, perceptual state, and signals.
//! NO interpretation. NO classification. NO semantic inference.
//! Layer: Community
//! Dependencies: sensory, math, motors
//! Affected Components: External consumers
//!
//! --------------------------
//! OUTPUT CONTRACT
//! --------------------------
//! The Community Edition output contains:
//! 1. Craft Performance (CP) - the multiplicative score
//! 2. Motor scores - individual contributions
//! 3. Perceptual state - where in the cycle we ended
//! 4. State transitions - history for replay
//! 5. Sensory signals - raw mathematical measures
//! 6. DNA hash - identifier for this perception
//!
//! The output does NOT contain:
//! - Classifications ("this is X")
//! - Confidence scores for classifications
//! - Explanatory narratives
//! - Semantic interpretations
//! - Ontological inferences
//!
//! --------------------------
//! CHANGE LOG
//! --------------------------
//! 2025-01-02 - Carlos Eduardo Favini - Initial creation (v1.1.0)
//! --------------------------

use serde::{Deserialize, Serialize};

use crate::math::craft::{CpResult, VetoCause};
use crate::sensory::signals::SensorySignals;
use crate::sensory::state::{PerceptualState, StateTransition};

/// Complete output of the Community Edition.
/// This is what gets emitted after processing raw input.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityOutput {
    // ═══════════════════════════════════════════════════════════════════════
    // CRAFT PERFORMANCE
    // ═══════════════════════════════════════════════════════════════════════
    
    /// Craft Performance value (0.0 = VETO, 0.0-1.0 = valid)
    pub craft_performance: f64,

    /// Whether veto was triggered
    pub vetoed: bool,

    /// Source of veto (if any)
    pub veto_source: Option<VetoCause>,

    /// Whether any clamping occurred during calculation
    pub calculation_clamped: bool,

    // ═══════════════════════════════════════════════════════════════════════
    // MOTOR SCORES (raw numbers, no interpretation)
    // ═══════════════════════════════════════════════════════════════════════
    
    /// M_P: Praxeological Motor score
    pub motor_praxis: f64,

    /// M_N: Nash Motor score
    pub motor_nash: f64,

    /// M_C: Chaos Motor score
    pub motor_chaos: f64,

    /// M_M: Meristic Motor score
    pub motor_meristic: f64,

    // ═══════════════════════════════════════════════════════════════════════
    // PERCEPTUAL STATE (state, not score)
    // ═══════════════════════════════════════════════════════════════════════
    
    /// Final perceptual state
    pub perceptual_state: PerceptualState,

    /// Whether Proto-Agency was detected
    pub proto_agency_detected: bool,

    /// Maximum abstraction level reached (0, 1, 2, or 2.5)
    pub max_level_reached: f64,

    /// State transitions (for replay)
    pub state_transitions: Vec<StateTransition>,

    // ═══════════════════════════════════════════════════════════════════════
    // SENSORY SIGNALS (pure mathematics)
    // ═══════════════════════════════════════════════════════════════════════
    
    /// All sensory signals computed
    pub sensory_signals: SensorySignals,

    // ═══════════════════════════════════════════════════════════════════════
    // METADATA
    // ═══════════════════════════════════════════════════════════════════════
    
    /// Hash of the DNA emitted
    pub dna_hash: String,

    /// Processing timestamp (nanoseconds since epoch)
    pub processed_at: u64,

    /// Input size in bytes
    pub input_size: usize,
}

impl CommunityOutput {
    /// Creates a new CommunityOutput from components
    pub fn new(
        cp_result: &CpResult,
        motor_praxis: f64,
        motor_nash: f64,
        motor_chaos: f64,
        motor_meristic: f64,
        perceptual_state: PerceptualState,
        state_transitions: Vec<StateTransition>,
        sensory_signals: SensorySignals,
        dna_hash: String,
        processed_at: u64,
        input_size: usize,
    ) -> Self {
        let (craft_performance, vetoed, veto_source, calculation_clamped) = match cp_result {
            CpResult::Valid {
                value, was_clamped, ..
            } => (*value, false, None, *was_clamped),
            CpResult::Vetoed { cause, .. } => (0.0, true, Some(cause.clone()), false),
            CpResult::Invalid { .. } => (0.0, true, None, false),
        };

        let proto_agency_detected = perceptual_state.is_proto_agency()
            || state_transitions.iter().any(|t| t.to.is_proto_agency());

        let max_level_reached = state_transitions
            .iter()
            .map(|t| t.to.level())
            .fold(0.0_f64, f64::max);

        Self {
            craft_performance,
            vetoed,
            veto_source,
            calculation_clamped,
            motor_praxis,
            motor_nash,
            motor_chaos,
            motor_meristic,
            perceptual_state,
            proto_agency_detected,
            max_level_reached,
            state_transitions,
            sensory_signals,
            dna_hash,
            processed_at,
            input_size,
        }
    }

    /// Serializes to JSON
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Serializes to compact JSON (single line)
    pub fn to_json_compact(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// Deserializes from JSON
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

/// Builder for CommunityOutput
pub struct CommunityOutputBuilder {
    motor_praxis: f64,
    motor_nash: f64,
    motor_chaos: f64,
    motor_meristic: f64,
    perceptual_state: PerceptualState,
    state_transitions: Vec<StateTransition>,
    sensory_signals: SensorySignals,
    dna_hash: String,
    processed_at: u64,
    input_size: usize,
}

impl CommunityOutputBuilder {
    /// Creates a new builder
    pub fn new() -> Self {
        Self {
            motor_praxis: 0.0,
            motor_nash: 0.0,
            motor_chaos: 0.0,
            motor_meristic: 0.0,
            perceptual_state: PerceptualState::Listening,
            state_transitions: Vec::new(),
            sensory_signals: SensorySignals::empty(),
            dna_hash: String::new(),
            processed_at: 0,
            input_size: 0,
        }
    }

    /// Sets motor scores
    pub fn motors(mut self, praxis: f64, nash: f64, chaos: f64, meristic: f64) -> Self {
        self.motor_praxis = praxis;
        self.motor_nash = nash;
        self.motor_chaos = chaos;
        self.motor_meristic = meristic;
        self
    }

    /// Sets perceptual state
    pub fn state(mut self, state: PerceptualState) -> Self {
        self.perceptual_state = state;
        self
    }

    /// Sets state transitions
    pub fn transitions(mut self, transitions: Vec<StateTransition>) -> Self {
        self.state_transitions = transitions;
        self
    }

    /// Sets sensory signals
    pub fn signals(mut self, signals: SensorySignals) -> Self {
        self.sensory_signals = signals;
        self
    }

    /// Sets DNA hash
    pub fn dna_hash(mut self, hash: String) -> Self {
        self.dna_hash = hash;
        self
    }

    /// Sets timestamp
    pub fn timestamp(mut self, ts: u64) -> Self {
        self.processed_at = ts;
        self
    }

    /// Sets input size
    pub fn input_size(mut self, size: usize) -> Self {
        self.input_size = size;
        self
    }

    /// Builds the output with a given CP result
    pub fn build(self, cp_result: &CpResult) -> CommunityOutput {
        CommunityOutput::new(
            cp_result,
            self.motor_praxis,
            self.motor_nash,
            self.motor_chaos,
            self.motor_meristic,
            self.perceptual_state,
            self.state_transitions,
            self.sensory_signals,
            self.dna_hash,
            self.processed_at,
            self.input_size,
        )
    }
}

impl Default for CommunityOutputBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::craft::CpResult;

    #[test]
    fn test_output_serialization() {
        let cp_result = CpResult::Valid {
            value: 0.75,
            was_clamped: false,
            unclamped_value: None,
        };

        let output = CommunityOutputBuilder::new()
            .motors(0.8, 0.9, 0.85, 0.95)
            .input_size(1000)
            .build(&cp_result);

        let json = output.to_json().unwrap();
        let restored = CommunityOutput::from_json(&json).unwrap();

        assert!((restored.craft_performance - 0.75).abs() < 0.01);
        assert!(!restored.vetoed);
    }

    #[test]
    fn test_output_with_veto() {
        let cp_result = CpResult::Vetoed {
            value: 0.0,
            cause: VetoCause::Praxeological,
        };

        let output = CommunityOutputBuilder::new()
            .motors(0.0, 0.9, 0.8, 0.9)
            .build(&cp_result);

        assert!(output.vetoed);
        assert_eq!(output.craft_performance, 0.0);
        assert!(matches!(output.veto_source, Some(VetoCause::Praxeological)));
    }

    #[test]
    fn test_proto_agency_detection() {
        use crate::sensory::state::{ProtoAgencyTrigger, StateTransition};

        let proto_state = PerceptualState::ProtoAgencyDetected {
            trigger: ProtoAgencyTrigger::new(),
        };

        // Create transitions that include proto-agency
        let transitions = vec![
            StateTransition::new(PerceptualState::Listening, PerceptualState::PerceivingCarrier, 1000, 0),
            StateTransition::new(PerceptualState::PerceivingCarrier, PerceptualState::PerceivingPattern, 2000, 1),
            StateTransition::new(PerceptualState::PerceivingPattern, PerceptualState::PerceivingStructure, 3000, 2),
            StateTransition::new(
                PerceptualState::PerceivingStructure,
                PerceptualState::ProtoAgencyDetected { trigger: ProtoAgencyTrigger::new() },
                4000,
                3,
            ),
        ];

        let cp_result = CpResult::Valid {
            value: 0.5,
            was_clamped: false,
            unclamped_value: None,
        };

        let output = CommunityOutputBuilder::new()
            .state(proto_state)
            .transitions(transitions)
            .build(&cp_result);

        assert!(output.proto_agency_detected);
        assert!((output.max_level_reached - 2.5).abs() < 0.01);
    }
}
