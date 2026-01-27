//! --------------------------
//! INFORMATION
//! --------------------------
//! Title: Probability Type (Q-Ready)
//! Author: Carlos Eduardo Favini
//! Date: 2025-01-22
//! Version: 1.0.0
//! Description: Fixed-point probability type for deterministic, Q-Ready
//!              amplitude representation. Normalized to [0, 1].
//! Layer: Community
//! Canon Reference: ROADMAP v2.0.0, Section 4 (MVP-4)
//!
//! --------------------------
//! CHANGE LOG
//! --------------------------
//! 2025-01-22 - Carlos Eduardo Favini - Initial creation (Fase 2 wave-like)
//! --------------------------

use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::{Add, Mul};

/// Fixed-point probability value normalized to [0, 1].
///
/// Internal representation uses u64 where:
/// - `0` represents 0.0
/// - `u64::MAX` represents 1.0
///
/// This provides:
/// - Deterministic arithmetic (no floating-point variance)
/// - Q-Ready compatibility (maps to quantum amplitudes)
/// - Cross-platform reproducibility
///
/// # Canon Reference
/// ROADMAP v2.0.0, Section 4:
/// > "CP ∈ ℝ[0,1] — Compatível com amplitudes quânticas"
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Probability(u64);

impl Probability {
    /// Zero probability (0.0)
    pub const ZERO: Self = Self(0);
    
    /// Full probability (1.0) — multiplicative identity
    pub const ONE: Self = Self(u64::MAX);
    
    /// Half probability (0.5)
    pub const HALF: Self = Self(u64::MAX / 2);

    /// Create from raw u64 value.
    #[inline]
    pub const fn from_raw(raw: u64) -> Self {
        Self(raw)
    }

    /// Get raw u64 value.
    #[inline]
    pub const fn raw(self) -> u64 {
        self.0
    }

    /// Create from f64, clamping to [0, 1].
    ///
    /// Note: Use sparingly — prefer fixed-point operations.
    pub fn from_f64(value: f64) -> Self {
        let clamped = value.clamp(0.0, 1.0);
        Self((clamped * (u64::MAX as f64)) as u64)
    }

    /// Convert to f64.
    ///
    /// Note: Use only for display/debugging — prefer fixed-point.
    pub fn to_f64(self) -> f64 {
        (self.0 as f64) / (u64::MAX as f64)
    }

    /// Create from u32 fixed-point (for Compact profile).
    ///
    /// Maps u32 range to full u64 range.
    #[inline]
    pub const fn from_u32(value: u32) -> Self {
        // Expand u32 to u64 by duplicating high bits
        let expanded = ((value as u64) << 32) | (value as u64);
        Self(expanded)
    }

    /// Convert to u32 fixed-point (for Compact profile).
    ///
    /// Returns high 32 bits.
    #[inline]
    pub const fn to_u32(self) -> u32 {
        (self.0 >> 32) as u32
    }

    /// Saturating multiplication (for combining probabilities).
    ///
    /// This is the correct operation for combining independent probabilities:
    /// P(A and B) = P(A) × P(B)
    #[inline]
    pub fn saturating_mul(self, other: Self) -> Self {
        // Use u128 for intermediate to avoid overflow
        let product = (self.0 as u128) * (other.0 as u128);
        // Normalize back to u64 range
        let normalized = (product >> 64) as u64;
        Self(normalized)
    }

    /// Saturating addition (for superposition).
    ///
    /// Clamps at ONE to maintain [0, 1] invariant.
    #[inline]
    pub fn saturating_add(self, other: Self) -> Self {
        Self(self.0.saturating_add(other.0))
    }

    /// Linear interpolation between two probabilities.
    ///
    /// `lerp(a, b, t)` = a + t × (b - a)
    pub fn lerp(self, other: Self, t: Self) -> Self {
        if self.0 <= other.0 {
            let diff = other.0 - self.0;
            let scaled = ((diff as u128 * t.0 as u128) >> 64) as u64;
            Self(self.0.saturating_add(scaled))
        } else {
            let diff = self.0 - other.0;
            let scaled = ((diff as u128 * t.0 as u128) >> 64) as u64;
            Self(self.0.saturating_sub(scaled))
        }
    }

    /// Check if this is effectively zero (for veto detection).
    #[inline]
    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }

    /// Check if this is effectively one (multiplicative identity).
    #[inline]
    pub const fn is_one(self) -> bool {
        self.0 == u64::MAX
    }
}

impl Default for Probability {
    /// Default amplitude is 1.0 (full intensity).
    fn default() -> Self {
        Self::ONE
    }
}

impl fmt::Debug for Probability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Probability({:.6})", self.to_f64())
    }
}

impl fmt::Display for Probability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.4}", self.to_f64())
    }
}

impl Mul for Probability {
    type Output = Self;
    
    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        self.saturating_mul(rhs)
    }
}

impl Add for Probability {
    type Output = Self;
    
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        self.saturating_add(rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants() {
        assert_eq!(Probability::ZERO.raw(), 0);
        assert_eq!(Probability::ONE.raw(), u64::MAX);
        assert!(Probability::ZERO.is_zero());
        assert!(Probability::ONE.is_one());
    }

    #[test]
    fn test_from_f64_clamping() {
        assert_eq!(Probability::from_f64(-0.5), Probability::ZERO);
        assert_eq!(Probability::from_f64(1.5), Probability::ONE);
        
        let half = Probability::from_f64(0.5);
        let diff = (half.to_f64() - 0.5).abs();
        assert!(diff < 1e-9);
    }

    #[test]
    fn test_multiplication_identity() {
        let p = Probability::from_f64(0.7);
        let result = p * Probability::ONE;
        // Should be approximately equal (within fixed-point precision)
        let diff = (result.to_f64() - p.to_f64()).abs();
        assert!(diff < 1e-9);
    }

    #[test]
    fn test_multiplication_zero() {
        let p = Probability::from_f64(0.7);
        let result = p * Probability::ZERO;
        assert!(result.is_zero());
    }

    #[test]
    fn test_u32_roundtrip() {
        let original: u32 = 0x80000000; // 0.5 in u32 fixed-point
        let prob = Probability::from_u32(original);
        let back = prob.to_u32();
        assert_eq!(original, back);
    }

    #[test]
    fn test_determinism() {
        // Same inputs must produce same outputs
        let a = Probability::from_f64(0.3);
        let b = Probability::from_f64(0.7);
        
        let r1 = a * b;
        let r2 = a * b;
        
        assert_eq!(r1.raw(), r2.raw());
    }

    #[test]
    fn test_saturating_add() {
        let a = Probability::from_f64(0.8);
        let b = Probability::from_f64(0.5);
        let sum = a + b;
        // Should saturate at ONE, not overflow
        assert!(sum.raw() <= Probability::ONE.raw());
    }
}