//! Canon v5.1 | GDC v1.0.0γ
//!
//! GDE (Genoma Digital Educator) - Emulador Externo
//!
//! ## CRITICAL NOTE
//! GDE é um EMULADOR EXTERNO ao core cognitivo.
//! Canon governa apenas a interface UNL ↔ Humano.
//!
//! v1.0.0γ: Adiciona persistência real (storage)

pub mod educator;
pub mod bridge;
pub mod storage;

pub use educator::GdeEducator;
pub use bridge::{UnlBridge, BridgeError};
pub use storage::{DnaRecord, DnaStorage, StorageError};
