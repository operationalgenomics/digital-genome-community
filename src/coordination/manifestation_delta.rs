//! Canon v5.1 | GDC v1.0.0δ
//!
//! Manifestação Delta - Resposta por Ressonância Estrutural (v1.0.0δ)
//!
//! Canon: LEI-AO-20-04 - "A manifestação de Workers ocorre por ressonância
//! estrutural (AF-15): GDCs compatíveis manifestam disponibilidade;
//! GDCs incompatíveis não manifestam."
//!
//! NOTA: Este módulo é para v1.0.0δ (Enxame). O módulo `manifestation.rs`
//! existente é da v0.8 e será migrado posteriormente.

use crate::coordination::budget_delta::BudgetDelta;
use crate::coordination::vibration::{Capability, StructuralRequirements, Vibration, WorkId};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// ID temporário de GDC (substituir por struct real quando disponível)
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GdcId(uuid::Uuid);

impl GdcId {
    pub fn new() -> Self {
        GdcId(uuid::Uuid::new_v4())
    }
}

impl Default for GdcId {
    fn default() -> Self {
        Self::new()
    }
}

/// Manifestação Delta (v1.0.0δ).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ManifestationDelta {
    pub gdc_id: GdcId,
    pub work_id: WorkId,
    pub available_budget: BudgetDelta,
    pub capabilities: HashSet<Capability>,
    pub compatibility_proof: CompatibilityProof,
}

impl ManifestationDelta {
    pub fn manifest_if_compatible(
        gdc_id: GdcId,
        vibration: &Vibration,
        available_budget: BudgetDelta,
        capabilities: HashSet<Capability>,
    ) -> Option<Self> {
        let compatibility = Self::check_structural_resonance(
            &vibration.structural_requirements,
            &capabilities,
            &available_budget,
        );
        
        compatibility.map(|proof| ManifestationDelta {
            gdc_id,
            work_id: vibration.work_id,
            available_budget,
            capabilities,
            compatibility_proof: proof,
        })
    }
    
    fn check_structural_resonance(
        requirements: &StructuralRequirements,
        capabilities: &HashSet<Capability>,
        budget: &BudgetDelta,
    ) -> Option<CompatibilityProof> {
        let has_all_capabilities = requirements
            .required_capabilities
            .iter()
            .all(|req| capabilities.contains(req));
        
        if !has_all_capabilities {
            return None;
        }
        
        let has_sufficient_budget = budget.max_bytes > 0;
        
        if !has_sufficient_budget {
            return None;
        }
        
        Some(CompatibilityProof {
            capabilities_match: true,
            budget_sufficient: true,
            ontological_match: true,
        })
    }
}

/// Prova de compatibilidade (AF-15).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CompatibilityProof {
    pub capabilities_match: bool,
    pub budget_sufficient: bool,
    pub ontological_match: bool,
}

impl CompatibilityProof {
    pub fn is_valid(&self) -> bool {
        self.capabilities_match && self.budget_sufficient && self.ontological_match
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::coordination::vibration::VibrationId;
    
    #[test]
    fn test_manifestation_delta_compatible() {
        let vibration = Vibration {
            id: VibrationId::new(),
            work_id: WorkId::new(),
            structural_requirements: StructuralRequirements::basic(),
            estimated_budget: BudgetDelta::default(),
            timestamp: 0,
        };
        
        let gdc_id = GdcId::new();
        let budget = BudgetDelta::default();
        
        let mut capabilities = HashSet::new();
        capabilities.insert(Capability::ProcessUnl);
        capabilities.insert(Capability::ComputeMotors);
        
        let manifestation = ManifestationDelta::manifest_if_compatible(
            gdc_id,
            &vibration,
            budget,
            capabilities,
        );
        
        assert!(manifestation.is_some());
    }
}
