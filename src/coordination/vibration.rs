//! Canon v5.1 | GDC v1.0.0δ
//!
//! Vibração - Sinal de Recrutamento de Workers
//!
//! Canon: LEI-AO-20-04 - "O GDC que recebe a solicitação inicial de cálculo
//! assume o papel de Rainha e emite um sinal de solicitação ('vibração')
//! buscando candidatos a Workers."
//!
//! ## Responsabilidade
//!
//! Quando um GDC vira QUEEN, ele emite uma vibração no enxame buscando
//! Workers compatíveis. A vibração carrega:
//! - ID do trabalho
//! - Requisitos estruturais (AF-15)
//! - Budget estimado necessário
//!
//! ## Não é Federação
//!
//! Vibração é comunicação descentralizada, não hierarquia permanente.

use crate::coordination::budget_delta::BudgetDelta;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

// =============================================================================
// VIBRAÇÃO
// =============================================================================

/// Vibração emitida pela Queen para recrutar Workers.
///
/// Canon: LEI-AO-20-04 §1
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
    
    /// Timestamp de emissão
    pub timestamp: u64,
}

impl Vibration {
    /// Emitir vibração (função da Queen).
    ///
    /// Canon: LEI-AO-20-04 - Queen emite vibração
    pub fn emit(
        work_id: WorkId,
        requirements: StructuralRequirements,
        estimated_budget: BudgetDelta,
    ) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        Vibration {
            id: VibrationId::new(),
            work_id,
            structural_requirements: requirements,
            estimated_budget,
            timestamp,
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
        
        let vibration = Vibration::emit(work_id, requirements, budget);
        
        assert_eq!(vibration.work_id, work_id);
        assert!(vibration.is_valid());
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
        let vib1 = Vibration::emit(
            WorkId::new(),
            StructuralRequirements::basic(),
            BudgetDelta::default(),
        );
        
        let vib2 = Vibration::emit(
            WorkId::new(),
            StructuralRequirements::basic(),
            BudgetDelta::default(),
        );
        
        assert_ne!(vib1.id, vib2.id);
    }
    
    #[test]
    fn test_work_ids_are_unique() {
        let work1 = WorkId::new();
        let work2 = WorkId::new();
        
        assert_ne!(work1, work2);
    }
    
    #[test]
    fn test_vibration_timestamp() {
        let vibration = Vibration::emit(
            WorkId::new(),
            StructuralRequirements::basic(),
            BudgetDelta::default(),
        );
        
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Timestamp deve ser próximo ao now (diferença < 1s)
        assert!(vibration.timestamp <= now);
        assert!(vibration.timestamp + 1 >= now);
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
