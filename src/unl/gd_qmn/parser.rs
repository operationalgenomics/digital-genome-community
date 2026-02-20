//! --------------------------
//! INFORMATION
//! --------------------------
//! Title: GD-QMN Parser (v0.8.0)
//! Author: Claude (sob supervisão de Carlos Eduardo Favini)
//! Date: 2026-02-15
//! Version: 0.8.0
//! Description: Parser de bytecode hexadecimal GD-QMN conforme Canon v5.1.
//!              Implementa GATE-QMN-01 (validação de fronteira).
//!
//! Canon References:
//!   - LEI-QMN-BORDA-01: Fronteira Estrutural do GDC
//!   - LEI-QMN-SERIAL-01: Serialização Canônica
//!   - LEI-QMN-INTEGRIDADE-TRIPLA-01: Verificação Tripla
//!   - GATE-QMN-01: Validação de Envelope
//!
//! --------------------------
//! CHANGE LOG
//! --------------------------
//! 2026-02-15 - Claude - Criação inicial (v0.8.0)
//! --------------------------

use super::instruction::{GdQmnInstruction, WaveEnvelope, Profile};
use super::core::{QmnFamily, QmnSubfamily, Opcode, Cargo, MotorOutput};

// =============================================================================
// ERROS DE PARSING
// =============================================================================

/// Erros que podem ocorrer durante parsing.
///
/// Todos erros de parsing resultam em Veto na fronteira (LEI-QMN-BORDA-01).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    /// Dados insuficientes
    InsufficientData,
    
    /// UID inválido (família, subfamily ou opcode não reconhecidos)
    InvalidUid,
    
    /// Amplitude inválida (≤ 0 ou NaN)
    InvalidAmplitude,
    
    /// Checksum do envelope falhou
    ChecksumOndaFailed,
    
    /// Checksum da carga falhou
    ChecksumCargaFailed,
    
    /// Checksum total falhou
    ChecksumTotalFailed,
    
    /// Perfil não reconhecido
    InvalidProfile,
    
    /// Formato de bytes inválido
    InvalidFormat,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InsufficientData => write!(f, "Insufficient data"),
            Self::InvalidUid => write!(f, "Invalid UID"),
            Self::InvalidAmplitude => write!(f, "Invalid amplitude"),
            Self::ChecksumOndaFailed => write!(f, "Envelope checksum failed"),
            Self::ChecksumCargaFailed => write!(f, "Cargo checksum failed"),
            Self::ChecksumTotalFailed => write!(f, "Total checksum failed"),
            Self::InvalidProfile => write!(f, "Invalid profile"),
            Self::InvalidFormat => write!(f, "Invalid format"),
        }
    }
}

impl std::error::Error for ParseError {}

impl ParseError {
    /// Colapsa erro para Veto (LEI-QMN-BORDA-01).
    pub fn to_motor_output(&self) -> MotorOutput {
        MotorOutput::Veto
    }
}

// =============================================================================
// PARSER GD-QMN
// =============================================================================

/// Parser de bytecode GD-QMN.
///
/// Implementa GATE-QMN-01 com validação tripla de integridade.
pub struct GdQmnParser;

impl GdQmnParser {
    /// Parse bytecode em instrução GD-QMN.
    ///
    /// Implementa GATE-QMN-01 (LEI-QMN-BORDA-01):
    /// - Valida UID
    /// - Valida amplitude > 0
    /// - Valida checksums triplos
    /// - Retorna Err(ParseError) em caso de falha → Veto + silêncio
    pub fn parse(bytes: &[u8]) -> Result<GdQmnInstruction, ParseError> {
        if bytes.len() < 40 {
            // Mínimo: envelope (35 bytes) + checksums (12 bytes) + tamanho cargo (4 bytes)
            return Err(ParseError::InsufficientData);
        }
        
        let mut offset = 0;
        
        // Parse envelope
        let envelope = Self::parse_envelope(&bytes[offset..], &mut offset)?;
        
        // Parse checksums (3 × 4 bytes = 12 bytes)
        let checksum_onda = u32::from_be_bytes([
            bytes[offset], bytes[offset+1], bytes[offset+2], bytes[offset+3]
        ]);
        offset += 4;
        
        let checksum_carga = u32::from_be_bytes([
            bytes[offset], bytes[offset+1], bytes[offset+2], bytes[offset+3]
        ]);
        offset += 4;
        
        let checksum_total = u32::from_be_bytes([
            bytes[offset], bytes[offset+1], bytes[offset+2], bytes[offset+3]
        ]);
        offset += 4;
        
        // Parse tamanho do cargo
        if offset + 4 > bytes.len() {
            return Err(ParseError::InsufficientData);
        }
        
        let cargo_size = u32::from_be_bytes([
            bytes[offset], bytes[offset+1], bytes[offset+2], bytes[offset+3]
        ]) as usize;
        offset += 4;
        
        // Parse payload
        if offset + cargo_size > bytes.len() {
            return Err(ParseError::InsufficientData);
        }
        
        let payload = bytes[offset..offset + cargo_size].to_vec();
        offset += cargo_size;
        
        // Parse schema hint
        if offset + 2 > bytes.len() {
            return Err(ParseError::InsufficientData);
        }
        
        let schema_hint = u16::from_be_bytes([bytes[offset], bytes[offset+1]]);
        
        // Criar Cargo
        let cargo = Cargo::new(payload, schema_hint);
        
        // Criar instrução
        let mut instruction = GdQmnInstruction::new(envelope, cargo);
        
        // Sobrescrever checksums com valores do bytecode
        instruction.checksum_onda = checksum_onda;
        instruction.checksum_carga = checksum_carga;
        instruction.checksum_total = checksum_total;
        
        // GATE-QMN-01: Validação tripla
        if !instruction.verify_integrity() {
            // Determinar qual checksum falhou
            if instruction.envelope.checksum() != checksum_onda {
                return Err(ParseError::ChecksumOndaFailed);
            } else if crc32_checksum(&instruction.cargo.payload) != checksum_carga {
                return Err(ParseError::ChecksumCargaFailed);
            } else {
                return Err(ParseError::ChecksumTotalFailed);
            }
        }
        
        Ok(instruction)
    }
    
    /// Parse envelope a partir de bytes.
    fn parse_envelope(bytes: &[u8], offset: &mut usize) -> Result<WaveEnvelope, ParseError> {
        if bytes.len() < 35 {
            return Err(ParseError::InsufficientData);
        }
        
        let mut pos = 0;
        
        // Family (1 byte)
        let family = QmnFamily::from_cardinality(bytes[pos]);
        pos += 1;
        
        // Subfamily (1 byte)
        let subfamily = match bytes[pos] {
            0x01 => QmnSubfamily::State,
            0x02 => QmnSubfamily::Relate,
            0x03 => QmnSubfamily::Derive,
            0x04 => QmnSubfamily::Meta,
            _ => return Err(ParseError::InvalidUid),
        };
        pos += 1;
        
        // Opcode (1 byte)
        let opcode = Opcode::from_u8(bytes[pos])
            .ok_or(ParseError::InvalidUid)?;
        pos += 1;
        
        // Profile (1 byte)
        let profile = match bytes[pos] {
            0x01 => Profile::Compact,
            0x02 => Profile::Standard,
            0x03 => Profile::Extended,
            _ => return Err(ParseError::InvalidProfile),
        };
        pos += 1;
        
        // Amplitude (8 bytes, big-endian)
        let amplitude = f64::from_be_bytes([
            bytes[pos], bytes[pos+1], bytes[pos+2], bytes[pos+3],
            bytes[pos+4], bytes[pos+5], bytes[pos+6], bytes[pos+7],
        ]);
        pos += 8;
        
        // GATE-QMN-01: Validar amplitude > 0
        if amplitude <= 0.0 || amplitude.is_nan() {
            return Err(ParseError::InvalidAmplitude);
        }
        
        // Frequency (8 bytes ou 0xFF... se None)
        let frequency = if bytes[pos..pos+8] == [0xFF; 8] {
            None
        } else {
            Some(u64::from_be_bytes([
                bytes[pos], bytes[pos+1], bytes[pos+2], bytes[pos+3],
                bytes[pos+4], bytes[pos+5], bytes[pos+6], bytes[pos+7],
            ]))
        };
        pos += 8;
        
        // Phase (4 bytes ou 0xFF... se None)
        let phase = if bytes[pos..pos+4] == [0xFF; 4] {
            None
        } else {
            Some(f32::from_be_bytes([
                bytes[pos], bytes[pos+1], bytes[pos+2], bytes[pos+3],
            ]))
        };
        pos += 4;
        
        // Duration (4 bytes ou 0xFF... se None)
        let duration = if bytes[pos..pos+4] == [0xFF; 4] {
            None
        } else {
            Some(u32::from_be_bytes([
                bytes[pos], bytes[pos+1], bytes[pos+2], bytes[pos+3],
            ]))
        };
        pos += 4;
        
        // Context (4 bytes ou 0xFF... se None)
        let context = if bytes[pos..pos+4] == [0xFF; 4] {
            None
        } else {
            Some(u32::from_be_bytes([
                bytes[pos], bytes[pos+1], bytes[pos+2], bytes[pos+3],
            ]))
        };
        pos += 4;
        
        // Flags (2 bytes, big-endian)
        let flags = u16::from_be_bytes([bytes[pos], bytes[pos+1]]);
        pos += 2;
        
        *offset += pos;
        
        Ok(WaveEnvelope {
            family,
            subfamily,
            opcode,
            profile,
            amplitude,
            frequency,
            phase,
            duration,
            context,
            flags,
        })
    }
    
    /// Parse de string hexadecimal (ex: "01020304...").
    pub fn parse_hex(hex: &str) -> Result<GdQmnInstruction, ParseError> {
        let hex_clean = hex.chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>();
        
        let bytes = hex::decode(&hex_clean)
            .map_err(|_| ParseError::InvalidFormat)?;
        
        Self::parse(&bytes)
    }
}

/// Função auxiliar CRC32 (duplicada temporariamente, será consolidada).
fn crc32_checksum(data: &[u8]) -> u32 {
    const POLYNOMIAL: u32 = 0xEDB88320;
    let mut crc: u32 = 0xFFFFFFFF;
    
    for &byte in data {
        crc ^= byte as u32;
        for _ in 0..8 {
            if crc & 1 != 0 {
                crc = (crc >> 1) ^ POLYNOMIAL;
            } else {
                crc >>= 1;
            }
        }
    }
    
    !crc
}

// =============================================================================
// TESTES
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::core::CoreOpcode;
    
    #[test]
    fn test_parse_valid_instruction() {
        // Criar instrução válida
        let envelope = WaveEnvelope::new(
            QmnFamily::Unary,
            QmnSubfamily::State,
            Opcode::Core(CoreOpcode::State),
            Profile::Standard,
            1.0,
        ).unwrap();
        
        let cargo = Cargo::new(vec![1, 2, 3, 4], 0x0001);
        let instruction = GdQmnInstruction::new(envelope, cargo);
        
        // Serializar
        let bytes = instruction.to_bytes();
        
        // Parse
        let parsed = GdQmnParser::parse(&bytes).unwrap();
        
        // Verificar
        assert_eq!(parsed.uid(), instruction.uid());
        assert_eq!(parsed.cargo.payload, instruction.cargo.payload);
    }
    
    #[test]
    fn test_parse_insufficient_data() {
        let bytes = vec![1, 2, 3]; // Muito curto
        let result = GdQmnParser::parse(&bytes);
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ParseError::InsufficientData);
    }
    
    #[test]
    fn test_parse_invalid_amplitude() {
        // Criar envelope com amplitude inválida manualmente
        let mut bytes = vec![0; 100];
        bytes[0] = 0x01; // Family
        bytes[1] = 0x01; // Subfamily
        bytes[2] = 0x01; // Opcode
        bytes[3] = 0x02; // Profile
        // bytes[4..12] = amplitude = 0.0 (inválido)
        
        let result = GdQmnParser::parse(&bytes);
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ParseError::InvalidAmplitude);
    }
    
    #[test]
    fn test_parse_checksum_tampering() {
        // Criar instrução válida
        let envelope = WaveEnvelope::new(
            QmnFamily::Unary,
            QmnSubfamily::State,
            Opcode::Core(CoreOpcode::State),
            Profile::Standard,
            1.0,
        ).unwrap();
        
        let cargo = Cargo::new(vec![1, 2, 3, 4], 0x0001);
        let instruction = GdQmnInstruction::new(envelope, cargo);
        
        // Serializar e tamper
        let mut bytes = instruction.to_bytes();
        bytes[50] ^= 0xFF; // Tamper com payload
        
        // Parse deve detectar
        let result = GdQmnParser::parse(&bytes);
        
        assert!(result.is_err());
        // Deve ser um dos erros de checksum
        match result.unwrap_err() {
            ParseError::ChecksumCargaFailed | 
            ParseError::ChecksumTotalFailed => {},
            e => panic!("Expected checksum error, got {:?}", e),
        }
    }
}
