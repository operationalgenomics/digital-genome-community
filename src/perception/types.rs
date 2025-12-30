//! --------------------------
//! INFORMATION
//! --------------------------
//! Title: Canonical Perception Types
//! Author: Carlos Eduardo Favini
//! Date: 2025-12-30
//! Version: 1.0.0
//! Description: Defines the ontological primitives for raw, unclassified phenomena.
//!              These types are strictly deterministic, platform-agnostic, and
//!              contain no physical time dependencies.
//! Layer: Community / Perception (Phase 1)
//! --------------------------

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

/// Represents a raw, unclassified phenomenon observed by the system.
///
/// This structure is strictly deterministic and platform-agnostic.
/// It contains no physical time (wall-clock) and no floating-point ambiguity.
/// It serves as the canonical input for the Reverse Engineering Motor.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RawPhenomenon {
    /// Unique identifier for this observation event.
    /// Used for traceability across the cognitive pipeline.
    pub id: Uuid,

    /// The raw byte stream of the phenomenon.
    /// The Community does not know what this data represents yet (Audio? Image? Protocol?).
    /// It is simply a sequence of bytes awaiting reverse engineering.
    pub data: Vec<u8>,

    /// The spatiotemporal context of the observation in abstract terms.
    /// Defines "where" and "when" (logically) the data exists.
    pub envelope: SpatiotemporalEnvelope,

    /// Structural context about the source of the data.
    /// Provides metadata about the transducer without classifying the signal type.
    pub source: SourceContext,
}

/// Defines the temporal and spatial boundaries of an observation.
/// strictly uses logical time to ensure replayability and determinism.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SpatiotemporalEnvelope {
    /// Logical timestamp marking the start of the capture.
    /// The meaning is derived from the specialized domain clock, not UTC.
    pub logical_start: LogicalTimestamp,

    /// Duration of the phenomenon expressed in logical ticks.
    /// Ensures duration is measured consistently during replay.
    pub logical_duration: u64,

    /// Structural dimensionality of the data container.
    /// Describes the topology of the data (e.g., linear stream vs grid) without semantic labels.
    pub dimensions: StructuralDimensions,
}

/// Abstract representation of time for deterministic replay.
/// Decouples logical ordering from physical execution time to prevent drift.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct LogicalTimestamp {
    /// The monotonic tick count from the source's epoch.
    pub tick: u128,

    /// Identifier of the clock domain (e.g., a specific sensor's oscillator ID).
    /// Prevents invalid comparisons between unrelated time sources.
    pub domain_id: u64,
}

/// Mathematical shape of the data container.
/// Used to verify structural invariants before processing.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StructuralDimensions {
    /// Vector describing the length of each axis.
    /// Empty vector implies a scalar value.
    /// [N] implies a linear stream.
    /// [X, Y] implies a grid or matrix.
    pub axes: Vec<u64>,
}

/// Context regarding the origin of the signal.
/// Purely structural identifiers; "SignalType" enums are strictly forbidden.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SourceContext {
    /// Unique identifier of the transducer instance that captured the data.
    pub transducer_id: String,

    /// Topology identifier (node ID in the graph) where the transducer is located.
    /// Optional, as some transducers may be transient or unmapped.
    pub node_id: Option<Uuid>,

    /// Resolution of the capture in bits per element (e.g., 8, 16, 24).
    /// Replaces floating point bit-depths to ensure integer precision.
    pub resolution_bits: u32,

    /// Native sampling rate numerator (Ticks Per Second).
    /// Part of the rational representation of frequency.
    pub sample_rate_numerator: u64,
    
    /// Native sampling rate denominator.
    /// Allows representation of fractional rates without floating point errors.
    pub sample_rate_denominator: u64,
    
    /// Static configuration metadata provided by the hardware.
    /// Restricted to Key-Value strings; semantic interpretation happens later.
    pub config_metadata: HashMap<String, String>,
}

/// Minimal structural invariant for indexing and quick comparisons.
/// Cryptographic hashing is deferred to later phases to keep Phase 1 pure.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StructuralInvariant {
    /// Exact byte length of the raw data.
    pub size_bytes: usize,
    
    /// Checksum of dimensions (e.g., product of axes).
    /// Used as a quick sanity check for structural integrity.
    pub total_elements: u64,
}