//! --------------------------
//! INFORMATION
//! --------------------------
//! Title: Motor Competition Module
//! Author: Carlos Eduardo Favini
//! Date: 2025-01-02
//! Version: 1.2.0
//! Description: Orchestration by competition and cooperation between motors.
//!              Motors compete for relevance while cooperating through reinforcement.
//!              "Neural networks compete for activation; the most useful reinforce themselves."
//! Layer: Community
//! Dependencies: motors, math
//! Affected Components: cognitive output, CP calculation
//!
//! --------------------------
//! INSIGHT #6: COMPETITION AND COOPERATION
//! --------------------------
//! The four motors (Praxis, Nash, Chaos, Meristic) don't just compute scores.
//! They exist in a dynamic relationship:
//! - COMPETITION: Motors compete for relevance based on their discriminative power
//! - COOPERATION: Motors that agree reinforce each other
//! - DOMINANCE: The motor with highest information gain "wins" the cycle
//! - CONSENSUS: Measures how much motors agree on the current perception
//!
//! This is NOT decision-making. It's observing the internal dynamics.
//!
//! --------------------------
//! CHANGE LOG
//! --------------------------
//! 2025-01-02 - Carlos Eduardo Favini - Initial creation (v1.2.0)
//! --------------------------

use serde::{Deserialize, Serialize};

/// Identifier for the four cognitive motors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MotorType {
    /// Praxeological motor (M_P)
    Praxis,
    /// Nash equilibrium motor (M_N)
    Nash,
    /// Chaos/stability motor (M_C)
    Chaos,
    /// Meristic generation motor (M_M)
    Meristic,
}

impl MotorType {
    /// Returns all motor types in canonical order.
    pub fn all() -> [MotorType; 4] {
        [Self::Praxis, Self::Nash, Self::Chaos, Self::Meristic]
    }

    /// Returns the index of this motor (0-3).
    pub fn index(&self) -> usize {
        match self {
            Self::Praxis => 0,
            Self::Nash => 1,
            Self::Chaos => 2,
            Self::Meristic => 3,
        }
    }

    /// Returns the motor type from an index.
    pub fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Self::Praxis),
            1 => Some(Self::Nash),
            2 => Some(Self::Chaos),
            3 => Some(Self::Meristic),
            _ => None,
        }
    }

    /// Returns the name of the motor.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Praxis => "Praxis",
            Self::Nash => "Nash",
            Self::Chaos => "Chaos",
            Self::Meristic => "Meristic",
        }
    }
}

/// Tracks the competition dynamics between motors.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotorCompetition {
    /// Current scores from each motor [Praxis, Nash, Chaos, Meristic]
    pub current_scores: [f64; 4],

    /// Relevance of each motor (how informative it is)
    /// Higher = motor provides more discrimination
    pub motor_relevance: [f64; 4],

    /// Which motor "won" this cycle (most informative)
    pub dominant_motor: Option<MotorType>,

    /// Consensus score: how much motors agree (0 = total disagreement, 1 = perfect agreement)
    pub consensus_score: f64,

    /// Variance in motor scores (high variance = high disagreement)
    pub score_variance: f64,

    /// History of dominance (for tracking patterns)
    pub dominance_history: Vec<MotorType>,
}

impl MotorCompetition {
    /// Creates a new competition analysis from motor scores.
    pub fn from_scores(scores: [f64; 4]) -> Self {
        let relevance = Self::compute_relevance(&scores);
        let dominant = Self::find_dominant(&relevance);
        let consensus = Self::compute_consensus(&scores);
        let variance = Self::compute_variance(&scores);

        Self {
            current_scores: scores,
            motor_relevance: relevance,
            dominant_motor: dominant,
            consensus_score: consensus,
            score_variance: variance,
            dominance_history: dominant.into_iter().collect(),
        }
    }

    /// Updates with new scores, tracking history.
    pub fn update(&mut self, scores: [f64; 4]) {
        self.current_scores = scores;
        self.motor_relevance = Self::compute_relevance(&scores);
        self.dominant_motor = Self::find_dominant(&self.motor_relevance);
        self.consensus_score = Self::compute_consensus(&scores);
        self.score_variance = Self::compute_variance(&scores);

        if let Some(dominant) = self.dominant_motor {
            self.dominance_history.push(dominant);
            // Keep only last 100 entries
            if self.dominance_history.len() > 100 {
                self.dominance_history.remove(0);
            }
        }
    }

    /// Computes relevance of each motor.
    /// Relevance is based on how much the motor deviates from neutral (0.5).
    /// A motor that always returns 0.5 provides no information.
    fn compute_relevance(scores: &[f64; 4]) -> [f64; 4] {
        let mut relevance = [0.0; 4];
        for (i, &score) in scores.iter().enumerate() {
            // Distance from neutral (0.5)
            // 0.0 or 1.0 = maximum relevance
            // 0.5 = minimum relevance
            relevance[i] = (score - 0.5).abs() * 2.0;
        }
        relevance
    }

    /// Finds the dominant motor (highest relevance).
    fn find_dominant(relevance: &[f64; 4]) -> Option<MotorType> {
        let max_relevance = relevance.iter().cloned().fold(0.0_f64, f64::max);

        // Only declare dominance if relevance is significant
        if max_relevance < 0.1 {
            return None;
        }

        relevance
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .and_then(|(i, _)| MotorType::from_index(i))
    }

    /// Computes consensus (agreement) between motors.
    /// High consensus = motors agree (similar scores).
    fn compute_consensus(scores: &[f64; 4]) -> f64 {
        let variance = Self::compute_variance(scores);
        // Maximum variance is 0.25 (when scores are 0, 0, 1, 1)
        // Map variance to consensus (inverse relationship)
        1.0 - (variance / 0.25).min(1.0)
    }

    /// Computes variance of motor scores.
    fn compute_variance(scores: &[f64; 4]) -> f64 {
        let mean: f64 = scores.iter().sum::<f64>() / 4.0;
        scores.iter().map(|&s| (s - mean).powi(2)).sum::<f64>() / 4.0
    }

    /// Returns the most frequently dominant motor in history.
    pub fn most_frequent_dominant(&self) -> Option<MotorType> {
        if self.dominance_history.is_empty() {
            return None;
        }

        let mut counts = [0usize; 4];
        for &motor in &self.dominance_history {
            counts[motor.index()] += 1;
        }

        counts
            .iter()
            .enumerate()
            .max_by_key(|(_, &count)| count)
            .and_then(|(i, _)| MotorType::from_index(i))
    }

    /// Returns the dominance ratio for each motor (0.0 to 1.0).
    pub fn dominance_ratios(&self) -> [f64; 4] {
        if self.dominance_history.is_empty() {
            return [0.0; 4];
        }

        let mut counts = [0usize; 4];
        for &motor in &self.dominance_history {
            counts[motor.index()] += 1;
        }

        let total = self.dominance_history.len() as f64;
        [
            counts[0] as f64 / total,
            counts[1] as f64 / total,
            counts[2] as f64 / total,
            counts[3] as f64 / total,
        ]
    }

    /// Returns true if motors are in high disagreement.
    pub fn is_high_disagreement(&self) -> bool {
        self.consensus_score < 0.5
    }

    /// Returns true if one motor is strongly dominant.
    pub fn is_strongly_dominant(&self) -> bool {
        if let Some(dominant) = self.dominant_motor {
            self.motor_relevance[dominant.index()] > 0.7
        } else {
            false
        }
    }
}

/// Represents cooperation dynamics between motor pairs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotorCooperation {
    /// Pairwise agreement scores [0][1] = agreement between Praxis and Nash
    pub pairwise_agreement: [[f64; 4]; 4],

    /// Cooperation clusters (motors that tend to agree)
    pub clusters: Vec<CooperationCluster>,
}

/// A cluster of motors that tend to cooperate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CooperationCluster {
    /// Motors in this cluster
    pub motors: Vec<MotorType>,

    /// Average agreement within cluster
    pub internal_agreement: f64,
}

impl MotorCooperation {
    /// Creates cooperation analysis from motor scores.
    pub fn from_scores(scores: &[f64; 4]) -> Self {
        let pairwise = Self::compute_pairwise_agreement(scores);
        let clusters = Self::find_clusters(&pairwise);

        Self {
            pairwise_agreement: pairwise,
            clusters,
        }
    }

    /// Computes pairwise agreement (1 - |score_i - score_j|).
    fn compute_pairwise_agreement(scores: &[f64; 4]) -> [[f64; 4]; 4] {
        let mut agreement = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                if i == j {
                    agreement[i][j] = 1.0;
                } else {
                    agreement[i][j] = 1.0 - (scores[i] - scores[j]).abs();
                }
            }
        }
        agreement
    }

    /// Finds clusters of cooperating motors.
    fn find_clusters(pairwise: &[[f64; 4]; 4]) -> Vec<CooperationCluster> {
        let mut clusters = Vec::new();
        let threshold = 0.8; // High agreement threshold

        // Simple clustering: find pairs with high agreement
        let mut visited = [false; 4];

        for i in 0..4 {
            if visited[i] {
                continue;
            }

            let mut cluster_motors = vec![MotorType::from_index(i).unwrap()];
            visited[i] = true;

            for j in (i + 1)..4 {
                if !visited[j] && pairwise[i][j] >= threshold {
                    cluster_motors.push(MotorType::from_index(j).unwrap());
                    visited[j] = true;
                }
            }

            if cluster_motors.len() >= 2 {
                // Calculate internal agreement
                let mut total_agreement = 0.0;
                let mut pair_count = 0;
                for m1 in &cluster_motors {
                    for m2 in &cluster_motors {
                        if m1.index() < m2.index() {
                            total_agreement += pairwise[m1.index()][m2.index()];
                            pair_count += 1;
                        }
                    }
                }

                let internal_agreement = if pair_count > 0 {
                    total_agreement / pair_count as f64
                } else {
                    1.0
                };

                clusters.push(CooperationCluster {
                    motors: cluster_motors,
                    internal_agreement,
                });
            }
        }

        clusters
    }

    /// Returns the agreement between two specific motors.
    pub fn agreement(&self, m1: MotorType, m2: MotorType) -> f64 {
        self.pairwise_agreement[m1.index()][m2.index()]
    }

    /// Returns pairs of motors with highest agreement.
    pub fn strongest_alliances(&self) -> Vec<(MotorType, MotorType, f64)> {
        let mut alliances = Vec::new();
        for i in 0..4 {
            for j in (i + 1)..4 {
                alliances.push((
                    MotorType::from_index(i).unwrap(),
                    MotorType::from_index(j).unwrap(),
                    self.pairwise_agreement[i][j],
                ));
            }
        }
        alliances.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));
        alliances
    }
}

/// Complete dynamics analysis combining competition and cooperation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotorDynamics {
    /// Competition analysis
    pub competition: MotorCompetition,

    /// Cooperation analysis
    pub cooperation: MotorCooperation,

    /// Overall dynamics health
    pub health: DynamicsHealth,
}

/// Health indicators for motor dynamics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicsHealth {
    /// Is the system in a balanced state?
    pub balanced: bool,

    /// Is one motor over-dominant?
    pub monopoly_risk: bool,

    /// Are motors oscillating rapidly?
    pub unstable: bool,
}

impl MotorDynamics {
    /// Creates a complete dynamics analysis.
    pub fn analyze(scores: [f64; 4]) -> Self {
        let competition = MotorCompetition::from_scores(scores);
        let cooperation = MotorCooperation::from_scores(&scores);

        let health = DynamicsHealth {
            balanced: competition.consensus_score > 0.6 && !competition.is_strongly_dominant(),
            monopoly_risk: competition.is_strongly_dominant(),
            unstable: competition.is_high_disagreement(),
        };

        Self {
            competition,
            cooperation,
            health,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_motor_type_index() {
        assert_eq!(MotorType::Praxis.index(), 0);
        assert_eq!(MotorType::Nash.index(), 1);
        assert_eq!(MotorType::Chaos.index(), 2);
        assert_eq!(MotorType::Meristic.index(), 3);
    }

    #[test]
    fn test_competition_perfect_consensus() {
        let scores = [0.8, 0.8, 0.8, 0.8];
        let competition = MotorCompetition::from_scores(scores);

        assert!(competition.consensus_score > 0.99);
        assert!(competition.score_variance < 0.01);
    }

    #[test]
    fn test_competition_total_disagreement() {
        // [0.0, 0.0, 1.0, 1.0] has maximum variance (0.25)
        // consensus = 1 - (0.25/0.25) = 0.0
        let scores = [0.0, 0.0, 1.0, 1.0];
        let competition = MotorCompetition::from_scores(scores);

        assert!(competition.consensus_score < 0.1, 
            "Expected consensus < 0.1, got {}", competition.consensus_score);
        assert!(competition.is_high_disagreement());
    }

    #[test]
    fn test_dominant_motor() {
        let scores = [1.0, 0.5, 0.5, 0.5]; // Praxis is clearly dominant
        let competition = MotorCompetition::from_scores(scores);

        assert_eq!(competition.dominant_motor, Some(MotorType::Praxis));
        assert!(competition.is_strongly_dominant());
    }

    #[test]
    fn test_cooperation_high_agreement() {
        // Praxis=0.9, Nash=0.9 (agree), Chaos=0.3, Meristic=0.3 (agree)
        let scores = [0.9, 0.9, 0.3, 0.3];
        let cooperation = MotorCooperation::from_scores(&scores);

        // Praxis and Nash should have high agreement (1 - |0.9-0.9| = 1.0)
        assert!(cooperation.agreement(MotorType::Praxis, MotorType::Nash) > 0.9);
        // Praxis and Chaos should have lower agreement (1 - |0.9-0.3| = 0.4)
        assert!(cooperation.agreement(MotorType::Praxis, MotorType::Chaos) < 0.5,
            "Expected agreement < 0.5, got {}", 
            cooperation.agreement(MotorType::Praxis, MotorType::Chaos));
    }

    #[test]
    fn test_dynamics_analysis() {
        let scores = [0.7, 0.7, 0.7, 0.7];
        let dynamics = MotorDynamics::analyze(scores);

        assert!(dynamics.health.balanced);
        assert!(!dynamics.health.monopoly_risk);
        assert!(!dynamics.health.unstable);
    }

    #[test]
    fn test_dominance_history() {
        let mut competition = MotorCompetition::from_scores([1.0, 0.5, 0.5, 0.5]);
        competition.update([0.5, 1.0, 0.5, 0.5]);
        competition.update([0.5, 0.5, 1.0, 0.5]);
        competition.update([1.0, 0.5, 0.5, 0.5]);
        competition.update([1.0, 0.5, 0.5, 0.5]);

        let ratios = competition.dominance_ratios();
        assert!(ratios[0] > ratios[1]); // Praxis dominated more
    }
}
