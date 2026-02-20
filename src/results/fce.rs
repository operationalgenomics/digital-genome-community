//! Canon v5.1 | GDC v0.9.0-final
//!
//! FCE(R) - Forma Canônica Estrutural do Resultado
//!
//! Canon: Especificação R(Σ) e FCE(R) (lines 1258-1344)
//!
//! ## Core Principle
//! FCE: R(Σ) → Estrutura_Normalizada
//!
//! ## Properties (§2)
//! 1. Total determinism: same R(Σ) ⇒ same FCE(R)
//! 2. Temporal independence: EDR arrival order irrelevant
//! 3. Completeness: depends only on explicit R(Σ) content
//! 4. No implicit state: forbidden to depend on environment
//! 5. Structural canonicalization: all collections ordered

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use super::result::CognitiveResult;
use super::phenotype::CanonicalForm;

// =============================================================================
// NORMALIZED STRUCTURE - FCE(R)
// =============================================================================

/// Normalized Structure - FCE(R) output.
///
/// Canonical structural form of cognitive result.
/// Exists BEFORE encoding (LEI-QMN-SERIAL-01).
///
/// Canon: §2 - FCE(R) pertence ao Domínio Estrutural (DE)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NormalizedStructure {
    /// Canonical form of underlying graph
    pub canonical_form: CanonicalForm,
    
    /// Ordered fields (deterministic)
    pub fields: BTreeMap<String, String>,
    
    /// Structural metadata
    pub metadata: StructuralMetadata,
}

/// Structural metadata (DE only - no DD fields).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StructuralMetadata {
    /// Number of components integrated
    pub component_count: usize,
    
    /// Integration complete
    pub complete: bool,
    
    /// Schema version
    pub schema_version: u16,
}

impl NormalizedStructure {
    /// Create new normalized structure.
    pub fn new(
        canonical_form: CanonicalForm,
        fields: BTreeMap<String, String>,
        component_count: usize,
        complete: bool,
    ) -> Self {
        Self {
            canonical_form,
            fields,
            metadata: StructuralMetadata {
                component_count,
                complete,
                schema_version: 1,
            },
        }
    }
    
    /// Check structural equality.
    ///
    /// Canon: §3 - CF(G₁) = CF(G₂) ⇒ FCE(R₁) = FCE(R₂)
    pub fn structurally_equal(&self, other: &Self) -> bool {
        self.canonical_form == other.canonical_form
            && self.fields == other.fields
            && self.metadata.complete == other.metadata.complete
    }
}

// =============================================================================
// FCE - FORMA CANÔNICA ESTRUTURAL
// =============================================================================

/// FCE - Canonical Structural Form computation.
///
/// Transforms R(Σ) into deterministic normalized structure.
///
/// Canon: §2 - Propriedades obrigatórias
#[derive(Debug, Clone)]
pub struct FCE;

impl FCE {
    /// Compute FCE(R) from cognitive result.
    ///
    /// Properties guaranteed:
    /// 1. Determinism: same R(Σ) → same FCE(R)
    /// 2. Temporal independence: EDR order irrelevant
    /// 3. Structural canonicalization: all collections ordered
    ///
    /// Canon: §2 - FCE: R(Σ) → Estrutura_Normalizada
    pub fn compute(result: &CognitiveResult) -> NormalizedStructure {
        // Step 1: Extract canonical form
        let canonical_form = result.canonical_form.clone();
        
        // Step 2: Normalize fields (ordered by key)
        let mut fields = BTreeMap::new();
        for (key, value) in &result.fields {
            fields.insert(key.clone(), value.clone());
        }
        
        // Step 3: Compute metadata
        let component_count = result.component_count;
        let complete = result.complete;
        
        NormalizedStructure::new(canonical_form, fields, component_count, complete)
    }
    
    /// Verify FCE determinism.
    ///
    /// Canon: §5 - Mesmo Σ ⇒ mesma FCE(R)
    pub fn verify_determinism(r1: &CognitiveResult, r2: &CognitiveResult) -> bool {
        let fce1 = Self::compute(r1);
        let fce2 = Self::compute(r2);
        
        fce1.structurally_equal(&fce2)
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::phenotype::{StructuralGraph, CanonicalForm};
    
    fn create_test_result(component_count: usize) -> CognitiveResult {
        let graph = StructuralGraph::new(
            vec!["A".to_string(), "B".to_string()],
            vec![(0, 1, "rel".to_string())],
        );
        let cf = CanonicalForm::compute(&graph);
        
        let mut fields = BTreeMap::new();
        fields.insert("key1".to_string(), "value1".to_string());
        fields.insert("key2".to_string(), "value2".to_string());
        
        CognitiveResult::new(cf, fields, component_count, true)
    }
    
    #[test]
    fn test_fce_deterministic() {
        let result = create_test_result(5);
        
        let fce1 = FCE::compute(&result);
        let fce2 = FCE::compute(&result);
        
        // Canon: §2.1 - Determinismo total
        assert_eq!(fce1, fce2);
    }
    
    #[test]
    fn test_fce_temporal_independence() {
        let r1 = create_test_result(5);
        let r2 = create_test_result(5);
        
        // Same content, potentially different construction order
        assert!(FCE::verify_determinism(&r1, &r2));
    }
    
    #[test]
    fn test_fce_structural_equality() {
        let result = create_test_result(3);
        let fce = FCE::compute(&result);
        
        // Verify fields are ordered (BTreeMap guarantees this)
        let keys: Vec<_> = fce.fields.keys().collect();
        assert_eq!(keys, vec!["key1", "key2"]);
    }
    
    #[test]
    fn test_fce_cf_relation() {
        let r1 = create_test_result(2);
        let r2 = create_test_result(2);
        
        let fce1 = FCE::compute(&r1);
        let fce2 = FCE::compute(&r2);
        
        // Canon: §3 - CF(G₁) = CF(G₂) ⇒ FCE(R₁) = FCE(R₂)
        if r1.canonical_form == r2.canonical_form {
            assert!(fce1.structurally_equal(&fce2));
        }
    }
}

