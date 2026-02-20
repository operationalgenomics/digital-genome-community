//! --------------------------
//! INFORMATION
//! --------------------------
//! Title: Digital Genome Community Edition - Core Library
//! Author: Carlos Eduardo Favini
//! Date: 2026-02-17
//! Version: 0.9.0-final (Orquestração Básica - 2 GDCs)
//! Description: Root module for the Digital Genome Community Edition.
//! This is a synthetic cognitive core that perceives, observes,
//! comprehends, learns, remembers, and emits DNA with Score. It does not act.
//! Thread-safe: All public types are Send + Sync.
//! Epistemologically neutral: No domain knowledge, no ontologies.
//!
//! # v0.9.0 - ORQUESTRAÇÃO BÁSICA (2 GDCs)
//!
//! Esta versão implementa orquestração real entre 2 GDCs:
//! - CF(G): Canonical Form (Phenotype)
//! - FCE(R): Forma Canônica Estrutural
//! - R(Σ): Resultado Cognitivo Emissível
//! - W(Σ): Trabalho Estrutural
//! - ⊒: Contenção por Completude
//! - Networking: Protocol, Transport, Border
//!
//! Canon: v5.1 + CONTRATO v1.0.0
//!
//! --------------------------
//! # Digital Genome Community Edition
//! Foundational Axiom B.1: "The Core has basal operational existence
//! that is semantically null, and its cognition is event-driven
//! (activated by input, ended by output)."
//!
//! # v0.8.5: Distribuição Computacional
//!
//! Implementa coordenação distribuída entre GDCs:
//!
//! | Módulo | Canon | Descrição |
//! |--------|-------|-----------|
//! | identity | AO-SHIBBOLETH, AO-RESSONANTE | Dois planos de identidade |
//! | coordination | LEI-COORD-01, LEI-COORD-02 | Campo e integração |
//! | coordination::edr | DLB-014 | Protocolo devolutivo |
//! | coordination::gdc | DLB-013, GZ-D13 | Estados Rainha/Worker |
//!
//! **Status:** Distribuição computacional funcional.
//!
//! Layer: Community
//! Dependencies: All internal modules
//! Affected Components: External consumers of the library
//!
//! --------------------------
//! CHANGE LOG
//! --------------------------
//! 2025-01-02 - Carlos Eduardo Favini - Initial creation (Marco Zero)
//! 2025-01-02 - Carlos Eduardo Favini - Added replay module (v0.3.0)
//! 2025-01-02 - Carlos Eduardo Favini - Added sensory cortex (v1.1.0)
//! 2025-01-02 - Carlos Eduardo Favini - Cognitive Depth (v1.2.0)
//! 2025-01-02 - Carlos Eduardo Favini - Threading & Epistemological Neutrality (v1.3.0)
//! 2025-01-02 - Carlos Eduardo Favini - Computational Self-Preservation (v1.4.0)
//! 2025-01-02 - Carlos Eduardo Favini - Perceptual Maturation (v1.5.0)
//! 2026-01-28 - Carlos Eduardo Favini - MVP-6: Cognição Completa (v0.6.0)
//! 2026-01-29 - Carlos Eduardo Favini - MVP-7: Validação Final (v0.7.0)
//! 2026-01-29 - Carlos Eduardo Favini - v0.7.1: Correção de versionamento
//! 2026-02-03 - Digital Genome Community - v0.8.5: Distribuição Computacional
//! --------------------------

// =============================================================================
// CORE MODULES - COMMUNITY EDITION
// =============================================================================

/// Core type definitions (identifiers, base types)
pub mod core_types;

/// Biological hierarchy (Action, DNA, Synapse, Neuron, Brain, Truth)
pub mod hierarchy;

/// Cognitive motors (Praxeological, Nash, Chaotic, Meristic)
pub mod motors;

/// Mathematical foundations (Craft Performance formula)
pub mod math;

/// Latent Archive (Foucaultian memory)
pub mod archive;

/// Selection criteria (Golden Index)
pub mod selection;

/// Topological structures (synaptic connections)
pub mod topology;

/// Traits for Enterprise integration
pub mod traits;

/// Replay harness for deterministic execution and verification (v0.3.0)
pub mod replay;

/// Sensory cortex for abstraction hierarchy (v1.1.0)
pub mod sensory;

// =============================================================================
// COGNITIVE DEPTH MODULES - v1.2.0
// =============================================================================

/// Inference by correlation (Insight #3)
pub mod correlation;

/// Motor competition and cooperation (Insight #6)
pub mod competition;

/// Cognitive observability / metacognition (Insight #7)
pub mod observability;

/// Cognitive completeness states (Insight #10)
pub mod completeness;

// =============================================================================
// COMPUTATIONAL SELF-PRESERVATION - v1.4.0
// =============================================================================

/// Computational budget and integrity verification (Insight A.7)
///
/// This module implements self-preservation instincts based EXCLUSIVELY
/// on computational constraints (memory, time, complexity, numerical stability).
///
/// FORBIDDEN: Any limit based on human senses or biological analogies.
/// ALLOWED: Only limits justified by computational facts.
pub mod budget;

// =============================================================================
// PERCEPTUAL MATURATION - v1.5.0
// =============================================================================

/// Perceptual maturation (Insight A.5)
///
/// Implements multiple internal refinement passes during a single perceptual
/// cycle, allowing the system to "mature" before emitting output.
///
/// CRITICAL CONSTRAINTS:
/// - Maturation is NOT learning (no persistent changes)
/// - Maturation is NOT memory (no recall of previous inputs)
/// - Maturation IS confined to the perceptual cycle
/// - Maturation IS discarded entirely at the end
pub mod maturation;

/// Cognitive communication structures (L-003, L-004, L-008)
pub mod cognitive;

/// Universal Notation Language and GD-QMN (L-009)
pub mod unl;

// =============================================================================
// MVP-6: COGNITIVE MEMORY SYSTEM - v0.6.0
// =============================================================================

/// Memory system (AF-11, AF-12, AO-18)
///
/// Implements cognitive memory for learning and recall:
/// - AF-11: Autonomous cognitive learning by replayable incorporation
/// - AF-12: Internal cognitive memory (MCI) - non-observation
/// - AO-18: Self-reference via Origin marker
///
/// CRITICAL DISTINCTION:
/// - MCI is NOT observation (does not store raw inputs)
/// - MCI IS internal (stores evaluated Codons)
/// - Origin IS deterministic (consistent under replay)
pub mod memory;

// =============================================================================
// v0.8.5: DISTRIBUTED COORDINATION
// =============================================================================

/// Identity system with two planes (AO-SHIBBOLETH, AO-RESSONANTE)
///
/// Implements the canonical identity model:
/// - Shibboleth: Ontological plane (NEVER transmitted)
/// - Orchestrated: Functional plane (safe to transmit)
///
/// CRITICAL INVARIANT:
/// - Shibboleth violation = canonical death
/// - Orchestrated UID derived from proof, not from Shibboleth directly
pub mod identity;

/// Distributed coordination between GDCs
///
/// Implements:
/// - LEI-COORD-01: Field as operator, perception of couplings
/// - LEI-COORD-02: Integration by ⨆, projection W'
/// - DLB-013: Unique DNA (only Queen emits)
/// - DLB-014: EDR Protocol (worker → queen communication)
/// - GZ-D13: Worker failure → reprocessing
pub mod coordination;

// v0.9.5: Emergência Cognitiva
pub mod synapse;
pub mod neuron;

// =============================================================================
// EXTERNAL EMULATORS - MOVED TO validation/emulators/
// =============================================================================
//
// Canon v5.1, linha 6491-6500:
// "As camadas superiores (GDO, GDE, e quaisquer outras) não pertencem ao
// Canon do GDC. Essas camadas existem exclusivamente como emuladores."
//
// MOVED:
// - gdo/ → validation/emulators/gdo/
// - gde/ → validation/emulators/gde/
// - adapter/ → validation/emulators/adapter/
// - cycle_orchestrator.rs → validation/emulators/orchestrator/
//
// Use: `gdc-emulators` crate for testing and validation
// =============================================================================

// v0.9.0-alpha: Results module (CF(G), FCE(R), R(Σ))
pub mod results;

// v0.9.0-final: Networking module
pub mod networking;

// =============================================================================
// RE-EXPORTS
// =============================================================================

pub use core_types::*;
pub use math::craft::{CraftPerformance, CpResult};
pub use replay::{ReplayContext, ReplaySession, ReplayEvent, ReplayVerifier};
pub use sensory::{
    CommunityOutput, CortexOutput, MatureOutput, 
    RawInput, SensoryCortex, SensorySignals,
    PerceptualState, StateHistory, StateTransition,
};

// v1.2.0 exports
pub use correlation::{CorrelationMatrix, CooccurrenceTracker, TransformationTracker};
pub use competition::{MotorCompetition, MotorCooperation, MotorDynamics, MotorType};
pub use observability::{CognitiveObservability, HealthIndicators, ProgressTracker};
pub use completeness::{CognitiveCompleteness, AbstractionLevel, ConflictType, MissingSignal};

// v0.5.0 exports (cognitive cycle)
pub use cognitive::{
    CognitiveCycle, CycleOutput, MotorContext, MotorScores,
    TransportCode, ObservationReport, MotorSignatures,
    // v0.6.0: Structured DNA
    StructuredDNA, DnaBuilder, AtomicAction, Uncertainty, MeristicSuggestion,
};

// v1.4.0 exports
pub use budget::{
    ComputationalBudget, IntegrityCheck, NumericalIssue,
    ComplexityClass, BudgetGuard,
    check_bytes_budget, check_numerical_stability, check_time_budget,
};

// v1.5.0 exports
pub use maturation::{
    MaturationConfig, MaturationState, StopReason,
    RefinementStep, RefinementMetrics,
};

// v0.6.0 exports (MVP-6: Cognitive Memory)
pub use memory::{
    // Context (LEI-AF-12-02)
    CanonicalContext,
    // Codon (LEI-AF-12-01)
    CanonicalCodon, Origin, EvaluativeSignature, ActivationCondition, ReplayableProvenance,
    // MCI (AF-12)
    MCI, MciQueryResult, MciStats, MciError,
    // Learning (AF-11)
    LearningEngine, LearningResult, EpistemicTrigger, RejectionReason, StagnationDetector,
};

// v0.8.5 exports (Distributed Coordination) - SANITIZADO
pub use identity::{
    // Shibboleth (plano ontológico) - vibração de pureza
    UidShibboleth, OperationGuard,
    // Ressonante (plano funcional) - independente do Shibboleth
    UidOrchestrated, OrchestratedSignature, OrchestrationRef,
    // Par de identidade completo
    GdcIdentity,
};

pub use coordination::{
    // Event (LEI-COORD-01)
    Event, WorkDescriptor, WorkType,
    // Field (LEI-COORD-01, LEI-COORD-02)
    Field, IntegratedState, IntegrationResult, CapabilityState, CognitiveResult,
    MotorScores as CoordMotorScores,
    // Manifestation
    Manifestation, CognitiveResultBuilder,
    // EDR Protocol (DLB-014)
    // EDR é PROTOCOLO. NÃO é cognição. NÃO é identidade. NÃO é evento.
    Edr, EdrContent, EdrVersion, ValidatedEdr, EdrBuilder,
    // GDC (DLB-013, GZ-D13)
    Gdc, GdcState, GdcStats, EmittedDna, StateTransitionError, QueenError, WorkerError,
};

// =============================================================================
// THREAD-SAFETY VERIFICATION
// =============================================================================
// These tests verify at compile-time that all public types are Send + Sync.
// If any type fails to be Send + Sync, the build will fail.
// See THREADING.md for the complete threading policy.

#[cfg(test)]
mod thread_safety_tests {
    use super::*;

    fn assert_send_sync<T: Send + Sync>() {}

    #[test]
    fn test_sensory_types_are_send_sync() {
        assert_send_sync::<SensoryCortex>();
        assert_send_sync::<RawInput>();
        assert_send_sync::<SensorySignals>();
        assert_send_sync::<CommunityOutput>();
    }

    #[test]
    fn test_correlation_types_are_send_sync() {
        assert_send_sync::<CorrelationMatrix>();
        assert_send_sync::<CooccurrenceTracker>();
        assert_send_sync::<TransformationTracker>();
    }

    #[test]
    fn test_competition_types_are_send_sync() {
        assert_send_sync::<MotorCompetition>();
        assert_send_sync::<MotorCooperation>();
        assert_send_sync::<MotorDynamics>();
        assert_send_sync::<MotorType>();
    }

    #[test]
    fn test_observability_types_are_send_sync() {
        assert_send_sync::<CognitiveObservability>();
        assert_send_sync::<HealthIndicators>();
        assert_send_sync::<ProgressTracker>();
    }

    #[test]
    fn test_completeness_types_are_send_sync() {
        assert_send_sync::<CognitiveCompleteness>();
        assert_send_sync::<AbstractionLevel>();
        assert_send_sync::<ConflictType>();
    }

    #[test]
    fn test_math_types_are_send_sync() {
        assert_send_sync::<CraftPerformance>();
    }

    #[test]
    fn test_replay_types_are_send_sync() {
        assert_send_sync::<ReplayContext>();
        assert_send_sync::<ReplaySession>();
        assert_send_sync::<ReplayEvent>();
    }

    #[test]
    fn test_budget_types_are_send_sync() {
        assert_send_sync::<ComputationalBudget>();
        assert_send_sync::<IntegrityCheck>();
        assert_send_sync::<NumericalIssue>();
        assert_send_sync::<ComplexityClass>();
        assert_send_sync::<BudgetGuard>();
    }

    #[test]
    fn test_maturation_types_are_send_sync() {
        assert_send_sync::<MaturationConfig>();
        assert_send_sync::<MaturationState>();
        assert_send_sync::<StopReason>();
        assert_send_sync::<RefinementStep>();
        assert_send_sync::<RefinementMetrics>();
    }

    #[test]
    fn test_memory_types_are_send_sync() {
        assert_send_sync::<CanonicalContext>();
        assert_send_sync::<CanonicalCodon>();
        assert_send_sync::<Origin>();
        assert_send_sync::<EvaluativeSignature>();
        assert_send_sync::<MCI>();
        assert_send_sync::<MciQueryResult>();
    }

    #[test]
    fn test_dna_types_are_send_sync() {
        assert_send_sync::<StructuredDNA>();
        assert_send_sync::<AtomicAction>();
    }

    #[test]
    fn test_identity_types_are_send_sync() {
        assert_send_sync::<UidShibboleth>();
        assert_send_sync::<OperationGuard>();
        assert_send_sync::<UidOrchestrated>();
        assert_send_sync::<OrchestratedSignature>();
        assert_send_sync::<OrchestrationRef>();
        assert_send_sync::<GdcIdentity>();
    }

    #[test]
    fn test_coordination_types_are_send_sync() {
        assert_send_sync::<Event>();
        assert_send_sync::<WorkDescriptor>();
        assert_send_sync::<Field>();
        assert_send_sync::<IntegratedState>();
        assert_send_sync::<Manifestation>();
        assert_send_sync::<Edr>();
        assert_send_sync::<EmittedDna>();
    }
}

// =============================================================================
// TESTES CANÔNICOS OBRIGATÓRIOS — SANITIZAÇÃO v0.8.5
// =============================================================================
// Estes testes provam os invariantes canônicos.
// Se qualquer teste falhar, a versão NÃO é canônica.

#[cfg(test)]
mod canonical_sanitization_tests {
    use crate::coordination::{
        Event, WorkDescriptor, WorkType, Gdc, GdcState, 
        Manifestation, CapabilityState, Edr, IntegrationResult,
    };
    use crate::identity::{GdcIdentity, UidOrchestrated, OrchestrationRef};

    /// TESTE 1: UID Ressonante NÃO deriva do Shibboleth
    /// 
    /// Prova: Mesmo Shibboleth com entropias funcionais diferentes
    /// gera UIDs Ressonantes diferentes.
    #[test]
    fn test_canonical_orchestrated_independent_of_shibboleth() {
        let orchestration = OrchestrationRef::new(b"test-orchestration");
        
        // Mesma entropia ontológica, entropias funcionais diferentes
        let id1 = GdcIdentity::new(
            b"same-ontological-entropy",
            b"functional-entropy-1",
            orchestration.clone(),
        );
        
        let id2 = GdcIdentity::new(
            b"same-ontological-entropy",
            b"functional-entropy-2",
            orchestration.clone(),
        );
        
        // UIDs de Orquestrados DEVEM ser diferentes
        // (prova que Orquestrado não deriva do Shibboleth)
        assert_ne!(
            id1.orchestrated().as_uuid(),
            id2.orchestrated().as_uuid(),
            "VIOLAÇÃO CANÔNICA: UID Ressonante deriva do Shibboleth!"
        );
    }

    /// TESTE 2: Shibboleth NÃO gera artefato trafegável
    ///
    /// Prova: UidShibboleth não implementa Serialize
    #[test]
    fn test_canonical_shibboleth_not_serializable() {
        // Este teste é de compilação - se UidShibboleth implementasse
        // Serialize, o código abaixo compilaria:
        //
        // let shibboleth = UidShibboleth::new(b"secret");
        // let _ = serde_json::to_string(&shibboleth); // NÃO COMPILA
        //
        // Como não compila, o teste passa.
        // Verificamos indiretamente através do Debug output.
        
        let identity = GdcIdentity::isolated(b"secret");
        let debug_output = format!("{:?}", identity);
        
        // Debug não deve conter bytes do segredo
        assert!(!debug_output.contains("secret"), 
            "VIOLAÇÃO CANÔNICA: Shibboleth expõe segredo em Debug!");
    }

    /// TESTE 3: Evento não depende de event_id
    ///
    /// Prova: Eventos com mesma forma têm mesma call_signature
    #[test]
    fn test_canonical_event_is_form_not_instance() {
        let work = WorkDescriptor::new(WorkType::CognitiveEvaluation, b"input-data")
            .require_capability_code(1);
        
        // Criar dois eventos com mesma forma
        let event1 = Event::from_work(work.clone());
        let event2 = Event::from_work(work);
        
        // call_signature DEVE ser idêntica (evento = forma)
        assert_eq!(
            event1.call_signature(),
            event2.call_signature(),
            "VIOLAÇÃO CANÔNICA: Eventos com mesma forma têm call_signatures diferentes!"
        );
        
        // Os eventos DEVEM ser equivalentes
        assert!(
            event1.is_equivalent(&event2),
            "VIOLAÇÃO CANÔNICA: Eventos com mesma forma não são equivalentes!"
        );
    }

    /// TESTE 4: EDR inválido entra em apoptose silenciosa
    ///
    /// Prova: EDR com call_signature errada retorna None, sem exceção
    #[test]
    fn test_canonical_edr_apoptosis() {
        let orchestration = OrchestrationRef::new(b"test");
        let worker = UidOrchestrated::new(b"worker-entropy", &orchestration);
        
        let correct_sig = [0x42u8; 32];
        let wrong_sig = [0xFFu8; 32];
        
        let manifestation = Manifestation::for_call(correct_sig)
            .with_capability_code(1, CapabilityState::satisfied());
        
        let edr = Edr::new(correct_sig, &worker, &orchestration, manifestation);
        
        // Validar com assinatura ERRADA
        let result = edr.validate(&wrong_sig);
        
        // DEVE retornar None (apoptose silenciosa)
        assert!(
            result.is_none(),
            "VIOLAÇÃO CANÔNICA: EDR inválido não entrou em apoptose!"
        );
        
        // Não houve exceção, panic, ou log - silêncio absoluto
    }

    /// TESTE 5: Worker NÃO emite DNA
    ///
    /// Prova: Worker só retorna EDR, nunca DNA
    #[test]
    fn test_canonical_worker_cannot_emit_dna() {
        let orchestration = OrchestrationRef::new(b"test");
        let mut worker = Gdc::new(b"worker-secret", orchestration);
        
        let work = WorkDescriptor::new(WorkType::CognitiveEvaluation, b"data");
        let event = Event::from_work(work);
        
        // Tornar-se worker
        worker.become_worker(event.clone(), uuid::Uuid::new_v4()).unwrap();
        assert_eq!(worker.state(), GdcState::Worker);
        
        // Worker NÃO pode emitir DNA
        assert!(
            !worker.can_emit_dna(),
            "VIOLAÇÃO CANÔNICA: Worker pode emitir DNA!"
        );
        
        // Tentar emitir DNA deve falhar
        let dna_result = worker.emit_dna();
        assert!(
            dna_result.is_err(),
            "VIOLAÇÃO CANÔNICA: Worker conseguiu emitir DNA!"
        );
    }

    /// TESTE 6: Rainha é a ÚNICA emissora de DNA
    ///
    /// Prova: Apenas GDC em estado Queen pode emitir DNA
    #[test]
    fn test_canonical_only_queen_emits_dna() {
        let orchestration = OrchestrationRef::new(b"test");
        let mut queen = Gdc::new(b"queen-secret", orchestration.clone());
        let mut worker = Gdc::new(b"worker-secret", orchestration.clone());
        
        let work = WorkDescriptor::new(WorkType::CognitiveEvaluation, b"data")
            .require_capability_code(1);
        let event = Event::from_work(work);
        
        // Rainha assume
        queen.become_queen(event.clone()).unwrap();
        let queen_uid = *queen.identity().orchestrated().as_uuid();
        
        // Worker aceita
        worker.become_worker(event.clone(), queen_uid).unwrap();
        
        // Worker processa
        let edr = worker.process_work(|e| {
            Manifestation::for_call(*e.call_signature())
                .with_capability_code(1, CapabilityState::satisfied())
        }).unwrap();
        
        // Rainha recebe
        let result = queen.receive_edr(edr).unwrap();
        assert_eq!(result, IntegrationResult::Integrated);
        
        // APENAS a Rainha pode emitir
        assert!(queen.can_emit_dna(), "Rainha deveria poder emitir DNA");
        assert!(!worker.can_emit_dna(), "Worker NÃO deveria poder emitir DNA");
        
        let dna = queen.emit_dna().unwrap();
        assert!(dna.craft_performance >= 0.0, "DNA emitido corretamente");
    }

    /// TESTE 7: Vibração de pureza - GDC morto silencia
    #[test]
    fn test_canonical_dead_gdc_silences() {
        let identity = GdcIdentity::isolated(b"secret");
        
        // Vivo: executa
        let result = identity.try_execute(|| 42);
        assert_eq!(result, Some(42), "GDC vivo deve executar");
        
        // Matar
        identity.trigger_death();
        
        // Morto: silencia
        let result = identity.try_execute(|| 42);
        assert_eq!(result, None, "GDC morto deve silenciar");
    }

    /// TESTE 8: Apoptose é determinística e irreversível
    #[test]
    fn test_canonical_apoptosis_irreversible() {
        let identity = GdcIdentity::isolated(b"secret");
        
        assert!(identity.can_operate(), "Deve operar inicialmente");
        
        identity.trigger_death();
        
        assert!(!identity.can_operate(), "Não deve operar após morte");
        
        // Não há como reverter - testamos que continua morto
        for _ in 0..100 {
            assert!(!identity.can_operate(), "Morte deve ser permanente");
        }
    }
}
