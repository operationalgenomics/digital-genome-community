//! --------------------------
//! INFORMATION
//! --------------------------
//! Title: EDR - Envelope Devolutivo de Retorno (SANITIZADO)
//! Author: Digital Genome Community
//! Date: 2026-02-04
//! Version: 0.8.5-sanitized
//!
//! # DECLARAÇÃO CANÔNICA
//!
//! ```text
//! EDR é PROTOCOLO.
//! EDR NÃO é cognição.
//! EDR NÃO é identidade.
//! EDR NÃO é evento.
//! ```
//!
//! # REGRAS ABSOLUTAS
//!
//! 1. SEM event_id no core (evento = forma, não instância)
//! 2. Apenas call_signature (derivada da forma Σ)
//! 3. Apoptose silenciosa (sem log, sem erro)
//! 4. Pertence à família de protocolos do ecossistema
//!
//! Canon: DLB-014
//! --------------------------

use super::manifestation::Manifestation;
use crate::identity::{OrchestratedSignature, UidOrchestrated, OrchestrationRef};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use uuid::Uuid;

/// Envelope Devolutivo de Retorno (EDR).
///
/// # PROTOCOLO, NÃO COGNIÇÃO
///
/// O EDR é um envelope de transporte. Ele:
/// - Carrega dados entre GDCs
/// - Permite rastreabilidade
/// - NÃO influencia decisão cognitiva
/// - NÃO define semântica
///
/// # REFERÊNCIA AO EVENTO
///
/// O evento é identificado APENAS pela forma (Σ):
/// - `call_signature` = hash da forma do chamado
/// - NÃO há event_id, UUID, timestamp, ou instância
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edr {
    /// Versão do protocolo
    version: EdrVersion,
    /// Assinatura do chamado (forma Σ) - ÚNICA referência ao evento
    call_signature: [u8; 32],
    /// UID Ressonante do worker emissor
    worker_uid: Uuid,
    /// Orquestração
    orchestration_id: Uuid,
    /// Conteúdo (manifestação serializada)
    content: EdrContent,
    /// Assinatura para rastreabilidade
    signature: OrchestratedSignature,
    /// Hash de integridade
    integrity_hash: [u8; 32],
    /// Validade (false = apoptose)
    valid: bool,
}

impl Edr {
    /// Cria novo EDR.
    ///
    /// # Parâmetros
    /// - `call_signature`: Forma do chamado (Σ)
    /// - `worker`: UID Ressonante
    /// - `orchestration`: Orquestração
    /// - `manifestation`: Conteúdo
    pub fn new(
        call_signature: [u8; 32],
        worker: &UidOrchestrated,
        orchestration: &OrchestrationRef,
        manifestation: Manifestation,
    ) -> Self {
        let content = EdrContent::from_manifestation(manifestation);
        let signature = worker.sign(&content.data);
        
        let mut edr = Self {
            version: EdrVersion::V1,
            call_signature,
            worker_uid: *worker.as_uuid(),
            orchestration_id: orchestration.id,
            content,
            signature,
            integrity_hash: [0u8; 32],
            valid: true,
        };
        
        edr.update_integrity();
        edr
    }

    /// Atualiza hash de integridade.
    fn update_integrity(&mut self) {
        let mut hasher = Sha256::new();
        hasher.update([self.version as u8]);
        hasher.update(self.call_signature);
        hasher.update(self.worker_uid.as_bytes());
        hasher.update(self.orchestration_id.as_bytes());
        hasher.update(&self.content.data);
        hasher.update(self.signature.checksum);
        
        let result = hasher.finalize();
        self.integrity_hash.copy_from_slice(&result);
    }

    /// Verifica integridade.
    fn verify_integrity(&self) -> bool {
        let mut hasher = Sha256::new();
        hasher.update([self.version as u8]);
        hasher.update(self.call_signature);
        hasher.update(self.worker_uid.as_bytes());
        hasher.update(self.orchestration_id.as_bytes());
        hasher.update(&self.content.data);
        hasher.update(self.signature.checksum);
        
        let result = hasher.finalize();
        let mut expected = [0u8; 32];
        expected.copy_from_slice(&result);
        
        self.integrity_hash == expected
    }

    /// Valida EDR contra call_signature esperada.
    ///
    /// # APOPTOSE
    ///
    /// Se falhar:
    /// - Retorna None
    /// - Sem log
    /// - Sem erro
    /// - Sem exceção
    ///
    /// Uma célula comprometida não avisa — ela desaparece.
    pub fn validate(mut self, expected: &[u8; 32]) -> Option<ValidatedEdr> {
        // Já inválido?
        if !self.valid {
            return None;
        }
        
        // call_signature bate?
        if &self.call_signature != expected {
            self.apoptosis();
            return None;
        }
        
        // Integridade OK?
        if !self.verify_integrity() {
            self.apoptosis();
            return None;
        }
        
        // Assinatura presente?
        if self.signature.is_empty() {
            self.apoptosis();
            return None;
        }
        
        Some(ValidatedEdr {
            call_signature: self.call_signature,
            worker_uid: self.worker_uid,
            content: self.content,
        })
    }

    /// APOPTOSE - morte silenciosa.
    fn apoptosis(&mut self) {
        self.valid = false;
        self.content.data.clear();
        // Silêncio absoluto
    }

    /// Verifica validade.
    pub fn is_valid(&self) -> bool {
        self.valid
    }

    /// Retorna call_signature.
    pub fn call_signature(&self) -> &[u8; 32] {
        &self.call_signature
    }
}

/// Versão do protocolo.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum EdrVersion {
    V1 = 1,
}

/// Conteúdo do EDR.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdrContent {
    /// Dados serializados
    pub data: Vec<u8>,
    /// Hash do conteúdo
    pub hash: [u8; 32],
}

impl EdrContent {
    /// Cria a partir de manifestação.
    pub fn from_manifestation(manifestation: Manifestation) -> Self {
        let data = bincode::serialize(&manifestation).unwrap_or_default();
        
        let mut hasher = Sha256::new();
        hasher.update(&data);
        let result = hasher.finalize();
        
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&result);
        
        Self { data, hash }
    }

    /// Extrai manifestação.
    pub fn into_manifestation(self) -> Option<Manifestation> {
        bincode::deserialize(&self.data).ok()
    }
}

/// EDR validado.
#[derive(Debug, Clone)]
pub struct ValidatedEdr {
    /// Call signature
    pub call_signature: [u8; 32],
    /// Worker
    pub worker_uid: Uuid,
    /// Conteúdo
    content: EdrContent,
}

impl ValidatedEdr {
    /// Extrai manifestação.
    pub fn into_manifestation(self) -> Option<Manifestation> {
        self.content.into_manifestation()
    }
}

/// Builder para EDR.
pub struct EdrBuilder {
    call_signature: Option<[u8; 32]>,
}

impl EdrBuilder {
    /// Novo builder.
    pub fn new() -> Self {
        Self { call_signature: None }
    }

    /// Define call_signature.
    pub fn with_call_signature(mut self, sig: [u8; 32]) -> Self {
        self.call_signature = Some(sig);
        self
    }

    /// Constrói EDR.
    pub fn build(
        self,
        worker: &UidOrchestrated,
        orchestration: &OrchestrationRef,
        manifestation: Manifestation,
    ) -> Option<Edr> {
        let call_signature = self.call_signature?;
        Some(Edr::new(call_signature, worker, orchestration, manifestation))
    }
}

impl Default for EdrBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::coordination::field::CapabilityState;

    fn test_call_sig() -> [u8; 32] {
        let mut sig = [0u8; 32];
        sig[0] = 0x42;
        sig
    }

    fn create_test_manifestation() -> Manifestation {
        Manifestation::new()
            .with_capability_code(1, CapabilityState::satisfied())
    }

    fn create_test_identity() -> (UidOrchestrated, OrchestrationRef) {
        let orchestration = OrchestrationRef::new(b"test");
        let uid = UidOrchestrated::new(b"test-entropy", &orchestration);
        (uid, orchestration)
    }

    #[test]
    fn test_edr_creation() {
        let sig = test_call_sig();
        let (worker, orch) = create_test_identity();
        let manifestation = create_test_manifestation();
        
        let edr = Edr::new(sig, &worker, &orch, manifestation);
        
        assert!(edr.is_valid());
        assert_eq!(edr.call_signature(), &sig);
    }

    #[test]
    fn test_validation_success() {
        let sig = test_call_sig();
        let (worker, orch) = create_test_identity();
        let manifestation = create_test_manifestation();
        
        let edr = Edr::new(sig, &worker, &orch, manifestation);
        let validated = edr.validate(&sig);
        
        assert!(validated.is_some());
    }

    #[test]
    fn test_validation_wrong_signature() {
        let sig = test_call_sig();
        let wrong = [0xFFu8; 32];
        let (worker, orch) = create_test_identity();
        let manifestation = create_test_manifestation();
        
        let edr = Edr::new(sig, &worker, &orch, manifestation);
        let validated = edr.validate(&wrong);
        
        assert!(validated.is_none()); // Apoptose silenciosa
    }

    #[test]
    fn test_apoptosis_is_silent() {
        let sig = test_call_sig();
        let (worker, orch) = create_test_identity();
        let manifestation = create_test_manifestation();
        
        let mut edr = Edr::new(sig, &worker, &orch, manifestation);
        edr.integrity_hash[0] ^= 0xFF; // Corrompe
        
        let validated = edr.validate(&sig);
        
        // Silêncio - apenas None, sem exceção/log
        assert!(validated.is_none());
    }
}
