//! --------------------------
//! INFORMATION
//! --------------------------
//! Title: Coordination Module - Distribuição Computacional (SANITIZADO)
//! Author: Digital Genome Community
//! Date: 2026-02-04
//! Version: 0.8.5-sanitized
//!
//! # Modelo de Coordenação
//!
//! - Evento = Forma (Σ), não instância
//! - Campo = Operador, não conjunto
//! - Capacidades = Códigos, não strings
//!
//! Canon: LEI-COORD-01, LEI-COORD-02, DLB-013, DLB-014
//! --------------------------

pub mod event;
pub mod field;
pub mod manifestation;
pub mod edr;
pub mod gdc;
pub mod containment;
pub mod multi_field;
pub mod multi_gdc;

// v1.0.0δ - Enxame Descentralizado
pub mod budget_delta;
pub mod vibration;
pub mod manifestation_delta;
pub mod fragmentation;
pub mod weaving;

pub use event::{Event, WorkDescriptor, WorkType};
pub use field::{Field, IntegratedState, IntegrationResult, CapabilityState, CognitiveResult, MotorScores};
pub use manifestation::{Manifestation, CognitiveResultBuilder};
pub use edr::{Edr, EdrContent, EdrVersion, ValidatedEdr, EdrBuilder};
pub use gdc::{Gdc, GdcState, GdcStats, EmittedDna, StateTransitionError, QueenError, WorkerError};

#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::identity::OrchestrationRef;

    /// Teste: fluxo completo Queen → Worker → DNA
    #[test]
    fn test_full_coordination_flow() {
        let orchestration = OrchestrationRef::new(b"test-orch");
        let mut queen = Gdc::new(b"queen-secret", orchestration.clone());
        let mut worker1 = Gdc::new(b"worker1-secret", orchestration.clone());
        let mut worker2 = Gdc::new(b"worker2-secret", orchestration.clone());

        // 1. Criar evento
        let work = WorkDescriptor::new(WorkType::CognitiveEvaluation, b"task-data")
            .require_capability_code(1);
        let event = Event::from_work(work);

        // 2. Rainha assume
        queen.become_queen(event.clone()).unwrap();
        assert_eq!(queen.state(), GdcState::Queen);

        // 3. Workers aceitam
        let queen_uid = *queen.identity().orchestrated().as_uuid();
        worker1.become_worker(event.clone(), queen_uid).unwrap();
        worker2.become_worker(event.clone(), queen_uid).unwrap();

        // 4. Workers processam
        let edr1 = worker1.process_work(|e| {
            let scores = MotorScores {
                praxeological: 0.8,
                nash: 0.7,
                chaotic: 0.9,
                meristic: 0.6,
            };
            let result = CognitiveResultBuilder::new()
                .with_scores(scores)
                .build();
            
            Manifestation::for_call(*e.call_signature())
                .with_capability_code(1, CapabilityState::satisfied())
                .with_cognitive_result(result)
        }).unwrap();

        let edr2 = worker2.process_work(|e| {
            let scores = MotorScores {
                praxeological: 0.9,
                nash: 0.8,
                chaotic: 0.7,
                meristic: 0.8,
            };
            let result = CognitiveResultBuilder::new()
                .with_scores(scores)
                .build();
            
            Manifestation::for_call(*e.call_signature())
                .with_capability_code(1, CapabilityState::satisfied())
                .with_cognitive_result(result)
        }).unwrap();

        // 5. Rainha recebe
        let result1 = queen.receive_edr(edr1).unwrap();
        assert_eq!(result1, IntegrationResult::Integrated);

        let result2 = queen.receive_edr(edr2).unwrap();
        assert_eq!(result2, IntegrationResult::Integrated);

        // 6. Rainha emite DNA
        assert!(queen.can_emit_dna());
        let dna = queen.emit_dna().unwrap();

        assert_eq!(dna.manifestation_count, 2);
        assert!(dna.craft_performance > 0.0);
        assert_eq!(queen.state(), GdcState::Idle);
    }

    /// Teste: idempotência
    #[test]
    fn test_manifestation_idempotency() {
        let orchestration = OrchestrationRef::new(b"test");
        let mut queen = Gdc::new(b"queen", orchestration.clone());
        let mut worker = Gdc::new(b"worker", orchestration.clone());

        let work = WorkDescriptor::new(WorkType::CognitiveEvaluation, b"data");
        let event = Event::from_work(work);

        queen.become_queen(event.clone()).unwrap();
        let queen_uid = *queen.identity().orchestrated().as_uuid();
        
        worker.become_worker(event.clone(), queen_uid).unwrap();
        let edr1 = worker.process_work(|e| {
            Manifestation::for_call(*e.call_signature())
                .with_capability_code(1, CapabilityState::satisfied())
        }).unwrap();
        worker.finish_work();

        worker.become_worker(event.clone(), queen_uid).unwrap();
        let edr2 = worker.process_work(|e| {
            Manifestation::for_call(*e.call_signature())
                .with_capability_code(1, CapabilityState::satisfied())
        }).unwrap();

        let r1 = queen.receive_edr(edr1).unwrap();
        assert_eq!(r1, IntegrationResult::Integrated);

        let r2 = queen.receive_edr(edr2).unwrap();
        assert_eq!(r2, IntegrationResult::Redundant);
    }

    /// Teste: apoptose silenciosa
    #[test]
    fn test_edr_apoptosis() {
        let orchestration = OrchestrationRef::new(b"test");
        let mut queen = Gdc::new(b"queen", orchestration.clone());
        let worker = Gdc::new(b"worker", orchestration.clone());

        let work = WorkDescriptor::new(WorkType::CognitiveEvaluation, b"data");
        let event = Event::from_work(work);
        let wrong_event = Event::from_work(
            WorkDescriptor::new(WorkType::CognitiveEvaluation, b"wrong")
        );

        queen.become_queen(event.clone()).unwrap();

        // EDR com call_signature errada
        let manifestation = Manifestation::for_call(*wrong_event.call_signature());
        let edr = Edr::new(
            *wrong_event.call_signature(),
            worker.identity().orchestrated(),
            worker.identity().orchestration(),
            manifestation,
        );

        // Apoptose silenciosa
        let result = queen.receive_edr(edr);
        assert!(matches!(result, Err(QueenError::InvalidEdr)));
    }
}
