//! Canon v5.1 | Tempo Lógico Canônico
//!
//! ## Conformidade Canônica
//!
//! Este módulo implementa tempo **completamente determinístico**, satisfazendo:
//! - **QM-01 (Pureza):** Tempo é função pura do estado, não do sistema operacional
//! - **QM-02 (Replay):** Mesmo estado sempre produz mesmo tempo
//!
//! ## Princípio Fundamental
//!
//! O Canon v5.1 exige que o GDC seja **deterministicamente replayável**.
//! Portanto, NENHUMA dependência de relógio do sistema operacional pode
//! existir no núcleo cognitivo.
//!
//! ## Design Canônico
//!
//! O tempo lógico É o cycle_count. Sem multiplicadores, sem constantes arbitrárias.
//!
//! ```text
//! logical_time = cycle_count
//! ```
//!
//! **CRÍTICO:** A única constante canônica é Veto = 0. Qualquer outro
//! número hardcoded (1000, 42, etc.) é VIOLAÇÃO do Canon.
//!
//! ## Garantia Canônica
//!
//! **TEOREMA:** Dados dois estados G₁ e G₂ com cycle_count idêntico,
//! LogicalTime(G₁) = LogicalTime(G₂) sempre.
//!
//! **PROVA:** LogicalTime é função identidade de cycle_count.
//! Portanto, mesmo cycle_count → mesmo LogicalTime. ∎

use serde::{Deserialize, Serialize};

/// Tempo lógico canônico - completamente determinístico
///
/// ## Invariante Canônico
///
/// LogicalTime NUNCA consulta o sistema operacional. Todo tempo é derivado
/// do estado do GDC (cycle_count).
///
/// O timestamp É exatamente o cycle_count. SEM multiplicadores arbitrários.
///
/// ## Uso Correto
///
/// ```
/// use digital_genome_community::LogicalTime;
///
/// // ✅ CORRETO - Timestamp é exatamente cycle_count
/// let time = LogicalTime::from_cycle(42);
/// assert_eq!(time.as_timestamp(), 42); // Direto, sem multiplicação
///
/// // ❌ INCORRETO - Tempo do sistema operacional (não faça)
/// // let timestamp = SystemTime::now(); // VIOLAÇÃO CANÔNICA
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct LogicalTime {
    /// Contador de ciclo atual
    ///
    /// Este valor vem do estado do orquestrador e é incrementado
    /// deterministicamente a cada ciclo cognitivo.
    cycle_count: u64,
}

impl LogicalTime {
    /// Criar tempo lógico a partir de contador de ciclo
    ///
    /// ## Garantia Canônica
    ///
    /// Esta função é **pura**: mesmo cycle_count sempre produz mesmo LogicalTime.
    ///
    /// ## Parâmetros
    ///
    /// - `cycle_count`: Contador de ciclo do estado do GDC (não do sistema operacional)
    ///
    /// ## Exemplo
    ///
    /// ```ignore
    /// let time = LogicalTime::from_cycle(42);
    /// assert_eq!(time.cycle_count(), 42);
    /// assert_eq!(time.as_timestamp(), 42 * 1000);
    /// ```
    pub fn from_cycle(cycle_count: u64) -> Self {
        LogicalTime { cycle_count }
    }

    /// Criar tempo lógico inicial (ciclo zero)
    ///
    /// Usado ao inicializar um novo orquestrador.
    pub fn zero() -> Self {
        LogicalTime { cycle_count: 0 }
    }

    /// Obter contador de ciclo atual
    ///
    /// ## Uso
    ///
    /// Este método expõe o cycle_count para que o orquestrador possa
    /// persistir/restaurar estado corretamente.
    pub fn cycle_count(&self) -> u64 {
        self.cycle_count
    }

    /// Obter timestamp lógico (identidade do cycle_count)
    ///
    /// ## Garantia Canônica
    ///
    /// O timestamp É exatamente o cycle_count. Sem multiplicadores.
    ///
    /// **CRÍTICO:** Versão anterior (v1.0.1 draft) multiplicava por 1000,
    /// violando o Canon (única constante permitida é Veto = 0).
    ///
    /// ## Semântica
    ///
    /// - O timestamp cresce monotonicamente com os ciclos (1:1)
    /// - Dois ciclos consecutivos diferem exatamente por 1
    /// - O timestamp pode ser usado para ordenação temporal determinística
    ///
    /// ## Retorno
    ///
    /// Timestamp = cycle_count (função identidade).
    pub fn as_timestamp(&self) -> u64 {
        self.cycle_count  // Direto, SEM multiplicação
    }

    /// Avançar para o próximo ciclo
    ///
    /// ## Garantia Canônica
    ///
    /// Esta operação é **pura** e **determinística**:
    /// - Não consulta estado externo
    /// - Sempre incrementa exatamente em 1
    /// - Resultado é previsível
    ///
    /// ## Uso
    ///
    /// O orquestrador chama este método ao completar cada ciclo cognitivo:
    ///
    /// ```ignore
    /// let mut time = LogicalTime::from_cycle(10);
    /// time = time.next_cycle();
    /// assert_eq!(time.cycle_count(), 11);
    /// ```
    pub fn next_cycle(self) -> Self {
        LogicalTime {
            cycle_count: self
                .cycle_count
                .checked_add(1)
                .expect("Logical time overflow - too many cycles"),
        }
    }

    /// Calcular diferença entre dois tempos lógicos
    ///
    /// ## Garantia Canônica
    ///
    /// A diferença é sempre determinística e representa número de ciclos
    /// entre os dois tempos.
    ///
    /// ## Retorno
    ///
    /// Número de ciclos entre `self` e `other`. Retorna `None` se
    /// `other` é posterior a `self` (evita underflow).
    pub fn cycles_since(&self, other: &LogicalTime) -> Option<u64> {
        self.cycle_count.checked_sub(other.cycle_count)
    }

    /// Avançar por múltiplos ciclos de uma vez
    ///
    /// ## Uso
    ///
    /// Útil para simulações ou testes onde queremos avançar rapidamente.
    ///
    /// ## Garantia
    ///
    /// Completamente determinístico - sempre adiciona exatamente `cycles`.
    pub fn advance_by(self, cycles: u64) -> Self {
        LogicalTime {
            cycle_count: self
                .cycle_count
                .checked_add(cycles)
                .expect("Logical time overflow"),
        }
    }
}

impl Default for LogicalTime {
    /// Tempo lógico padrão é ciclo zero
    fn default() -> Self {
        LogicalTime::zero()
    }
}

impl std::fmt::Display for LogicalTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LogicalTime(cycle={}, timestamp={})",
            self.cycle_count,
            self.as_timestamp()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logical_time_deterministic() {
        // Mesmo ciclo sempre produz mesmo tempo
        let time1 = LogicalTime::from_cycle(42);
        let time2 = LogicalTime::from_cycle(42);
        assert_eq!(time1, time2);
        assert_eq!(time1.as_timestamp(), time2.as_timestamp());
    }

    #[test]
    fn test_logical_time_monotonic() {
        // Tempo cresce monotonicamente
        let time1 = LogicalTime::from_cycle(10);
        let time2 = time1.next_cycle();
        let time3 = time2.next_cycle();

        assert!(time1 < time2);
        assert!(time2 < time3);
        assert_eq!(time2.cycle_count(), 11);
        assert_eq!(time3.cycle_count(), 12);
    }

    #[test]
    fn test_timestamp_calculation() {
        // Timestamp É cycle_count (função identidade)
        let time = LogicalTime::from_cycle(5);
        assert_eq!(time.as_timestamp(), 5);  // Direto, sem × 1000
    }

    #[test]
    fn test_cycles_since() {
        let time1 = LogicalTime::from_cycle(10);
        let time2 = LogicalTime::from_cycle(15);

        assert_eq!(time2.cycles_since(&time1), Some(5));
        assert_eq!(time1.cycles_since(&time2), None); // Underflow
    }

    #[test]
    fn test_advance_by() {
        let time = LogicalTime::from_cycle(10);
        let advanced = time.advance_by(5);
        assert_eq!(advanced.cycle_count(), 15);
    }

    #[test]
    fn test_zero_initialization() {
        let time = LogicalTime::zero();
        assert_eq!(time.cycle_count(), 0);
        assert_eq!(time.as_timestamp(), 0);
    }

    #[test]
    fn test_deterministic_replay() {
        // Simular replay: mesmo ciclo → mesmo resultado
        let cycle_count = 100;

        // Primeira "execução"
        let time1 = LogicalTime::from_cycle(cycle_count);
        let timestamp1 = time1.as_timestamp();

        // Segunda "execução" (replay)
        let time2 = LogicalTime::from_cycle(cycle_count);
        let timestamp2 = time2.as_timestamp();

        // DEVE ser idêntico (determinismo canônico)
        assert_eq!(time1, time2);
        assert_eq!(timestamp1, timestamp2);
    }
}
