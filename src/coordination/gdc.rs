//! --------------------------
//! INFORMATION
//! --------------------------
//! Title: GDC States - Rainha e Worker
//! Author: Digital Genome Community
//! Date: 2026-02-03
//! Version: 0.8.5
//! Description: Implementa os estados operacionais do GDC (DLB-012, DLB-013).
//!
//! # Estados do GDC
//!
//! - IDLE: Aguardando ativação
//! - QUEEN: Coordenando evento (emite DNA)
//! - WORKER: Processando para uma Rainha (não emite DNA)
//!
//! # Regra Fundamental (DLB-013)
//!
//! APENAS A RAINHA EMITE DNA.
//! Workers devolvem cálculos e UNLs, NUNCA DNA.
//!
//! Canon: DLB-012, DLB-013, GZ-D13
//! --------------------------

use super::event::Event;
use super::field::Field;
use super::manifestation::Manifestation;
use super::edr::Edr;
use crate::identity::{GdcIdentity, OrchestrationRef};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Estado operacional do GDC.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum GdcState {
    /// Aguardando ativação
    #[default]
    Idle,
    /// Coordenando evento (rainha)
    Queen,
    /// Processando para rainha (worker)
    Worker,
}

/// GDC - Genoma Digital Cognitivo.
///
/// Instância completa de um nó cognitivo.
#[derive(Debug)]
pub struct Gdc {
    /// Identidade de dois planos
    identity: GdcIdentity,
    /// Estado operacional atual
    state: GdcState,
    /// Evento atual (quando Queen ou Worker)
    current_event: Option<Event>,
    /// Campo (quando Queen)
    field: Option<Field>,
    /// Rainha atual (quando Worker)
    queen_id: Option<Uuid>,
    /// Estatísticas
    stats: GdcStats,
}

impl Gdc {
    /// Cria um novo GDC.
    ///
    /// # Entropias Independentes
    /// 
    /// Por conveniência, este método deriva entropia funcional
    /// da entropia local. Isso NÃO significa derivação entre planos.
    pub fn new(local_entropy: &[u8], orchestration: OrchestrationRef) -> Self {
        // Derivar entropia funcional (conveniente, mas independente)
        let mut functional_entropy = Vec::from(local_entropy);
        functional_entropy.extend_from_slice(b":functional");
        
        Self {
            identity: GdcIdentity::new(local_entropy, &functional_entropy, orchestration),
            state: GdcState::Idle,
            current_event: None,
            field: None,
            queen_id: None,
            stats: GdcStats::default(),
        }
    }

    /// Cria GDC isolado (sem orquestração).
    pub fn isolated(local_entropy: &[u8]) -> Self {
        Self {
            identity: GdcIdentity::isolated(local_entropy),
            state: GdcState::Idle,
            current_event: None,
            field: None,
            queen_id: None,
            stats: GdcStats::default(),
        }
    }

    /// Retorna a identidade do GDC.
    pub fn identity(&self) -> &GdcIdentity {
        &self.identity
    }

    /// Retorna o estado atual.
    pub fn state(&self) -> GdcState {
        self.state
    }

    /// Verifica se está vivo (pode operar).
    pub fn is_alive(&self) -> bool {
        self.identity.can_operate()
    }

    // =========================================================================
    // TRANSIÇÕES DE ESTADO
    // =========================================================================

    /// Transição IDLE → QUEEN: Assume coordenação de evento.
    ///
    /// # Retorno
    /// - Ok: Transição bem-sucedida
    /// - Err: Motivo da falha
    pub fn become_queen(&mut self, event: Event) -> Result<(), StateTransitionError> {
        if !self.is_alive() {
            return Err(StateTransitionError::GdcDead);
        }

        if self.state != GdcState::Idle {
            return Err(StateTransitionError::InvalidTransition {
                from: self.state,
                to: GdcState::Queen,
            });
        }

        self.state = GdcState::Queen;
        self.field = Some(Field::new(event.clone()));
        self.current_event = Some(event);
        self.stats.queen_activations += 1;

        Ok(())
    }

    /// Transição IDLE → WORKER: Aceita trabalho de uma rainha.
    pub fn become_worker(&mut self, event: Event, queen_id: Uuid) -> Result<(), StateTransitionError> {
        if !self.is_alive() {
            return Err(StateTransitionError::GdcDead);
        }

        if self.state != GdcState::Idle {
            return Err(StateTransitionError::InvalidTransition {
                from: self.state,
                to: GdcState::Worker,
            });
        }

        self.state = GdcState::Worker;
        self.current_event = Some(event);
        self.queen_id = Some(queen_id);
        self.stats.worker_activations += 1;

        Ok(())
    }

    /// Transição * → IDLE: Retorna ao estado ocioso.
    pub fn become_idle(&mut self) {
        self.state = GdcState::Idle;
        self.current_event = None;
        self.field = None;
        self.queen_id = None;
    }

    // =========================================================================
    // OPERAÇÕES DE RAINHA
    // =========================================================================

    /// [QUEEN] Recebe EDR de um worker.
    ///
    /// # Retorno
    /// - Ok(IntegrationResult): Resultado da integração
    /// - Err: EDR inválido ou GDC não é rainha
    pub fn receive_edr(&mut self, edr: Edr) -> Result<super::field::IntegrationResult, QueenError> {
        if self.state != GdcState::Queen {
            return Err(QueenError::NotQueen);
        }

        let event = self.current_event.as_ref()
            .ok_or(QueenError::NoActiveEvent)?;
        
        let field = self.field.as_mut()
            .ok_or(QueenError::NoActiveEvent)?;

        // Validar EDR usando call_signature (CANÔNICO)
        let validated = edr.validate(event.call_signature())
            .ok_or(QueenError::InvalidEdr)?;

        // Extrair manifestação
        let manifestation = validated.into_manifestation()
            .ok_or(QueenError::InvalidEdr)?;

        // Integrar no campo
        let result = field.integrate(manifestation);
        
        self.stats.edrs_received += 1;

        Ok(result)
    }

    /// [QUEEN] Verifica se pode emitir DNA.
    pub fn can_emit_dna(&self) -> bool {
        if self.state != GdcState::Queen {
            return false;
        }

        self.field.as_ref()
            .map(|f| f.is_absorbing())
            .unwrap_or(false)
    }

    /// [QUEEN] Emite DNA único.
    ///
    /// # Invariante (DLB-013)
    /// APENAS a rainha pode chamar este método.
    /// Retorna o DNA estruturado e retorna ao estado IDLE.
    pub fn emit_dna(&mut self) -> Result<EmittedDna, QueenError> {
        if self.state != GdcState::Queen {
            return Err(QueenError::NotQueen);
        }

        if !self.can_emit_dna() {
            return Err(QueenError::NotAbsorbing);
        }

        let event = self.current_event.take()
            .ok_or(QueenError::NoActiveEvent)?;
        
        let field = self.field.take()
            .ok_or(QueenError::NoActiveEvent)?;

        // Construir DNA a partir do estado integrado
        let dna = EmittedDna::from_field(
            event,
            field,
            self.identity.orchestrated().clone(),
        );

        self.stats.dnas_emitted += 1;
        self.become_idle();

        Ok(dna)
    }

    // =========================================================================
    // OPERAÇÕES DE WORKER
    // =========================================================================

    /// [WORKER] Processa trabalho e gera manifestação.
    ///
    /// # Parâmetros
    /// - processor: Função que processa o evento e retorna manifestação
    ///
    /// # Retorno
    /// EDR pronto para enviar à rainha
    pub fn process_work<F>(&mut self, processor: F) -> Result<Edr, WorkerError>
    where
        F: FnOnce(&Event) -> Manifestation,
    {
        if self.state != GdcState::Worker {
            return Err(WorkerError::NotWorker);
        }

        let event = self.current_event.as_ref()
            .ok_or(WorkerError::NoActiveEvent)?;

        // Processar trabalho
        let manifestation = processor(event);

        // Criar EDR com call_signature (CANÔNICO)
        let edr = Edr::new(
            *event.call_signature(),
            self.identity.orchestrated(),
            self.identity.orchestration(),
            manifestation,
        );

        self.stats.manifestations_sent += 1;

        Ok(edr)
    }

    /// [WORKER] Finaliza trabalho e retorna ao IDLE.
    pub fn finish_work(&mut self) {
        if self.state == GdcState::Worker {
            self.become_idle();
        }
    }

    // =========================================================================
    // FALHA E REPROCESSAMENTO (GZ-D13)
    // =========================================================================

    /// Marca falha no processamento atual.
    ///
    /// Conforme GZ-D13: Falha de worker resulta em reprocessamento por outro.
    pub fn mark_failure(&mut self) {
        self.stats.failures += 1;
        self.become_idle();
    }

    /// Retorna estatísticas.
    pub fn stats(&self) -> &GdcStats {
        &self.stats
    }
}

/// DNA emitido pela rainha.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmittedDna {
    /// ID único do DNA
    pub id: Uuid,
    /// Call signature do evento (forma Σ)
    pub call_signature: [u8; 32],
    /// UID da rainha que emitiu
    pub queen_uid: Uuid,
    /// Craft Performance integrado
    pub craft_performance: f64,
    /// Dados do DNA (GD-QMN)
    pub data: Vec<u8>,
    /// Número de manifestações integradas
    pub manifestation_count: usize,
}

impl EmittedDna {
    /// Cria DNA a partir do campo integrado.
    fn from_field(
        event: Event,
        field: Field,
        queen_uid: crate::identity::UidOrchestrated,
    ) -> Self {
        use sha2::{Sha256, Digest};
        
        // Pegar count antes de consumir
        let count = field.manifestation_count();
        
        // Extrair estado e calcular CP
        let state = field.into_integrated_state();
        let cp = state.craft_performance();

        // Serializar dados
        let data = bincode::serialize(&state).unwrap_or_default();

        // Gerar ID do DNA
        let mut hasher = Sha256::new();
        hasher.update(event.call_signature());
        hasher.update(queen_uid.as_uuid().as_bytes());
        hasher.update(&data);
        let result = hasher.finalize();
        
        let mut id_bytes = [0u8; 16];
        id_bytes.copy_from_slice(&result[..16]);

        Self {
            id: Uuid::from_bytes(id_bytes),
            call_signature: *event.call_signature(),
            queen_uid: *queen_uid.as_uuid(),
            craft_performance: cp,
            data,
            manifestation_count: count,
        }
    }
}

/// Estatísticas do GDC.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GdcStats {
    /// Vezes que atuou como rainha
    pub queen_activations: u64,
    /// Vezes que atuou como worker
    pub worker_activations: u64,
    /// EDRs recebidos (como rainha)
    pub edrs_received: u64,
    /// Manifestações enviadas (como worker)
    pub manifestations_sent: u64,
    /// DNAs emitidos
    pub dnas_emitted: u64,
    /// Falhas registradas
    pub failures: u64,
}

/// Erro de transição de estado.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StateTransitionError {
    /// GDC está morto
    GdcDead,
    /// Transição inválida
    InvalidTransition { from: GdcState, to: GdcState },
}

/// Erro de operação de rainha.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QueenError {
    /// GDC não está no estado rainha
    NotQueen,
    /// Nenhum evento ativo
    NoActiveEvent,
    /// EDR inválido
    InvalidEdr,
    /// Não atingiu absorção
    NotAbsorbing,
}

/// Erro de operação de worker.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkerError {
    /// GDC não está no estado worker
    NotWorker,
    /// Nenhum evento ativo
    NoActiveEvent,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::coordination::event::{WorkDescriptor, WorkType};
    use crate::coordination::field::CapabilityState;

    fn create_test_event() -> Event {
        let work = WorkDescriptor::new(WorkType::CognitiveEvaluation, b"test");
        Event::from_work(work)
    }

    #[test]
    fn test_gdc_creation() {
        let gdc = Gdc::isolated(b"secret");
        
        assert!(gdc.is_alive());
        assert_eq!(gdc.state(), GdcState::Idle);
    }

    #[test]
    fn test_become_queen() {
        let mut gdc = Gdc::isolated(b"secret");
        let event = create_test_event();
        
        let result = gdc.become_queen(event);
        
        assert!(result.is_ok());
        assert_eq!(gdc.state(), GdcState::Queen);
    }

    #[test]
    fn test_become_worker() {
        let mut gdc = Gdc::isolated(b"secret");
        let event = create_test_event();
        // UUID fixo para testes (namespace DNS + "test-queen")
        let queen_id = Uuid::new_v5(&Uuid::NAMESPACE_DNS, b"test-queen");
        
        let result = gdc.become_worker(event, queen_id);
        
        assert!(result.is_ok());
        assert_eq!(gdc.state(), GdcState::Worker);
    }

    #[test]
    fn test_invalid_transition() {
        let mut gdc = Gdc::isolated(b"secret");
        let event = create_test_event();
        
        gdc.become_queen(event.clone()).unwrap();
        
        // Não pode virar worker quando já é rainha
        let queen_id = Uuid::new_v5(&Uuid::NAMESPACE_DNS, b"test-queen");
        let result = gdc.become_worker(event, queen_id);
        
        assert!(matches!(result, Err(StateTransitionError::InvalidTransition { .. })));
    }

    #[test]
    fn test_worker_process() {
        let mut gdc = Gdc::isolated(b"secret");
        let event = create_test_event();
        
        let queen_id = Uuid::new_v5(&Uuid::NAMESPACE_DNS, b"test-queen");
        gdc.become_worker(event.clone(), queen_id).unwrap();
        
        let edr = gdc.process_work(|e| {
            Manifestation::for_call(*e.call_signature())
                .with_capability_code(1, CapabilityState::satisfied())
        });
        
        assert!(edr.is_ok());
    }
}
