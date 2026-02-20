//! --------------------------
//! INFORMATION
//! --------------------------
//! Title: Manifestation (Ω) - Capacidade Manifestada (SANITIZADO)
//! Author: Digital Genome Community
//! Date: 2026-02-04
//! Version: 0.8.5-sanitized
//!
//! # REGRAS CANÔNICAS
//!
//! 1. SEM strings de significado humano
//! 2. Capacidades identificadas por CÓDIGO numérico
//! 3. Evento referenciado por call_signature (forma Σ)
//! 4. NÃO contém DNA
//!
//! Canon: LEI-COORD-02, DLB-013, DLB-014
//! --------------------------

use super::field::{CapabilityState, CognitiveResult, MotorScores};
use crate::OrchestratedSignature;
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use std::collections::HashMap;

/// Manifestação (Ω) - Resposta de um worker.
///
/// # Invariantes
///
/// 1. NÃO contém DNA
/// 2. Contém apenas cálculos e UNLs
/// 3. Capacidades por CÓDIGO (u32), não string
/// 4. Evento por call_signature, não ID
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifestation {
    /// Referência ao evento (forma Σ)
    call_signature: [u8; 32],
    /// Capacidades por código numérico
    capabilities: HashMap<u32, CapabilityState>,
    /// Resultados cognitivos
    cognitive_results: Vec<CognitiveResult>,
    /// Assinatura (rastreabilidade)
    signature: OrchestratedSignature,
    /// Hash
    hash: [u8; 32],
}

impl Manifestation {
    /// Cria manifestação sem referência (para construção).
    pub fn new() -> Self {
        let mut m = Self {
            call_signature: [0u8; 32],
            capabilities: HashMap::new(),
            cognitive_results: Vec::new(),
            signature: OrchestratedSignature::empty(),
            hash: [0u8; 32],
        };
        m.update_hash();
        m
    }

    /// Cria manifestação para call_signature específica.
    pub fn for_call(call_signature: [u8; 32]) -> Self {
        let mut m = Self::new();
        m.call_signature = call_signature;
        m.update_hash();
        m
    }

    /// Define call_signature.
    pub fn with_call_signature(mut self, sig: [u8; 32]) -> Self {
        self.call_signature = sig;
        self.update_hash();
        self
    }

    /// Adiciona capacidade por código.
    ///
    /// # Códigos de Capacidade
    /// 
    /// Capacidades são identificadas por u32, não string.
    /// Exemplos de convenção:
    /// - 0x0001: avaliação cognitiva
    /// - 0x0002: geração de DNA
    /// - 0x0003: consulta de memória
    pub fn with_capability_code(mut self, code: u32, state: CapabilityState) -> Self {
        self.capabilities.insert(code, state);
        self.update_hash();
        self
    }

    /// Adiciona resultado cognitivo.
    pub fn with_cognitive_result(mut self, result: CognitiveResult) -> Self {
        self.cognitive_results.push(result);
        self.update_hash();
        self
    }

    /// Atualiza hash.
    fn update_hash(&mut self) {
        let mut hasher = Sha256::new();
        hasher.update(self.call_signature);
        
        // Ordenar capacidades para determinismo
        let mut caps: Vec<_> = self.capabilities.iter().collect();
        caps.sort_by_key(|(k, _)| *k);
        
        for (code, state) in caps {
            hasher.update(code.to_le_bytes());
            hasher.update([if state.is_satisfied() { 1 } else { 0 }]);
        }
        
        for result in &self.cognitive_results {
            hasher.update(result.hash);
        }
        
        let result = hasher.finalize();
        self.hash.copy_from_slice(&result);
    }

    /// Retorna call_signature.
    pub fn call_signature(&self) -> &[u8; 32] {
        &self.call_signature
    }

    /// Retorna capacidades.
    pub fn capabilities(&self) -> impl Iterator<Item = (&u32, &CapabilityState)> {
        self.capabilities.iter()
    }

    /// Retorna estado de capacidade.
    pub fn capability_state(&self, code: u32) -> Option<&CapabilityState> {
        self.capabilities.get(&code)
    }

    /// Retorna resultados cognitivos.
    pub fn cognitive_results(&self) -> &[CognitiveResult] {
        &self.cognitive_results
    }

    /// Retorna hash.
    pub fn hash(&self) -> &[u8; 32] {
        &self.hash
    }
}

impl Default for Manifestation {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder para resultados cognitivos.
pub struct CognitiveResultBuilder {
    motor_scores: MotorScores,
    data: Vec<u8>,
}

impl CognitiveResultBuilder {
    /// Novo builder.
    pub fn new() -> Self {
        Self {
            motor_scores: MotorScores::default(),
            data: Vec::new(),
        }
    }

    /// Define scores.
    pub fn with_scores(mut self, scores: MotorScores) -> Self {
        self.motor_scores = scores;
        self
    }

    /// Define dados.
    pub fn with_data(mut self, data: Vec<u8>) -> Self {
        self.data = data;
        self
    }

    /// Constrói resultado.
    pub fn build(self) -> CognitiveResult {
        let mut hasher = Sha256::new();
        hasher.update(self.motor_scores.praxeological.to_le_bytes());
        hasher.update(self.motor_scores.nash.to_le_bytes());
        hasher.update(self.motor_scores.chaotic.to_le_bytes());
        hasher.update(self.motor_scores.meristic.to_le_bytes());
        hasher.update(&self.data);
        
        let result = hasher.finalize();
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&result);
        
        CognitiveResult {
            hash,
            motor_scores: self.motor_scores,
            data: self.data,
        }
    }
}

impl Default for CognitiveResultBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manifestation_creation() {
        let sig = [0x42u8; 32];
        let m = Manifestation::for_call(sig)
            .with_capability_code(1, CapabilityState::satisfied());
        
        assert_eq!(m.call_signature(), &sig);
    }

    #[test]
    fn test_capability_by_code() {
        let m = Manifestation::new()
            .with_capability_code(0x0001, CapabilityState::satisfied())
            .with_capability_code(0x0002, CapabilityState::unsatisfied());
        
        assert!(m.capability_state(0x0001).unwrap().is_satisfied());
        assert!(!m.capability_state(0x0002).unwrap().is_satisfied());
    }

    #[test]
    fn test_cognitive_result_builder() {
        let scores = MotorScores {
            praxeological: 0.8,
            nash: 0.7,
            chaotic: 0.9,
            meristic: 0.6,
        };
        
        let result = CognitiveResultBuilder::new()
            .with_scores(scores)
            .build();
        
        assert_eq!(result.motor_scores.praxeological, 0.8);
    }
}
