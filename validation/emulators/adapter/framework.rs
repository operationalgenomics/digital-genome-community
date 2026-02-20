//! Canon v5.1 | GDC v1.0.0β
//!
//! Adapter Framework - Engine de coordenação de Adapters
//!
//! Canon: LEI-AF-14-01 (Adapter Estrutural Canônico Trans-Kingdom)
//!
//! ## Responsabilidades
//!
//! 1. Registrar Adapters por Domain
//! 2. Coordenar adaptação X → UNL
//! 3. Validar determinismo
//! 4. Auditar mapeamentos
//!
//! ## Não-Responsabilidades
//!
//! - NÃO processa UNL (responsabilidade do GDC)
//! - NÃO persiste DNA (responsabilidade do GDE)
//! - NÃO orquestra pipeline (responsabilidade do GDO)

use crate::adapter::{
    Adapter, AdapterError, Domain, UnlNormalized,
};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

// =============================================================================
// ADAPTER FRAMEWORK
// =============================================================================

/// Adapter Framework Engine.
///
/// Coordena múltiplos adapters, cada um responsável por um Domain.
///
/// # Canon: LEI-AF-14-01 §5 - Framework de Coordenação
///
/// ## Exemplo
///
/// ```rust,ignore
/// let mut framework = AdapterFramework::new();
///
/// // Registrar adapters
/// framework.register(Domain::Industrial, IndustrialAdapter::new());
/// framework.register(Domain::Financial, FinancialAdapter::new());
///
/// // Adaptar sinais
/// let sensor_data = read_sensor();
/// let unl = framework.adapt(Domain::Industrial, &sensor_data)?;
///
/// // Validar determinismo
/// assert!(framework.validate_determinism(Domain::Industrial, &sensor_data));
/// ```
pub struct AdapterFramework {
    /// Registered adapters by domain
    adapters: Arc<RwLock<HashMap<Domain, Box<dyn Adapter>>>>,
    
    /// Audit log (optional)
    audit_enabled: bool,
    
    /// Audit entries
    audit_log: Arc<RwLock<Vec<AuditEntry>>>,
}

/// Audit entry for adapter execution.
#[derive(Debug, Clone)]
pub struct AuditEntry {
    pub domain: Domain,
    pub input_hash: [u8; 32],
    pub output_hash: [u8; 32],
    pub timestamp_utc: u64,
    pub success: bool,
    pub error_message: Option<String>,
}

impl AdapterFramework {
    /// Create new adapter framework.
    pub fn new() -> Self {
        Self {
            adapters: Arc::new(RwLock::new(HashMap::new())),
            audit_enabled: false,
            audit_log: Arc::new(RwLock::new(Vec::new())),
        }
    }
    
    /// Create new adapter framework with audit enabled.
    pub fn with_audit() -> Self {
        Self {
            adapters: Arc::new(RwLock::new(HashMap::new())),
            audit_enabled: true,
            audit_log: Arc::new(RwLock::new(Vec::new())),
        }
    }
    
    /// Register adapter for a domain.
    ///
    /// # Canon: LEI-AF-14-01 §5
    ///
    /// Replaces existing adapter if domain already registered.
    pub fn register<A: Adapter + 'static>(&self, domain: Domain, adapter: A) -> Result<(), FrameworkError> {
        let mut adapters = self.adapters
            .write()
            .map_err(|_| FrameworkError::LockError)?;
        
        adapters.insert(domain, Box::new(adapter));
        Ok(())
    }
    
    /// Check if domain has registered adapter.
    pub fn has_adapter(&self, domain: Domain) -> bool {
        if let Ok(adapters) = self.adapters.read() {
            adapters.contains_key(&domain)
        } else {
            false
        }
    }
    
    /// Adapt raw input to UNL using registered adapter.
    ///
    /// # Canon: LEI-AF-14-01 §1 - Mapeamento Determinístico
    ///
    /// Returns error if:
    /// - Domain not registered
    /// - Adapter fails
    pub fn adapt(&self, domain: Domain, input: &[u8]) -> Result<UnlNormalized, FrameworkError> {
        // Get adapter
        let adapters = self.adapters
            .read()
            .map_err(|_| FrameworkError::LockError)?;
        
        let adapter = adapters
            .get(&domain)
            .ok_or(FrameworkError::DomainNotRegistered(domain))?;
        
        // Compute input hash (for audit)
        let input_hash = self.compute_hash(input);
        
        // Adapt
        let result = adapter.adapt(input);
        
        // Audit if enabled
        if self.audit_enabled {
            match &result {
                Ok(unl) => {
                    let output_hash = self.compute_unl_hash(unl);
                    self.log_audit(AuditEntry {
                        domain,
                        input_hash,
                        output_hash,
                        timestamp_utc: self.get_timestamp(),
                        success: true,
                        error_message: None,
                    });
                }
                Err(e) => {
                    self.log_audit(AuditEntry {
                        domain,
                        input_hash,
                        output_hash: [0u8; 32],
                        timestamp_utc: self.get_timestamp(),
                        success: false,
                        error_message: Some(e.to_string()),
                    });
                }
            }
        }
        
        result.map_err(FrameworkError::AdapterError)
    }
    
    /// Validate adapter determinism.
    ///
    /// # Canon: LEI-AF-14-01 §1
    ///
    /// Executes adapter twice on same input and verifies outputs are identical.
    ///
    /// Returns:
    /// - `true` if deterministic (outputs identical)
    /// - `false` if non-deterministic or adapter fails
    pub fn validate_determinism(&self, domain: Domain, input: &[u8]) -> bool {
        // Execute twice
        let result1 = self.adapt(domain, input);
        let result2 = self.adapt(domain, input);
        
        // Compare
        match (result1, result2) {
            (Ok(unl1), Ok(unl2)) => {
                // Compare tokens
                if unl1.tokens != unl2.tokens {
                    return false;
                }
                
                // Compare relations
                if unl1.relations != unl2.relations {
                    return false;
                }
                
                // Compare metadata hashes
                if unl1.metadata.source_hash != unl2.metadata.source_hash {
                    return false;
                }
                
                true
            }
            _ => false, // If either fails, not deterministic
        }
    }
    
    /// Get audit log entries.
    pub fn audit_log(&self) -> Vec<AuditEntry> {
        if let Ok(log) = self.audit_log.read() {
            log.clone()
        } else {
            Vec::new()
        }
    }
    
    /// Clear audit log.
    pub fn clear_audit_log(&self) -> Result<(), FrameworkError> {
        let mut log = self.audit_log
            .write()
            .map_err(|_| FrameworkError::LockError)?;
        log.clear();
        Ok(())
    }
    
    /// Get list of registered domains.
    pub fn registered_domains(&self) -> Vec<Domain> {
        if let Ok(adapters) = self.adapters.read() {
            adapters.keys().copied().collect()
        } else {
            Vec::new()
        }
    }
    
    // -------------------------------------------------------------------------
    // PRIVATE HELPERS
    // -------------------------------------------------------------------------
    
    fn compute_hash(&self, data: &[u8]) -> [u8; 32] {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&result);
        hash
    }
    
    fn compute_unl_hash(&self, unl: &UnlNormalized) -> [u8; 32] {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        
        // Hash tokens
        for token in &unl.tokens {
            hasher.update(token.as_bytes());
        }
        
        // Hash relations
        for rel in &unl.relations {
            hasher.update(rel.from_token.as_bytes());
            hasher.update(rel.to_token.as_bytes());
            hasher.update(rel.relation_type.as_bytes());
        }
        
        let result = hasher.finalize();
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&result);
        hash
    }
    
    fn log_audit(&self, entry: AuditEntry) {
        if let Ok(mut log) = self.audit_log.write() {
            log.push(entry);
        }
    }
    
    fn get_timestamp(&self) -> u64 {
        // In production, use actual timestamp
        // For determinism validation, return 0
        0
    }
}

impl Default for AdapterFramework {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// FRAMEWORK ERROR
// =============================================================================

/// Framework error enumeration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FrameworkError {
    /// Domain not registered
    DomainNotRegistered(Domain),
    
    /// Adapter error
    AdapterError(AdapterError),
    
    /// Lock error (internal)
    LockError,
}

impl std::fmt::Display for FrameworkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DomainNotRegistered(domain) => {
                write!(f, "Domain not registered: {}", domain)
            }
            Self::AdapterError(e) => write!(f, "Adapter error: {}", e),
            Self::LockError => write!(f, "Internal lock error"),
        }
    }
}

impl std::error::Error for FrameworkError {}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapter::{AdapterVersion, SchemaDefinition, AdapterMetadata, Relation};
    
    // Mock adapter for testing
    struct TestAdapter;
    
    impl Adapter for TestAdapter {
        fn adapt(&self, input: &[u8]) -> Result<UnlNormalized, AdapterError> {
            let tokens: Vec<String> = input
                .iter()
                .map(|b| format!("t_{:02x}", b))
                .collect();
            
            let relations: Vec<Relation> = (0..tokens.len() - 1)
                .map(|i| Relation {
                    from_token: tokens[i].clone(),
                    to_token: tokens[i + 1].clone(),
                    relation_type: "seq".to_string(),
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
                    domain: self.domain(),
                    adapter_version: self.version(),
                    timestamp_utc: 0,
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
                name: "Test".to_string(),
                version: "1.0.0".to_string(),
                description: "Test adapter".to_string(),
                fields: vec![],
            }
        }
    }
    
    #[test]
    fn test_framework_creation() {
        let framework = AdapterFramework::new();
        assert_eq!(framework.registered_domains().len(), 0);
    }
    
    #[test]
    fn test_framework_register() {
        let framework = AdapterFramework::new();
        
        framework.register(Domain::Industrial, TestAdapter).unwrap();
        
        assert!(framework.has_adapter(Domain::Industrial));
        assert!(!framework.has_adapter(Domain::Financial));
    }
    
    #[test]
    fn test_framework_adapt() {
        let framework = AdapterFramework::new();
        framework.register(Domain::Industrial, TestAdapter).unwrap();
        
        let input = vec![0xCA, 0xFE];
        let unl = framework.adapt(Domain::Industrial, &input).unwrap();
        
        assert_eq!(unl.tokens.len(), 2);
        assert_eq!(unl.tokens[0], "t_ca");
        assert_eq!(unl.tokens[1], "t_fe");
    }
    
    #[test]
    fn test_framework_domain_not_registered() {
        let framework = AdapterFramework::new();
        
        let input = vec![0x01, 0x02];
        let result = framework.adapt(Domain::Financial, &input);
        
        assert!(result.is_err());
        match result {
            Err(FrameworkError::DomainNotRegistered(Domain::Financial)) => (),
            _ => panic!("Expected DomainNotRegistered error"),
        }
    }
    
    #[test]
    fn test_framework_determinism_validation() {
        let framework = AdapterFramework::new();
        framework.register(Domain::Industrial, TestAdapter).unwrap();
        
        let input = vec![0xDE, 0xAD];
        
        // TestAdapter is deterministic
        assert!(framework.validate_determinism(Domain::Industrial, &input));
    }
    
    #[test]
    fn test_framework_audit_log() {
        let framework = AdapterFramework::with_audit();
        framework.register(Domain::Industrial, TestAdapter).unwrap();
        
        let input = vec![0xBE, 0xEF];
        let _ = framework.adapt(Domain::Industrial, &input);
        
        let log = framework.audit_log();
        assert_eq!(log.len(), 1);
        assert_eq!(log[0].domain, Domain::Industrial);
        assert!(log[0].success);
    }
    
    #[test]
    fn test_framework_multiple_adapters() {
        let framework = AdapterFramework::new();
        
        framework.register(Domain::Industrial, TestAdapter).unwrap();
        framework.register(Domain::Financial, TestAdapter).unwrap();
        
        let domains = framework.registered_domains();
        assert_eq!(domains.len(), 2);
        assert!(domains.contains(&Domain::Industrial));
        assert!(domains.contains(&Domain::Financial));
    }
}
