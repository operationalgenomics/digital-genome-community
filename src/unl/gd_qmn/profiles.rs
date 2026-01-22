//! GD-QMN Encoding Profiles

use serde::{Deserialize, Serialize};

/// Compact profile (64 bits) - IoT/sensors
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CompactCode {
    pub family: u8,      // 4 bits
    pub code: u16,       // 12 bits  
    pub amplitude: u16,  // 16 bits (fixed-point)
    pub reserved: u32,   // 32 bits
}

/// Standard profile (128 bits) - Reference
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct StandardCode {
    pub family: u8,         // 8 bits
    pub code: u16,          // 16 bits
    pub amplitude: f32,     // 32 bits
    pub frequency: u32,     // 32 bits
    pub flags: u16,         // 16 bits
    pub checksum: u16,      // 16 bits
    pub reserved: u8,       // 8 bits
}

/// Extended profile (256 bits) - Full fidelity
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ExtendedCode {
    pub family: u8,         // 8 bits
    pub code: u16,          // 16 bits
    pub amplitude: f64,     // 64 bits
    pub frequency: u64,     // 64 bits
    pub phase: f32,         // 32 bits
    pub duration: u32,      // 32 bits
    pub context: u32,       // 32 bits
    pub checksum: u8,       // 8 bits
}

impl CompactCode {
    pub fn encode(&self) -> [u8; 8] {
        let mut buf = [0u8; 8];
        buf[0] = self.family;
        buf[1..3].copy_from_slice(&self.code.to_le_bytes());
        buf[3..5].copy_from_slice(&self.amplitude.to_le_bytes());
        buf[4..8].copy_from_slice(&self.reserved.to_le_bytes());
        buf
    }
}

impl StandardCode {
    pub fn encode(&self) -> [u8; 16] {
        let mut buf = [0u8; 16];
        buf[0] = self.family;
        buf[1..3].copy_from_slice(&self.code.to_le_bytes());
        buf[3..7].copy_from_slice(&self.amplitude.to_le_bytes());
        buf[7..11].copy_from_slice(&self.frequency.to_le_bytes());
        buf[11..13].copy_from_slice(&self.flags.to_le_bytes());
        buf[13..15].copy_from_slice(&self.checksum.to_le_bytes());
        buf[15] = self.reserved;
        buf
    }
}

impl ExtendedCode {
    pub fn encode(&self) -> [u8; 32] {
        let mut buf = [0u8; 32];
        buf[0] = self.family;
        buf[1..3].copy_from_slice(&self.code.to_le_bytes());
        buf[3..11].copy_from_slice(&self.amplitude.to_le_bytes());
        buf[11..19].copy_from_slice(&self.frequency.to_le_bytes());
        buf[19..23].copy_from_slice(&self.phase.to_le_bytes());
        buf[23..27].copy_from_slice(&self.duration.to_le_bytes());
        buf[27..31].copy_from_slice(&self.context.to_le_bytes());
        buf[31] = self.checksum;
        buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compact_size() {
        let c = CompactCode { family: 1, code: 0, amplitude: 0, reserved: 0 };
        assert_eq!(c.encode().len(), 8);
    }

    #[test]
    fn test_standard_size() {
        let s = StandardCode { 
            family: 1, code: 0, amplitude: 0.0, frequency: 0, 
            flags: 0, checksum: 0, reserved: 0 
        };
        assert_eq!(s.encode().len(), 16);
    }

    #[test]
    fn test_extended_size() {
        let e = ExtendedCode {
            family: 1, code: 0, amplitude: 0.0, frequency: 0,
            phase: 0.0, duration: 0, context: 0, checksum: 0
        };
        assert_eq!(e.encode().len(), 32);
    }
}
