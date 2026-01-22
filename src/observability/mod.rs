//! --------------------------
//! INFORMATION
//! --------------------------
//! Title: Cognitive Observability Module
//! Author: Carlos Eduardo Favini
//! Date: 2025-01-02
//! Version: 1.2.0
//! Description: Metacognition - observing the observer.
//! Exposes internal processing states as indicators.
//! "Humans perceive when they are thinking too much or stuck."
//! Layer: Community
//! Dependencies: sensory, competition
//! Affected Components: cognitive output
//!
//! --------------------------
//! INSIGHT #7: COGNITIVE OBSERVABILITY
//! --------------------------
//! The system should be able to observe its own cognitive state:
//! - Progress: Is perception advancing through levels?
//! - Stagnation: Has the system been stuck at the same state?
//! - Divergence: Are internal components disagreeing?
//! - Oscillation: Is the system rapidly switching between states?
//!
//! This enables:
//! - Human intervention when needed
//! - Prevention of infinite loops
//! - Operational confidence
//!
//! This is OBSERVATION, not intervention. Community observes.
//! Enterprise acts on these observations.
//!
//! --------------------------
//! CHANGE LOG
//! --------------------------
//! 2025-01-02 - Carlos Eduardo Favini - Initial creation (v1.2.0)
//! --------------------------

use serde::{Deserialize, Serialize};

use crate::competition::MotorCompetition;
use crate::sensory::state::{PerceptualState, StateTransition};

/// Health indicators for the cognitive system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthIndicators {
    /// System may be stuck (no progress)
    pub possibly_stuck: bool,

    /// High divergence between motors
    pub high_divergence: bool,

    /// Rapid oscillation between states
    pub rapid_oscillation: bool,

    /// Processing taking too long
    pub timeout_risk: bool,

    /// Overall health score (0 = critical, 1 = healthy)
    pub overall_health: f64,
}

impl Default for HealthIndicators {
    fn default() -> Self {
        Self {
            possibly_stuck: false,
            high_divergence: false,
            rapid_oscillation: false,
            timeout_risk: false,
            overall_health: 1.0,
        }
    }
}

impl HealthIndicators {
    /// Creates healthy indicators.
    pub fn healthy() -> Self {
        Self::default()
    }

    /// Returns true if any warning is active.
    pub fn has_warnings(&self) -> bool {
        self.possibly_stuck || self.high_divergence || self.rapid_oscillation || self.timeout_risk
    }

    /// Returns the count of active warnings.
    pub fn warning_count(&self) -> usize {
        let mut count = 0;
        if self.possibly_stuck {
            count += 1;
        }
        if self.high_divergence {
            count += 1;
        }
        if self.rapid_oscillation {
            count += 1;
        }
        if self.timeout_risk {
            count += 1;
        }
        count
    }
}

/// Progress tracking for cognitive processing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressTracker {
    /// Cycles without state change
    pub cycles_without_progress: usize,

    /// Maximum level reached so far
    pub max_level_reached: f64,

    /// Time at current level (nanoseconds)
    pub time_at_current_level_ns: u64,

    /// Total processing time (nanoseconds)
    pub total_processing_time_ns: u64,

    /// Number of level advances
    pub level_advances: usize,

    /// Number of level regressions
    pub level_regressions: usize,
}

impl Default for ProgressTracker {
    fn default() -> Self {
        Self {
            cycles_without_progress: 0,
            max_level_reached: 0.0,
            time_at_current_level_ns: 0,
            total_processing_time_ns: 0,
            level_advances: 0,
            level_regressions: 0,
        }
    }
}

impl ProgressTracker {
    /// Creates a new progress tracker.
    pub fn new() -> Self {
        Self::default()
    }

    /// Updates the tracker based on a state transition.
    pub fn record_transition(&mut self, transition: &StateTransition) {
        let from_level = transition.from.level();
        let to_level = transition.to.level();

        if to_level > from_level {
            self.level_advances += 1;
            self.cycles_without_progress = 0;
        } else if to_level < from_level {
            self.level_regressions += 1;
        } else {
            self.cycles_without_progress += 1;
        }

        if to_level > self.max_level_reached {
            self.max_level_reached = to_level;
        }

        self.time_at_current_level_ns = transition.timestamp_ns;
    }

    /// Returns the progress ratio (advances / total transitions).
    pub fn progress_ratio(&self) -> f64 {
        let total = self.level_advances + self.level_regressions;
        if total == 0 {
            return 1.0;
        }
        self.level_advances as f64 / total as f64
    }

    /// Returns true if the system appears stuck.
    pub fn is_stuck(&self, threshold: usize) -> bool {
        self.cycles_without_progress >= threshold
    }
}

/// Divergence tracking between cognitive components.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DivergenceTracker {
    /// Current divergence between motors (0 = agreement, 1 = total disagreement)
    pub motor_divergence: f64,

    /// Historical divergence values
    pub divergence_history: Vec<f64>,

    /// Average divergence over history
    pub average_divergence: f64,

    /// Maximum divergence observed
    pub max_divergence: f64,
}

impl Default for DivergenceTracker {
    fn default() -> Self {
        Self {
            motor_divergence: 0.0,
            divergence_history: Vec::new(),
            average_divergence: 0.0,
            max_divergence: 0.0,
        }
    }
}

impl DivergenceTracker {
    /// Creates a new divergence tracker.
    pub fn new() -> Self {
        Self::default()
    }

    /// Updates with motor competition data.
    pub fn update_from_competition(&mut self, competition: &MotorCompetition) {
        // Divergence is inverse of consensus
        self.motor_divergence = 1.0 - competition.consensus_score;

        self.divergence_history.push(self.motor_divergence);

        // Keep only last 100 entries
        if self.divergence_history.len() > 100 {
            self.divergence_history.remove(0);
        }

        // Update statistics
        if self.motor_divergence > self.max_divergence {
            self.max_divergence = self.motor_divergence;
        }

        self.average_divergence =
            self.divergence_history.iter().sum::<f64>() / self.divergence_history.len() as f64;
    }

    /// Returns true if divergence is critically high.
    pub fn is_critical(&self, threshold: f64) -> bool {
        self.motor_divergence > threshold
    }

    /// Returns the trend (-1 = decreasing, 0 = stable, 1 = increasing).
    pub fn trend(&self) -> i32 {
        if self.divergence_history.len() < 5 {
            return 0;
        }

        let recent: f64 = self.divergence_history.iter().rev().take(5).sum::<f64>() / 5.0;
        let older: f64 = self.divergence_history.iter().rev().skip(5).take(5).sum::<f64>() / 5.0;

        if recent > older + 0.1 {
            1
        } else if recent < older - 0.1 {
            -1
        } else {
            0
        }
    }
}

/// Oscillation detection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OscillationDetector {
    /// Recent state sequence
    pub recent_states: Vec<String>,

    /// Detected oscillation pattern (if any)
    pub oscillation_pattern: Option<Vec<String>>,

    /// Oscillation frequency (state changes per cycle)
    pub oscillation_frequency: f64,
}

impl Default for OscillationDetector {
    fn default() -> Self {
        Self {
            recent_states: Vec::new(),
            oscillation_pattern: None,
            oscillation_frequency: 0.0,
        }
    }
}

impl OscillationDetector {
    /// Creates a new oscillation detector.
    pub fn new() -> Self {
        Self::default()
    }

    /// Records a new state.
    pub fn record_state(&mut self, state: &PerceptualState) {
        let state_name = state.name().to_string();
        self.recent_states.push(state_name);

        // Keep only last 20 states
        if self.recent_states.len() > 20 {
            self.recent_states.remove(0);
        }

        self.detect_oscillation();
    }

    /// Detects oscillation patterns in recent states.
    fn detect_oscillation(&mut self) {
        if self.recent_states.len() < 4 {
            self.oscillation_pattern = None;
            self.oscillation_frequency = 0.0;
            return;
        }

        // Count state changes
        let changes = self
            .recent_states
            .windows(2)
            .filter(|w| w[0] != w[1])
            .count();

        self.oscillation_frequency = changes as f64 / (self.recent_states.len() - 1) as f64;

        // Detect simple A-B-A-B pattern
        if self.recent_states.len() >= 4 {
            let last = &self.recent_states[self.recent_states.len() - 4..];
            if last[0] == last[2] && last[1] == last[3] && last[0] != last[1] {
                self.oscillation_pattern = Some(vec![last[0].clone(), last[1].clone()]);
            } else {
                self.oscillation_pattern = None;
            }
        }
    }

    /// Returns true if rapid oscillation is detected.
    pub fn is_oscillating(&self) -> bool {
        self.oscillation_pattern.is_some() || self.oscillation_frequency > 0.8
    }
}

/// Complete cognitive observability system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveObservability {
    /// Health indicators
    pub health: HealthIndicators,

    /// Progress tracking
    pub progress: ProgressTracker,

    /// Divergence tracking
    pub divergence: DivergenceTracker,

    /// Oscillation detection
    pub oscillation: OscillationDetector,

    /// Timestamp of last update
    pub last_updated_ns: u64,
}

impl Default for CognitiveObservability {
    fn default() -> Self {
        Self {
            health: HealthIndicators::default(),
            progress: ProgressTracker::default(),
            divergence: DivergenceTracker::default(),
            oscillation: OscillationDetector::default(),
            last_updated_ns: 0,
        }
    }
}

impl CognitiveObservability {
    /// Creates a new observability system.
    pub fn new() -> Self {
        Self::default()
    }

    /// Updates from a state transition.
    pub fn record_transition(&mut self, transition: &StateTransition) {
        self.progress.record_transition(transition);
        self.oscillation.record_state(&transition.to);
        self.last_updated_ns = transition.timestamp_ns;
        self.update_health();
    }

    /// Updates from motor competition.
    pub fn record_competition(&mut self, competition: &MotorCompetition) {
        self.divergence.update_from_competition(competition);
        self.update_health();
    }

    /// Sets the total processing time.
    pub fn set_processing_time(&mut self, time_ns: u64) {
        self.progress.total_processing_time_ns = time_ns;
        self.update_health();
    }

    /// Updates health indicators based on current state.
    fn update_health(&mut self) {
        self.health.possibly_stuck = self.progress.is_stuck(5);
        self.health.high_divergence = self.divergence.is_critical(0.7);
        self.health.rapid_oscillation = self.oscillation.is_oscillating();
        self.health.timeout_risk = self.progress.total_processing_time_ns > 1_000_000_000; // 1 second

        // Calculate overall health
        let penalties = self.health.warning_count() as f64 * 0.25;
        self.health.overall_health = (1.0 - penalties).max(0.0);
    }

    /// Returns a human-readable status summary.
    pub fn status_summary(&self) -> String {
        let mut warnings = Vec::new();

        if self.health.possibly_stuck {
            warnings.push("STUCK");
        }
        if self.health.high_divergence {
            warnings.push("DIVERGENT");
        }
        if self.health.rapid_oscillation {
            warnings.push("OSCILLATING");
        }
        if self.health.timeout_risk {
            warnings.push("TIMEOUT_RISK");
        }

        if warnings.is_empty() {
            "HEALTHY".to_string()
        } else {
            warnings.join(" | ")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sensory::state::PerceptualState;

    #[test]
    fn test_health_indicators_default() {
        let health = HealthIndicators::default();
        assert!(!health.has_warnings());
        assert_eq!(health.overall_health, 1.0);
    }

    #[test]
    fn test_progress_tracker() {
        let mut tracker = ProgressTracker::new();

        let t1 = StateTransition::new(
            PerceptualState::Listening,
            PerceptualState::PerceivingCarrier,
            1000,
            0,
        );
        tracker.record_transition(&t1);

        assert_eq!(tracker.level_advances, 0); // Same level (0.0)

        let t2 = StateTransition::new(
            PerceptualState::PerceivingCarrier,
            PerceptualState::PerceivingPattern,
            2000,
            1,
        );
        tracker.record_transition(&t2);

        assert_eq!(tracker.level_advances, 1);
        assert!((tracker.max_level_reached - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_progress_stuck_detection() {
        let mut tracker = ProgressTracker::new();

        // Simulate 5 transitions at same level
        for i in 0..5 {
            let t = StateTransition::new(
                PerceptualState::PerceivingPattern,
                PerceptualState::PerceivingPattern,
                i * 1000,
                i,
            );
            tracker.record_transition(&t);
        }

        assert!(tracker.is_stuck(5));
    }

    #[test]
    fn test_divergence_tracker() {
        let mut tracker = DivergenceTracker::new();
        let competition = MotorCompetition::from_scores([0.0, 0.5, 0.5, 1.0]);

        tracker.update_from_competition(&competition);

        assert!(tracker.motor_divergence > 0.3);
        assert!(!tracker.is_critical(0.9));
    }

    #[test]
    fn test_oscillation_detection() {
        let mut detector = OscillationDetector::new();

        // Simulate A-B-A-B pattern
        detector.record_state(&PerceptualState::PerceivingPattern);
        detector.record_state(&PerceptualState::PerceivingStructure);
        detector.record_state(&PerceptualState::PerceivingPattern);
        detector.record_state(&PerceptualState::PerceivingStructure);

        assert!(detector.is_oscillating());
        assert!(detector.oscillation_pattern.is_some());
    }

    #[test]
    fn test_cognitive_observability_status() {
        let obs = CognitiveObservability::new();
        assert_eq!(obs.status_summary(), "HEALTHY");
    }
}
