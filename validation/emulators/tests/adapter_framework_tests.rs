//! Canon v5.1 | GDC v1.0.0β
//!
//! Adapter Framework Tests - Validação do engine
//!
//! MOVED TO: validation/emulators/tests/
//! Canonical compliance: External emulator tests

use gdc_emulators::adapter::*;

// =============================================================================
// DETERMINISTIC ADAPTER - SEMPRE RETORNA MESMO OUTPUT
// =============================================================================

struct DeterministicAdapter;

impl Adapter for DeterministicAdapter {
    fn adapt(&self, input: &[u8]) -> Result<UnlNormalized, AdapterError> {
        let tokens: Vec<String> = input
            .iter()
            .map(|b| format!("det_{:02x}", b))
            .collect();
        
        let relations: Vec<Relation> = (0..tokens.len() - 1)
            .map(|i| Relation {
                from_token: tokens[i].clone(),
                to_token: tokens[i + 1].clone(),
                relation_type: "next".to_string(),
            })
            .collect();
        
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(input);
        let result = hasher.finalize();
        let mut source_hash = [0u8; 32];
        source_hash.copy_from_slice(&result);
        
        Ok(UnlNormalized {
            tokens,
            relations,
            metadata: AdapterMetadata {
                domain: Domain::Industrial,
                adapter_version: AdapterVersion::new(1, 0, 0),
                timestamp_utc: 0, // Determinístico
                source_hash,
            },
        })
    }
    
    fn version(&self) -> AdapterVersion {
        AdapterVersion::new(1, 0, 0)
    }
    
    fn domain(&self) -> Domain {
        Domain::Industrial
    }
    
    fn schema(&self) -> SchemaDefinition {
        SchemaDefinition {
            name: "Deterministic".to_string(),
            version: "1.0.0".to_string(),
            description: "Always deterministic".to_string(),
            fields: vec![],
        }
    }
}

// =============================================================================
// NON-DETERMINISTIC ADAPTER - RETORNA OUTPUT DIFERENTE (TESTE NEGATIVO)
// =============================================================================

struct NonDeterministicAdapter {
    counter: std::sync::atomic::AtomicU64,
}

impl NonDeterministicAdapter {
    fn new() -> Self {
        Self {
            counter: std::sync::atomic::AtomicU64::new(0),
        }
    }
}

impl Adapter for NonDeterministicAdapter {
    fn adapt(&self, input: &[u8]) -> Result<UnlNormalized, AdapterError> {
        // INTENCIONALLY NON-DETERMINISTIC: usa counter
        let count = self.counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        
        let tokens: Vec<String> = input
            .iter()
            .map(|b| format!("ndet_{}_{:02x}", count, b))
            .collect();
        
        let relations: Vec<Relation> = vec![];
        
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(input);
        hasher.update(&count.to_le_bytes()); // NON-DETERMINISTIC
        let result = hasher.finalize();
        let mut source_hash = [0u8; 32];
        source_hash.copy_from_slice(&result);
        
        Ok(UnlNormalized {
            tokens,
            relations,
            metadata: AdapterMetadata {
                domain: Domain::Financial,
                adapter_version: AdapterVersion::new(1, 0, 0),
                timestamp_utc: 0,
                source_hash,
            },
        })
    }
    
    fn version(&self) -> AdapterVersion {
        AdapterVersion::new(1, 0, 0)
    }
    
    fn domain(&self) -> Domain {
        Domain::Financial
    }
    
    fn schema(&self) -> SchemaDefinition {
        SchemaDefinition {
            name: "NonDeterministic".to_string(),
            version: "1.0.0".to_string(),
            description: "Intentionally non-deterministic".to_string(),
            fields: vec![],
        }
    }
}

// =============================================================================
// TESTS - FRAMEWORK BASICS
// =============================================================================

#[test]
fn test_framework_new() {
    let framework = AdapterFramework::new();
    assert_eq!(framework.registered_domains().len(), 0);
}

#[test]
fn test_framework_register_adapter() {
    let framework = AdapterFramework::new();
    
    let result = framework.register(Domain::Industrial, DeterministicAdapter);
    assert!(result.is_ok());
    
    assert!(framework.has_adapter(Domain::Industrial));
    assert!(!framework.has_adapter(Domain::Financial));
}

#[test]
fn test_framework_register_multiple_domains() {
    let framework = AdapterFramework::new();
    
    framework.register(Domain::Industrial, DeterministicAdapter).unwrap();
    framework.register(Domain::Financial, NonDeterministicAdapter::new()).unwrap();
    
    let domains = framework.registered_domains();
    assert_eq!(domains.len(), 2);
    assert!(domains.contains(&Domain::Industrial));
    assert!(domains.contains(&Domain::Financial));
}

#[test]
fn test_framework_adapt_success() {
    let framework = AdapterFramework::new();
    framework.register(Domain::Industrial, DeterministicAdapter).unwrap();
    
    let input = vec![0xCA, 0xFE, 0xBA, 0xBE];
    let unl = framework.adapt(Domain::Industrial, &input).unwrap();
    
    assert_eq!(unl.tokens.len(), 4);
    assert_eq!(unl.tokens[0], "det_ca");
    assert_eq!(unl.tokens[1], "det_fe");
    assert_eq!(unl.relations.len(), 3); // n-1 relations
}

#[test]
fn test_framework_adapt_domain_not_registered() {
    let framework = AdapterFramework::new();
    
    let input = vec![0x01, 0x02];
    let result = framework.adapt(Domain::Healthcare, &input);
    
    assert!(result.is_err());
    
    match result {
        Err(FrameworkError::DomainNotRegistered(Domain::Healthcare)) => (),
        _ => panic!("Expected DomainNotRegistered(Healthcare)"),
    }
}

// =============================================================================
// TESTS - DETERMINISM VALIDATION (CRITICAL)
// =============================================================================

#[test]
fn test_validate_determinism_positive() {
    // Canon: LEI-AF-14-01 §1 - Determinismo
    let framework = AdapterFramework::new();
    framework.register(Domain::Industrial, DeterministicAdapter).unwrap();
    
    let input = vec![0xDE, 0xAD, 0xBE, 0xEF];
    
    // MUST return true for deterministic adapter
    assert!(
        framework.validate_determinism(Domain::Industrial, &input),
        "Deterministic adapter MUST pass validation"
    );
}

#[test]
fn test_validate_determinism_negative() {
    // Canon: LEI-AF-14-01 §1 - Violação de determinismo
    let framework = AdapterFramework::new();
    framework.register(Domain::Financial, NonDeterministicAdapter::new()).unwrap();
    
    let input = vec![0xCA, 0xFE];
    
    // MUST return false for non-deterministic adapter
    assert!(
        !framework.validate_determinism(Domain::Financial, &input),
        "Non-deterministic adapter MUST fail validation"
    );
}

#[test]
fn test_determinism_multiple_inputs() {
    let framework = AdapterFramework::new();
    framework.register(Domain::Industrial, DeterministicAdapter).unwrap();
    
    // Test with different inputs
    let inputs = vec![
        vec![0x01],
        vec![0x01, 0x02],
        vec![0xCA, 0xFE, 0xBA, 0xBE],
    ];
    
    for input in inputs {
        assert!(
            framework.validate_determinism(Domain::Industrial, &input),
            "Deterministic adapter must pass for all inputs"
        );
    }
}

// =============================================================================
// TESTS - AUDIT LOG
// =============================================================================

#[test]
fn test_framework_audit_enabled() {
    let framework = AdapterFramework::with_audit();
    framework.register(Domain::Industrial, DeterministicAdapter).unwrap();
    
    let input = vec![0xAB, 0xCD];
    let _ = framework.adapt(Domain::Industrial, &input);
    
    let log = framework.audit_log();
    assert_eq!(log.len(), 1);
    assert_eq!(log[0].domain, Domain::Industrial);
    assert!(log[0].success);
    assert!(log[0].error_message.is_none());
}

#[test]
fn test_framework_audit_multiple_calls() {
    let framework = AdapterFramework::with_audit();
    framework.register(Domain::Industrial, DeterministicAdapter).unwrap();
    
    // Multiple adaptations
    for i in 0..5 {
        let input = vec![i];
        let _ = framework.adapt(Domain::Industrial, &input);
    }
    
    let log = framework.audit_log();
    assert_eq!(log.len(), 5);
    
    // All should be successful
    for entry in log {
        assert!(entry.success);
        assert_eq!(entry.domain, Domain::Industrial);
    }
}

#[test]
fn test_framework_audit_clear() {
    let framework = AdapterFramework::with_audit();
    framework.register(Domain::Industrial, DeterministicAdapter).unwrap();
    
    let input = vec![0x01, 0x02];
    let _ = framework.adapt(Domain::Industrial, &input);
    
    assert_eq!(framework.audit_log().len(), 1);
    
    framework.clear_audit_log().unwrap();
    assert_eq!(framework.audit_log().len(), 0);
}

// =============================================================================
// TESTS - INTEGRATION
// =============================================================================

#[test]
fn test_framework_full_workflow() {
    // Simula workflow completo
    let framework = AdapterFramework::with_audit();
    
    // 1. Registrar adapters
    framework.register(Domain::Industrial, DeterministicAdapter).unwrap();
    framework.register(Domain::Financial, NonDeterministicAdapter::new()).unwrap();
    
    // 2. Verificar registro
    assert_eq!(framework.registered_domains().len(), 2);
    
    // 3. Adaptar industrial (determinístico)
    let industrial_data = vec![0xCA, 0xFE];
    let unl_industrial = framework.adapt(Domain::Industrial, &industrial_data).unwrap();
    assert!(framework.validate_determinism(Domain::Industrial, &industrial_data));
    
    // 4. Adaptar financial (não-determinístico)
    let financial_data = vec![0xBA, 0xBE];
    let unl_financial = framework.adapt(Domain::Financial, &financial_data).unwrap();
    assert!(!framework.validate_determinism(Domain::Financial, &financial_data));
    
    // 5. Verificar audit log
    let log = framework.audit_log();
    assert!(log.len() >= 4); // 2 adapt + 2×2 validate
    
    // 6. Verificar outputs diferentes
    assert_ne!(unl_industrial.tokens, unl_financial.tokens);
}

#[test]
fn test_framework_same_structure_different_domains() {
    // Canon: LEI-AF-14-01 §4 - Trans-Reino
    let framework = AdapterFramework::new();
    
    framework.register(Domain::Industrial, DeterministicAdapter).unwrap();
    framework.register(Domain::Financial, DeterministicAdapter).unwrap();
    
    let same_input = vec![0xDE, 0xAD];
    
    let unl_industrial = framework.adapt(Domain::Industrial, &same_input).unwrap();
    let unl_financial = framework.adapt(Domain::Financial, &same_input).unwrap();
    
    // Mesmo adapter, então tokens devem ser iguais
    assert_eq!(unl_industrial.tokens, unl_financial.tokens);
    
    // Mas metadata deve indicar domínios diferentes
    // (Nota: neste teste ambos usam DeterministicAdapter que retorna Industrial,
    //  em produção cada adapter retornaria seu próprio domínio)
}
