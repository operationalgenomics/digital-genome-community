//! Canon v5.1 | GDC v1.0.0δ → v1.0.1
//!
//! Vibração - Sinal de Recrutamento de Workers (Canônico Determinístico)
//!
//! Canon: LEI-AO-20-04 - "O GDC que recebe a solicitação inicial de cálculo
//! assume o papel de Rainha e emite um sinal de solicitação ('vibração')
//! buscando candidatos a Workers."
//!
//! ## Conformidade Canônica v1.0.1
//!
//! **MUDANÇA CRÍTICA:** Este módulo agora é **completamente determinístico**.
//!
//! Removido: `SystemTime::now()` (violação de QM-01/QM-02)
//! Adicionado: `LogicalTime` derivado do estado do orquestrador
//!
//! ## Responsabilidade
//!
//! Quando um GDC vira QUEEN, ele emite uma vibração no enxame buscando
//! Workers compatíveis. A vibração carrega:
//! - ID do trabalho
//! - Requisitos estruturais (AF-15)
//! - Budget estimado necessário
//! - **Timestamp lógico** (não mais tempo real)
//!
//! ## Não é Federação
//!
//! Vibração é comunicação descentralizada, não hierarquia permanente.

use crate::coordination::budget_delta::BudgetDelta;
use crate::core_types::LogicalTime;
use serde::{Deserialize, Serialize};

// =============================================================================
// VIBRAÇÃO
// =============================================================================

/// Vibração emitida pela Queen para recrutar Workers.
///
/// Canon: LEI-AO-20-04 §1
///
/// ## Invariante Canônico v1.0.1
///
/// O timestamp é **lógico** (derivado de cycle_count), não real.
/// Isso garante replay determinístico (QM-02).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Vibration {
    /// ID único da vibração
    pub id: VibrationId,
    
    /// ID do trabalho
    pub work_id: WorkId,
    
    /// Requisitos estruturais (AF-15)
    pub structural_requirements: StructuralRequirements,
    
    /// Budget estimado necessário
    pub estimated_budget: BudgetDelta,
    
    /// Timestamp lógico de emissão (NÃO tempo real)
    ///
    /// ## Mudança v1.0.1
    ///
    /// Este campo agora contém tempo lógico derivado do cycle_count
    /// do orquestrador, garantindo determinismo completo.
    pub timestamp: u64,
}

impl Vibration {
    /// Emitir vibração (função da Queen) - VERSÃO CANÔNICA
    ///
    /// Canon: LEI-AO-20-04 - Queen emite vibração
    ///
    /// ## Mudança v1.0.1 (BREAKING CHANGE)
    ///
    /// Agora requer `logical_time` como parâmetro. Este tempo deve vir
    /// do estado do orquestrador, NÃO do sistema operacional.
    ///
    /// ## Garantia Canônica
    ///
    /// Esta função é agora **determinística**: mesmos inputs (incluindo
    /// logical_time com mesmo cycle_count) produzem mesma vibração.
    ///
    /// ## Exemplo
    ///
    /// ```ignore
    /// // ✅ CORRETO - Tempo do estado do orquestrador
    /// let logical_time = LogicalTime::from_cycle(orchestrator.cycle_count());
    /// let vibration = Vibration::emit(work_id, requirements, budget, &logical_time);
    ///
    /// // ❌ INCORRETO - Não fazer isto (não-determinístico)
    /// // let timestamp = SystemTime::now(); // VIOLAÇÃO CANÔNICA
    /// ```
    pub fn emit(
        work_id: WorkId,
        requirements: StructuralRequirements,
        estimated_budget: BudgetDelta,
        logical_time: &LogicalTime,
    ) -> Self {
        // Usar dados da vibração para gerar ID determinístico
        let id = VibrationId::from_vibration_data(
            &requirements,
            &estimated_budget,
            logical_time.as_timestamp(), // sequence = timestamp (já determinístico)
        );
        
        Vibration {
            id,
            work_id,
            structural_requirements: requirements,
            estimated_budget,
            timestamp: logical_time.as_timestamp(),
        }
    }
    
    /// Verificar se vibração é válida.
    pub fn is_valid(&self) -> bool {
        // Budget deve ser positivo
        self.estimated_budget.max_bytes > 0
    }
}

// =============================================================================
// REQUISITOS ESTRUTURAIS (AF-15)
// =============================================================================

/// Requisitos estruturais para compatibilidade.
///
/// Canon: AF-15 - Ressonância estrutural
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StructuralRequirements {
    /// Versão GD-QMN mínima requerida
    pub min_gd_qmn_version: String,
    
    /// Capacidades necessárias
    pub required_capabilities: Vec<Capability>,
    
    /// Compatibilidade ontológica
    pub ontological_compatibility: OntologicalCompatibility,
}

impl StructuralRequirements {
    /// Criar requisitos básicos.
    pub fn basic() -> Self {
        StructuralRequirements {
            min_gd_qmn_version: "1.0.0".to_string(),
            required_capabilities: vec![
                Capability::ProcessUnl,
                Capability::ComputeMotors,
            ],
            ontological_compatibility: OntologicalCompatibility::Universal,
        }
    }
    
    /// Criar requisitos avançados.
    pub fn advanced(capabilities: Vec<Capability>) -> Self {
        StructuralRequirements {
            min_gd_qmn_version: "1.0.0".to_string(),
            required_capabilities: capabilities,
            ontological_compatibility: OntologicalCompatibility::Specific {
                domain: "general".to_string(),
            },
        }
    }
}

// =============================================================================
// CAPACIDADES
// =============================================================================

/// Capacidades que um Worker pode ter.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Capability {
    /// Processar UNL
    ProcessUnl,
    
    /// Computar 4 motores
    ComputeMotors,
    
    /// Armazenar estado temporário
    TemporaryStorage,
    
    /// Cálculo distribuído
    DistributedCompute,
}

// =============================================================================
// COMPATIBILIDADE ONTOLÓGICA
// =============================================================================

/// Compatibilidade ontológica (AF-15).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OntologicalCompatibility {
    /// Universal (qualquer domínio)
    Universal,
    
    /// Específico de domínio
    Specific {
        domain: String,
    },
}

// =============================================================================
// IDENTIFICADORES (v1.0.1 Fase 2 - CANÔNICOS)
// =============================================================================

use crate::core_types::CanonicalId;

/// ID de vibração (canônico determinístico).
///
/// ## Mudança v1.0.1 Fase 2
///
/// Agora usa CanonicalId internamente (SHA-256 determinístico).
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct VibrationId(CanonicalId);

impl VibrationId {
    /// Criar ID canônico a partir de dados da vibração.
    ///
    /// ## Parâmetros
    ///
    /// - `requirements`: Requisitos estruturais
    /// - `budget`: Budget estimado
    /// - `sequence`: Contador sequencial do estado (determinístico)
    ///
    /// ## Garantia Canônica
    ///
    /// Mesmos inputs sempre produzem mesmo ID (QM-02).
    pub fn from_vibration_data(
        requirements: &StructuralRequirements,
        budget: &BudgetDelta,
        sequence: u64,
    ) -> Self {
        let mut payload = Vec::new();
        
        // Serializar componentes
        let req_bytes = bincode::serialize(requirements)
            .expect("StructuralRequirements must serialize");
        let budget_bytes = bincode::serialize(budget)
            .expect("BudgetDelta must serialize");
        
        payload.extend_from_slice(&req_bytes);
        payload.extend_from_slice(&budget_bytes);
        
        VibrationId(CanonicalId::from_context(
            "vibration",
            &payload,
            sequence,
        ))
    }
}

/// ID de trabalho (canônico determinístico).
///
/// ## Mudança v1.0.1 Fase 2
///
/// Agora usa CanonicalId internamente (SHA-256 determinístico).
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WorkId(CanonicalId);

impl WorkId {
    /// Criar ID canônico a partir do estímulo.
    ///
    /// ## Parâmetros
    ///
    /// - `stimulus`: Bytes do estímulo que define o trabalho
    /// - `sequence`: Contador sequencial do estado (determinístico)
    ///
    /// ## Garantia Canônica
    ///
    /// Mesmo estímulo e sequência sempre produzem mesmo WorkId.
    pub fn from_stimulus(stimulus: &[u8], sequence: u64) -> Self {
        WorkId(CanonicalId::from_context(
            "work",
            stimulus,
            sequence,
        ))
    }
}

// =============================================================================
// TESTES
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_vibration_creation() {
        let work_id = WorkId::from_stimulus(b"test-work", 0);
        let requirements = StructuralRequirements::basic();
        let budget = BudgetDelta::default();
        let logical_time = LogicalTime::from_cycle(0);
        
        let vibration = Vibration::emit(work_id, requirements, budget, &logical_time);
        
        assert_eq!(vibration.work_id, work_id);
        assert!(vibration.is_valid());
        assert_eq!(vibration.timestamp, 0); // Ciclo 0 → timestamp 0
    }
    
    #[test]
    fn test_structural_requirements_basic() {
        let req = StructuralRequirements::basic();
        
        assert_eq!(req.min_gd_qmn_version, "1.0.0");
        assert!(req.required_capabilities.contains(&Capability::ProcessUnl));
        assert!(req.required_capabilities.contains(&Capability::ComputeMotors));
    }
    
    #[test]
    fn test_vibration_ids_are_unique() {
        // v1.0.1: IDs determinísticos - timestamps diferentes → VibrationIds diferentes
        let time1 = LogicalTime::from_cycle(0);
        let time2 = LogicalTime::from_cycle(1);
        
        let vib1 = Vibration::emit(
            WorkId::from_stimulus(b"test-work", 0),
            StructuralRequirements::basic(),
            BudgetDelta::default(),
            &time1,
        );
        
        let vib2 = Vibration::emit(
            WorkId::from_stimulus(b"test-work", 0),
            StructuralRequirements::basic(),
            BudgetDelta::default(),
            &time2,
        );
        
        // IDs diferentes porque timestamp (sequence) é diferente
        assert_ne!(vib1.id, vib2.id);
    }
    
    #[test]
    fn test_work_ids_are_unique() {
        // Sequências diferentes → IDs diferentes (determinístico)
        let work1 = WorkId::from_stimulus(b"stimulus1", 0);
        let work2 = WorkId::from_stimulus(b"stimulus2", 0);
        
        assert_ne!(work1, work2);
        
        // Mesma sequência mas estímulos diferentes
        let work3 = WorkId::from_stimulus(b"stimulus1", 1);
        assert_ne!(work1, work3);
    }
    
    #[test]
    fn test_vibration_timestamp_deterministic() {
        // NOVO TESTE v1.0.1: Verificar determinismo de timestamp
        let requirements = StructuralRequirements::basic();
        let budget = BudgetDelta::default();
        let work_id = WorkId::from_stimulus(b"test_stimulus", 0);
        
        // Mesmo ciclo deve produzir mesmo timestamp
        let time1 = LogicalTime::from_cycle(42);
        let vib1 = Vibration::emit(work_id, requirements.clone(), budget.clone(), &time1);
        
        let time2 = LogicalTime::from_cycle(42);
        let vib2 = Vibration::emit(work_id, requirements.clone(), budget.clone(), &time2);
        
        // Timestamps devem ser idênticos (determinismo)
        assert_eq!(vib1.timestamp, vib2.timestamp);
        assert_eq!(vib1.timestamp, 42); // timestamp = cycle_count (direto)
    }
    
    #[test]
    fn test_vibration_timestamp_monotonic() {
        // NOVO TESTE v1.0.1: Timestamp cresce com ciclos
        let requirements = StructuralRequirements::basic();
        let budget = BudgetDelta::default();
        let work_id = WorkId::from_stimulus(b"test_stimulus", 0);
        
        let time1 = LogicalTime::from_cycle(10);
        let vib1 = Vibration::emit(work_id, requirements.clone(), budget.clone(), &time1);
        
        let time2 = LogicalTime::from_cycle(20);
        let vib2 = Vibration::emit(work_id, requirements.clone(), budget.clone(), &time2);
        
        // Timestamp deve ser monotônico
        assert!(vib2.timestamp > vib1.timestamp);
    }
    
    #[test]
    fn test_ontological_compatibility_universal() {
        let compat = OntologicalCompatibility::Universal;
        
        match compat {
            OntologicalCompatibility::Universal => {}, // ✅
            _ => panic!("Expected Universal"),
        }
    }
    
    #[test]
    fn test_ontological_compatibility_specific() {
        let compat = OntologicalCompatibility::Specific {
            domain: "industrial".to_string(),
        };
        
        match compat {
            OntologicalCompatibility::Specific { domain } => {
                assert_eq!(domain, "industrial");
            },
            _ => panic!("Expected Specific"),
        }
    }
    
    #[test]
    fn test_vibration_id_deterministic() {
        // v1.0.1 Fase 2: IDs canônicos são determinísticos
        let req = StructuralRequirements::basic();
        let budget = BudgetDelta::default();
        
        // Mesmos inputs → mesmo ID
        let id1 = VibrationId::from_vibration_data(&req, &budget, 0);
        let id2 = VibrationId::from_vibration_data(&req, &budget, 0);
        assert_eq!(id1, id2);
        
        // Sequências diferentes → IDs diferentes
        let id3 = VibrationId::from_vibration_data(&req, &budget, 1);
        assert_ne!(id1, id3);
    }
    
    #[test]
    fn test_work_id_deterministic() {
        // v1.0.1 Fase 2: IDs canônicos são determinísticos
        let stimulus = b"test_stimulus";
        
        // Mesmo estímulo e sequência → mesmo ID
        let id1 = WorkId::from_stimulus(stimulus, 0);
        let id2 = WorkId::from_stimulus(stimulus, 0);
        assert_eq!(id1, id2);
        
        // Estímulos diferentes → IDs diferentes
        let id3 = WorkId::from_stimulus(b"different", 0);
        assert_ne!(id1, id3);
    }
}
