//! Canon v5.1 | GDC v0.9.0-final
//!
//! Protocol - Wire protocol entre GDCs
//!
//! Canon: LEI-AO-24-04 (Agnosticismo Estrutural de Rede)
//!
//! ## Core Principle
//! Protocol is structural, not technological.
//! Transport layer never influences cognitive layers 1/2.

use crate::unl::gd_qmn::{GdQmnInstruction, GdQmnParser};
use serde::{Deserialize, Serialize};

/// Message types in GDC↔GDC protocol.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProtocolMessage {
    /// Work distribution (Queen → Worker)
    WorkDistribution {
        instruction: Vec<u8>,
        work_id: String,
    },
    
    /// EDR return (Worker → Queen)
    EdrReturn {
        instruction: Vec<u8>,
        work_id: String,
    },
    
    /// Vibration of necessity (Queen → Workers)
    VibrationNeed {
        need_type: String,
    },
}

/// Protocol encoder/decoder.
pub struct ProtocolCodec;

impl ProtocolCodec {
    /// Encode instruction to wire format.
    pub fn encode(instruction: &GdQmnInstruction) -> Vec<u8> {
        instruction.to_bytes()
    }
    
    /// Decode wire format to instruction.
    pub fn decode(bytes: &[u8]) -> Result<GdQmnInstruction, String> {
        GdQmnParser::parse(bytes)
            .map_err(|e| format!("Decode error: {:?}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unl::gd_qmn::*;
    
    #[test]
    fn test_protocol_roundtrip() {
        let envelope = WaveEnvelope::new(
            QmnFamily::Unary,
            QmnSubfamily::State,
            Opcode::Core(CoreOpcode::State),
            ProfileV2::Standard,
            1.0,
        ).unwrap();
        
        let cargo = Cargo::new(vec![1, 2, 3], 0x0001);
        let instruction = GdQmnInstruction::new(envelope, cargo);
        
        let encoded = ProtocolCodec::encode(&instruction);
        let decoded = ProtocolCodec::decode(&encoded).unwrap();
        
        assert_eq!(instruction.to_bytes(), decoded.to_bytes());
    }
}
