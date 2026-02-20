//! --------------------------
//! INFORMATION
//! --------------------------
//! Title: GD-QMN: Genoma Digital Quantum Mnemonic Notation
//! Author: Claude B (sob supervisão de Carlos Eduardo Favini)
//! Date: 2026-01-29
//! Version: 0.8.0
//! Description: Wave-like encoding implementing UNL specification.
//!              Módulo central da notação quântica mnemônica.
//!
//! Canon References:
//!   - DLB-008: "GD-QMN como dinâmica do sentido"
//!   - DLB-009: "Três perfis (Compact/Standard/Extended)"
//!   - DLB-019 a DLB-024: Estruturas fundamentais
//!
//! --------------------------
//! CHANGE LOG
//! --------------------------
//! 2026-01-29 - Claude B - v0.8.0: Novos módulos core e profiles_v2
//! --------------------------

// Módulos v0.8.0 (novos)
pub mod core;
pub mod profiles;
pub mod instruction;
pub mod parser;
pub mod executor;

// Módulos legados (compatibilidade v0.7.x)
pub mod families;

// Re-exports v0.8.0 (preferidos)
pub use core::{
    MotorOutput, SyncFailure,
    QmnFamily, QmnSubfamily,
    CoreOpcode, WaveOpcode, Opcode,
    Cargo,
};

pub use profiles::{
    CompactCode as CompactCodeV2,
    StandardCode as StandardCodeV2,
    ExtendedCode as ExtendedCodeV2,
};

pub use instruction::{
    GdQmnInstruction,
    WaveEnvelope,
    Profile as ProfileV2,
    EffectMode,
};

pub use parser::{
    GdQmnParser,
    ParseError,
};

pub use executor::{
    GdQmnExecutor,
    ExecutionContext,
};

// Re-exports legados (compatibilidade)
pub use profiles::{CompactCode, StandardCode, ExtendedCode};

use serde::{Deserialize, Serialize};

/// A GD-QMN code point (legado, compatibilidade).
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

/// Code family (legado F1-F6, mantido para compatibilidade).
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

    // Testes v0.8.0
    #[test]
    fn test_motor_output_veto() {
        let v = MotorOutput::Veto;
        assert!(v.is_veto());
    }

    #[test]
    fn test_qmn_family_cardinality() {
        assert_eq!(QmnFamily::Binary.cardinality(), Some(2));
    }

    #[test]
    fn test_cargo_integrity() {
        let c = Cargo::new(vec![1, 2, 3], 0);
        assert!(c.verify());
    }
}
