//! --------------------------
//! INFORMATION
//! --------------------------
//! Title: UID Orquestrado - Plano Funcional (SANITIZADO)
//! Author: Digital Genome Community
//! Date: 2026-02-04
//! Version: 0.8.5-sanitized
//!
//! # REGRAS CANÔNICAS ABSOLUTAS
//!
//! 1. INDEPENDENTE do Shibboleth (planos disjuntos)
//! 2. PODE trafegar (é funcional, não ontológico)
//! 3. SEM tempo (timestamp, relógio, deadline)
//! 4. SEM strings de significado humano no core
//!
//! Canon: AO-RESSONANTE, LEI-RESS-01
//! --------------------------

use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use uuid::Uuid;

/// UID Orquestrado - Identidade Funcional do GDC.
///
/// # Invariantes Canônicos
///
/// 1. **INDEPENDENTE** - NÃO deriva do Shibboleth
/// 2. **PODE trafegar** - Seguro para transmissão
/// 3. **SEM tempo** - Determinístico por estrutura
/// 4. **Auditável** - Para rastreabilidade operacional
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UidOrchestrated {
    /// UUID único
    id: Uuid,
    /// Contexto da orquestração (hash, não string)
    orchestration_context: [u8; 8],
}

impl UidOrchestrated {
    /// Cria UID Orquestrado de forma determinística.
    ///
    /// # SEM TEMPO
    /// Este método NÃO usa timestamp, relógio ou qualquer
    /// referência temporal. A unicidade vem da entropia.
    pub fn new(local_entropy: &[u8], orchestration: &OrchestrationRef) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(b"gdc.orchestrated.v1");
        hasher.update(local_entropy);
        hasher.update(orchestration.context);
        let result = hasher.finalize();
        
        // Primeiros 16 bytes para UUID
        let mut uuid_bytes = [0u8; 16];
        uuid_bytes.copy_from_slice(&result[..16]);
        
        // Ajustar bits para UUID v5
        uuid_bytes[6] = (uuid_bytes[6] & 0x0f) | 0x50;
        uuid_bytes[8] = (uuid_bytes[8] & 0x3f) | 0x80;
        
        // Bytes 16-23 para contexto
        let mut orchestration_context = [0u8; 8];
        orchestration_context.copy_from_slice(&result[16..24]);
        
        Self {
            id: Uuid::from_bytes(uuid_bytes),
            orchestration_context,
        }
    }

    /// Cria UID para GDC isolado (sem orquestração).
    pub fn isolated(local_entropy: &[u8]) -> Self {
        Self::new(local_entropy, &OrchestrationRef::isolated())
    }

    /// Retorna o UUID.
    pub fn as_uuid(&self) -> &Uuid {
        &self.id
    }

    /// Assina payload para rastreabilidade.
    ///
    /// Esta assinatura é para AUDITORIA, não segurança criptográfica.
    pub fn sign(&self, payload: &[u8]) -> OrchestratedSignature {
        let mut hasher = Sha256::new();
        hasher.update(self.id.as_bytes());
        hasher.update(payload);
        let result = hasher.finalize();
        
        let mut checksum = [0u8; 32];
        checksum.copy_from_slice(&result);
        
        OrchestratedSignature {
            signer_id: self.id,
            checksum,
        }
    }

    /// Verifica assinatura.
    pub fn verify_signature(&self, payload: &[u8], signature: &OrchestratedSignature) -> bool {
        if signature.signer_id != self.id {
            return false;
        }
        
        let expected = self.sign(payload);
        expected.checksum == signature.checksum
    }
}

impl std::fmt::Display for UidOrchestrated {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GDC:{}", &self.id.to_string()[..8])
    }
}

/// Assinatura orquestrada para rastreabilidade.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrchestratedSignature {
    /// ID do assinante
    pub signer_id: Uuid,
    /// Checksum
    pub checksum: [u8; 32],
}

impl OrchestratedSignature {
    /// Assinatura vazia.
    pub fn empty() -> Self {
        Self {
            signer_id: Uuid::nil(),
            checksum: [0u8; 32],
        }
    }

    /// Verifica se vazia.
    pub fn is_empty(&self) -> bool {
        self.signer_id.is_nil()
    }
}

/// Referência a uma orquestração.
///
/// # SEM STRINGS HUMANAS
/// O identificador é um hash, não uma string legível.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrchestrationRef {
    /// ID da orquestração (hash)
    pub id: Uuid,
    /// Contexto para derivação
    pub context: [u8; 16],
}

impl OrchestrationRef {
    /// Cria referência a partir de identificador.
    ///
    /// O identificador é convertido em hash, não armazenado como string.
    pub fn new(identifier: &[u8]) -> Self {
        let id = Uuid::new_v5(&Uuid::NAMESPACE_DNS, identifier);
        
        let mut hasher = Sha256::new();
        hasher.update(b"orchestration:");
        hasher.update(identifier);
        let result = hasher.finalize();
        
        let mut context = [0u8; 16];
        context.copy_from_slice(&result[..16]);
        
        Self { id, context }
    }

    /// Cria referência para GDC isolado.
    pub fn isolated() -> Self {
        Self {
            id: Uuid::nil(),
            context: [0u8; 16],
        }
    }

    /// Verifica se isolado.
    pub fn is_isolated(&self) -> bool {
        self.id.is_nil()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orchestrated_determinism() {
        let entropy = b"test-entropy";
        let orchestration = OrchestrationRef::new(b"test-orch");
        
        let uid1 = UidOrchestrated::new(entropy, &orchestration);
        let uid2 = UidOrchestrated::new(entropy, &orchestration);
        
        // Mesma entropia + orquestração = mesmo UID
        assert_eq!(uid1.as_uuid(), uid2.as_uuid());
    }

    #[test]
    fn test_different_orchestrations() {
        let entropy = b"same-entropy";
        let orch_a  = OrchestrationRef::new(b"orchestration-a");
        let orch_b = OrchestrationRef::new(b"orchestration-b");
        
        let uid_a = UidOrchestrated::new(entropy, &orch_a );
        let uid_b = UidOrchestrated::new(entropy, &orch_b);
        
        // Orquestrações diferentes = UIDs diferentes
        assert_ne!(uid_a.as_uuid(), uid_b.as_uuid());
    }

    #[test]
    fn test_signature() {
        let uid = UidOrchestrated::isolated(b"test");
        
        let payload = b"test-payload";
        let signature = uid.sign(payload);
        
        assert!(uid.verify_signature(payload, &signature));
        assert!(!uid.verify_signature(b"wrong", &signature));
    }

    #[test]
    fn test_isolated() {
        let orch  = OrchestrationRef::isolated();
        assert!(orch.is_isolated());
    }
}
