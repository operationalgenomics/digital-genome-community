//! --------------------------
//! INFORMATION
//! --------------------------
//! Title: GD-QMN Instruction Structure (v0.8.0)
//! Author: Claude (sob supervisão de Carlos Eduardo Favini)
//! Date: 2026-02-15
//! Version: 0.8.0
//! Description: Estrutura completa de instrução GD-QMN conforme Canon v5.1.
//!              Implementa LEI-QMN-SEPARACAO-01, LEI-QMN-SERIAL-01, LEI-QMN-INTEGRIDADE-TRIPLA-01.
//!
//! Canon References:
//!   - AO-QMN-01: Núcleo Estrutural Executável
//!   - LEI-QMN-ID-01: Identidade Estrutural
//!   - LEI-QMN-PROFILE-01: Perfis de Representação
//!   - LEI-QMN-SEPARACAO-01: Disjunção Onda/Carga
//!   - LEI-QMN-SERIAL-01: Serialização Canônica
//!   - LEI-QMN-INTEGRIDADE-TRIPLA-01: Checksums em 3 camadas
//!   - LEI-QMN-AMP-01: Amplitude > 0 sempre
//!   - LEI-QMN-VETO-01: Veto ontológico
//!
//! --------------------------
//! CHANGE LOG
//! --------------------------
//! 2026-02-15 - Claude - Criação inicial (v0.8.0)
//! --------------------------

use serde::{Deserialize, Serialize};
use super::core::{QmnFamily, QmnSubfamily, Opcode, Cargo};

// =============================================================================
// MODO DE EFEITO (MODE)
// =============================================================================

/// Natureza do efeito da instrução (LEI-QMN-MODE-01).
///
/// Mode é atributo canônico, imutável, não transportado no bytecode.
/// Inferido pelo MVE (Motor Vibracional Executável) a partir do UID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EffectMode {
    /// Transforma estado existente
    Transformative,
    /// Define estrutura (sem transformação de conteúdo)
    Structural,
    /// Estabelece relações entre estados
    Relational,
    /// Inibe ou suprime (veto controlado)
    Inhibitory,
}

// =============================================================================
// PERFIS GD-QMN (LEI-QMN-PROFILE-01)
// =============================================================================

/// Perfil de representação GD-QMN.
///
/// Três perfis obrigatórios em v1.0.0:
/// - Compact: mínimo (IoT, sensores)
/// - Standard: balanceado (referência)
/// - Extended: completo (análise, auditoria)
///
/// Canon: LEI-QMN-PROFILE-01
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum Profile {
    Compact = 0x01,
    Standard = 0x02,
    Extended = 0x03,
}

// =============================================================================
// ENVELOPE (ONDA) - LEI-QMN-SEPARACAO-01
// =============================================================================

/// Envelope wave-like (Onda) da instrução.
///
/// Governa comportamento cognitivo, não conteúdo.
/// Separado da carga (payload) por LEI-QMN-SEPARACAO-01.
///
/// Campos validados por checksum_onda (LEI-QMN-INTEGRIDADE-TRIPLA-01 §I).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WaveEnvelope {
    /// Cardinalidade (Family)
    pub family: QmnFamily,
    
    /// Classe operacional (Subfamily)
    pub subfamily: QmnSubfamily,
    
    /// Opcode dentro da subfamily
    pub opcode: Opcode,
    
    /// Perfil de representação
    pub profile: Profile,
    
    /// Amplitude (sempre > 0, LEI-QMN-AMP-01)
    pub amplitude: f64,
    
    /// Frequência (opcional, wave opcodes)
    pub frequency: Option<u64>,
    
    /// Fase (opcional, wave opcodes)
    pub phase: Option<f32>,
    
    /// Duração (opcional)
    pub duration: Option<u32>,
    
    /// Contexto (opcional)
    pub context: Option<u32>,
    
    /// Flags (opcional)
    pub flags: u16,
}

impl WaveEnvelope {
    /// Cria novo envelope com validação.
    pub fn new(
        family: QmnFamily,
        subfamily: QmnSubfamily,
        opcode: Opcode,
        profile: Profile,
        amplitude: f64,
    ) -> Result<Self, &'static str> {
        // LEI-QMN-AMP-01: Amplitude > 0 sempre
        if amplitude <= 0.0 {
            return Err("Amplitude must be > 0 (LEI-QMN-AMP-01)");
        }
        
        Ok(Self {
            family,
            subfamily,
            opcode,
            profile,
            amplitude,
            frequency: None,
            phase: None,
            duration: None,
            context: None,
            flags: 0,
        })
    }
    
    /// Retorna UID <Family, Subfamily, Opcode> (LEI-QMN-ID-01).
    pub fn uid(&self) -> (QmnFamily, QmnSubfamily, Opcode) {
        (self.family, self.subfamily, self.opcode)
    }
    
    /// Calcula checksum_onda (LEI-QMN-INTEGRIDADE-TRIPLA-01 §I).
    ///
    /// Usa CRC32 sobre bytes canônicos do envelope.
    /// Não usa SHA-256 (proibido por Canon).
    pub fn checksum(&self) -> u32 {
        let bytes = self.serialize_canonical();
        crc32_checksum(&bytes)
    }
    
    /// Serializa envelope em forma canônica (LEI-QMN-SERIAL-01).
    ///
    /// Determinístico, independente de plataforma.
    fn serialize_canonical(&self) -> Vec<u8> {
        let mut bytes = vec![
            self.family as u8,
            self.subfamily as u8,
            self.opcode.as_u8(),
            self.profile as u8,
        ];
        
        // Amplitude (8 bytes, big-endian para canonicidade)
        bytes.extend_from_slice(&self.amplitude.to_be_bytes());
        
        // Frequency (8 bytes ou 0xFF... se None)
        match self.frequency {
            Some(f) => bytes.extend_from_slice(&f.to_be_bytes()),
            None => bytes.extend_from_slice(&[0xFF; 8]),
        }
        
        // Phase (4 bytes ou 0xFF... se None)
        match self.phase {
            Some(p) => bytes.extend_from_slice(&p.to_be_bytes()),
            None => bytes.extend_from_slice(&[0xFF; 4]),
        }
        
        // Duration (4 bytes ou 0xFF... se None)
        match self.duration {
            Some(d) => bytes.extend_from_slice(&d.to_be_bytes()),
            None => bytes.extend_from_slice(&[0xFF; 4]),
        }
        
        // Context (4 bytes ou 0xFF... se None)
        match self.context {
            Some(c) => bytes.extend_from_slice(&c.to_be_bytes()),
            None => bytes.extend_from_slice(&[0xFF; 4]),
        }
        
        // Flags (2 bytes, big-endian)
        bytes.extend_from_slice(&self.flags.to_be_bytes());
        
        bytes
    }
}

// =============================================================================
// INSTRUÇÃO COMPLETA - LEI-QMN-SEPARACAO-01
// =============================================================================

/// Instrução GD-QMN completa: Onda + Carga.
///
/// Dois domínios disjuntos (LEI-QMN-SEPARACAO-01):
/// - Envelope (onda): comportamento cognitivo
/// - Cargo (carga): conteúdo transportado
///
/// Três checksums (LEI-QMN-INTEGRIDADE-TRIPLA-01):
/// - checksum_onda: integridade do envelope
/// - checksum_carga: integridade do payload
/// - checksum_total: vinculação onda↔carga
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GdQmnInstruction {
    /// Envelope wave-like (onda)
    pub envelope: WaveEnvelope,
    
    /// Carga (payload)
    pub cargo: Cargo,
    
    /// Checksum do envelope (LEI-QMN-INTEGRIDADE-TRIPLA-01 §I)
    pub(crate) checksum_onda: u32,
    
    /// Checksum da carga (LEI-QMN-INTEGRIDADE-TRIPLA-01 §II)
    pub(crate) checksum_carga: u32,
    
    /// Checksum total (LEI-QMN-INTEGRIDADE-TRIPLA-01 §III)
    pub(crate) checksum_total: u32,
}

impl GdQmnInstruction {
    /// Cria nova instrução com cálculo automático de checksums.
    pub fn new(envelope: WaveEnvelope, cargo: Cargo) -> Self {
        let checksum_onda = envelope.checksum();
        let checksum_carga = crc32_checksum(&cargo.payload);
        
        // checksum_total: vincula onda e carga
        let mut vinculacao = Vec::new();
        vinculacao.extend_from_slice(&checksum_onda.to_be_bytes());
        vinculacao.extend_from_slice(&checksum_carga.to_be_bytes());
        let checksum_total = crc32_checksum(&vinculacao);
        
        Self {
            envelope,
            cargo,
            checksum_onda,
            checksum_carga,
            checksum_total,
        }
    }
    
    /// Valida integridade tripla (GATE-QMN-01).
    ///
    /// Retorna true se todos os três checksums estão válidos.
    pub fn verify_integrity(&self) -> bool {
        // §I: Checksum do envelope
        let computed_onda = self.envelope.checksum();
        if computed_onda != self.checksum_onda {
            return false;
        }
        
        // §II: Checksum da carga
        let computed_carga = crc32_checksum(&self.cargo.payload);
        if computed_carga != self.checksum_carga {
            return false;
        }
        
        // §III: Checksum da vinculação
        let mut vinculacao = Vec::new();
        vinculacao.extend_from_slice(&self.checksum_onda.to_be_bytes());
        vinculacao.extend_from_slice(&self.checksum_carga.to_be_bytes());
        let computed_total = crc32_checksum(&vinculacao);
        if computed_total != self.checksum_total {
            return false;
        }
        
        true
    }
    
    /// Retorna UID da instrução (LEI-QMN-ID-01).
    pub fn uid(&self) -> (QmnFamily, QmnSubfamily, Opcode) {
        self.envelope.uid()
    }
    
    /// Retorna perfil da instrução.
    pub fn profile(&self) -> Profile {
        self.envelope.profile
    }
    
    /// Serializa instrução em bytes canônicos (LEI-QMN-SERIAL-01).
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        
        // Envelope canônico
        bytes.extend_from_slice(&self.envelope.serialize_canonical());
        
        // Checksums (big-endian)
        bytes.extend_from_slice(&self.checksum_onda.to_be_bytes());
        bytes.extend_from_slice(&self.checksum_carga.to_be_bytes());
        bytes.extend_from_slice(&self.checksum_total.to_be_bytes());
        
        // Tamanho do cargo (4 bytes, big-endian)
        bytes.extend_from_slice(&(self.cargo.payload.len() as u32).to_be_bytes());
        
        // Cargo payload
        bytes.extend_from_slice(&self.cargo.payload);
        
        // Schema hint (2 bytes, big-endian)
        bytes.extend_from_slice(&self.cargo.schema_hint.to_be_bytes());
        
        bytes
    }
}

// =============================================================================
// UTILITÁRIOS
// =============================================================================

/// Calcula CRC32 (não-criptográfico, determinístico).
///
/// Usado para checksums conforme LEI-QMN-INTEGRIDADE-TRIPLA-01.
/// SHA-256 é proibido (Canon).
fn crc32_checksum(data: &[u8]) -> u32 {
    const POLYNOMIAL: u32 = 0xEDB88320; // CRC-32 (IEEE)
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
    use super::super::core::{CoreOpcode};
    
    #[test]
    fn test_envelope_amplitude_validation() {
        // LEI-QMN-AMP-01: Amplitude > 0 sempre
        let result = WaveEnvelope::new(
            QmnFamily::Unary,
            QmnSubfamily::State,
            Opcode::Core(CoreOpcode::State),
            Profile::Standard,
            0.0, // INVÁLIDO
        );
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Amplitude must be > 0 (LEI-QMN-AMP-01)");
    }
    
    #[test]
    fn test_envelope_valid_creation() {
        let envelope = WaveEnvelope::new(
            QmnFamily::Unary,
            QmnSubfamily::State,
            Opcode::Core(CoreOpcode::State),
            Profile::Standard,
            1.0,
        ).unwrap();
        
        assert_eq!(envelope.amplitude, 1.0);
        assert_eq!(envelope.family, QmnFamily::Unary);
    }
    
    #[test]
    fn test_uid_extraction() {
        let envelope = WaveEnvelope::new(
            QmnFamily::Binary,
            QmnSubfamily::Relate,
            Opcode::Core(CoreOpcode::Reference),
            Profile::Standard,
            0.5,
        ).unwrap();
        
        let (f, s, _o) = envelope.uid();
        assert_eq!(f, QmnFamily::Binary);
        assert_eq!(s, QmnSubfamily::Relate);
    }
    
    #[test]
    fn test_checksum_determinism() {
        let envelope = WaveEnvelope::new(
            QmnFamily::Unary,
            QmnSubfamily::State,
            Opcode::Core(CoreOpcode::State),
            Profile::Standard,
            1.0,
        ).unwrap();
        
        let c1 = envelope.checksum();
        let c2 = envelope.checksum();
        
        // Mesma entrada → mesmo checksum
        assert_eq!(c1, c2);
    }
    
    #[test]
    fn test_instruction_integrity() {
        let envelope = WaveEnvelope::new(
            QmnFamily::Unary,
            QmnSubfamily::State,
            Opcode::Core(CoreOpcode::State),
            Profile::Standard,
            1.0,
        ).unwrap();
        
        let cargo = Cargo::new(vec![1, 2, 3, 4], 0x0001);
        let instruction = GdQmnInstruction::new(envelope, cargo);
        
        // Deve passar na verificação tripla
        assert!(instruction.verify_integrity());
    }
    
    #[test]
    fn test_instruction_tampering_detection() {
        let envelope = WaveEnvelope::new(
            QmnFamily::Unary,
            QmnSubfamily::State,
            Opcode::Core(CoreOpcode::State),
            Profile::Standard,
            1.0,
        ).unwrap();
        
        let cargo = Cargo::new(vec![1, 2, 3, 4], 0x0001);
        let mut instruction = GdQmnInstruction::new(envelope, cargo);
        
        // Tamper com cargo
        instruction.cargo.payload[0] = 0xFF;
        
        // Deve falhar na verificação
        assert!(!instruction.verify_integrity());
    }
    
    #[test]
    fn test_serialization_determinism() {
        let envelope = WaveEnvelope::new(
            QmnFamily::Unary,
            QmnSubfamily::State,
            Opcode::Core(CoreOpcode::State),
            Profile::Standard,
            1.0,
        ).unwrap();
        
        let cargo = Cargo::new(vec![1, 2, 3, 4], 0x0001);
        let instruction = GdQmnInstruction::new(envelope, cargo);
        
        let bytes1 = instruction.to_bytes();
        let bytes2 = instruction.to_bytes();
        
        // LEI-QMN-SERIAL-01: mesma entrada → mesmos bytes
        assert_eq!(bytes1, bytes2);
    }
}
