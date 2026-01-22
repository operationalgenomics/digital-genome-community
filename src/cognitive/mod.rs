//! Cognitive Module - GDC↔GDO Communication Structures
//!
//! # Canon References
//! - L-003: PerceptualFrame (GDO responsibility, documented here)
//! - L-004: ObservationReport (5 fields, implemented here)
//! - L-008: GD-QMN Transport (BOF/EOF family)

mod cycle;

pub use cycle::{CognitiveCycle, CycleOutput, MotorScores, MotorContext};

use serde::{Deserialize, Serialize};

/// Technical certificate of cognitive cycle (L-004).
/// 
/// Exactly 5 fields, non-semantic, non-interpretive.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ObservationReport {
    /// 1. Unique cycle ID for correlation
    pub cycle_id: [u8; 16],
    /// 2. SHA-256 of received PerceptualFrame
    pub frame_fingerprint: [u8; 32],
    /// 3. GD-QMN markers used (BOF, EOF, etc.)
    pub protocol_markers: Vec<TransportCode>,
    /// 4. Motor output signatures (4 hashes)
    pub motor_signatures: MotorSignatures,
    /// 5. SHA-256 of emitted DNA
    pub dna_fingerprint: [u8; 32],
}

/// Motor vector signatures (hashes, not values).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MotorSignatures {
    pub praxis: [u8; 32],
    pub nash: [u8; 32],
    pub chaos: [u8; 32],
    pub meristic: [u8; 32],
}

impl MotorSignatures {
    pub fn empty() -> Self {
        Self { praxis: [0; 32], nash: [0; 32], chaos: [0; 32], meristic: [0; 32] }
    }
}

/// GD-QMN Transport codes (L-008, Family F6).
/// 
/// > "Without BOF/EOF, no valid stimulus. With BOF/EOF, GDC always reacts."
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum TransportCode {
    BOF = 0x0001,      // Begin Of File
    EOF = 0x0002,      // End Of File  
    BOFR = 0x0003,     // Begin Of Fragment
    EOFR = 0x0004,     // End Of Fragment
    VERSION = 0x0010,
    CHECKSUM = 0x0011,
}

impl TransportCode {
    pub fn value(self) -> u16 { self as u16 }
    
    pub fn from_value(v: u16) -> Option<Self> {
        match v {
            0x0001 => Some(Self::BOF),
            0x0002 => Some(Self::EOF),
            0x0003 => Some(Self::BOFR),
            0x0004 => Some(Self::EOFR),
            0x0010 => Some(Self::VERSION),
            0x0011 => Some(Self::CHECKSUM),
            _ => None,
        }
    }
    
    pub fn is_delimiter(self) -> bool {
        matches!(self, Self::BOF | Self::EOF | Self::BOFR | Self::EOFR)
    }
}

// PerceptualFrame: Lives in GDO, not GDC. BOF/EOF delimit its transmission.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_observation_report_has_5_fields() {
        let r = ObservationReport {
            cycle_id: [0; 16],
            frame_fingerprint: [0; 32],
            protocol_markers: vec![TransportCode::BOF, TransportCode::EOF],
            motor_signatures: MotorSignatures::empty(),
            dna_fingerprint: [0; 32],
        };
        assert_eq!(r.protocol_markers.len(), 2);
    }

    #[test]
    fn test_transport_code_roundtrip() {
        assert_eq!(TransportCode::from_value(0x0001), Some(TransportCode::BOF));
        assert!(TransportCode::BOF.is_delimiter());
    }
}
