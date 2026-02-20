//! --------------------------
//! INFORMATION
//! --------------------------
//! Title: Field R(Σ) - Operador de Acoplamento (SANITIZADO)
//! Author: Digital Genome Community
//! Date: 2026-02-04
//! Version: 0.8.5-sanitized
//!
//! # Campo = Operador, não Conjunto
//!
//! O campo É um operador que integra manifestações.
//! NÃO enumera GDCs. NÃO armazena identidades.
//!
//! # REGRAS CANÔNICAS
//!
//! 1. Capacidades por CÓDIGO (u32), não string
//! 2. Evento por call_signature, não ID
//!
//! Canon: LEI-COORD-01, LEI-COORD-02
//! --------------------------

use super::event::Event;
use super::manifestation::Manifestation;
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use std::collections::HashMap;

/// Campo R(Σ) - Operador de integração.
#[derive(Debug, Clone)]
pub struct Field {
    /// Evento associado
    event: Event,
    /// Estado integrado
    integrated_state: IntegratedState,
    /// Contador de manifestações
    manifestation_count: usize,
}

impl Field {
    /// Cria campo para evento.
    pub fn new(event: Event) -> Self {
        Self {
            event,
            integrated_state: IntegratedState::empty(),
            manifestation_count: 0,
        }
    }

    /// Aplica ⨆ (join) com manifestação.
    pub fn integrate(&mut self, manifestation: Manifestation) -> IntegrationResult {
        // Compatibilidade por call_signature
        if manifestation.call_signature() != self.event.call_signature() {
            return IntegrationResult::Incompatible;
        }

        // Idempotência
        if self.integrated_state.absorbs(&manifestation) {
            return IntegrationResult::Redundant;
        }

        // Join
        self.integrated_state.join(manifestation);
        self.manifestation_count += 1;

        IntegrationResult::Integrated
    }

    /// Estado integrado.
    pub fn state(&self) -> &IntegratedState {
        &self.integrated_state
    }

    /// Verifica absorção estrutural.
    pub fn is_absorbing(&self) -> bool {
        self.integrated_state.satisfies_work(self.event.work())
    }

    /// Evento associado.
    pub fn event(&self) -> &Event {
        &self.event
    }

    /// Contagem de manifestações.
    pub fn manifestation_count(&self) -> usize {
        self.manifestation_count
    }

    /// Extrai estado integrado.
    pub fn into_integrated_state(self) -> IntegratedState {
        self.integrated_state
    }
}

/// Resultado de integração.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegrationResult {
    /// Integrado com sucesso
    Integrated,
    /// Redundante (já absorvido)
    Redundant,
    /// Incompatível
    Incompatible,
}

/// Estado integrado do campo.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegratedState {
    /// Capacidades por código
    capabilities: HashMap<u32, CapabilityState>,
    /// Resultados cognitivos
    cognitive_results: Vec<CognitiveResult>,
    /// Hash do estado
    state_hash: Option<[u8; 32]>,
}

impl IntegratedState {
    /// Estado vazio.
    pub fn empty() -> Self {
        Self {
            capabilities: HashMap::new(),
            cognitive_results: Vec::new(),
            state_hash: None,
        }
    }

    /// Verifica se absorve manifestação (idempotência).
    ///
    /// Uma manifestação é redundante se:
    /// 1. Todas capacidades já estão satisfeitas, E
    /// 2. Todos resultados cognitivos já existem (mesmo hash)
    pub fn absorbs(&self, manifestation: &Manifestation) -> bool {
        // Verificar capacidades
        for (code, state) in manifestation.capabilities() {
            match self.capabilities.get(code) {
                Some(existing) if existing.is_satisfied() && state.is_satisfied() => {
                    // OK, já satisfeita
                }
                _ => return false, // Capacidade nova ou diferente
            }
        }
        
        // Verificar resultados cognitivos
        // Se tem novos resultados, NÃO é redundante
        for result in manifestation.cognitive_results() {
            let already_exists = self.cognitive_results
                .iter()
                .any(|r| r.hash == result.hash);
            if !already_exists {
                return false; // Resultado novo
            }
        }
        
        true // Totalmente redundante
    }

    /// Verifica compatibilidade.
    pub fn is_compatible_with(&self, manifestation: &Manifestation) -> bool {
        // Verificar conflitos
        for (code, state) in manifestation.capabilities() {
            if let Some(existing) = self.capabilities.get(code) {
                if existing.conflicts_with(state) {
                    return false;
                }
            }
        }
        true
    }

    /// Aplica join com manifestação.
    pub fn join(&mut self, manifestation: Manifestation) {
        // Mesclar capacidades
        for (code, state) in manifestation.capabilities() {
            self.capabilities
                .entry(*code)
                .and_modify(|e| e.merge(state))
                .or_insert_with(|| state.clone());
        }

        // Adicionar resultados cognitivos
        for result in manifestation.cognitive_results() {
            if !self.cognitive_results.iter().any(|r| r.hash == result.hash) {
                self.cognitive_results.push(result.clone());
            }
        }

        self.update_hash();
    }

    /// Verifica se satisfaz trabalho.
    pub fn satisfies_work(&self, work: &super::event::WorkDescriptor) -> bool {
        for &code in work.required_capability_codes() {
            match self.capabilities.get(&code) {
                Some(state) if state.is_satisfied() => {}
                _ => return false,
            }
        }
        true
    }

    /// Retorna capacidades.
    pub fn capabilities(&self) -> &HashMap<u32, CapabilityState> {
        &self.capabilities
    }

    /// Retorna resultados cognitivos.
    pub fn cognitive_results(&self) -> &[CognitiveResult] {
        &self.cognitive_results
    }

    /// Calcula Craft Performance.
    pub fn craft_performance(&self) -> f64 {
        if self.cognitive_results.is_empty() {
            return 0.0;
        }

        // CP = ∏ scores
        let mut cp = 1.0;
        for result in &self.cognitive_results {
            let scores = &result.motor_scores;
            cp *= scores.praxeological;
            cp *= scores.nash;
            cp *= scores.chaotic;
            cp *= scores.meristic;
        }
        
        cp.powf(1.0 / (4.0 * self.cognitive_results.len() as f64))
    }

    /// Atualiza hash.
    fn update_hash(&mut self) {
        let mut hasher = Sha256::new();
        
        let mut caps: Vec<_> = self.capabilities.iter().collect();
        caps.sort_by_key(|(k, _)| *k);
        
        for (code, state) in caps {
            hasher.update(code.to_le_bytes());
            hasher.update([if state.is_satisfied() { 1 } else { 0 }]);
        }
        
        let result = hasher.finalize();
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&result);
        self.state_hash = Some(hash);
    }
}

/// Estado de capacidade.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityState {
    /// Satisfeita
    pub satisfied: bool,
    /// Valor (opcional)
    pub value: Option<f64>,
}

impl CapabilityState {
    /// Satisfeita.
    pub fn satisfied() -> Self {
        Self { satisfied: true, value: None }
    }

    /// Não satisfeita.
    pub fn unsatisfied() -> Self {
        Self { satisfied: false, value: None }
    }

    /// Com valor.
    pub fn with_value(value: f64) -> Self {
        Self { satisfied: true, value: Some(value) }
    }

    /// Verifica se satisfeita.
    pub fn is_satisfied(&self) -> bool {
        self.satisfied
    }

    /// Verifica conflito.
    pub fn conflicts_with(&self, other: &CapabilityState) -> bool {
        match (self.value, other.value) {
            (Some(a), Some(b)) => (a - b).abs() > f64::EPSILON && a.signum() != b.signum(),
            _ => false,
        }
    }

    /// Mescla.
    pub fn merge(&mut self, other: &CapabilityState) {
        self.satisfied = self.satisfied || other.satisfied;
        self.value = match (self.value, other.value) {
            (Some(a), Some(b)) => Some((a + b) / 2.0),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        };
    }
}

impl Default for CapabilityState {
    fn default() -> Self {
        Self::unsatisfied()
    }
}

/// Resultado cognitivo.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveResult {
    /// Hash
    pub hash: [u8; 32],
    /// Scores dos motores
    pub motor_scores: MotorScores,
    /// Dados
    pub data: Vec<u8>,
}

/// Scores dos motores.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MotorScores {
    /// Motor praxeológico
    pub praxeological: f64,
    /// Motor de Nash
    pub nash: f64,
    /// Motor caótico
    pub chaotic: f64,
    /// Motor merístico
    pub meristic: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::coordination::event::{WorkDescriptor, WorkType};

    #[test]
    fn test_field_integration() {
        let work = WorkDescriptor::new(WorkType::CognitiveEvaluation, b"data")
            .require_capability_code(1);
        let event = Event::from_work(work);
        
        let mut field = Field::new(event.clone());
        
        let m = Manifestation::for_call(*event.call_signature())
            .with_capability_code(1, CapabilityState::satisfied());
        
        let result = field.integrate(m);
        assert_eq!(result, IntegrationResult::Integrated);
    }

    #[test]
    fn test_idempotency() {
        let work = WorkDescriptor::new(WorkType::CognitiveEvaluation, b"data");
        let event = Event::from_work(work);
        let mut field = Field::new(event.clone());
        
        let m1 = Manifestation::for_call(*event.call_signature())
            .with_capability_code(1, CapabilityState::satisfied());
        let m2 = Manifestation::for_call(*event.call_signature())
            .with_capability_code(1, CapabilityState::satisfied());
        
        field.integrate(m1);
        let r2 = field.integrate(m2);
        
        assert_eq!(r2, IntegrationResult::Redundant);
    }
}
