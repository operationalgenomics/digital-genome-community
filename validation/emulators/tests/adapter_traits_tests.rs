//! Canon v5.1 | GDC v1.0.0β
//!
//! Adapter Traits Tests - Validação da interface
//!
//! MOVED TO: validation/emulators/tests/

use gdc_emulators::adapter::*;

// =============================================================================
// MOCK ADAPTER - PARA TESTES
// =============================================================================

struct MockAdapter;

impl Adapter for MockAdapter {
    fn adapt(&self, input: &[u8]) -> Result<UnlNormalized, AdapterError> {
        // Mock: converte bytes em tokens hexadecimais
        let tokens: Vec<String> = input
            .iter()
            .enumerate()
            .map(|(i, b)| format!("token_{}_{:02x}", i, b))
            .collect();
        
        // Mock: relações sequenciais
        let relations: Vec<Relation> = (0..tokens.len() - 1)
            .map(|i| Relation {
                from_token: tokens[i].clone(),
                to_token: tokens[i + 1].clone(),
                relation_type: "next".to_string(),
            })
            .collect();
        
        // Metadata
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(input);
        let result = hasher.finalize();
        let mut source_hash = [0u8; 32];
        source_hash.copy_from_slice(&result);
        
        let metadata = AdapterMetadata {
            domain: self.domain(),
            adapter_version: self.version(),
            timestamp_utc: 0, // Determinístico: sempre 0 em mock
            source_hash,
        };
        
        Ok(UnlNormalized {
            tokens,
            relations,
            metadata,
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
            name: "MockSchema".to_string(),
            version: "1.0.0".to_string(),
            description: "Mock adapter for testing".to_string(),
            fields: vec![
                FieldDefinition {
                    name: "data".to_string(),
                    field_type: FieldType::Binary,
                    required: true,
                    description: "Raw binary data".to_string(),
                },
            ],
        }
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[test]
fn test_adapter_trait_implementation() {
    let adapter = MockAdapter;
    
    // Verificar métodos base
    assert_eq!(adapter.version().to_string(), "1.0.0");
    assert_eq!(adapter.domain(), Domain::Industrial);
    
    let schema = adapter.schema();
    assert_eq!(schema.name, "MockSchema");
    assert_eq!(schema.fields.len(), 1);
}

#[test]
fn test_adapter_determinism() {
    // Canon: LEI-AF-14-01 §1 - Determinismo
    let adapter = MockAdapter;
    let input = vec![0xCA, 0xFE, 0xBA, 0xBE];
    
    let unl1 = adapter.adapt(&input).unwrap();
    let unl2 = adapter.adapt(&input).unwrap();
    
    // Mesmo input → Mesmo output
    assert_eq!(unl1.tokens, unl2.tokens);
    assert_eq!(unl1.relations, unl2.relations);
    assert_eq!(unl1.metadata.source_hash, unl2.metadata.source_hash);
}

#[test]
fn test_adapter_sensitivity() {
    // Inputs diferentes → Outputs diferentes
    let adapter = MockAdapter;
    
    let input1 = vec![0xCA, 0xFE];
    let input2 = vec![0xBA, 0xBE];
    
    let unl1 = adapter.adapt(&input1).unwrap();
    let unl2 = adapter.adapt(&input2).unwrap();
    
    // Outputs diferentes
    assert_ne!(unl1.tokens, unl2.tokens);
    assert_ne!(unl1.metadata.source_hash, unl2.metadata.source_hash);
}

#[test]
fn test_unl_structure() {
    let adapter = MockAdapter;
    let input = vec![0x01, 0x02, 0x03];
    
    let unl = adapter.adapt(&input).unwrap();
    
    // Verificar estrutura UNL
    assert_eq!(unl.tokens.len(), 3);
    assert_eq!(unl.relations.len(), 2); // n-1 relações
    
    // Verificar relações sequenciais
    assert_eq!(unl.relations[0].from_token, unl.tokens[0]);
    assert_eq!(unl.relations[0].to_token, unl.tokens[1]);
}

#[test]
fn test_metadata_hash() {
    let adapter = MockAdapter;
    let input = vec![0xDE, 0xAD, 0xBE, 0xEF];
    
    let unl = adapter.adapt(&input).unwrap();
    
    // Hash deve ser consistente
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(&input);
    let expected_hash = hasher.finalize();
    
    assert_eq!(&unl.metadata.source_hash[..], &expected_hash[..]);
}

#[test]
fn test_domain_serialization() {
    // Verificar que Domain é serializável
    let domain = Domain::Industrial;
    let serialized = serde_json::to_string(&domain).unwrap();
    let deserialized: Domain = serde_json::from_str(&serialized).unwrap();
    
    assert_eq!(domain, deserialized);
}

#[test]
fn test_adapter_version_ordering() {
    let v1 = AdapterVersion::new(1, 0, 0);
    let v2 = AdapterVersion::new(1, 0, 1);
    let v3 = AdapterVersion::new(2, 0, 0);
    
    // Verificar igualdade
    assert_eq!(v1, AdapterVersion::new(1, 0, 0));
    assert_ne!(v1, v2);
    assert_ne!(v2, v3);
}
