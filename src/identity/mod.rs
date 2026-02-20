//! --------------------------
//! INFORMATION
//! --------------------------
//! Title: Identity Module - Sistema de Identidade (SANITIZADO)
//! Author: Digital Genome Community
//! Date: 2026-02-04
//! Version: 0.8.5-sanitized
//!
//! # Dois Planos DISJUNTOS
//!
//! | Plano | UID | Trafega? | Declara? |
//! |-------|-----|----------|----------|
//! | Ontológico | Shibboleth | ❌ NUNCA | ❌ NUNCA |
//! | Funcional | Orquestrado | ✅ SIM | N/A |
//!
//! # VIBRAÇÃO DE PUREZA
//!
//! Pureza é INFERIDA, nunca declarada:
//! - GDC puro RESPONDE (executa operações)
//! - GDC violado SILENCIA (retorna None)
//!
//! Canon: AO-SHIBBOLETH, AO-RESSONANTE, LEI-RESS-01
//! --------------------------

mod shibboleth;
pub mod orchestrated;

pub use shibboleth::{UidShibboleth, OperationGuard};
pub use orchestrated::{UidOrchestrated, OrchestratedSignature, OrchestrationRef};

use std::sync::Arc;

/// Par de identidade completo de um GDC.
///
/// # Planos Disjuntos
///
/// - Shibboleth: ontológico, NUNCA trafega
/// - Orquestrado: funcional, PODE trafegar
///
/// # Vibração de Pureza
///
/// A pureza não é declarada. É inferida:
/// - `try_execute()` retorna Some → puro
/// - `try_execute()` retorna None → violado (silêncio)
#[derive(Clone)]
pub struct GdcIdentity {
    /// Plano ontológico - NUNCA expor
    shibboleth: Arc<UidShibboleth>,
    /// Plano funcional - pode ser compartilhado
    orchestrated: UidOrchestrated,
    /// Referência da orquestração
    orchestration: OrchestrationRef,
}

impl GdcIdentity {
    /// Cria nova identidade.
    ///
    /// # Entropias SEPARADAS
    /// - `ontological_entropy`: APENAS para Shibboleth
    /// - `functional_entropy`: APENAS para Orquestrado
    ///
    /// NÃO há derivação entre elas.
    pub fn new(
        ontological_entropy: &[u8],
        functional_entropy: &[u8],
        orchestration: OrchestrationRef,
    ) -> Self {
        Self {
            shibboleth: Arc::new(UidShibboleth::new(ontological_entropy)),
            orchestrated: UidOrchestrated::new(functional_entropy, &orchestration),
            orchestration,
        }
    }

    /// Cria identidade isolada (conveniência).
    pub fn isolated(entropy: &[u8]) -> Self {
        // Deriva entropia funcional (conveniência, não dependência)
        let mut functional = Vec::from(entropy);
        functional.extend_from_slice(b":f");
        
        Self::new(entropy, &functional, OrchestrationRef::isolated())
    }

    /// Retorna UID Orquestrado (seguro para compartilhar).
    pub fn orchestrated(&self) -> &UidOrchestrated {
        &self.orchestrated
    }

    /// Retorna orquestração.
    pub fn orchestration(&self) -> &OrchestrationRef {
        &self.orchestration
    }

    /// Tenta executar operação.
    ///
    /// # VIBRAÇÃO DE PUREZA
    ///
    /// - Se puro: executa e retorna Some(resultado)
    /// - Se violado: silencia e retorna None
    ///
    /// Esta é a ÚNICA forma de inferir pureza.
    /// NÃO há método que "declara" se está puro.
    #[inline]
    pub fn try_execute<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce() -> R,
    {
        self.shibboleth.execute_if_alive(f)
    }

    /// Verifica se pode operar (uso interno).
    ///
    /// NOTA: Prefira `try_execute()` para vibração correta.
    #[inline]
    pub fn can_operate(&self) -> bool {
        self.shibboleth.can_operate()
    }

    /// Cria guarda de operação.
    pub fn operation_guard(&self) -> OperationGuard<'_> {
        OperationGuard::new(&self.shibboleth)
    }

    /// Dispara morte (silêncio absoluto).
    ///
    /// IRREVERSÍVEL. Sem log. Sem erro.
    pub fn trigger_death(&self) {
        self.shibboleth.trigger_death();
    }

    /// Assina payload.
    pub fn sign(&self, payload: &[u8]) -> OrchestratedSignature {
        self.orchestrated.sign(payload)
    }

    /// Troca de orquestração.
    ///
    /// O Shibboleth NÃO é afetado (planos disjuntos).
    pub fn change_orchestration(&mut self, new_orchestration: OrchestrationRef, new_entropy: &[u8]) {
        if !self.shibboleth.can_operate() {
            return; // Silêncio
        }
        
        self.orchestrated = UidOrchestrated::new(new_entropy, &new_orchestration);
        self.orchestration = new_orchestration;
    }
}

impl std::fmt::Debug for GdcIdentity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GdcIdentity")
            .field("orchestrated", &self.orchestrated)
            .field("shibboleth", &self.shibboleth)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alive_gdc_executes() {
        let identity = GdcIdentity::isolated(b"secret");
        
        let result = identity.try_execute(|| 42);
        assert_eq!(result, Some(42)); // Responde
    }

    #[test]
    fn test_dead_gdc_silences() {
        let identity = GdcIdentity::isolated(b"secret");
        
        identity.trigger_death();
        
        let result = identity.try_execute(|| 42);
        assert_eq!(result, None); // Silêncio
    }

    #[test]
    fn test_planes_are_independent() {
        let orch = OrchestrationRef::new(b"test");
        
        let id1 = GdcIdentity::new(b"onto", b"func-1", orch.clone());
        let id2 = GdcIdentity::new(b"onto", b"func-2", orch.clone());
        
        // Orquestrados diferentes (entropias diferentes)
        assert_ne!(id1.orchestrated().as_uuid(), id2.orchestrated().as_uuid());
    }

    #[test]
    fn test_orchestration_change() {
        let mut identity = GdcIdentity::isolated(b"secret");
        
        let old_uid = *identity.orchestrated().as_uuid();
        
        identity.change_orchestration(
            OrchestrationRef::new(b"production"),
            b"new-entropy"
        );
        
        // UID mudou, GDC continua operando
        assert_ne!(old_uid, *identity.orchestrated().as_uuid());
        assert!(identity.can_operate());
    }
}
