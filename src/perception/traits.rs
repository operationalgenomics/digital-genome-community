//! --------------------------
//! INFORMATION
//! --------------------------
//! Title: Canonical Perception Traits
//! Author: Carlos Eduardo Favini
//! Date: 2025-12-30
//! Version: 1.0.0
//! Description: Defines pure synchronous contracts for signal acquisition and
//!              human interaction. Implementations reside in Enterprise.
//! Layer: Community / Perception (Phase 1)
//! --------------------------

use thiserror::Error;
use crate::perception::types::{RawPhenomenon, ApprovalStatus};

/// Error types related to perception interface failures.
#[derive(Debug, Error)]
pub enum InterfaceError {
    /// The requested communication channel is not available.
    #[error("Channel unavailable: {0}")]
    ChannelUnavailable(String),
    
    /// The interaction violated the established protocol.
    #[error("Protocol violation: {0}")]
    ProtocolViolation(String),
    
    /// The underlying system resources are exhausted.
    #[error("Resource exhausted")]
    ResourceExhausted,
    
    /// The source has no more data to provide.
    #[error("Source depleted")]
    SourceDepleted,
}

/// Status of a human interaction request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApprovalStatus {
    /// The request has been sent but not yet answered.
    Pending,
    /// The request was explicitly approved by a human.
    Approved,
    /// The request was explicitly rejected by a human.
    Rejected,
    /// The status cannot be determined.
    Unknown,
}

/// Defines the contract for a human interface channel.
/// Pure synchronous definition. The Enterprise implementation handles IO/Async/Scheduling.
pub trait HumanInterfaceChannel: Send + Sync {
    /// Proposes a message to a human recipient.
    /// Returns a tracking ID immediately; does not await delivery or reading.
    fn propose_message(&self, recipient_id: &str, content: &str) -> Result<String, InterfaceError>;

    /// Checks the status of a previous request.
    /// Must be non-blocking and return the current known state.
    fn check_approval(&self, request_id: &str) -> Result<ApprovalStatus, InterfaceError>;
}

/// Defines the contract for a transducer that produces raw phenomena.
/// The trait is pure; the implementation manages the hardware buffer and interrupts.
pub trait PhenomenonSource: Send + Sync {
    /// Returns the next available phenomenon from the buffer, if any.
    /// Must be non-blocking to ensure the cognitive loop never stalls.
    fn poll_next(&mut self) -> Result<Option<RawPhenomenon>, InterfaceError>;
    
    /// Returns the unique identifier of this transducer instance.
    fn transducer_id(&self) -> &str;
}