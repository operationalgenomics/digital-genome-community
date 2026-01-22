//! --------------------------
//! INFORMATION
//! --------------------------
//! Title: Cognitive Completeness Module
//! Author: Carlos Eduardo Favini
//! Date: 2025-01-02
//! Version: 1.2.0
//! Description: Accepting incompleteness as a valid cognitive state.
//! "Thinking is not concluding - it is sustaining ambiguities."
//! Layer: Community
//! Dependencies: sensory
//! Affected Components: cognitive output
//!
//! --------------------------
//! INSIGHT #10: INCOMPLETENESS AS VALID STATE
//! --------------------------
//! Traditional systems require complete, consistent outputs.
//! Biological cognition embraces:
//! - PARTIAL: Some levels complete, others inconclusive
//! - PROVISIONAL: Tentative conclusions awaiting more data
//! - CONTRADICTORY: Conflicting signals that cannot be resolved
//!
//! This is not a failure mode - it is honest representation
//! of the actual cognitive state.
//!
//! --------------------------
//! CHANGE LOG
//! --------------------------
//! 2025-01-02 - Carlos Eduardo Favini - Initial creation (v1.2.0)
//! --------------------------

use serde::{Deserialize, Serialize};

/// Abstraction levels that can be complete or incomplete.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AbstractionLevel {
    /// Level 0: Carrier (raw signal)
    Carrier,
    /// Level 1: Pattern (repetition, rhythm)
    Pattern,
    /// Level 2: Structure (organization)
    Structure,
    /// Level 2.5: Proto-Agency (suspected intentionality)
    ProtoAgency,
}

impl AbstractionLevel {
    /// Returns the numeric level.
    pub fn level(&self) -> f64 {
        match self {
            Self::Carrier => 0.0,
            Self::Pattern => 1.0,
            Self::Structure => 2.0,
            Self::ProtoAgency => 2.5,
        }
    }

    /// Returns all levels in order.
    pub fn all() -> [AbstractionLevel; 4] {
        [
            Self::Carrier,
            Self::Pattern,
            Self::Structure,
            Self::ProtoAgency,
        ]
    }
}

/// Types of signals that might be missing.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MissingSignal {
    /// Not enough samples for reliable analysis
    InsufficientSamples,
    /// Entropy calculation inconclusive
    EntropyInconclusive,
    /// Autocorrelation below noise floor
    AutocorrelationBelowNoise,
    /// No clear periodicity
    PeriodicityUndetermined,
    /// Statistical tests inconclusive
    StatisticalTestInconclusive,
    /// Conflicting indicators
    ConflictingIndicators,
}

/// Types of cognitive conflicts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConflictType {
    /// Pattern detected but structure says random
    PatternStructureConflict,
    /// High entropy but high compressibility
    EntropyCompressibilityConflict,
    /// Periodicity detected but runs test says random
    PeriodicityRandomnessConflict,
    /// Motors in strong disagreement
    MotorDisagreement {
        /// Motors that disagree
        motors: Vec<String>,
    },
    /// Multiple proto-agency triggers but none dominant
    AmbiguousProtoAgency,
}

/// The completeness state of a cognitive perception.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CognitiveCompleteness {
    /// Perception is complete - all levels processed successfully
    Complete,

    /// Perception is partial - some levels inconclusive
    Partial {
        /// Levels that completed successfully
        completed_levels: Vec<AbstractionLevel>,
        /// Levels that could not be conclusively processed
        inconclusive_levels: Vec<AbstractionLevel>,
        /// What signals are missing
        missing_signals: Vec<MissingSignal>,
    },

    /// Contradiction detected - conflicting signals
    Contradictory {
        /// The type of conflict
        conflict_type: ConflictType,
        /// Levels affected by the contradiction
        affected_levels: Vec<AbstractionLevel>,
    },

    /// Provisional result - tentative conclusion
    Provisional {
        /// Confidence in the provisional result (0.0 to 1.0)
        confidence: f64,
        /// What additional signals would increase confidence
        would_help: Vec<MissingSignal>,
        /// Current best guess for each level
        tentative_results: Vec<(AbstractionLevel, TentativeResult)>,
    },
}

impl CognitiveCompleteness {
    /// Creates a complete state.
    pub fn complete() -> Self {
        Self::Complete
    }

    /// Creates a partial state.
    pub fn partial(
        completed: Vec<AbstractionLevel>,
        inconclusive: Vec<AbstractionLevel>,
        missing: Vec<MissingSignal>,
    ) -> Self {
        Self::Partial {
            completed_levels: completed,
            inconclusive_levels: inconclusive,
            missing_signals: missing,
        }
    }

    /// Creates a contradictory state.
    pub fn contradictory(conflict: ConflictType, affected: Vec<AbstractionLevel>) -> Self {
        Self::Contradictory {
            conflict_type: conflict,
            affected_levels: affected,
        }
    }

    /// Creates a provisional state.
    pub fn provisional(
        confidence: f64,
        would_help: Vec<MissingSignal>,
        tentative: Vec<(AbstractionLevel, TentativeResult)>,
    ) -> Self {
        Self::Provisional {
            confidence: confidence.clamp(0.0, 1.0),
            would_help,
            tentative_results: tentative,
        }
    }

    /// Returns true if the state is complete.
    pub fn is_complete(&self) -> bool {
        matches!(self, Self::Complete)
    }

    /// Returns true if the state has contradictions.
    pub fn has_contradictions(&self) -> bool {
        matches!(self, Self::Contradictory { .. })
    }

    /// Returns true if the state is provisional.
    pub fn is_provisional(&self) -> bool {
        matches!(self, Self::Provisional { .. })
    }

    /// Returns the confidence level (1.0 for complete, variable for others).
    pub fn confidence(&self) -> f64 {
        match self {
            Self::Complete => 1.0,
            Self::Partial { completed_levels, .. } => {
                completed_levels.len() as f64 / AbstractionLevel::all().len() as f64
            }
            Self::Contradictory { .. } => 0.0,
            Self::Provisional { confidence, .. } => *confidence,
        }
    }

    /// Returns a human-readable summary.
    pub fn summary(&self) -> String {
        match self {
            Self::Complete => "COMPLETE".to_string(),
            Self::Partial {
                completed_levels,
                inconclusive_levels,
                ..
            } => {
                format!(
                    "PARTIAL ({}/{} levels)",
                    completed_levels.len(),
                    completed_levels.len() + inconclusive_levels.len()
                )
            }
            Self::Contradictory { conflict_type, .. } => {
                format!("CONTRADICTORY ({:?})", conflict_type)
            }
            Self::Provisional { confidence, .. } => {
                format!("PROVISIONAL ({:.0}% confidence)", confidence * 100.0)
            }
        }
    }
}

/// A tentative result for a level.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TentativeResult {
    /// The tentative conclusion
    pub conclusion: TentativeConclusion,
    /// Confidence in this specific result
    pub confidence: f64,
    /// Alternative interpretations
    pub alternatives: Vec<TentativeConclusion>,
}

/// Possible tentative conclusions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TentativeConclusion {
    /// Likely signal (vs noise)
    LikelySignal,
    /// Likely noise (vs signal)
    LikelyNoise,
    /// Likely periodic
    LikelyPeriodic,
    /// Likely aperiodic
    LikelyAperiodic,
    /// Likely structured
    LikelyStructured,
    /// Likely random
    LikelyRandom,
    /// Likely intentional (proto-agency)
    LikelyIntentional,
    /// Likely mechanical (not proto-agency)
    LikelyMechanical,
    /// Cannot determine
    Undetermined,
}

impl TentativeResult {
    /// Creates a new tentative result.
    pub fn new(
        conclusion: TentativeConclusion,
        confidence: f64,
        alternatives: Vec<TentativeConclusion>,
    ) -> Self {
        Self {
            conclusion,
            confidence: confidence.clamp(0.0, 1.0),
            alternatives,
        }
    }

    /// Creates an undetermined result.
    pub fn undetermined() -> Self {
        Self {
            conclusion: TentativeConclusion::Undetermined,
            confidence: 0.0,
            alternatives: Vec::new(),
        }
    }
}

/// Builder for constructing completeness states.
#[derive(Debug, Clone, Default)]
pub struct CompletenessBuilder {
    completed: Vec<AbstractionLevel>,
    inconclusive: Vec<AbstractionLevel>,
    missing: Vec<MissingSignal>,
    conflicts: Vec<(ConflictType, Vec<AbstractionLevel>)>,
    tentative: Vec<(AbstractionLevel, TentativeResult)>,
}

impl CompletenessBuilder {
    /// Creates a new builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Marks a level as complete.
    pub fn level_complete(mut self, level: AbstractionLevel) -> Self {
        if !self.completed.contains(&level) {
            self.completed.push(level);
        }
        self.inconclusive.retain(|l| *l != level);
        self
    }

    /// Marks a level as inconclusive.
    pub fn level_inconclusive(mut self, level: AbstractionLevel, missing: MissingSignal) -> Self {
        if !self.inconclusive.contains(&level) {
            self.inconclusive.push(level);
        }
        self.completed.retain(|l| *l != level);
        if !self.missing.contains(&missing) {
            self.missing.push(missing);
        }
        self
    }

    /// Adds a conflict.
    pub fn add_conflict(mut self, conflict: ConflictType, affected: Vec<AbstractionLevel>) -> Self {
        self.conflicts.push((conflict, affected));
        self
    }

    /// Adds a tentative result.
    pub fn tentative_result(mut self, level: AbstractionLevel, result: TentativeResult) -> Self {
        self.tentative.push((level, result));
        self
    }

    /// Builds the completeness state.
    pub fn build(self) -> CognitiveCompleteness {
        // Priority: Contradictory > Partial > Provisional > Complete

        if !self.conflicts.is_empty() {
            let (conflict_type, affected_levels) = self.conflicts.into_iter().next().unwrap();
            return CognitiveCompleteness::Contradictory {
                conflict_type,
                affected_levels,
            };
        }

        if !self.inconclusive.is_empty() {
            return CognitiveCompleteness::Partial {
                completed_levels: self.completed,
                inconclusive_levels: self.inconclusive,
                missing_signals: self.missing,
            };
        }

        if !self.tentative.is_empty() {
            let avg_confidence: f64 = self.tentative.iter().map(|(_, r)| r.confidence).sum::<f64>()
                / self.tentative.len() as f64;

            return CognitiveCompleteness::Provisional {
                confidence: avg_confidence,
                would_help: self.missing,
                tentative_results: self.tentative,
            };
        }

        if self.completed.len() == AbstractionLevel::all().len() {
            CognitiveCompleteness::Complete
        } else {
            // Not all levels complete, not explicitly inconclusive
            CognitiveCompleteness::Partial {
                completed_levels: self.completed,
                inconclusive_levels: self.inconclusive,
                missing_signals: self.missing,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complete_state() {
        let state = CognitiveCompleteness::complete();
        assert!(state.is_complete());
        assert_eq!(state.confidence(), 1.0);
    }

    #[test]
    fn test_partial_state() {
        let state = CognitiveCompleteness::partial(
            vec![AbstractionLevel::Carrier, AbstractionLevel::Pattern],
            vec![AbstractionLevel::Structure],
            vec![MissingSignal::StatisticalTestInconclusive],
        );

        assert!(!state.is_complete());
        assert!(!state.has_contradictions());
        assert!(state.confidence() > 0.0 && state.confidence() < 1.0);
    }

    #[test]
    fn test_contradictory_state() {
        let state = CognitiveCompleteness::contradictory(
            ConflictType::PatternStructureConflict,
            vec![AbstractionLevel::Pattern, AbstractionLevel::Structure],
        );

        assert!(state.has_contradictions());
        assert_eq!(state.confidence(), 0.0);
    }

    #[test]
    fn test_provisional_state() {
        let state = CognitiveCompleteness::provisional(
            0.7,
            vec![MissingSignal::InsufficientSamples],
            vec![(
                AbstractionLevel::Pattern,
                TentativeResult::new(TentativeConclusion::LikelyPeriodic, 0.7, vec![]),
            )],
        );

        assert!(state.is_provisional());
        assert!((state.confidence() - 0.7).abs() < 0.01);
    }

    #[test]
    fn test_builder_complete() {
        let state = CompletenessBuilder::new()
            .level_complete(AbstractionLevel::Carrier)
            .level_complete(AbstractionLevel::Pattern)
            .level_complete(AbstractionLevel::Structure)
            .level_complete(AbstractionLevel::ProtoAgency)
            .build();

        assert!(state.is_complete());
    }

    #[test]
    fn test_builder_partial() {
        let state = CompletenessBuilder::new()
            .level_complete(AbstractionLevel::Carrier)
            .level_inconclusive(
                AbstractionLevel::Pattern,
                MissingSignal::AutocorrelationBelowNoise,
            )
            .build();

        assert!(!state.is_complete());
        assert!(matches!(state, CognitiveCompleteness::Partial { .. }));
    }

    #[test]
    fn test_builder_conflict_priority() {
        let state = CompletenessBuilder::new()
            .level_complete(AbstractionLevel::Carrier)
            .level_inconclusive(
                AbstractionLevel::Pattern,
                MissingSignal::AutocorrelationBelowNoise,
            )
            .add_conflict(
                ConflictType::EntropyCompressibilityConflict,
                vec![AbstractionLevel::Carrier, AbstractionLevel::Structure],
            )
            .build();

        // Conflict takes priority over partial
        assert!(state.has_contradictions());
    }

    #[test]
    fn test_summary() {
        assert_eq!(CognitiveCompleteness::complete().summary(), "COMPLETE");

        let partial = CognitiveCompleteness::partial(
            vec![AbstractionLevel::Carrier],
            vec![AbstractionLevel::Pattern],
            vec![],
        );
        assert!(partial.summary().contains("PARTIAL"));
    }
}
