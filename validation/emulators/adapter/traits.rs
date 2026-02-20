//! Canon v5.1 | GDC v1.0.0β
//!
//! Adapter Traits - Interface canônica para mapeamento X → UNL
//!
//! Canon: LEI-AF-14-01 (Adapter Estrutural Canônico Trans-Kingdom)
//!
//! ## Princípios Fundamentais
//!
//! 1. **Adapter é mapeamento, não cognição**
//! 2. **Determinismo absoluto** (f(x) = f(x) sempre)
//! 3. **Auditabilidade total** (mapeamento explícito)
//! 4. **Sem estado externo** (sem clock, sem rede, sem filesystem)
//!
//! ## Separação de Responsabilidades
//!
//! - Adapter → Traduz (X → UNL)
//! - GDC → Processa (UNL → DNA)
//! - GDE → Persiste (DNA → Storage)
//! - GDO → Coordena (Pipeline)

use serde::{Deserialize, Serialize};
use std::fmt;

// =============================================================================
// DOMAIN - DOMÍNIO DE ORIGEM
// =============================================================================

/// Domain: origem do sinal adaptado.
///
/// Canon: LEI-AF-14-01 §4 - Universalidade Trans-Reino
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Domain {
    /// Industrial IoT (sensores, atuadores, PLCs)
    Industrial,
    
    /// Financial Markets (preços, volumes, transações)
    Financial,
    
    /// Healthcare (sinais vitais, exames, diagnósticos)
    Healthcare,
    
    /// Biological (genômica, proteômica, metabolômica)
    Biological,
    
    /// Custom domain (user-defined)
    Custom(u16),
}

impl fmt::Display for Domain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Industrial => write!(f, "Industrial"),
            Self::Financial => write!(f, "Financial"),
            Self::Healthcare => write!(f, "Healthcare"),
            Self::Biological => write!(f, "Biological"),
            Self::Custom(id) => write!(f, "Custom({})", id),
        }
    }
}

// =============================================================================
// ADAPTER VERSION - VERSIONAMENTO DE ADAPTER
// =============================================================================

/// Adapter version: semantic versioning.
///
/// Canon: LEI-AF-14-01 §2 - Auditabilidade
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AdapterVersion {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}

impl AdapterVersion {
    pub fn new(major: u16, minor: u16, patch: u16) -> Self {
        Self { major, minor, patch }
    }
}

impl fmt::Display for AdapterVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

// =============================================================================
// SCHEMA DEFINITION - DEFINIÇÃO DE SCHEMA
// =============================================================================

/// Schema definition: estrutura esperada do input.
///
/// Canon: LEI-AF-14-01 §2 - Transparência
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchemaDefinition {
    /// Nome do schema
    pub name: String,
    
    /// Versão do schema
    pub version: String,
    
    /// Descrição textual
    pub description: String,
    
    /// Campos esperados
    pub fields: Vec<FieldDefinition>,
}

/// Field definition in schema.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FieldDefinition {
    pub name: String,
    pub field_type: FieldType,
    pub required: bool,
    pub description: String,
}

/// Field type enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FieldType {
    Integer,
    Float,
    String,
    Boolean,
    Timestamp,
    Binary,
}

// =============================================================================
// UNL NORMALIZED - SAÍDA NORMALIZADA
// =============================================================================

/// UNL Normalized: output canônico do Adapter.
///
/// Canon: LEI-AF-14-01 §1 - Mapeamento determinístico
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnlNormalized {
    /// Tokens estruturais (UNL)
    pub tokens: Vec<String>,
    
    /// Relações estruturais
    pub relations: Vec<Relation>,
    
    /// Metadata de origem (apenas auditoria)
    pub metadata: AdapterMetadata,
}

/// Structural relation in UNL.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Relation {
    pub from_token: String,
    pub to_token: String,
    pub relation_type: String,
}

/// Adapter metadata (audit only).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdapterMetadata {
    pub domain: Domain,
    pub adapter_version: AdapterVersion,
    pub timestamp_utc: u64,
    pub source_hash: [u8; 32],
}

// =============================================================================
// ADAPTER ERROR - ERROS DE ADAPTAÇÃO
// =============================================================================

/// Adapter error enumeration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdapterError {
    /// Input inválido (malformed)
    InvalidInput(String),
    
    /// Schema mismatch
    SchemaMismatch(String),
    
    /// Field missing (required)
    MissingField(String),
    
    /// Type conversion failed
    TypeConversionFailed(String),
    
    /// Internal error
    InternalError(String),
}

impl fmt::Display for AdapterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            Self::SchemaMismatch(msg) => write!(f, "Schema mismatch: {}", msg),
            Self::MissingField(field) => write!(f, "Missing required field: {}", field),
            Self::TypeConversionFailed(msg) => write!(f, "Type conversion failed: {}", msg),
            Self::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for AdapterError {}

// =============================================================================
// ADAPTER TRAIT - INTERFACE CANÔNICA
// =============================================================================

/// Adapter trait: mapeamento determinístico X → UNL.
///
/// # Canon: LEI-AF-14-01 §§6-7
///
/// ## Propriedades Obrigatórias
///
/// 1. **Determinismo:** mesma entrada → mesma UNL (sempre)
/// 2. **Auditabilidade:** mapeamento explícito e documentado
/// 3. **Sem heurística oculta:** regras transparentes
/// 4. **Sem injeção de estado externo:** sem clock, rede, filesystem
///
/// ## Exemplo
///
/// ```rust,ignore
/// struct MyAdapter;
///
/// impl Adapter for MyAdapter {
///     fn adapt(&self, input: &[u8]) -> Result<UnlNormalized, AdapterError> {
///         // Parse input
///         let data = parse(input)?;
///         
///         // Map to UNL tokens
///         let tokens = map_to_tokens(data);
///         
///         // Map to relations
///         let relations = map_to_relations(data);
///         
///         Ok(UnlNormalized { tokens, relations, metadata })
///     }
///     
///     fn version(&self) -> AdapterVersion {
///         AdapterVersion::new(1, 0, 0)
///     }
///     
///     fn domain(&self) -> Domain {
///         Domain::Industrial
///     }
///     
///     fn schema(&self) -> SchemaDefinition {
///         SchemaDefinition {
///             name: "MySchema".to_string(),
///             version: "1.0.0".to_string(),
///             description: "My adapter schema".to_string(),
///             fields: vec![],
///         }
///     }
/// }
/// ```
pub trait Adapter: Send + Sync {
    /// Adapt raw input to normalized UNL.
    ///
    /// # Canon: LEI-AF-14-01 §1
    ///
    /// MUST be deterministic: f(x) = f(x) sempre.
    ///
    /// MUST NOT access:
    /// - System clock (time-dependent)
    /// - Network (non-deterministic)
    /// - Filesystem (external state)
    /// - Random generators (non-deterministic)
    fn adapt(&self, input: &[u8]) -> Result<UnlNormalized, AdapterError>;
    
    /// Get adapter version.
    ///
    /// # Canon: LEI-AF-14-01 §2 - Auditabilidade
    fn version(&self) -> AdapterVersion;
    
    /// Get adapter domain.
    ///
    /// # Canon: LEI-AF-14-01 §4 - Trans-Reino
    fn domain(&self) -> Domain;
    
    /// Get adapter schema definition.
    ///
    /// # Canon: LEI-AF-14-01 §2 - Transparência
    fn schema(&self) -> SchemaDefinition;
    
    /// Validate input against schema (optional).
    ///
    /// Default: always valid.
    fn validate(&self, _input: &[u8]) -> Result<(), AdapterError> {
        Ok(())
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_domain_display() {
        assert_eq!(Domain::Industrial.to_string(), "Industrial");
        assert_eq!(Domain::Financial.to_string(), "Financial");
        assert_eq!(Domain::Custom(42).to_string(), "Custom(42)");
    }
    
    #[test]
    fn test_adapter_version_display() {
        let version = AdapterVersion::new(1, 2, 3);
        assert_eq!(version.to_string(), "1.2.3");
    }
    
    #[test]
    fn test_adapter_error_display() {
        let err = AdapterError::InvalidInput("test".to_string());
        assert!(err.to_string().contains("Invalid input"));
    }
}
