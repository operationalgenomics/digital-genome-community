//! --------------------------
//! INFORMATION
//! --------------------------
//! Title: GD-QMN Core Structures
//! Author: Claude B (sob supervisão de Carlos Eduardo Favini)
//! Date: 2026-01-29
//! Version: 0.8.0
//! Description: Estruturas fundamentais do GD-QMN conforme MODO DELIBERAÇÃO.
//!              Implementa DLB-019 a DLB-024 do backlog canônico.
//!
//! Canon References:
//!   - DLB-019: MotorOutput (ZERO ontológico)
//!   - DLB-020: SyncFailure (erro tipado)
//!   - DLB-021: QmnFamily + QmnSubfamily
//!   - DLB-022: CoreOpcode + WaveOpcode (ISA)
//!   - DLB-024: Cargo (transporte determinístico)
//!
//! --------------------------
//! CHANGE LOG
//! --------------------------
//! 2026-01-29 - Claude B - Criação inicial (v0.8.0)
//! --------------------------

use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};

// =============================================================================
// DLB-019: MOTOR OUTPUT (ZERO ONTOLÓGICO)
// =============================================================================

/// Estado do motor cognitivo — valor ou veto ontológico.
///
/// ZERO não é número, é estado. Esta enum força tratamento explícito
/// e elimina ambiguidades IEEE 754 (0.0, -0.0, NaN).
///
/// # Canon
/// - DLB-019: "ZERO é enum, não número"
/// - LEI-ZERO-01: "Impede confusão semântica permanentemente"
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub enum MotorOutput {
    /// Valor válido no intervalo (0, 1]
    Value(f64),
    /// Veto ontológico — estado, não número
    #[default]
    Veto,
}

impl MotorOutput {
    /// Cria MotorOutput a partir de f64, convertendo 0.0 para Veto.
    pub fn from_f64(v: f64) -> Self {
        if v == 0.0 {
            Self::Veto
        } else {
            Self::Value(v)
        }
    }

    /// Retorna o valor se presente, ou 0.0 se Veto (para cálculos).
    pub fn as_f64(&self) -> f64 {
        match self {
            Self::Value(v) => *v,
            Self::Veto => 0.0,
        }
    }

    /// Verifica se é Veto.
    pub fn is_veto(&self) -> bool {
        matches!(self, Self::Veto)
    }

    /// Verifica se é valor válido.
    pub fn is_value(&self) -> bool {
        matches!(self, Self::Value(_))
    }
}

// =============================================================================
// DLB-020: SYNC FAILURE (ERRO TIPADO)
// =============================================================================

/// Erro de sincronização — propagado internamente, colapsa para Veto na fronteira.
///
/// Mutex poison não é "realidade não observável"; é sinal de bug.
/// Erro tipado preserva diagnóstico interno.
///
/// # Canon
/// - DLB-020: "Erro tipado interno + Veto na fronteira"
/// - LEI-SYNC-01: "Preserva auditabilidade"
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SyncFailure {
    /// Mutex foi envenenado por panic em outra thread
    MutexPoisoned,
    /// Timeout ao tentar adquirir lock
    LockTimeout,
    /// Contenção de recursos
    ResourceContention,
    /// Erro genérico de sincronização
    Other(String),
}

impl std::fmt::Display for SyncFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MutexPoisoned => write!(f, "Mutex poisoned"),
            Self::LockTimeout => write!(f, "Lock timeout"),
            Self::ResourceContention => write!(f, "Resource contention"),
            Self::Other(msg) => write!(f, "Sync failure: {}", msg),
        }
    }
}

impl std::error::Error for SyncFailure {}

impl SyncFailure {
    /// Colapsa erro para Veto na fronteira cognitiva.
    pub fn to_motor_output(&self) -> MotorOutput {
        MotorOutput::Veto
    }
}

// =============================================================================
// DLB-021: FAMÍLIAS GD-QMN (CARDINALIDADE + SUBFAMILY)
// =============================================================================

/// Nível 1: Cardinalidade (family) — neutro, matemático, estável.
///
/// # Canon
/// - DLB-021: "Cardinalidade sozinha não captura intenção, mas é neutra"
/// - LEI-QMN-04: "Famílias = Cardinalidade + Subfamily"
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum QmnFamily {
    /// Estado puro (1 elemento)
    Unary = 0x01,
    /// Relação A↔B (2 elementos)
    Binary = 0x02,
    /// Relação A↔B↔C (3 elementos)
    Ternary = 0x03,
    /// 4 elementos
    Quaternary = 0x04,
    /// 5 elementos
    Quinary = 0x05,
    /// 6 elementos
    Senary = 0x06,
    /// 7 elementos
    Septenary = 0x07,
    /// 8 elementos
    Octonary = 0x08,
    /// N elementos (cardinalidade no payload)
    NAry = 0xFE,
    /// Combina outros (agregador)
    Aggregator = 0xFF,
}

impl QmnFamily {
    /// Cria família a partir de cardinalidade.
    pub fn from_cardinality(n: u8) -> Self {
        match n {
            1 => Self::Unary,
            2 => Self::Binary,
            3 => Self::Ternary,
            4 => Self::Quaternary,
            5 => Self::Quinary,
            6 => Self::Senary,
            7 => Self::Septenary,
            8 => Self::Octonary,
            0xFF => Self::Aggregator,
            _ => Self::NAry,
        }
    }

    /// Retorna cardinalidade.
    pub fn cardinality(&self) -> Option<u8> {
        match self {
            Self::Unary => Some(1),
            Self::Binary => Some(2),
            Self::Ternary => Some(3),
            Self::Quaternary => Some(4),
            Self::Quinary => Some(5),
            Self::Senary => Some(6),
            Self::Septenary => Some(7),
            Self::Octonary => Some(8),
            Self::NAry => None,       // Cardinalidade no payload
            Self::Aggregator => None, // Variável
        }
    }
}

/// Nível 2: Classe operacional (subfamily) — não ontológica.
///
/// Não afirma "o que o mundo é"; afirma tipo de transformação interna.
/// Analogia com funções neurais (associar, compor, modular).
///
/// # Canon
/// - DLB-021: "Subfamily = operacional, não ontológico"
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum QmnSubfamily {
    /// Declara/manipula estado
    State = 0x01,
    /// Constrói vínculo entre estados
    Relate = 0x02,
    /// Deriva novo estado de existentes
    Derive = 0x03,
    /// Opera sobre operações (meta)
    Meta = 0x04,
}

// =============================================================================
// DLB-022: ISA MÍNIMO (NÚCLEO + EXTENSÕES WAVE-LIKE)
// =============================================================================

/// Opcodes núcleo — invariantes operacionais.
///
/// # Regra de Ouro (DLB-022)
/// Opcode novo só nasce quando:
/// 1. Impossível expressar por composição
/// 2. Recorrente o bastante
/// 3. Mantém determinismo
///
/// # Canon
/// - DLB-022: "5 núcleo + 4 wave = ISA mínimo"
/// - LEI-ISA-01: "Mínimo em primitivos, suficiente em composição"
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum CoreOpcode {
    /// Ausência ontológica (não é NOOP, é vazio)
    Void = 0x00,
    /// Declarar/emitir estado
    State = 0x01,
    /// Apontar para estado existente / binding
    Reference = 0x02,
    /// Composição estruturada (agregação)
    Combine = 0x03,
    /// Derivar novo estado de inputs
    Derive = 0x04,
}

/// Opcodes extensão — wave-like e coordenação.
///
/// Parâmetros de onda para sincronização e modulação.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum WaveOpcode {
    /// Sincronização / phase-lock (coerência)
    Sync = 0x10,
    /// Bifurcação controlada (superposição operável)
    Fork = 0x11,
    /// Modulação de amplitude (+)
    Amplify = 0x12,
    /// Modulação de amplitude (-)
    Attenuate = 0x13,
}

/// Opcode unificado (core ou wave).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Opcode {
    Core(CoreOpcode),
    Wave(WaveOpcode),
}

impl Opcode {
    /// Retorna o byte do opcode.
    pub fn as_u8(&self) -> u8 {
        match self {
            Self::Core(c) => *c as u8,
            Self::Wave(w) => *w as u8,
        }
    }

    /// Cria opcode a partir de byte.
    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            0x00 => Some(Self::Core(CoreOpcode::Void)),
            0x01 => Some(Self::Core(CoreOpcode::State)),
            0x02 => Some(Self::Core(CoreOpcode::Reference)),
            0x03 => Some(Self::Core(CoreOpcode::Combine)),
            0x04 => Some(Self::Core(CoreOpcode::Derive)),
            0x10 => Some(Self::Wave(WaveOpcode::Sync)),
            0x11 => Some(Self::Wave(WaveOpcode::Fork)),
            0x12 => Some(Self::Wave(WaveOpcode::Amplify)),
            0x13 => Some(Self::Wave(WaveOpcode::Attenuate)),
            _ => None,
        }
    }
}

// =============================================================================
// DLB-024: CARGO (TRANSPORTE DETERMINÍSTICO)
// =============================================================================

/// Container de transporte determinístico.
///
/// # Cuidados (DLB-024)
/// 1. `schema_hint` = dica, nunca requisito no núcleo
/// 2. Hash de payload canônico (mesma serialização = mesmo hash)
/// 3. Payload: raw (borda) ou canonicalized (camadas)
///
/// # Canon
/// - DLB-024: "Cargo com bytes + hash + hint"
/// - LEI-QMN-02: "Campo Cargo como transporte"
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cargo {
    /// Bytes canônicos do payload
    pub payload: Vec<u8>,
    /// SHA-256 do payload canonicalizado
    pub content_hash: [u8; 32],
    /// Dica de schema (apenas dica, nunca requisito)
    pub schema_hint: u16,
}

impl Cargo {
    /// Cria novo Cargo a partir de payload, calculando hash automaticamente.
    pub fn new(payload: Vec<u8>, schema_hint: u16) -> Self {
        let content_hash = Self::compute_hash(&payload);
        Self {
            payload,
            content_hash,
            schema_hint,
        }
    }

    /// Cria Cargo vazio.
    pub fn empty() -> Self {
        Self::new(Vec::new(), 0)
    }

    /// Computa SHA-256 do payload.
    fn compute_hash(payload: &[u8]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(payload);
        let result = hasher.finalize();
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&result);
        hash
    }

    /// Verifica integridade do Cargo.
    pub fn verify(&self) -> bool {
        let computed = Self::compute_hash(&self.payload);
        computed == self.content_hash
    }

    /// Tamanho do payload em bytes.
    pub fn len(&self) -> usize {
        self.payload.len()
    }

    /// Verifica se payload está vazio.
    pub fn is_empty(&self) -> bool {
        self.payload.is_empty()
    }
}

impl Default for Cargo {
    fn default() -> Self {
        Self::empty()
    }
}

// =============================================================================
// TESTES
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // DLB-019 Tests
    #[test]
    fn test_motor_output_veto() {
        let v = MotorOutput::Veto;
        assert!(v.is_veto());
        assert!(!v.is_value());
        assert_eq!(v.as_f64(), 0.0);
    }

    #[test]
    fn test_motor_output_value() {
        let v = MotorOutput::Value(0.5);
        assert!(!v.is_veto());
        assert!(v.is_value());
        assert_eq!(v.as_f64(), 0.5);
    }

    #[test]
    fn test_motor_output_from_zero() {
        let v = MotorOutput::from_f64(0.0);
        assert!(v.is_veto());
    }

    // DLB-020 Tests
    #[test]
    fn test_sync_failure_to_veto() {
        let err = SyncFailure::MutexPoisoned;
        let output = err.to_motor_output();
        assert!(output.is_veto());
    }

    // DLB-021 Tests
    #[test]
    fn test_family_cardinality() {
        assert_eq!(QmnFamily::Unary.cardinality(), Some(1));
        assert_eq!(QmnFamily::Binary.cardinality(), Some(2));
        assert_eq!(QmnFamily::Aggregator.cardinality(), None);
    }

    #[test]
    fn test_family_from_cardinality() {
        assert_eq!(QmnFamily::from_cardinality(1), QmnFamily::Unary);
        assert_eq!(QmnFamily::from_cardinality(2), QmnFamily::Binary);
        assert_eq!(QmnFamily::from_cardinality(100), QmnFamily::NAry);
    }

    // DLB-022 Tests
    #[test]
    fn test_opcode_roundtrip() {
        assert_eq!(Opcode::from_u8(0x00), Some(Opcode::Core(CoreOpcode::Void)));
        assert_eq!(Opcode::from_u8(0x10), Some(Opcode::Wave(WaveOpcode::Sync)));
        assert_eq!(Opcode::from_u8(0xFF), None);
    }

    // DLB-024 Tests
    #[test]
    fn test_cargo_hash_verification() {
        let cargo = Cargo::new(vec![1, 2, 3, 4], 0x0001);
        assert!(cargo.verify());
    }

    #[test]
    fn test_cargo_tamper_detection() {
        let mut cargo = Cargo::new(vec![1, 2, 3, 4], 0x0001);
        cargo.payload[0] = 0xFF; // Tamper
        assert!(!cargo.verify());
    }

    #[test]
    fn test_cargo_empty() {
        let cargo = Cargo::empty();
        assert!(cargo.is_empty());
        assert!(cargo.verify());
    }
}
