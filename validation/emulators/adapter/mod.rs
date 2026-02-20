//! Canon v5.1 | GDC v1.0.0β
//!
//! Adapter Module - Trans-Kingdom Learning
//!
//! Canon: LEI-AF-14-01 (Adapter Estrutural Canônico Trans-Kingdom)
//!
//! ## Propósito
//!
//! Demonstrar universalidade do GDC através de Adapters determinísticos
//! que mapeiam sinais de domínios diferentes para UNL normalizada.
//!
//! ## Componentes
//!
//! - `traits` - Interface canônica Adapter
//! - `framework` - Engine de adaptação
//! - `industrial` - Adapter para IoT industrial
//! - `financial` - Adapter para mercados financeiros
//! - `validation` - Validação de determinismo (próximo)
//!
//! ## Princípios
//!
//! 1. Adapter traduz, não processa
//! 2. GDC é agnóstico à origem do sinal
//! 3. Determinismo é inviolável
//! 4. Auditabilidade é obrigatória

pub mod traits;
pub mod framework;
pub mod industrial;
pub mod financial;

// Re-exports
pub use traits::{
    Adapter,
    AdapterError,
    AdapterMetadata,
    AdapterVersion,
    Domain,
    FieldDefinition,
    FieldType,
    Relation,
    SchemaDefinition,
    UnlNormalized,
};

pub use framework::{
    AdapterFramework,
    AuditEntry,
    FrameworkError,
};

pub use industrial::{
    IndustrialAdapter,
    IndustrialSensorData,
};

pub use financial::{
    FinancialAdapter,
    FinancialMarketData,
};
