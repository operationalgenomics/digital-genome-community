//! GD-QMN: Genoma Digital Quantum Mnemonic Notation
//!
//! Wave-like encoding implementing UNL specification.

pub mod profiles;
pub mod families;

pub use profiles::{CompactCode, StandardCode, ExtendedCode};

use serde::{Deserialize, Serialize};

/// A GD-QMN code point.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GdQmn {
    pub family: Family,
    pub code: u16,
    pub profile: Profile,
}

/// Encoding profile (bit width).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Profile {
    /// 64 bits - IoT, sensors, minimal
    Compact,
    /// 128 bits - Standard reference
    Standard,
    /// 256 bits - Phase, duration, extended
    Extended,
}

impl Profile {
    pub fn bits(&self) -> u16 {
        match self {
            Self::Compact => 64,
            Self::Standard => 128,
            Self::Extended => 256,
        }
    }

    pub fn bytes(&self) -> usize {
        (self.bits() as usize) / 8
    }
}

/// Code family (F1-F6).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum Family {
    F1Transduction = 1,
    F2Composition = 2,
    F3Motors = 3,
    F4Emission = 4,
    F5Scale = 5,
    F6Operational = 6,
}

impl GdQmn {
    pub fn new(family: Family, code: u16, profile: Profile) -> Self {
        Self { family, code, profile }
    }

    /// Returns full code as u32 (family << 16 | code).
    pub fn full_code(&self) -> u32 {
        ((self.family as u32) << 16) | (self.code as u32)
    }
}

impl Family {
    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            1 => Some(Self::F1Transduction),
            2 => Some(Self::F2Composition),
            3 => Some(Self::F3Motors),
            4 => Some(Self::F4Emission),
            5 => Some(Self::F5Scale),
            6 => Some(Self::F6Operational),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_bits() {
        assert_eq!(Profile::Compact.bits(), 64);
        assert_eq!(Profile::Standard.bits(), 128);
        assert_eq!(Profile::Extended.bits(), 256);
    }

    #[test]
    fn test_gd_qmn_full_code() {
        let q = GdQmn::new(Family::F6Operational, 0x0001, Profile::Standard);
        assert_eq!(q.full_code(), 0x00060001);
    }
}
