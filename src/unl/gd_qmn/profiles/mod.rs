//! --------------------------
//! INFORMATION
//! --------------------------
//! Title: GD-QMN Encoding Profiles
//! Author: Claude B (sob supervisão de Carlos Eduardo Favini)
//! Date: 2026-01-29
//! Version: 0.8.0
//! Description: Perfis de codificação GD-QMN (Compact, Standard, Extended).
//!              Analogia celular: neurotransmissor, hormônio, proteína.
//!
//! Canon References:
//!   - DLB-009: Três perfis (Compact/Standard/Extended)
//!   - DLB-021: QmnFamily + QmnSubfamily
//!   - DLB-022: CoreOpcode (ISA)
//!   - DLB-024: Cargo
//!
//! --------------------------
//! CHANGE LOG
//! --------------------------
//! 2026-01-29 - Claude B - Atualização para v0.8.0 (integração core.rs)
//! --------------------------

use serde::{Deserialize, Serialize};
use super::core::{QmnFamily, QmnSubfamily, Opcode, CoreOpcode, Cargo};

// =============================================================================
// COMPACT PROFILE (~64+ bits) - Sinal rápido, local (neurotransmissor)
// =============================================================================

/// Perfil compacto — sinal rápido e local.
///
/// Analogia: Neurotransmissor — comunicação sináptica rápida.
/// Uso: IoT, sensores, alta frequência, sinais simples.
///
/// # Canon
/// - DLB-009: "Compact para eficiência"
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompactCode {
    /// Família por cardinalidade (8 bits)
    pub family: QmnFamily,
    /// Classe operacional (8 bits)
    pub subfamily: QmnSubfamily,
    /// Opcode do ISA (8 bits)
    pub opcode: Opcode,
    /// Amplitude / intensidade (16 bits, fixed-point)
    pub amplitude: u16,
    /// Espaço reservado (32 bits)
    pub reserved: u32,
    /// Carga opcional (variável)
    pub cargo: Option<Cargo>,
}

impl CompactCode {
    /// Cria novo CompactCode.
    pub fn new(
        family: QmnFamily,
        subfamily: QmnSubfamily,
        opcode: Opcode,
        amplitude: u16,
    ) -> Self {
        Self {
            family,
            subfamily,
            opcode,
            amplitude,
            reserved: 0,
            cargo: None,
        }
    }

    /// Cria CompactCode simples (STATE unário).
    pub fn state(amplitude: u16) -> Self {
        Self::new(
            QmnFamily::Unary,
            QmnSubfamily::State,
            Opcode::Core(CoreOpcode::State),
            amplitude,
        )
    }

    /// Adiciona cargo.
    pub fn with_cargo(mut self, cargo: Cargo) -> Self {
        self.cargo = Some(cargo);
        self
    }

    /// Codifica parte fixa em bytes (64 bits = 8 bytes).
    pub fn encode_fixed(&self) -> [u8; 8] {
        let mut buf = [0u8; 8];
        buf[0] = self.family as u8;
        buf[1] = self.subfamily as u8;
        buf[2] = self.opcode.as_u8();
        buf[3..5].copy_from_slice(&self.amplitude.to_le_bytes());
        buf[5..8].copy_from_slice(&self.reserved.to_le_bytes()[..3]);
        buf
    }
}

impl Default for CompactCode {
    fn default() -> Self {
        Self::new(
            QmnFamily::Unary,
            QmnSubfamily::State,
            Opcode::Core(CoreOpcode::Void),
            0,
        )
    }
}

// =============================================================================
// STANDARD PROFILE (~128+ bits) - Operação geral (hormônio)
// =============================================================================

/// Perfil padrão — operação geral equilibrada.
///
/// Analogia: Hormônio — sinal sistêmico com alcance maior.
/// Uso: Referência, operações normais, equilíbrio entre precisão e eficiência.
///
/// # Canon
/// - DLB-009: "Standard como referência"
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StandardCode {
    /// Família por cardinalidade (8 bits)
    pub family: QmnFamily,
    /// Classe operacional (8 bits)
    pub subfamily: QmnSubfamily,
    /// Opcode do ISA (8 bits)
    pub opcode: Opcode,
    /// Amplitude / intensidade (32 bits, float)
    pub amplitude: f32,
    /// Frequência / taxa de oscilação (32 bits)
    pub frequency: u32,
    /// Flags de modificação (16 bits)
    pub flags: u16,
    /// Checksum da onda (16 bits) — integridade estrutural
    pub checksum_wave: u16,
    /// Espaço reservado (8 bits)
    pub reserved: u8,
    /// Carga opcional (variável)
    pub cargo: Option<Cargo>,
}

impl StandardCode {
    /// Cria novo StandardCode.
    pub fn new(
        family: QmnFamily,
        subfamily: QmnSubfamily,
        opcode: Opcode,
        amplitude: f32,
        frequency: u32,
    ) -> Self {
        let mut code = Self {
            family,
            subfamily,
            opcode,
            amplitude,
            frequency,
            flags: 0,
            checksum_wave: 0,
            reserved: 0,
            cargo: None,
        };
        code.checksum_wave = code.compute_checksum();
        code
    }

    /// Adiciona cargo.
    pub fn with_cargo(mut self, cargo: Cargo) -> Self {
        self.cargo = Some(cargo);
        self
    }

    /// Computa checksum da parte fixa.
    fn compute_checksum(&self) -> u16 {
        let bytes = self.encode_fixed_no_checksum();
        let sum: u32 = bytes.iter().map(|&b| b as u32).sum();
        (sum % 65536) as u16
    }

    /// Codifica sem checksum (para cálculo do checksum).
    fn encode_fixed_no_checksum(&self) -> [u8; 14] {
        let mut buf = [0u8; 14];
        buf[0] = self.family as u8;
        buf[1] = self.subfamily as u8;
        buf[2] = self.opcode.as_u8();
        buf[3..7].copy_from_slice(&self.amplitude.to_le_bytes());
        buf[7..11].copy_from_slice(&self.frequency.to_le_bytes());
        buf[11..13].copy_from_slice(&self.flags.to_le_bytes());
        buf[13] = self.reserved;
        buf
    }

    /// Codifica parte fixa em bytes (128 bits = 16 bytes).
    pub fn encode_fixed(&self) -> [u8; 16] {
        let mut buf = [0u8; 16];
        buf[0] = self.family as u8;
        buf[1] = self.subfamily as u8;
        buf[2] = self.opcode.as_u8();
        buf[3..7].copy_from_slice(&self.amplitude.to_le_bytes());
        buf[7..11].copy_from_slice(&self.frequency.to_le_bytes());
        buf[11..13].copy_from_slice(&self.flags.to_le_bytes());
        buf[13..15].copy_from_slice(&self.checksum_wave.to_le_bytes());
        buf[15] = self.reserved;
        buf
    }

    /// Verifica integridade da onda.
    pub fn verify_wave(&self) -> bool {
        self.checksum_wave == self.compute_checksum()
    }
}

impl Default for StandardCode {
    fn default() -> Self {
        Self::new(
            QmnFamily::Unary,
            QmnSubfamily::State,
            Opcode::Core(CoreOpcode::Void),
            0.0,
            0,
        )
    }
}

// =============================================================================
// EXTENDED PROFILE (~256+ bits) - Fidelidade total (proteína)
// =============================================================================

/// Perfil estendido — máxima fidelidade.
///
/// Analogia: Proteína/nutriente — carga pesada, informação completa.
/// Uso: Pesquisa, máxima precisão, todos os parâmetros wave-like.
///
/// # Canon
/// - DLB-009: "Extended para fidelidade total"
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtendedCode {
    /// Família por cardinalidade (8 bits)
    pub family: QmnFamily,
    /// Classe operacional (8 bits)
    pub subfamily: QmnSubfamily,
    /// Opcode do ISA (8 bits)
    pub opcode: Opcode,
    /// Amplitude / intensidade (64 bits, double)
    pub amplitude: f64,
    /// Frequência / taxa de oscilação (64 bits)
    pub frequency: u64,
    /// Fase relativa no ciclo (32 bits)
    pub phase: f32,
    /// Duração / persistência (32 bits)
    pub duration: u32,
    /// Contexto / domínio (32 bits)
    pub context: u32,
    /// Checksum da onda (8 bits)
    pub checksum_wave: u8,
    /// Carga opcional (variável)
    pub cargo: Option<Cargo>,
}

impl ExtendedCode {
    /// Cria novo ExtendedCode.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        family: QmnFamily,
        subfamily: QmnSubfamily,
        opcode: Opcode,
        amplitude: f64,
        frequency: u64,
        phase: f32,
        duration: u32,
        context: u32,
    ) -> Self {
        let mut code = Self {
            family,
            subfamily,
            opcode,
            amplitude,
            frequency,
            phase,
            duration,
            context,
            checksum_wave: 0,
            cargo: None,
        };
        code.checksum_wave = code.compute_checksum();
        code
    }

    /// Adiciona cargo.
    pub fn with_cargo(mut self, cargo: Cargo) -> Self {
        self.cargo = Some(cargo);
        self
    }

    /// Computa checksum da parte fixa (XOR de todos os bytes).
    fn compute_checksum(&self) -> u8 {
        let bytes = self.encode_fixed_no_checksum();
        bytes.iter().fold(0u8, |acc, &b| acc ^ b)
    }

    /// Codifica sem checksum.
    fn encode_fixed_no_checksum(&self) -> [u8; 31] {
        let mut buf = [0u8; 31];
        buf[0] = self.family as u8;
        buf[1] = self.subfamily as u8;
        buf[2] = self.opcode.as_u8();
        buf[3..11].copy_from_slice(&self.amplitude.to_le_bytes());
        buf[11..19].copy_from_slice(&self.frequency.to_le_bytes());
        buf[19..23].copy_from_slice(&self.phase.to_le_bytes());
        buf[23..27].copy_from_slice(&self.duration.to_le_bytes());
        buf[27..31].copy_from_slice(&self.context.to_le_bytes());
        buf
    }

    /// Codifica parte fixa em bytes (256 bits = 32 bytes).
    pub fn encode_fixed(&self) -> [u8; 32] {
        let mut buf = [0u8; 32];
        buf[0] = self.family as u8;
        buf[1] = self.subfamily as u8;
        buf[2] = self.opcode.as_u8();
        buf[3..11].copy_from_slice(&self.amplitude.to_le_bytes());
        buf[11..19].copy_from_slice(&self.frequency.to_le_bytes());
        buf[19..23].copy_from_slice(&self.phase.to_le_bytes());
        buf[23..27].copy_from_slice(&self.duration.to_le_bytes());
        buf[27..31].copy_from_slice(&self.context.to_le_bytes());
        buf[31] = self.checksum_wave;
        buf
    }

    /// Verifica integridade da onda.
    pub fn verify_wave(&self) -> bool {
        self.checksum_wave == self.compute_checksum()
    }
}

impl Default for ExtendedCode {
    fn default() -> Self {
        Self::new(
            QmnFamily::Unary,
            QmnSubfamily::State,
            Opcode::Core(CoreOpcode::Void),
            0.0,
            0,
            0.0,
            0,
            0,
        )
    }
}

// =============================================================================
// CONVERSÕES ENTRE PERFIS
// =============================================================================

impl From<CompactCode> for StandardCode {
    /// Converte Compact para Standard (sem perda, valores default para novos campos).
    fn from(c: CompactCode) -> Self {
        Self {
            family: c.family,
            subfamily: c.subfamily,
            opcode: c.opcode,
            amplitude: c.amplitude as f32,
            frequency: 0,
            flags: 0,
            checksum_wave: 0,
            reserved: 0,
            cargo: c.cargo,
        }
    }
}

impl From<StandardCode> for ExtendedCode {
    /// Converte Standard para Extended (sem perda, valores default para novos campos).
    fn from(s: StandardCode) -> Self {
        Self {
            family: s.family,
            subfamily: s.subfamily,
            opcode: s.opcode,
            amplitude: s.amplitude as f64,
            frequency: s.frequency as u64,
            phase: 0.0,
            duration: 0,
            context: 0,
            checksum_wave: 0,
            cargo: s.cargo,
        }
    }
}

impl From<CompactCode> for ExtendedCode {
    /// Converte Compact para Extended diretamente.
    fn from(c: CompactCode) -> Self {
        StandardCode::from(c).into()
    }
}

// =============================================================================
// TESTES
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compact_size() {
        let c = CompactCode::default();
        assert_eq!(c.encode_fixed().len(), 8);
    }

    #[test]
    fn test_standard_size() {
        let s = StandardCode::default();
        assert_eq!(s.encode_fixed().len(), 16);
    }

    #[test]
    fn test_extended_size() {
        let e = ExtendedCode::default();
        assert_eq!(e.encode_fixed().len(), 32);
    }

    #[test]
    fn test_standard_checksum() {
        let s = StandardCode::new(
            QmnFamily::Binary,
            QmnSubfamily::Relate,
            Opcode::Core(CoreOpcode::Combine),
            0.75,
            1000,
        );
        assert!(s.verify_wave());
    }

    #[test]
    fn test_extended_checksum() {
        let e = ExtendedCode::new(
            QmnFamily::Ternary,
            QmnSubfamily::Derive,
            Opcode::Core(CoreOpcode::Derive),
            0.999,
            44100,
            0.5,
            1000,
            42,
        );
        assert!(e.verify_wave());
    }

    #[test]
    fn test_compact_to_standard_conversion() {
        let c = CompactCode::state(32768);
        let s: StandardCode = c.into();
        assert_eq!(s.family, QmnFamily::Unary);
        assert_eq!(s.amplitude, 32768.0);
    }

    #[test]
    fn test_compact_to_extended_conversion() {
        let c = CompactCode::state(16384);
        let e: ExtendedCode = c.into();
        assert_eq!(e.family, QmnFamily::Unary);
        assert_eq!(e.amplitude, 16384.0);
    }

    #[test]
    fn test_compact_with_cargo() {
        let cargo = Cargo::new(vec![1, 2, 3], 0x0001);
        let c = CompactCode::state(100).with_cargo(cargo);
        assert!(c.cargo.is_some());
        assert!(c.cargo.unwrap().verify());
    }
}
