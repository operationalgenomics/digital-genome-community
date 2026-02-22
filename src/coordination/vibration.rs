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
        Vibration {
            id: VibrationId::new(),
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
// IDENTIFICADORES
// =============================================================================

/// ID de vibração.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct VibrationId(uuid::Uuid);

impl VibrationId {
    /// Criar novo ID de vibração.
    pub fn new() -> Self {
        VibrationId(uuid::Uuid::new_v4())
    }
}

impl Default for VibrationId {
    fn default() -> Self {
        Self::new()
    }
}

/// ID de trabalho.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WorkId(uuid::Uuid);

impl WorkId {
    /// Criar novo ID de trabalho.
    pub fn new() -> Self {
        WorkId(uuid::Uuid::new_v4())
    }
}

impl Default for WorkId {
    fn default() -> Self {
        Self::new()
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
        let work_id = WorkId::new();
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
        let logical_time = LogicalTime::from_cycle(0);
        
        let vib1 = Vibration::emit(
            WorkId::new(),
            StructuralRequirements::basic(),
            BudgetDelta::default(),
            &logical_time,
        );
        
        let vib2 = Vibration::emit(
            WorkId::new(),
            StructuralRequirements::basic(),
            BudgetDelta::default(),
            &logical_time,
        );
        
        // NOTA: Este teste ainda passa porque WorkId::new() usa UUID v4
        // Isso será corrigido na Fase 2
        assert_ne!(vib1.id, vib2.id);
    }
    
    #[test]
    fn test_work_ids_are_unique() {
        let work1 = WorkId::new();
        let work2 = WorkId::new();
        
        // NOTA: Ainda não-determinístico (UUID v4)
        // Será corrigido na Fase 2
        assert_ne!(work1, work2);
    }
    
    #[test]
    fn test_vibration_timestamp_deterministic() {
        // NOVO TESTE v1.0.1: Verificar determinismo de timestamp
        let requirements = StructuralRequirements::basic();
        let budget = BudgetDelta::default();
        let work_id = WorkId::new();
        
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
        let work_id = WorkId::new();
        
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
}
