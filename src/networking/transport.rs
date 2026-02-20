//! Canon v5.1 | GDC v0.9.0-final
//!
//! Transport - Transporte agnóstico
//!
//! Canon: LEI-AO-24-04
//!
//! ## Core Principle
//! Transport is external to Canon.
//! Camada 3 nunca influencia camadas 1/2 cognitivas.

use super::protocol::ProtocolMessage;

/// Transport abstraction.
///
/// Canon: AO-24 - GDC não decide topologia
pub trait Transport: Send + Sync {
    /// Send message to endpoint.
    fn send(&self, endpoint: &str, message: &ProtocolMessage) -> Result<(), TransportError>;
    
    /// Receive message from endpoint.
    fn receive(&self, endpoint: &str) -> Result<Option<ProtocolMessage>, TransportError>;
}

/// Transport errors (external to Canon).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransportError {
    /// Connection failed
    ConnectionFailed,
    
    /// Timeout
    Timeout,
    
    /// Invalid endpoint
    InvalidEndpoint,
}

/// In-memory transport for testing.
pub struct InMemoryTransport {
    // Simplified for v0.9.0
}

impl InMemoryTransport {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for InMemoryTransport {
    fn default() -> Self {
        Self::new()
    }
}

impl Transport for InMemoryTransport {
    fn send(&self, _endpoint: &str, _message: &ProtocolMessage) -> Result<(), TransportError> {
        // Placeholder
        Ok(())
    }
    
    fn receive(&self, _endpoint: &str) -> Result<Option<ProtocolMessage>, TransportError> {
        // Placeholder
        Ok(None)
    }
}
