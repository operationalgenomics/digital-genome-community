//! Canon v5.1 | GDC v0.9.0-final
//!
//! Networking module - Wire protocol entre GDCs
//!
//! Canon:
//! - LEI-QMN-BORDA-01/02
//! - LEI-AO-24-04

pub mod border;
pub mod protocol;
pub mod transport;

pub use border::{BorderValidator, BorderRejection, HandshakeValidator};
pub use protocol::{ProtocolMessage, ProtocolCodec};
pub use transport::{Transport, TransportError, InMemoryTransport};
