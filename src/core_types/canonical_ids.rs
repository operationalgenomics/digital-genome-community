//! Canon v5.1 | IDs Canônicos Determinísticos
//!
//! ## Conformidade Canônica v1.0.1 Fase 2
//!
//! Este módulo implementa IDs **completamente determinísticos**, satisfazendo:
//! - **QM-01 (Pureza):** IDs são função pura do contexto e payload
//! - **QM-02 (Replay):** Mesmos inputs sempre produzem mesmo ID
//!
//! ## Princípio Fundamental
//!
//! O Canon v5.1 exige replay determinístico. UUID v4 é **aleatório** e viola isso.
//!
//! **Violação (v1.0.0):**
//! ```ignore
//! let id = VibrationId(Uuid::new_v4());  // ❌ Aleatório
//! ```
//!
//! **Canônico (v1.0.1):**
//! ```ignore
//! let id = VibrationId::from_context("vibration", &payload, sequence);  // ✅ Determinístico
//! ```
//!
//! ## Design Canônico
//!
//! IDs são derivados criptograficamente:
//!
//! ```text
//! id = SHA-256(context || sequence || payload)[0..16]
//! ```
//!
//! Onde:
//! - `context`: String identificando tipo ("vibration", "work", etc.)
//! - `sequence`: Número sequencial do estado (determinístico)
//! - `payload`: Bytes que definem o objeto (serializado)
//!
//! ## Garantia Canônica
//!
//! **TEOREMA:** Dados mesmos (context, sequence, payload),
//! CanonicalId sempre produz mesmo resultado.
//!
//! **PROVA:** SHA-256 é função determinística. Mesmos inputs → mesmo hash. ∎

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// ID canônico - sempre determinístico (128 bits)
///
/// ## Invariante Canônico
///
/// CanonicalId NUNCA usa aleatoriedade. Todo ID é derivado
/// criptograficamente do contexto e payload.
///
/// ## Estrutura
///
/// - 128 bits (16 bytes) de hash SHA-256
/// - Primeiros 16 bytes do hash completo
/// - Colisões praticamente impossíveis (2^128 espaço)
///
/// ## Uso Correto
///
/// ```
/// use digital_genome_community::CanonicalId;
///
/// // ✅ CORRETO - Derivado deterministicamente
/// let id = CanonicalId::from_context(
///     "vibration",
///     b"payload_bytes",
///     42,  // sequence do estado
/// );
///
/// // ❌ INCORRETO - Aleatório (não faça isto)
/// // let id = Uuid::new_v4();  // VIOLAÇÃO CANÔNICA
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CanonicalId([u8; 16]);

impl CanonicalId {
    /// Gerar ID deterministicamente a partir de contexto e dados
    ///
    /// ## Garantia Canônica
    ///
    /// Esta função é **pura**: mesmos inputs sempre produzem mesmo ID.
    ///
    /// ## Algoritmo
    ///
    /// ```text
    /// hasher = SHA-256()
    /// hasher.update(context.as_bytes())
    /// hasher.update(sequence.to_le_bytes())
    /// hasher.update(payload)
    /// hash = hasher.finalize()
    /// id = hash[0..16]
    /// ```
    ///
    /// ## Parâmetros
    ///
    /// - `context`: Identificador de tipo ("vibration", "work", "worker", "gdc")
    /// - `payload`: Bytes que definem este objeto (serializado)
    /// - `sequence`: Número sequencial derivado do estado (não do relógio)
    ///
    /// ## Exemplo
    ///
    /// ```rust
    /// use digital_genome_community::CanonicalId;
    ///
    /// let payload = b"structural_requirements_serialized";
    /// let id = CanonicalId::from_context("vibration", payload, 0);
    ///
    /// // Mesmo input → mesmo ID (determinismo)
    /// let id2 = CanonicalId::from_context("vibration", payload, 0);
    /// assert_eq!(id, id2);
    /// ```
    pub fn from_context(context: &str, payload: &[u8], sequence: u64) -> Self {
        let mut hasher = Sha256::new();

        // Hash de todos os inputs de forma determinística
        hasher.update(context.as_bytes());
        hasher.update(&sequence.to_le_bytes());
        hasher.update(payload);

        let hash = hasher.finalize();

        // Pegar primeiros 128 bits (16 bytes)
        let mut id_bytes = [0u8; 16];
        id_bytes.copy_from_slice(&hash[0..16]);

        CanonicalId(id_bytes)
    }

    /// Criar ID a partir de bytes diretos (para deserialização)
    ///
    /// ## Uso
    ///
    /// Tipicamente não chamado diretamente. Usado por serde.
    pub fn from_bytes(bytes: [u8; 16]) -> Self {
        CanonicalId(bytes)
    }

    /// Obter bytes do ID (para serialização)
    pub fn as_bytes(&self) -> &[u8; 16] {
        &self.0
    }

    /// Converter para formato hexadecimal (para logging/debugging)
    ///
    /// ## Retorno
    ///
    /// String hex de 32 caracteres (16 bytes × 2 chars/byte)
    ///
    /// ## Exemplo
    ///
    /// ```rust
    /// use digital_genome_community::CanonicalId;
    ///
    /// let id = CanonicalId::from_context("test", b"data", 0);
    /// println!("ID: {}", id.to_hex());
    /// // Output: "a3f2b1c4..." (32 chars hex)
    /// ```
    pub fn to_hex(&self) -> String {
        hex::encode(&self.0)
    }

    /// Criar ID "zero" (usado como placeholder ou valor inicial)
    ///
    /// ## Semântica
    ///
    /// ID zero tem significado especial em alguns contextos
    /// (similar a Veto = 0 para scores).
    pub fn zero() -> Self {
        CanonicalId([0u8; 16])
    }

    /// Verificar se ID é zero
    pub fn is_zero(&self) -> bool {
        self.0.iter().all(|&b| b == 0)
    }
}

impl Default for CanonicalId {
    fn default() -> Self {
        CanonicalId::zero()
    }
}

impl std::fmt::Display for CanonicalId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canonical_id_deterministic() {
        // Mesmo input sempre produz mesmo ID
        let id1 = CanonicalId::from_context("test", b"payload", 42);
        let id2 = CanonicalId::from_context("test", b"payload", 42);
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_canonical_id_different_context() {
        // Contextos diferentes → IDs diferentes
        let id1 = CanonicalId::from_context("vibration", b"data", 0);
        let id2 = CanonicalId::from_context("work", b"data", 0);
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_canonical_id_different_sequence() {
        // Sequências diferentes → IDs diferentes
        let id1 = CanonicalId::from_context("test", b"data", 0);
        let id2 = CanonicalId::from_context("test", b"data", 1);
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_canonical_id_different_payload() {
        // Payloads diferentes → IDs diferentes
        let id1 = CanonicalId::from_context("test", b"payload1", 0);
        let id2 = CanonicalId::from_context("test", b"payload2", 0);
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_canonical_id_hex_format() {
        let id = CanonicalId::from_context("test", b"data", 0);
        let hex = id.to_hex();
        assert_eq!(hex.len(), 32); // 16 bytes × 2 hex chars
    }

    #[test]
    fn test_canonical_id_zero() {
        let zero = CanonicalId::zero();
        assert!(zero.is_zero());
        assert_eq!(zero.to_hex(), "00000000000000000000000000000000");
    }

    #[test]
    fn test_canonical_id_replay_determinism() {
        // Simular replay: mesmo contexto → mesmo ID
        let context = "vibration";
        let payload = b"structural_requirements_serialized";
        let sequence = 100;

        // "Primeira execução"
        let id1 = CanonicalId::from_context(context, payload, sequence);

        // "Segunda execução" (replay)
        let id2 = CanonicalId::from_context(context, payload, sequence);

        // DEVE ser idêntico (determinismo canônico)
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_canonical_id_collision_resistance() {
        // IDs próximos devem ser diferentes (resistência a colisões)
        let id1 = CanonicalId::from_context("test", b"data", 0);
        let id2 = CanonicalId::from_context("test", b"data", 1);
        let id3 = CanonicalId::from_context("test", b"datb", 0); // Um byte diferente

        assert_ne!(id1, id2);
        assert_ne!(id1, id3);
        assert_ne!(id2, id3);
    }
}
