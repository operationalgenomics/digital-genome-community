use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

/// Represents a raw, unclassified phenomenon observed by the system.
///
/// This structure is strictly deterministic and platform-agnostic.
/// It contains no physical time (wall-clock) and no floating-point ambiguity.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RawPhenomenon {
    /// Unique identifier for this observation event.
    pub id: Uuid,

    /// The raw byte stream of the phenomenon.
    /// The Community does not know what this data represents yet.
    pub data: Vec<u8>,

    /// Where and when this phenomenon was observed (in logical time).
    pub envelope: SpatiotemporalEnvelope,

    /// Structural context about the source.
    pub source: SourceContext,
}

/// Defines the temporal and spatial boundaries of an observation in abstract terms.
/// No wall-clock time allowed.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SpatiotemporalEnvelope {
    /// Logical timestamp of capture start.
    /// Meaning is derived from the specialized domain clock, not UTC.
    pub logical_start: LogicalTimestamp,

    /// Duration in logical ticks.
    pub logical_duration: u64,

    /// Structural dimensionality of the data (e.g., [1024] for stream, [640, 480] for grid).
    /// Replaces semantic strings like "1D" or "2D".
    pub dimensions: StructuralDimensions,
}

/// Abstract representation of time for deterministic replay.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct LogicalTimestamp {
    /// The tick count.
    pub tick: u128,

    /// Identifier of the clock domain (e.g., a specific sensor's oscillator id).
    /// Allows correlation only between compatible domains.
    pub domain_id: u64,
}

/// Mathematical shape of the data container.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StructuralDimensions {
    /// Length of each axis.
    /// Empty = scalar.
    /// [N] = linear.
    /// [X, Y] = grid.
    pub axes: Vec<u64>,
}

/// Context regarding the origin of the signal.
/// Purely structural identifiers, no "SignalType" enums.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SourceContext {
    /// Unique identifier of the transducer instance.
    pub transducer_id: String,

    /// Topology identifier (node ID in the graph).
    pub node_id: Option<Uuid>,

    /// Resolution of the capture in bits per element (e.g., 8, 16, 24).
    /// Replaces floating point bit-depths.
    pub resolution_bits: u32,

    /// Native sampling rate in Ticks Per Second (integer).
    /// If fractional rates are needed, Enterprise must normalize or use rational encoding.
    pub sample_rate_numerator: u64,
    pub sample_rate_denominator: u64,
    
    /// Static configuration metadata (Key-Value strings only).
    pub config_metadata: HashMap<String, String>,
}

/// Minimal structural invariant for indexing.
/// No cryptographic hashing (prohibited in Phase 1).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StructuralInvariant {
    /// Exact byte length.
    pub size_bytes: usize,
    
    /// Checksum of dimensions (e.g., product of axes).
    pub total_elements: u64,
}