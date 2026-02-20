//! --------------------------
//! INFORMATION
//! --------------------------
//! Title: Event (Σ) - Forma do Chamado (SANITIZADO)
//! Author: Digital Genome Community
//! Date: 2026-02-04
//! Version: 0.8.5-sanitized
//!
//! # Evento = Σ (Forma)
//!
//! O evento é padrão lógico atemporal, não ocorrência histórica.
//! Dois chamados com mesma forma = mesmo evento.
//!
//! # REGRAS CANÔNICAS
//!
//! 1. SEM event_id/UUID (evento = forma)
//! 2. Capacidades por CÓDIGO (u32), não string
//! 3. SEM tempo (timestamp, deadline)
//!
//! Canon: LEI-COORD-01
//! --------------------------

use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use uuid::Uuid;

/// Evento (Σ) - Forma do chamado.
///
/// # Invariantes
///
/// 1. Evento = padrão lógico atemporal
/// 2. Dois chamados com mesma forma = mesmo evento
/// 3. call_signature é a ÚNICA referência
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Event {
    /// ID derivado da forma (para compatibilidade)
    id: Uuid,
    /// Hash da forma = call_signature
    form_hash: [u8; 32],
    /// Descritor de trabalho
    work_descriptor: WorkDescriptor,
}

impl Event {
    /// Cria evento a partir de descritor.
    pub fn from_work(work: WorkDescriptor) -> Self {
        let form_hash = work.compute_hash();
        
        // ID derivado da forma (determinístico)
        let mut id_bytes = [0u8; 16];
        id_bytes.copy_from_slice(&form_hash[..16]);
        id_bytes[6] = (id_bytes[6] & 0x0f) | 0x50;
        id_bytes[8] = (id_bytes[8] & 0x3f) | 0x80;
        
        Self {
            id: Uuid::from_bytes(id_bytes),
            form_hash,
            work_descriptor: work,
        }
    }

    /// ID do evento (derivado da forma).
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Hash da forma.
    pub fn form_hash(&self) -> &[u8; 32] {
        &self.form_hash
    }

    /// Call signature (= form_hash).
    pub fn call_signature(&self) -> &[u8; 32] {
        &self.form_hash
    }

    /// Descritor de trabalho.
    pub fn work(&self) -> &WorkDescriptor {
        &self.work_descriptor
    }

    /// Eventos equivalentes?
    pub fn is_equivalent(&self, other: &Event) -> bool {
        self.form_hash == other.form_hash
    }
}

/// Descritor de trabalho.
///
/// # Capacidades por Código
///
/// Capacidades são u32, não strings.
/// Exemplos:
/// - 0x0001: avaliação cognitiva
/// - 0x0002: geração de DNA
/// - 0x0003: consulta de memória
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WorkDescriptor {
    /// Tipo de trabalho
    pub work_type: WorkType,
    /// Hash da entrada
    pub input_hash: [u8; 32],
    /// Capacidades requeridas (códigos)
    pub required_capability_codes: Vec<u32>,
    /// Contexto (bytes, não string)
    pub context: Vec<u8>,
}

impl WorkDescriptor {
    /// Novo descritor.
    pub fn new(work_type: WorkType, input: &[u8]) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(input);
        let result = hasher.finalize();
        
        let mut input_hash = [0u8; 32];
        input_hash.copy_from_slice(&result);
        
        Self {
            work_type,
            input_hash,
            required_capability_codes: Vec::new(),
            context: Vec::new(),
        }
    }

    /// Adiciona capacidade por código.
    pub fn require_capability_code(mut self, code: u32) -> Self {
        self.required_capability_codes.push(code);
        self
    }

    /// Define contexto.
    pub fn with_context(mut self, context: Vec<u8>) -> Self {
        self.context = context;
        self
    }

    /// Códigos de capacidade requeridos.
    pub fn required_capability_codes(&self) -> &[u32] {
        &self.required_capability_codes
    }

    /// Calcula hash da forma.
    pub fn compute_hash(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update([self.work_type as u8]);
        hasher.update(self.input_hash);
        
        // Ordenar códigos para determinismo
        let mut codes = self.required_capability_codes.clone();
        codes.sort();
        for code in codes {
            hasher.update(code.to_le_bytes());
        }
        
        hasher.update(&self.context);
        
        let result = hasher.finalize();
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&result);
        hash
    }
}

/// Tipos de trabalho.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum WorkType {
    /// Avaliação cognitiva
    CognitiveEvaluation = 1,
    /// Geração de DNA
    DnaGeneration = 2,
    /// Consulta de memória
    MemoryQuery = 3,
    /// Síntese
    Synthesis = 4,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_determinism() {
        let work = WorkDescriptor::new(WorkType::CognitiveEvaluation, b"data")
            .require_capability_code(1);
        
        let e1 = Event::from_work(work.clone());
        let e2 = Event::from_work(work);
        
        assert_eq!(e1.call_signature(), e2.call_signature());
        assert!(e1.is_equivalent(&e2));
    }

    #[test]
    fn test_different_work_different_event() {
        let w1 = WorkDescriptor::new(WorkType::CognitiveEvaluation, b"data1");
        let w2 = WorkDescriptor::new(WorkType::CognitiveEvaluation, b"data2");
        
        let e1 = Event::from_work(w1);
        let e2 = Event::from_work(w2);
        
        assert!(!e1.is_equivalent(&e2));
    }

    #[test]
    fn test_capability_codes() {
        let work = WorkDescriptor::new(WorkType::CognitiveEvaluation, b"data")
            .require_capability_code(0x0001)
            .require_capability_code(0x0002);
        
        assert_eq!(work.required_capability_codes().len(), 2);
    }
}
