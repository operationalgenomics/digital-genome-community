use thiserror::Error;
use crate::perception::types::RawPhenomenon;

/// Error types related to perception interfaces.
#[derive(Debug, Error)]
pub enum InterfaceError {
    #[error("Channel unavailable: {0}")]
    ChannelUnavailable(String),
    #[error("Protocol violation: {0}")]
    ProtocolViolation(String),
    #[error("Resource exhausted")]
    ResourceExhausted,
}

/// Defines the contract for a human interface channel.
/// Pure synchronous definition. Implementation (Enterprise) handles IO/Async.
pub trait HumanInterfaceChannel: Send + Sync {
    /// Proposes a message to a recipient.
    /// Returns a tracking ID immediately, does not await delivery.
    fn propose_message(&self, recipient_id: &str, content: &str) -> Result<String, InterfaceError>;

    /// Checks the status of a previous request.
    fn check_approval(&self, request_id: &str) -> Result<ApprovalStatus, InterfaceError>;
}

/// Status of a human interaction request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApprovalStatus {
    Pending,
    Approved,
    Rejected,
    Unknown,
}

/// Defines the contract for a transducer that produces raw phenomena.
/// The trait is pure; the implementation manages the hardware buffer.
pub trait PhenomenonSource: Send + Sync {
    /// returns the next available phenomenon from the buffer, if any.
    /// Non-blocking.
    fn poll_next(&mut self) -> Result<Option<RawPhenomenon>, InterfaceError>;
    
    /// Returns the unique ID of this transducer.
    fn transducer_id(&self) -> &str;
}