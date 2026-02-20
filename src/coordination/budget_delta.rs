//! Canon v5.1 | GDC v1.0.0δ
//!
//! Budget Delta - Budget Simplificado para v1.0.0δ
//!
//! NOTA: Este é um budget temporário para v1.0.0δ.
//! NÃO substitui ComputationalBudget do Core.

use serde::{Deserialize, Serialize};

/// Budget simplificado para v1.0.0δ (Enxame).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BudgetDelta {
    /// Bytes máximos
    pub max_bytes: usize,
    
    /// Iterações máximas
    pub max_iterations: usize,
}

impl BudgetDelta {
    /// Budget padrão.
    pub fn default_budget() -> Self {
        BudgetDelta {
            max_bytes: 100 * 1024 * 1024, // 100 MB
            max_iterations: 10_000,
        }
    }
    
    /// Verificar se válido.
    pub fn is_valid(&self) -> bool {
        self.max_bytes > 0 && self.max_iterations > 0
    }
}

impl Default for BudgetDelta {
    fn default() -> Self {
        Self::default_budget()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_budget_delta_creation() {
        let budget = BudgetDelta::default();
        assert!(budget.is_valid());
        assert_eq!(budget.max_bytes, 100 * 1024 * 1024);
    }
}
