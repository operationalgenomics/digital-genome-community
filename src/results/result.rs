//! Canon v5.1 | GDC v0.9.0-final
//!
//! R(Σ) - Cognitive Result
//!
//! Canon: Especificação R(Σ) (lines 1273-1287)
//!
//! ## Definition
//! R(Σ) = Resultado Cognitivo Emissível
//!
//! Structure produced by Queen after integrating sufficient EDR returns
//! from W(Σ), ready to be emitted as final result.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use super::phenotype::CanonicalForm;

// =============================================================================
// COGNITIVE RESULT - R(Σ)
// =============================================================================

/// Cognitive Result R(Σ).
///
/// Structure produced by Queen after integrating EDR returns.
/// Not a chunk, not RawInput - born from Σ, W(Σ) and canonical EDRs.
///
/// Canon: §1 - R(Σ) é resultado, não chunk, não RawInput
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CognitiveResult {
    /// Canonical form of integrated structure
    pub canonical_form: CanonicalForm,
    
    /// Structured fields (Structural Domain - DE)
    pub fields: BTreeMap<String, String>,
    
    /// Number of components integrated
    pub component_count: usize,
    
    /// Integration is complete
    pub complete: bool,
    
    /// Result type classification
    pub result_type: ResultType,
}

impl CognitiveResult {
    /// Create new cognitive result.
    pub fn new(
        canonical_form: CanonicalForm,
        fields: BTreeMap<String, String>,
        component_count: usize,
        complete: bool,
    ) -> Self {
        Self {
            canonical_form,
            fields,
            component_count,
            complete,
            result_type: ResultType::DNA, // Default
        }
    }
    
    /// Create with specific type.
    pub fn with_type(mut self, result_type: ResultType) -> Self {
        self.result_type = result_type;
        self
    }
    
    /// Check if result is complete and ready for emission.
    pub fn is_emissible(&self) -> bool {
        self.complete
    }
}

// =============================================================================
// RESULT TYPE
// =============================================================================

/// Result Type classification.
///
/// Canon: AO-21 - Workers return calculations/UNLs, only Queen emits DNA
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResultType {
    /// DNA emission (Queen only)
    DNA,
    
    /// Calculation result (Worker return)
    Calculation,
    
    /// UNL state (Worker return)
    UNL,
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::phenotype::{StructuralGraph, CanonicalForm};
    
    #[test]
    fn test_cognitive_result_creation() {
        let graph = StructuralGraph::new(
            vec!["X".to_string(), "Y".to_string()],
            vec![(0, 1, "link".to_string())],
        );
        let cf = CanonicalForm::compute(&graph);
        
        let mut fields = BTreeMap::new();
        fields.insert("field1".to_string(), "value1".to_string());
        
        let result = CognitiveResult::new(cf, fields, 3, true);
        
        assert_eq!(result.component_count, 3);
        assert!(result.complete);
        assert!(result.is_emissible());
    }
    
    #[test]
    fn test_result_type_classification() {
        let graph = StructuralGraph::new(
            vec!["A".to_string()],
            vec![],
        );
        let cf = CanonicalForm::compute(&graph);
        
        let result = CognitiveResult::new(cf, BTreeMap::new(), 1, true)
            .with_type(ResultType::Calculation);
        
        assert_eq!(result.result_type, ResultType::Calculation);
    }
    
    #[test]
    fn test_incomplete_result_not_emissible() {
        let graph = StructuralGraph::new(vec!["A".to_string()], vec![]);
        let cf = CanonicalForm::compute(&graph);
        
        let result = CognitiveResult::new(cf, BTreeMap::new(), 1, false);
        
        assert!(!result.is_emissible());
    }
}

