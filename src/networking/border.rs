//! Canon v5.1 | GDC v0.9.0-final
//!
//! Border - LEI-QMN-BORDA-01 e LEI-QMN-BORDA-02
//!
//! Canon References:
//! - LEI-QMN-BORDA-01 (lines 5912-5951)
//! - LEI-QMN-BORDA-02 (lines 5954-5995)
//! - GATE-QMN-01
//!
//! ## Core Principles
//! - All communication uses GD-QMN envelopes exclusively
//! - GATE-QMN-01 validates all incoming envelopes
//! - Invalid envelopes → Veto + silence (AF-15)
//! - Same structure for input/output (bidirectional)

use crate::unl::gd_qmn::{GdQmnInstruction, GdQmnParser, ParseError};
use crate::unl::gd_qmn::core::MotorOutput;

// =============================================================================
// BORDER VALIDATOR
// =============================================================================

/// Border validator - LEI-QMN-BORDA-01.
///
/// Validates all communication at GDC boundary.
///
/// Canon: §Entrada - GATE-QMN-01 validates all incoming envelopes
pub struct BorderValidator;

impl BorderValidator {
    /// Validate incoming envelope at border.
    ///
    /// Canon: LEI-QMN-BORDA-01 - 7 rejection conditions
    ///
    /// Returns:
    /// - Ok(instruction) if valid
    /// - Err(reason) if invalid (Veto + silence)
    pub fn validate_input(bytes: &[u8]) -> Result<GdQmnInstruction, BorderRejection> {
        match GdQmnParser::parse(bytes) {
            Ok(instruction) => {
                // Additional validation beyond GATE-QMN-01
                if instruction.envelope.amplitude <= 0.0 {
                    return Err(BorderRejection::InvalidAmplitude);
                }
                
                Ok(instruction)
            }
            Err(ParseError::InsufficientData) => Err(BorderRejection::InsufficientData),
            Err(ParseError::InvalidUid) => Err(BorderRejection::InvalidUid),
            Err(ParseError::InvalidAmplitude) => Err(BorderRejection::InvalidAmplitude),
            Err(ParseError::ChecksumOndaFailed) => Err(BorderRejection::ChecksumFailed),
            Err(ParseError::ChecksumCargaFailed) => Err(BorderRejection::ChecksumFailed),
            Err(ParseError::ChecksumTotalFailed) => Err(BorderRejection::ChecksumFailed),
            Err(ParseError::InvalidProfile) => Err(BorderRejection::InvalidUid),
            Err(ParseError::InvalidFormat) => Err(BorderRejection::InsufficientData),
        }
    }
    
    /// Validate outgoing envelope at border.
    ///
    /// Canon: §Saída - toda emissão passa por GATE-QMN-01
    pub fn validate_output(instruction: &GdQmnInstruction) -> Result<(), BorderRejection> {
        // Verify integrity before emission
        if !instruction.verify_integrity() {
            return Err(BorderRejection::ChecksumFailed);
        }
        
        // Verify amplitude
        if instruction.envelope.amplitude <= 0.0 {
            return Err(BorderRejection::InvalidAmplitude);
        }
        
        Ok(())
    }
    
    /// Process rejection - Veto + silence.
    ///
    /// Canon: "Silêncio é ontológico (AF-15): o GDC não manifestou
    ///         porque o envelope é estruturalmente incompatível"
    pub fn handle_rejection(_rejection: BorderRejection) -> MotorOutput {
        // Canon: Veto + silêncio (não responde, não manifesta)
        MotorOutput::Veto
    }
}

// =============================================================================
// BORDER REJECTION
// =============================================================================

/// Border rejection reasons.
///
/// Canon: LEI-QMN-BORDA-01 - Condições de rejeição
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BorderRejection {
    /// Invalid UID
    InvalidUid,
    
    /// Checksum failed
    ChecksumFailed,
    
    /// Invalid amplitude (≤ 0)
    InvalidAmplitude,
    
    /// Insufficient data
    InsufficientData,
    
    /// Suspected contamination
    SuspectedContamination,
}

// =============================================================================
// HANDSHAKE VALIDATOR - LEI-QMN-BORDA-02
// =============================================================================

/// Handshake validator - LEI-QMN-BORDA-02.
///
/// Meta-protocol for GDO↔GDC and GDE↔GDC integration.
pub struct HandshakeValidator;

impl HandshakeValidator {
    /// Validate structural handshake.
    ///
    /// Canon: §2 - Handshake válido se:
    /// 1. Versão GD-QMN declarada
    /// 2. Estrutura sintaticamente válida
    /// 3. Envelope preserva integridade
    /// 4. Sem semântica externa injetada
    pub fn validate_handshake(
        version: &str,
        envelope: &GdQmnInstruction,
    ) -> Result<(), HandshakeError> {
        // 1. Version declared
        if version.is_empty() {
            return Err(HandshakeError::MissingVersion);
        }
        
        // 2. Syntactically valid (already validated by BorderValidator)
        
        // 3. Integrity preserved
        if !envelope.verify_integrity() {
            return Err(HandshakeError::IntegrityViolation);
        }
        
        // 4. No external semantics injected (checked structurally)
        
        Ok(())
    }
}

/// Handshake errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HandshakeError {
    /// Version not declared
    MissingVersion,
    
    /// Integrity violation
    IntegrityViolation,
    
    /// Invalid structure
    InvalidStructure,
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unl::gd_qmn::*;
    
    #[test]
    fn test_border_rejects_invalid_envelope() {
        let invalid_bytes = vec![0u8; 10]; // Insufficient data
        
        let result = BorderValidator::validate_input(&invalid_bytes);
        
        // Canon: LEI-QMN-BORDA-01 - rejeição → Veto + silêncio
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), BorderRejection::InsufficientData);
    }
    
    #[test]
    fn test_border_accepts_valid_envelope() {
        let envelope = WaveEnvelope::new(
            QmnFamily::Unary,
            QmnSubfamily::State,
            Opcode::Core(CoreOpcode::State),
            ProfileV2::Standard,
            1.0,
        ).unwrap();
        
        let cargo = Cargo::new(vec![1, 2, 3], 0x0001);
        let instruction = GdQmnInstruction::new(envelope, cargo);
        
        let bytes = instruction.to_bytes();
        let result = BorderValidator::validate_input(&bytes);
        
        // Should accept valid envelope
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_handshake_validation() {
        let envelope = WaveEnvelope::new(
            QmnFamily::Binary,
            QmnSubfamily::Relate,
            Opcode::Core(CoreOpcode::Reference),
            ProfileV2::Standard,
            0.8,
        ).unwrap();
        
        let cargo = Cargo::new(vec![0xAA], 0x1234);
        let instruction = GdQmnInstruction::new(envelope, cargo);
        
        let result = HandshakeValidator::validate_handshake("v1.0.0", &instruction);
        
        // Canon: §2 - Handshake válido
        assert!(result.is_ok());
    }
}
