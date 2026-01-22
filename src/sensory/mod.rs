//! --------------------------
//! INFORMATION
//! --------------------------
//! Title: Sensory Module
//! Author: Carlos Eduardo Favini
//! Date: 2025-01-02
//! Version: 1.2.0
//! Description: The sensory cortex of the Digital Genome.
//! Processes raw input through abstraction levels 0 → 1 → 2 → 2.5.
//! ONLY mathematics. ZERO interpretation.
//! Layer: Community
//! Dependencies: serde, rustfft
//! Affected Components: motors, output
//!
//! --------------------------
//! ABSTRACTION HIERARCHY
//! --------------------------
//! Level 0: Carrier (Raw Signal)
//!   - Entropy, basic statistics
//!   - "What is the signal?"
//!
//! Level 1: Pattern (Repetition, Rhythm)
//!   - Autocorrelation, periodicity
//!   - "Does it repeat?"
//!
//! Level 2: Structure (Organization)
//!   - Local vs global entropy, compressibility
//!   - "Is it organized?"
//!
//! Level 2.5: Proto-Agency (Suspected Intentionality)
//!   - NOT a classification - a STATE TRANSITION
//!   - Triggered by mathematical conditions
//!   - "Does it seem intentional?"
//!
//! Level 3: Semantics (NOT IN COMMUNITY)
//!   - Requires Enterprise Edition
//!   - "What does it mean?"
//!
//! --------------------------
//! CHANGE LOG
//! --------------------------
//! 2025-01-02 - Carlos Eduardo Favini - Initial creation (v1.1.0)
//! --------------------------

/// Perceptual states and transitions
pub mod state;

/// Sensory signals (pure mathematics)
pub mod signals;

/// Level 0: Carrier analysis
pub mod carrier;

/// Level 1: Pattern analysis
pub mod pattern;

/// Level 2: Structure analysis
pub mod structure;

/// Level 2.5: Proto-Agency detection
pub mod proto_agency;

/// Sensory cortex pipeline
pub mod cortex;

/// Community output structure
pub mod output;

// Re-exports
pub use cortex::{CortexOutput, MatureOutput, RawInput, SensoryCortex};
pub use output::{CommunityOutput, CommunityOutputBuilder};
pub use signals::SensorySignals;
pub use state::{PerceptualState, ProtoAgencyTrigger, StateHistory, StateTransition};
