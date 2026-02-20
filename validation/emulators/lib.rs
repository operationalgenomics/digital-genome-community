//! GDC Emulators - EXTERNAL TO CANON
//!
//! **CRITICAL NOTICE:**
//! Canon v5.1, linha 6491-6500:
//!
//! > "As camadas superiores (GDO, GDE, e quaisquer outras) **não pertencem
//! > ao Canon do GDC**. Essas camadas existem exclusivamente como emuladores,
//! > geradores de estímulo e simuladores para fins de teste, estresse e
//! > validação do GDC."
//! >
//! > **Proibições:**
//! > * ❌ Inserir regras de camadas externas no Canon
//! > * ❌ Misturar contratos externos com invariantes do GDC
//! > * ❌ Permitir contaminação estrutural entre camadas
//!
//! ## Propósito
//!
//! Este módulo contém **emuladores externos** que NÃO fazem parte do
//! núcleo cognitivo canônico do GDC. São ferramentas de validação,
//! teste e demonstração.
//!
//! ## Componentes
//!
//! - **GDO** - Genoma Digital Orchestrator (emulador de orquestração)
//! - **GDE** - Genoma Digital Educator (emulador de tradução)
//! - **Adapter** - Adaptadores Trans-Kingdom (Community Edition)
//! - **Orchestrator** - Orquestrador de ciclo fechado (validação γ)
//!
//! ## Conformidade Canônica
//!
//! Estes emuladores:
//! - ✅ Estão FORA do repositório Core (validation/)
//! - ✅ NÃO contaminam estrutura canônica
//! - ✅ São governados por regras diferentes
//! - ✅ Podem usar tecnologias não-canônicas (JSON, etc.)
//!
//! ## Uso
//!
//! ```rust
//! use gdc_emulators::gdo::GdoOrchestrator;
//! use gdc_emulators::gde::GdeEducator;
//! use gdc_emulators::adapter::{IndustrialAdapter, FinancialAdapter};
//! ```

pub mod gdo;
pub mod gde;
pub mod adapter;
pub mod orchestrator;
pub mod swarm;

// Re-exports para conveniência
pub use gdo::{GdoOrchestrator, StimulusGenerator};
pub use gde::{GdeEducator, DnaStorage, DnaRecord};
pub use adapter::{
    Adapter, AdapterFramework,
    IndustrialAdapter, FinancialAdapter,
    Domain, UnlNormalized,
};
pub use orchestrator::CycleOrchestrator;
