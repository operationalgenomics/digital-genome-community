//! --------------------------
//! INFORMATION
//! --------------------------
//! Title: Nash Motor (M_N)
//! Author: Carlos Eduardo Favini
//! Date: 2025-01-02
//! Version: 1.2.0
//! Description: Motor based on Nash equilibrium theory.
//! Evaluates collective equilibrium of actions.
//! Works with conscious agents, automata, or hybrid systems.
//! Layer: Community
//! Dependencies: Consagrated mathematics (game theory, equilibrium theory)
//! Affected Components: math/craft
//!
//! --------------------------
//! OVERFLOW PROTECTION (v0.2.0)
//! --------------------------
//! This motor uses checked arithmetic where overflow is possible.
//! Games are limited to prevent overflow:
//! - MAX_PLAYERS = 10
//! - MAX_ACTIONS = 100
//! - MAX_PAYOFF_MAGNITUDE = 10^12
//!
//! --------------------------
//! CHANGE LOG
//! --------------------------
//! 2025-01-02 - Carlos Eduardo Favini - Initial creation
//! 2025-01-02 - Carlos Eduardo Favini - Added dimensional validation
//! 2025-01-02 - Carlos Eduardo Favini - Added was_clamped signaling (v0.2.0)
//! 2025-01-02 - Carlos Eduardo Favini - Added overflow protection (v0.2.0)
//! --------------------------

use serde::{Deserialize, Serialize};

use super::CognitiveMotor;

/// Maximum number of players to prevent overflow.
pub const MAX_PLAYERS: usize = 10;

/// Maximum actions per player to prevent overflow.
pub const MAX_ACTIONS_PER_PLAYER: usize = 100;

/// Maximum payoff magnitude to prevent overflow.
pub const MAX_PAYOFF_MAGNITUDE: i64 = 1_000_000_000_000; // 10^12

/// Nash Motor (M_N).
///
/// Evaluates collective equilibrium of actions using Nash equilibrium
/// theory and extensions.
///
/// # Function
/// - For conscious agents: calculates optimal collective outcomes
/// - For automata: orchestrates action sequences
/// - For hybrid systems: balances both
///
/// # Restrictions (Constitutional)
/// - Does NOT impose optimal decisions
/// - Does NOT force cooperation
/// - Does NOT execute coordination
///
/// # Output
/// - Possible equilibrium states
/// - Structural costs of each configuration
/// - Continuous score [0.0, 1.0]
#[derive(Debug, Clone)]
pub struct NashMotor;

/// Input for the Nash Motor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NashInput {
    /// Number of players/agents in the game.
    pub num_players: usize,

    /// Number of actions available to each player.
    pub action_sizes: Vec<usize>,

    /// Payoff matrices for each player (flattened).
    /// payoffs[i] is player i's payoff for each joint action profile.
    /// Length of payoffs[i] must equal product of action_sizes.
    pub payoffs: Vec<Vec<i64>>,

    /// Current strategy profile (probability distributions).
    /// strategies[i][a] is probability player i plays action a.
    /// Values are scaled integers (sum = scale for each player).
    /// Length of strategies[i] must equal action_sizes[i].
    pub strategies: Vec<Vec<u64>>,

    /// Scale factor for integer probability representation.
    /// Must be > 0. Strategies sum to this value.
    pub scale: u64,
}

/// Validation error for Nash input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NashValidationError {
    /// Scale must be greater than zero.
    ZeroScale,
    /// Number of players must be greater than zero.
    ZeroPlayers,
    /// Too many players (overflow risk).
    TooManyPlayers { max: usize, got: usize },
    /// Too many actions (overflow risk).
    TooManyActions { player: usize, max: usize, got: usize },
    /// Payoff magnitude too large (overflow risk).
    PayoffTooLarge { player: usize, profile: usize, max: i64, got: i64 },
    /// action_sizes length must equal num_players.
    ActionSizesMismatch { expected: usize, got: usize },
    /// payoffs length must equal num_players.
    PayoffsMismatch { expected: usize, got: usize },
    /// strategies length must equal num_players.
    StrategiesMismatch { expected: usize, got: usize },
    /// payoffs[i] length must equal total_profiles.
    PayoffDimensionMismatch { player: usize, expected: usize, got: usize },
    /// strategies[i] length must equal action_sizes[i].
    StrategyDimensionMismatch { player: usize, expected: usize, got: usize },
    /// Each action_size must be > 0.
    ZeroActionSize { player: usize },
}

impl std::fmt::Display for NashValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ZeroScale => write!(f, "Scale must be > 0"),
            Self::ZeroPlayers => write!(f, "Number of players must be > 0"),
            Self::TooManyPlayers { max, got } => {
                write!(f, "Too many players: max={}, got={}", max, got)
            }
            Self::TooManyActions { player, max, got } => {
                write!(f, "Too many actions for player {}: max={}, got={}", player, max, got)
            }
            Self::PayoffTooLarge { player, profile, max, got } => {
                write!(f, "Payoff too large at [{}][{}]: max=±{}, got={}", player, profile, max, got)
            }
            Self::ActionSizesMismatch { expected, got } => {
                write!(f, "action_sizes.len() = {} but num_players = {}", got, expected)
            }
            Self::PayoffsMismatch { expected, got } => {
                write!(f, "payoffs.len() = {} but num_players = {}", got, expected)
            }
            Self::StrategiesMismatch { expected, got } => {
                write!(f, "strategies.len() = {} but num_players = {}", got, expected)
            }
            Self::PayoffDimensionMismatch { player, expected, got } => {
                write!(f, "payoffs[{}].len() = {} but total_profiles = {}", player, got, expected)
            }
            Self::StrategyDimensionMismatch { player, expected, got } => {
                write!(f, "strategies[{}].len() = {} but action_sizes[{}] = {}", player, got, player, expected)
            }
            Self::ZeroActionSize { player } => {
                write!(f, "action_sizes[{}] must be > 0", player)
            }
        }
    }
}

impl std::error::Error for NashValidationError {}

/// Output from the Nash Motor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NashOutput {
    /// Whether the input was valid.
    pub valid: bool,

    /// Validation error if invalid.
    pub validation_error: Option<String>,

    /// Epsilon vector: deviation incentive for each player.
    /// ε_i = max(0, BR_i - U_i)
    pub epsilon_vector: Vec<u64>,

    /// Equilibrium quality: η_eq = 1 / (1 + d̄)
    /// where d̄ = (Σ ε_i) / (n × U_max)
    pub eta_equilibrium: f64,

    /// Final motor score: M_N = η_eq
    /// Returns 0.0 if input is invalid.
    pub score: f64,

    /// Whether the final score was clamped to [0, 1].
    pub was_clamped: bool,

    /// Original score before clamping (if was_clamped is true).
    pub unclamped_score: Option<f64>,
}

impl NashOutput {
    /// Creates an invalid output with zero score.
    fn invalid(error: NashValidationError) -> Self {
        Self {
            valid: false,
            validation_error: Some(error.to_string()),
            epsilon_vector: Vec::new(),
            eta_equilibrium: 0.0,
            score: 0.0,
            was_clamped: false,
            unclamped_score: None,
        }
    }

    /// Creates a valid output.
    fn valid(epsilon_vector: Vec<u64>, eta_equilibrium: f64) -> Self {
        let needs_clamping = eta_equilibrium < 0.0 || eta_equilibrium > 1.0 || !eta_equilibrium.is_finite();
        let final_score = if needs_clamping {
            if eta_equilibrium.is_nan() { 0.0 } else { eta_equilibrium.clamp(0.0, 1.0) }
        } else {
            eta_equilibrium
        };

        Self {
            valid: true,
            validation_error: None,
            epsilon_vector,
            eta_equilibrium,
            score: final_score,
            was_clamped: needs_clamping,
            unclamped_score: if needs_clamping { Some(eta_equilibrium) } else { None },
        }
    }
}

impl NashMotor {
    /// Creates a new Nash Motor.
    pub fn new() -> Self {
        Self
    }

    /// Validates input dimensions and constraints.
    fn validate(input: &NashInput) -> Result<(), NashValidationError> {
        // Scale must be > 0
        if input.scale == 0 {
            return Err(NashValidationError::ZeroScale);
        }

        // Must have players
        if input.num_players == 0 {
            return Err(NashValidationError::ZeroPlayers);
        }

        // Check player limit for overflow protection
        if input.num_players > MAX_PLAYERS {
            return Err(NashValidationError::TooManyPlayers {
                max: MAX_PLAYERS,
                got: input.num_players,
            });
        }

        // action_sizes length must match num_players
        if input.action_sizes.len() != input.num_players {
            return Err(NashValidationError::ActionSizesMismatch {
                expected: input.num_players,
                got: input.action_sizes.len(),
            });
        }

        // All action_sizes must be > 0 and within limits
        for (i, &size) in input.action_sizes.iter().enumerate() {
            if size == 0 {
                return Err(NashValidationError::ZeroActionSize { player: i });
            }
            if size > MAX_ACTIONS_PER_PLAYER {
                return Err(NashValidationError::TooManyActions {
                    player: i,
                    max: MAX_ACTIONS_PER_PLAYER,
                    got: size,
                });
            }
        }

        // payoffs length must match num_players
        if input.payoffs.len() != input.num_players {
            return Err(NashValidationError::PayoffsMismatch {
                expected: input.num_players,
                got: input.payoffs.len(),
            });
        }

        // strategies length must match num_players
        if input.strategies.len() != input.num_players {
            return Err(NashValidationError::StrategiesMismatch {
                expected: input.num_players,
                got: input.strategies.len(),
            });
        }

        // Calculate total profiles
        let total_profiles: usize = input.action_sizes.iter().product();

        // Validate each player's dimensions and payoff magnitudes
        for (i, (player_payoffs, player_strategies)) in input.payoffs.iter()
            .zip(input.strategies.iter())
            .enumerate()
        {
            // payoffs[i] must have total_profiles entries
            if player_payoffs.len() != total_profiles {
                return Err(NashValidationError::PayoffDimensionMismatch {
                    player: i,
                    expected: total_profiles,
                    got: player_payoffs.len(),
                });
            }

            // Check payoff magnitudes
            for (j, &payoff) in player_payoffs.iter().enumerate() {
                if payoff.abs() > MAX_PAYOFF_MAGNITUDE {
                    return Err(NashValidationError::PayoffTooLarge {
                        player: i,
                        profile: j,
                        max: MAX_PAYOFF_MAGNITUDE,
                        got: payoff,
                    });
                }
            }

            // strategies[i] must have action_sizes[i] entries
            if player_strategies.len() != input.action_sizes[i] {
                return Err(NashValidationError::StrategyDimensionMismatch {
                    player: i,
                    expected: input.action_sizes[i],
                    got: player_strategies.len(),
                });
            }
        }

        Ok(())
    }

    /// Converts a strategy profile index to joint action profile.
    fn index_to_profile(action_sizes: &[usize], mut index: usize) -> Vec<usize> {
        let n = action_sizes.len();
        let mut profile = vec![0; n];

        for i in (0..n).rev() {
            profile[i] = index % action_sizes[i];
            index /= action_sizes[i];
        }

        profile
    }

    /// Calculates total number of joint action profiles.
    fn total_profiles(action_sizes: &[usize]) -> usize {
        action_sizes.iter().product()
    }

    /// Calculates joint probability of a profile (scaled).
    /// Returns probability scaled by input.scale.
    fn joint_probability(strategies: &[&[u64]], profile: &[usize], scale: u64) -> u64 {
        let n = strategies.len();
        let s = scale as u128;

        let mut prod: u128 = 1;
        for (i, strat) in strategies.iter().enumerate() {
            prod = prod.saturating_mul(strat[profile[i]] as u128);
        }

        // Normalize by scale^(n-1) to keep result in scale
        let n_minus_1 = n.saturating_sub(1) as u32;
        let denom = s.saturating_pow(n_minus_1);
        if denom == 0 {
            return 0;
        }
        (prod / denom) as u64
    }

    /// Calculates expected utility for player i under current strategy profile.
    /// Note: `_player` parameter maintained for API symmetry with `expected_utility_pure_action`.
    /// The payoffs slice is already player-specific when passed to this function.
    fn expected_utility(
        _player: usize,
        payoffs: &[i64],
        action_sizes: &[usize],
        strategies: &[&[u64]],
        scale: u64,
    ) -> i64 {
        let s = scale as i128;
        let total = Self::total_profiles(action_sizes);
        let mut acc: i128 = 0;

        for idx in 0..total {
            let profile = Self::index_to_profile(action_sizes, idx);
            let q = Self::joint_probability(strategies, &profile, scale) as i128;
            acc = acc.saturating_add(q.saturating_mul(payoffs[idx] as i128));
        }

        // Avoid division by zero (scale already validated > 0)
        (acc / s) as i64
    }

    /// Calculates expected utility for player i playing action a_i (pure strategy).
    /// Other players maintain their mixed strategies.
    fn expected_utility_pure_action(
        player: usize,
        action: usize,
        payoffs: &[i64],
        action_sizes: &[usize],
        strategies: &[&[u64]],
        scale: u64,
    ) -> i64 {
        let total = Self::total_profiles(action_sizes);
        let mut acc: i128 = 0;

        for idx in 0..total {
            let profile = Self::index_to_profile(action_sizes, idx);
            
            // Only consider profiles where this player plays this action
            if profile[player] != action {
                continue;
            }

            // Calculate probability of opponents' joint action
            let mut opponent_prob: u128 = 1;
            for (i, &p_action) in profile.iter().enumerate() {
                if i != player {
                    opponent_prob = opponent_prob.saturating_mul(strategies[i][p_action] as u128);
                }
            }

            acc = acc.saturating_add((opponent_prob as i128).saturating_mul(payoffs[idx] as i128));
        }

        // Normalize by scale^(n-1) where n-1 is number of opponents
        let n_opponents = (action_sizes.len() - 1) as u32;
        let denom = (scale as i128).saturating_pow(n_opponents);
        if denom == 0 {
            return 0;
        }
        (acc / denom) as i64
    }

    /// Calculates best response utility for player i.
    /// Returns the maximum expected utility achievable by deviating to any pure action.
    fn best_response(
        player: usize,
        payoffs: &[i64],
        action_sizes: &[usize],
        strategies: &[&[u64]],
        scale: u64,
    ) -> i64 {
        (0..action_sizes[player])
            .map(|a| Self::expected_utility_pure_action(player, a, payoffs, action_sizes, strategies, scale))
            .max()
            .unwrap_or(i64::MIN)
    }

    /// Calculates epsilon (deviation incentive) for each player.
    fn calculate_epsilon_vector(
        num_players: usize,
        payoffs: &[Vec<i64>],
        action_sizes: &[usize],
        strategies: &[&[u64]],
        scale: u64,
    ) -> Vec<u64> {
        let mut eps = vec![0u64; num_players];

        for i in 0..num_players {
            let u_current = Self::expected_utility(i, &payoffs[i], action_sizes, strategies, scale);
            let br = Self::best_response(i, &payoffs[i], action_sizes, strategies, scale);

            let gap = br - u_current;
            eps[i] = if gap > 0 { gap as u64 } else { 0 };
        }

        eps
    }

    /// Calculates equilibrium quality from epsilon vector.
    /// η_eq = 1 / (1 + d̄)
    fn calculate_eta_equilibrium(epsilon: &[u64], u_max: u64) -> f64 {
        if epsilon.is_empty() || u_max == 0 {
            return 1.0;
        }

        let d_sum: u64 = epsilon.iter().sum();
        let d_bar = (d_sum as f64) / ((epsilon.len() as f64) * (u_max as f64));

        1.0 / (1.0 + d_bar)
    }
}

impl Default for NashMotor {
    fn default() -> Self {
        Self::new()
    }
}

impl CognitiveMotor for NashMotor {
    type Input = NashInput;
    type Output = NashOutput;

    /// Evaluates the game and produces M_N score.
    ///
    /// Formula: M_N = η_eq = 1 / (1 + d̄)
    ///
    /// Returns zero score if input is invalid.
    fn evaluate(&self, input: &Self::Input) -> Self::Output {
        // Validate input
        if let Err(e) = Self::validate(input) {
            return NashOutput::invalid(e);
        }

        // Build strategy references
        let strategies: Vec<&[u64]> = input.strategies.iter().map(|s| s.as_slice()).collect();

        // Calculate epsilon vector
        let epsilon_vector = Self::calculate_epsilon_vector(
            input.num_players,
            &input.payoffs,
            &input.action_sizes,
            &strategies,
            input.scale,
        );

        // Find maximum payoff magnitude for normalization
        let u_max = input
            .payoffs
            .iter()
            .flat_map(|p| p.iter())
            .map(|&x| x.unsigned_abs())
            .max()
            .unwrap_or(1);

        // Calculate equilibrium quality
        let eta_equilibrium = Self::calculate_eta_equilibrium(&epsilon_vector, u_max);

        NashOutput::valid(epsilon_vector, eta_equilibrium)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_zero_scale() {
        let motor = NashMotor::new();

        let input = NashInput {
            num_players: 2,
            action_sizes: vec![2, 2],
            payoffs: vec![vec![1, 0, 0, 1], vec![1, 0, 0, 1]],
            strategies: vec![vec![50, 50], vec![50, 50]],
            scale: 0, // Invalid
        };

        let output = motor.evaluate(&input);
        assert!(!output.valid);
        assert_eq!(output.score, 0.0);
    }

    #[test]
    fn test_validation_too_many_players() {
        let motor = NashMotor::new();

        let input = NashInput {
            num_players: 15, // Exceeds MAX_PLAYERS
            action_sizes: vec![2; 15],
            payoffs: vec![vec![1; 32768]; 15],
            strategies: vec![vec![50, 50]; 15],
            scale: 100,
        };

        let output = motor.evaluate(&input);
        assert!(!output.valid);
        assert!(output.validation_error.unwrap().contains("Too many players"));
    }

    #[test]
    fn test_validation_payoff_too_large() {
        let motor = NashMotor::new();

        let input = NashInput {
            num_players: 2,
            action_sizes: vec![2, 2],
            payoffs: vec![
                vec![1, 0, 0, 10_000_000_000_000i64], // Exceeds MAX_PAYOFF_MAGNITUDE
                vec![1, 0, 0, 1],
            ],
            strategies: vec![vec![50, 50], vec![50, 50]],
            scale: 100,
        };

        let output = motor.evaluate(&input);
        assert!(!output.valid);
        assert!(output.validation_error.unwrap().contains("Payoff too large"));
    }

    #[test]
    fn test_validation_dimension_mismatch() {
        let motor = NashMotor::new();

        let input = NashInput {
            num_players: 2,
            action_sizes: vec![2, 2, 2], // Wrong: 3 sizes for 2 players
            payoffs: vec![vec![1, 0, 0, 1], vec![1, 0, 0, 1]],
            strategies: vec![vec![50, 50], vec![50, 50]],
            scale: 100,
        };

        let output = motor.evaluate(&input);
        assert!(!output.valid);
    }

    #[test]
    fn test_symmetric_game_equilibrium() {
        let motor = NashMotor::new();

        // Coordination game: (A,A)=1, (A,B)=0, (B,A)=0, (B,B)=1
        // Both playing A is a Nash equilibrium
        let input = NashInput {
            num_players: 2,
            action_sizes: vec![2, 2],
            payoffs: vec![
                vec![1, 0, 0, 1], // Player 0
                vec![1, 0, 0, 1], // Player 1
            ],
            strategies: vec![
                vec![100, 0], // Player 0: 100% A
                vec![100, 0], // Player 1: 100% A
            ],
            scale: 100,
        };

        let output = motor.evaluate(&input);

        assert!(output.valid);
        // At Nash equilibrium, no player has incentive to deviate
        assert!(output.epsilon_vector.iter().all(|&e| e == 0));
        assert!((output.eta_equilibrium - 1.0).abs() < 1e-10);
        assert!(!output.was_clamped);
    }

    #[test]
    fn test_prisoners_dilemma_equilibrium() {
        let motor = NashMotor::new();

        // Prisoner's Dilemma: (C,C)=(-1,-1), (C,D)=(-3,0), (D,C)=(0,-3), (D,D)=(-2,-2)
        // Both playing D is the unique Nash equilibrium
        let input = NashInput {
            num_players: 2,
            action_sizes: vec![2, 2],
            payoffs: vec![
                vec![-1, -3, 0, -2], // Player 0: C=0, D=1
                vec![-1, 0, -3, -2], // Player 1: C=0, D=1
            ],
            strategies: vec![
                vec![0, 100], // Player 0: 100% Defect
                vec![0, 100], // Player 1: 100% Defect
            ],
            scale: 100,
        };

        let output = motor.evaluate(&input);

        assert!(output.valid);
        // At Nash equilibrium (D,D), no player benefits from switching to C
        assert!(output.epsilon_vector.iter().all(|&e| e == 0));
    }

    #[test]
    fn test_motor_determinism() {
        let motor = NashMotor::new();

        let input = NashInput {
            num_players: 2,
            action_sizes: vec![2, 2],
            payoffs: vec![vec![1, 0, 0, 1], vec![1, 0, 0, 1]],
            strategies: vec![vec![50, 50], vec![50, 50]],
            scale: 100,
        };

        let output1 = motor.evaluate(&input);
        let output2 = motor.evaluate(&input.clone());

        assert!((output1.score - output2.score).abs() < 1e-10);
    }

    #[test]
    fn test_score_bounds() {
        let motor = NashMotor::new();

        let input = NashInput {
            num_players: 2,
            action_sizes: vec![2, 2],
            payoffs: vec![vec![5, -10, -10, 5], vec![5, -10, -10, 5]],
            strategies: vec![vec![75, 25], vec![25, 75]],
            scale: 100,
        };

        let output = motor.evaluate(&input);

        assert!(output.valid);
        assert!(output.score >= 0.0);
        assert!(output.score <= 1.0);
    }
}
