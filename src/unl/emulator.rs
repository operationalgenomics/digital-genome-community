//! GDO Emulator - Minimal observer for testing GDC
//!
//! This emulator simulates the GDO (Genoma Digital Observador) for
//! testing purposes. It creates PerceptualFrames and sends them to GDC.

use crate::cognitive::{TransportCode, ObservationReport, MotorSignatures};
use crate::sensory::{RawInput, SensoryCortex};
use sha2::{Sha256, Digest};

/// Minimal GDO emulator for testing.
pub struct GdoEmulator {
    cycle_counter: u64,
}

/// A framed perception ready for GDC.
#[derive(Debug, Clone)]
pub struct PerceptualFrame {
    pub frame_id: [u8; 16],
    pub bof: TransportCode,
    pub payload: Vec<u8>,
    pub eof: TransportCode,
    pub checksum: [u8; 32],
}

impl GdoEmulator {
    pub fn new() -> Self {
        Self { cycle_counter: 0 }
    }

    /// Creates a framed perception from raw bytes.
    pub fn frame(&mut self, data: &[u8]) -> PerceptualFrame {
        self.cycle_counter += 1;
        
        let mut frame_id = [0u8; 16];
        frame_id[8..].copy_from_slice(&self.cycle_counter.to_le_bytes());

        let checksum = Self::hash(data);

        PerceptualFrame {
            frame_id,
            bof: TransportCode::BOF,
            payload: data.to_vec(),
            eof: TransportCode::EOF,
            checksum,
        }
    }

    /// Sends frame to GDC and receives report.
    pub fn observe(&mut self, data: &[u8]) -> ObservationReport {
        let frame = self.frame(data);
        let cortex = SensoryCortex::new();
        
        // Warm-up (determinism)
        let _ = cortex.perceive(&RawInput::from_bytes(vec![0u8; 64]));
        
        // Process
        let output = cortex.perceive(&RawInput::from_bytes(frame.payload.clone()));
        
        ObservationReport {
            cycle_id: frame.frame_id,
            frame_fingerprint: frame.checksum,
            protocol_markers: vec![frame.bof, frame.eof],
            motor_signatures: MotorSignatures::empty(), // Placeholder
            dna_fingerprint: Self::hash(output.signals.entropy.to_le_bytes().as_slice()),
        }
    }

    fn hash(data: &[u8]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finalize().into()
    }
}

impl Default for GdoEmulator {
    fn default() -> Self {
        Self::new()
    }
}

impl PerceptualFrame {
    /// Returns true if frame has valid delimiters.
    pub fn is_valid(&self) -> bool {
        self.bof == TransportCode::BOF && self.eof == TransportCode::EOF
    }

    /// Returns payload size.
    pub fn payload_size(&self) -> usize {
        self.payload.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frame_creation() {
        let mut emu = GdoEmulator::new();
        let frame = emu.frame(&[1, 2, 3]);
        assert!(frame.is_valid());
        assert_eq!(frame.payload_size(), 3);
    }

    #[test]
    fn test_observe_returns_report() {
        let mut emu = GdoEmulator::new();
        let report = emu.observe(&[1, 2, 3, 4, 5]);
        assert_eq!(report.protocol_markers.len(), 2);
    }

    #[test]
    fn test_cycle_counter_increments() {
        let mut emu = GdoEmulator::new();
        let f1 = emu.frame(&[1]);
        let f2 = emu.frame(&[2]);
        assert_ne!(f1.frame_id, f2.frame_id);
    }
}
